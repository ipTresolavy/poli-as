// Example : mov r0, r1, r2

use crate::token::register::Register;

use super::barrel_shifter::BarrelShifterExpression;

#[derive(Debug, Copy, Clone)]
pub struct ThreeRegsExpression {
    pub reg_d: Register,
    pub reg_m: Register,
    pub reg_n: Register,
    pub barrel_shifter: Option<BarrelShifterExpression>,
}

impl ThreeRegsExpression {
    pub fn new(
        reg_d: Register,
        reg_m: Register,
        reg_n: Register,
        barrel_shifter: Option<BarrelShifterExpression>,
    ) -> Self {
        Self {
            reg_d,
            reg_m,
            reg_n,
            barrel_shifter,
        }
    }

    pub fn to_machine_code(&self) -> u32 {
        let reg_d = self.reg_d.to_num() as u32;
        let reg_m = self.reg_m.to_num() as u32;
        let reg_n = self.reg_n.to_num() as u32;
        let barrel_shifter = self
            .barrel_shifter
            .map(|bs| bs.to_machine_code())
            .unwrap_or(0);

        (reg_d << 12) | (reg_m << 16) | reg_n | barrel_shifter
    }
}
