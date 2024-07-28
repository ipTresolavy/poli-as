// example: add r0 r1 #0x1234

use crate::token::{immediate::Immediate, register::Register};

use super::barrel_shifter::BarrelShifterExpression;

#[derive(Debug)]
pub struct TwoRegsLiteralExpression {
    pub reg_d: Register,
    pub reg_m: Register,
    pub literal: Immediate,
    pub barrel_shifter: Option<BarrelShifterExpression>,
}

impl TwoRegsLiteralExpression {
    pub fn new(
        reg_d: Register,
        reg_m: Register,
        literal: Immediate,
        barrel_shifter: Option<BarrelShifterExpression>,
    ) -> Self {
        Self {
            reg_d,
            reg_m,
            literal,
            barrel_shifter,
        }
    }
}
