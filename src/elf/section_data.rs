use object::write::elf::Rel;
use object::write::elf::Sym;

#[derive(Debug)]
pub enum SectionData {
    Bytes(Vec<u8>),
    Symbols(Vec<(String, bool, Sym)>),
    RelocationEntries(Vec<(String, bool, Rel)>),
}

impl SectionData {
    pub fn len(&self) -> usize {
        match self {
            SectionData::Bytes(v) => v.len(),
            SectionData::RelocationEntries(v) => v.len(),
            SectionData::Symbols(v) => v.len(),
        }
    }
}
