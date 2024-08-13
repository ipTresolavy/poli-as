use object::elf::ELFOSABI_SYSV;
use object::elf::EM_ARM;
use object::elf::ET_REL;
use object::elf::EV_CURRENT;
// TODO: checar como vai ser a configuração das seções de texto e dados (little-endia enforçado
// pela lib ou por nós?)
use object::elf::SHF_ALLOC;
use object::elf::SHF_EXECINSTR;
use object::elf::SHF_INFO_LINK;
use object::elf::SHF_MERGE;
use object::elf::SHF_STRINGS;
use object::elf::SHF_WRITE;
use object::elf::SHT_DYNSYM;
use object::elf::SHT_NOBITS;
use object::elf::SHT_NULL;
use object::elf::SHT_PROGBITS;
use object::elf::SHT_REL;
use object::elf::SHT_RELA;
use object::elf::SHT_STRTAB;
use object::elf::SHT_SYMTAB;
use object::write::elf::FileHeader;
use object::write::elf::SectionHeader;
use object::write::elf::SectionIndex;
use object::write::elf::SymbolIndex;
use object::write::elf::Writer;
use object::write::StringId;
use object::write::{Object, StreamingBuffer};
use object::U32;
use object::{Architecture, Endianness};
use std::fs::File;
use std::io::BufWriter;

use super::section_data::IntermediateSectionId;
use super::section_data::SectionData;

#[derive(Debug)]
pub struct ElfWriter {
    sections: Vec<(IntermediateSectionId, String, SectionHeader, SectionData)>,
}

impl Clone for ElfWriter {
    fn clone(&self) -> ElfWriter {
        ElfWriter {
            sections: self.sections.clone(),
        }
    }
}

