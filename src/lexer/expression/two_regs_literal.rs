// example: add r0 r1 #0x1234

use crate::token::{immediate::Immediate, register::Register};

pub struct TwoRegsLiteralExpression {
    pub reg_d: Register,
    pub reg_m: Register,
    pub literal: Immediate,
}
