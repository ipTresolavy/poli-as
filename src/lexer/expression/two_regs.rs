// mov r0, r0

use crate::token::register::Register;

#[derive(Debug, Copy, Clone)]
pub struct TwoRegsExpression {
    pub reg_d: Register,
    pub reg_m: Register,
}

impl TwoRegsExpression {
    pub fn new(reg_d: Register, reg_m: Register) -> Self {
        Self { reg_d, reg_m }
    }

    pub fn to_machine_code(&self) -> u32 {
        let reg_d = self.reg_d.to_num() as u32;
        let reg_m = self.reg_m.to_num() as u32;
        (reg_d << 12) | (reg_m)
    }
}
