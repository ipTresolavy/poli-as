use object::write::elf::Rel;
use object::write::elf::Sym;

#[derive(Debug)]
pub enum SectionData {
    Bytes(Vec<u8>),
    Symbols(Vec<(usize, String, bool, Sym)>),
    RelocationEntries(Vec<(usize, bool, Rel)>),
}

impl SectionData {
    pub fn len(&self) -> usize {
        match self {
            SectionData::Bytes(v) => v.len(),
            SectionData::Symbols(v) => v.len(),
            SectionData::RelocationEntries(v) => v.len(),
        }
    }

    pub fn add_symbol(
        &mut self,
        section_id: usize,
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
            name: None,
            section: None,
            st_info,
            st_other: 0,
            st_shndx: st_shndx.unwrap_or(0),
            st_value: st_value as u64,
            st_size: st_size as u64,
        };

        vec.push((section_id, symbol_name, st_shndx.is_some(), sym));
        Some(vec.len() - 1)
    }

    pub fn add_relocation_entry(
        &mut self,
        referenced_symbol_id: usize,
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
            r_sym: r_sym.unwrap_or(0),
            r_type,
            r_addend: 0,
        };

        vec.push((referenced_symbol_id, r_sym.is_some(), rel_ent));
        true
    }
}
