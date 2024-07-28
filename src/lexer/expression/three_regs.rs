// Example : mov r0, r1, r2

use crate::token::register::Register;

use super::barrel_shifter::BarrelShifterExpression;

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
}
