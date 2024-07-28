use crate::token::{immediate::Immediate, register::Register};
// Determines wether pre os post indexing is used

#[derive(Debug)]
pub struct PreIndex {
    pub write_back: bool,
}

#[derive(Debug)]
pub enum IndexMode {
    Pre(PreIndex),
    Post,
    None,
}

#[derive(Debug)]
pub struct LoadStoreImmediateExpression {
    pub destination: Register,
    pub base: Register,
    pub offset: Option<Immediate>,
    pub index_mode: IndexMode,
}

impl LoadStoreImmediateExpression {
    pub fn new(
        destination: Register,
        register: Register,
        offset: Option<Immediate>,
        index_mode: IndexMode,
    ) -> Self {
        match index_mode {
            IndexMode::Pre(_) => Self {
                destination,
                base: register,
                offset,
                index_mode,
            },
            IndexMode::Post | IndexMode::None => Self {
                destination,
                base: register,
                offset,
                index_mode,
            },
        }
    }
}
