use object::write::elf::Class;
use object::write::elf::Rel;
use object::write::elf::Sym;

pub type IntermediateSectionId = usize;
pub type ReferencedSymbolId = usize;

#[derive(Debug, Clone)]
pub enum SectionData {
    Bytes(Vec<u8>),
    Symbols(Vec<(IntermediateSectionId, String, bool, Sym)>),
    RelocationEntries(Vec<(ReferencedSymbolId, bool, Rel)>),
}

impl SectionData {
    pub fn len(&self) -> usize {
        let class = Class { is_64: false };
        match self {
            SectionData::Bytes(v) => v.len(),
            SectionData::Symbols(v) => v.len() * class.sym_size(),
            SectionData::RelocationEntries(v) => v.len() * class.rel_size(false),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add_symbol(
        &mut self,
        section_id: IntermediateSectionId,
        symbol_name: String,
        st_value: u32,
        st_size: u32,
        st_info: u8,
        st_shndx: Option<u16>,
    ) -> Option<usize> {
        let vec = match self {
            SectionData::Symbols(v) => v,
            _ => return None,
        };
        let sym = Sym {
            name: None,    // must be set later
            section: None, // must be set later, if st_shndx is None
            st_info,
            st_other: 0,
            st_shndx: st_shndx.unwrap_or(0), // must be set later, if section is None
            st_value: st_value as u64,
            st_size: st_size as u64,
        };

        vec.push((section_id, symbol_name, st_shndx.is_some(), sym));
        Some(vec.len() - 1)
    }

    pub fn add_relocation_entry(
        &mut self,
        referenced_symbol_id: ReferencedSymbolId,
        r_offset: u32,
        r_sym: Option<u32>,
        r_type: u32,
    ) -> bool {
        let vec = match self {
            SectionData::RelocationEntries(v) => v,
            _ => return false,
        };

        let rel_ent = Rel {
            r_offset: r_offset as u64,
            r_sym: r_sym.unwrap_or(0), // must be set later if None
            r_type,
            r_addend: 0,
        };

        vec.push((referenced_symbol_id, r_sym.is_some(), rel_ent));
        true
    }
}