impl ElfWriter {
    pub fn new() -> ElfWriter {
        let object = Object::new(
            object::BinaryFormat::Elf,
            Architecture::Arm,
            Endianness::Little,
        );

        ElfWriter {
            sections: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_section(&mut self, sh_name: String, data: SectionData) -> IntermediateSectionId {
        let sh_type = match sh_name.as_str() {
            ".text" | ".data" | ".rodata" | ".comment" => SHT_PROGBITS,
            ".bss" => SHT_NOBITS,
            ".ARM.attributes" => 0x70000003,
            ".strtab" | ".shstrtab" => SHT_STRTAB,
            ".symtab" => SHT_SYMTAB,
            s if s.starts_with(".rel") => SHT_REL,
            s if s.starts_with(".debug") => SHT_PROGBITS,
            _ => SHT_NULL,
        };

        let sh_flags = match sh_name.as_str() {
            ".text" => SHF_ALLOC | SHF_EXECINSTR,
            ".data" | ".bss" => SHF_WRITE | SHF_ALLOC,
            ".rodata" => SHF_ALLOC,
            ".debug_str" | ".comment" => SHF_MERGE | SHF_STRINGS,
            s if s.starts_with(".rel") => SHF_INFO_LINK,
            _ => 0,
        };

        let sh_addralign = match sh_name.as_str() {
            ".text" | ".bss" | ".rodata" | ".debug_frame" | ".symtab" => 0x4,
            ".data" | ".comment" | ".strtab" | ".shstrtab" => 0x1,
            ".ARM.attributes" => 0x1,
            s if s.starts_with(".rel") => 0x4,
            s if s.starts_with(".debug") => 0x1,
            _ => 0,
        };

        let sh_entsize = if sh_name.eq(".debug_str") | sh_name.eq(".comment") {
            0x1
        } else {
            match sh_type {
                SHT_REL => 0x8,
                SHT_SYMTAB => 0x10,
                _ => 0,
            }
        };

        let section_header = SectionHeader {
            name: None, // must be set later
            sh_type,
            sh_flags: sh_flags as u64,
            sh_addr: 0,
            sh_offset: 0, // must be set later
            sh_size: data.len() as u64,
            sh_link: 0, // must be set later
            sh_info: 0, // must be set later
            sh_addralign,
            sh_entsize,
        };

        let section_id;
        if sh_name.starts_with(".rel") {
            if let Some(pos) = self
                .sections
                .iter()
                .position(|(_, name, _, _)| sh_name.ends_with(name.as_str()))
            {
                section_id = pos + 1;
                self.sections
                    .insert(pos + 1, (section_id, sh_name, section_header, data));
            } else {
                section_id = self.sections.len();
                self.sections
                    .push((section_id, sh_name, section_header, data));
            }
        } else {
            section_id = self.sections.len();
            self.sections
                .push((section_id, sh_name, section_header, data));
        }
        section_id
    }

    #[must_use]
    fn reserve_section_indexes<'b>(
        &'b mut self,
        writer: &mut Writer<'b>,
    ) -> (
        Vec<Option<StringId>>,
        Vec<SectionIndex>,
        Vec<StringId>,
        Vec<SymbolIndex>,
    ) {
        writer.require_strtab();

        let mut section_name_indexes: Vec<Option<StringId>> = Vec::new();
        let mut section_indexes: Vec<SectionIndex> = Vec::new();
        let mut string_indexes: Vec<StringId> = Vec::new();
        let mut symbol_indexes: Vec<SymbolIndex> = Vec::new();

        section_indexes.push(writer.reserve_null_section_index());
        //writer.write_null_section_header();

        for section in &mut self.sections {
            match section.2.sh_type {
                SHT_SYMTAB => {
                    // must be the last section
                    section_name_indexes.push(None);
                    section_indexes.push(writer.reserve_symtab_section_index());
                    symbol_indexes.push(writer.reserve_null_symbol_index());
                    if let SectionData::Symbols(sym_vec) = &mut section.3 {
                        for sym in sym_vec {
                            let symbol_name_index = writer.add_string(sym.1.as_bytes());
                            sym.3.name = Some(symbol_name_index);
                            string_indexes.push(symbol_name_index);
                            symbol_indexes.push(writer.reserve_symbol_index(None));
                            if !sym.2 {
                                sym.3.section = Some(section_indexes[sym.0 + 1]);
                                sym.3.st_shndx = section_indexes[sym.0 + 1].0 as u16;
                            }
                        }
                    }
                }
                _ => {
                    let section_name_index = writer.add_section_name(section.1.as_bytes());
                    section.2.name = Some(section_name_index);
                    section_name_indexes.push(Some(section_name_index));
                    section_indexes.push(writer.reserve_section_index());
                }
            };
        }

        section_name_indexes.push(None);
        section_indexes.push(writer.reserve_strtab_section_index());
        section_name_indexes.push(None);
        section_indexes.push(writer.reserve_shstrtab_section_index());
        (
            section_name_indexes,
            section_indexes,
            string_indexes,
            symbol_indexes,
        )
    }

    fn solve_dependencies(
        &mut self,
        writer: &mut Writer,
        section_indexes: Vec<SectionIndex>,
        symbol_indexes: Vec<SymbolIndex>,
    ) -> bool {
        let mut updates = Vec::new();

        // First pass: Gather information with immutable borrow
        for (i, section) in self.sections.clone().iter().enumerate() {
            if section.2.sh_type == SHT_REL || section.2.sh_type == SHT_RELA {
                if let Some(target_section_name) = section.1.strip_prefix(".rel") {
                    if let Some((target_index, _)) = self
                        .sections
                        .iter()
                        .enumerate()
                        .find(|(_, (_, name, _, _))| name == target_section_name)
                    {
                        updates.push((
                            i,
                            writer.symtab_index().0,
                            section_indexes[target_index + 1].0,
                        ));
                    }
                }
            } else if section.2.sh_type == SHT_SYMTAB || section.2.sh_type == SHT_DYNSYM {
                // Find the associated string table section
                let sh_link = section_indexes[section_indexes.len() - 2].0 as u32;

                // For SHT_SYMTAB, calculate sh_info based on the last local symbol
                let sh_info = if section.2.sh_type == SHT_SYMTAB {
                    if let SectionData::Symbols(symbols) = &section.3 {
                        let last_local_index =
                            symbols.iter().rposition(|(_, _, is_local, _)| *is_local);
                        if let Some(last_local_index) = last_local_index {
                            last_local_index + 1
                        } else {
                            1 // No local symbols, so sh_info should be 1
                        }
                    } else {
                        1 // No local symbols, so sh_info should be 1
                    }
                } else {
                    0 // For SHT_DYNSYM, sh_info is not used in the same way
                };

                updates.push((i, sh_link, sh_info as u32));
            }
        }

        // Second pass: Apply the gathered information with mutable borrow
        for (i, sh_link, sh_info) in updates {
            let section = &mut self.sections[i];
            section.2.sh_link = sh_link;
            section.2.sh_info = sh_info;
            if let SectionData::RelocationEntries(rel_ents) = &mut section.3 {
                for rel_ent in rel_ents {
                    if !rel_ent.1 {
                        rel_ent.2.r_sym = symbol_indexes[rel_ent.0 + 1].0 as u32;
                    }
                }
            }
        }

        true
    }

    fn reserve_ranges(&mut self, writer: &mut Writer) {
        writer.reserve_file_header();
        writer.reserve_section_headers();
        writer.reserve_symtab();
        writer.reserve_strtab();
        writer.reserve_shstrtab();
        for section in &mut self.sections {
            if section.2.sh_type == SHT_REL || section.2.sh_type == SHT_RELA {
                section.2.sh_offset = writer
                    .reserve_relocations(section.3.len(), section.2.sh_type == SHT_RELA)
                    as u64;
            }
        }
    }

    fn write_file_header(&self, writer: &mut Writer) -> bool {
        let file_header = FileHeader {
            os_abi: ELFOSABI_SYSV,
            abi_version: EV_CURRENT,
            e_type: ET_REL,
            e_machine: EM_ARM,
            e_entry: 0,
            e_flags: 0x5000000,
        };

        if let Err(e) = writer.write_file_header(&file_header) {
            print!("Error while writing file header: {}", e);
            return false;
        }

        true
    }
}
