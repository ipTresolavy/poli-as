use crate::token::register::Register;

use super::{barrel_shifter::BarrelShifterExpression, ls_imm_index::IndexMode};
// Determines wether pre os post indexing is used

#[derive(Debug, Copy, Clone)]
pub struct LoadStoreRegisterExpression {
    pub destination: Register,
    pub base: Register,
    pub negative: bool,
    pub offset: Register,
    pub index_mode: IndexMode,
    pub barrel_shifter: Option<BarrelShifterExpression>,
}

impl LoadStoreRegisterExpression {
    pub fn new(
        destination: Register,
        register: Register,
        offset: Register,
        negative: bool,
        index_mode: IndexMode,
        barrel_shifter: Option<BarrelShifterExpression>,
    ) -> Self {
        match index_mode {
            IndexMode::Pre(_) => Self {
                destination,
                base: register,
                offset,
                negative,
                index_mode,
                barrel_shifter,
            },
            IndexMode::Post | IndexMode::None => Self {
                destination,
                base: register,
                offset,
                negative,
                index_mode,
                barrel_shifter,
            },
        }
    }
}
