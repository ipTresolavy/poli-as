use object::elf::SHF_ALLOC;
use object::elf::SHF_EXECINSTR;
use object::elf::SHF_INFO_LINK;
use object::elf::SHF_MERGE;
use object::elf::SHF_STRINGS;
use object::elf::SHF_WRITE;
use object::elf::SHT_NOBITS;
use object::elf::SHT_NULL;
use object::elf::SHT_PROGBITS;
use object::elf::SHT_REL;
use object::elf::SHT_STRTAB;
use object::elf::SHT_SYMTAB;
use object::write::elf::SectionHeader;
use object::write::{Object, StreamingBuffer};
use object::{Architecture, Endianness};
use std::fs::File;
use std::io::BufWriter;

use super::section_data::SectionData;

#[derive(Debug)]
pub struct ElfWriter<'a> {
    object: Object<'a>,
    streaming_buffer: StreamingBuffer<BufWriter<File>>,
    section_headers: Vec<(usize, String, SectionHeader, SectionData)>,
}

impl<'a> ElfWriter<'a> {
    pub fn new(file_name: String) -> ElfWriter<'a> {
        let endianess = Endianness::Little;
        let object = Object::new(object::BinaryFormat::Elf, Architecture::Arm, endianess);
        let file = File::create(file_name).expect("Was not able to create output file");
        let buf_writer = BufWriter::new(file);
        let streaming_buffer = StreamingBuffer::new(buf_writer);

        ElfWriter {
            object,
            streaming_buffer,
            section_headers: vec![],
        }
    }

    pub fn add_section(&mut self, section_id: usize, sh_name: String, data: SectionData) {
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
            name: None, // will be set later
            sh_type,
            sh_flags: sh_flags as u64,
            sh_addr: 0,
            sh_offset: 0, // will be set later
            sh_size: data.len() as u64,
            sh_link: 0, // will be set later
            sh_info: 0, // will be set later
            sh_addralign,
            sh_entsize,
        };

        if sh_name.starts_with(".rel") {
            if let Some(pos) = self
                .section_headers
                .iter()
                .position(|(_, name, _, _)| sh_name.ends_with(name.as_str()))
            {
                self.section_headers
                    .insert(pos + 1, (section_id, sh_name, section_header, data));
            } else {
                self.section_headers
                    .push((section_id, sh_name, section_header, data));
            }
        } else {
            self.section_headers
                .push((section_id, sh_name, section_header, data));
        }
    }
}
