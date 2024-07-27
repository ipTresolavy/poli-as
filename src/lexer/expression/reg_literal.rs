// Example : mov r0, #0x1234

use crate::token::{immediate::Immediate, register::Register};

pub struct RegLiteralExpression {
    pub register: Register,
    pub literal: Immediate,
}
