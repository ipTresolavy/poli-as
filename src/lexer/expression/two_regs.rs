// mov r0, r0

use crate::token::register::Register;

#[derive(Debug)]
pub struct TwoRegsExpression {
    pub reg_d: Register,
    pub reg_m: Register,
}

impl TwoRegsExpression {
    pub fn new(reg_d: Register, reg_m: Register) -> Self {
        Self { reg_d, reg_m }
    }
}
