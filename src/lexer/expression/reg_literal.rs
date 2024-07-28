// Example : mov r0, #0x1234

use crate::token::{immediate::Immediate, register::Register};

#[derive(Debug)]
pub struct RegLiteralExpression {
    pub register: Register,
    pub literal: Immediate,
}

impl RegLiteralExpression {
    pub fn new(register: Register, literal: Immediate) -> Self {
        Self { register, literal }
    }
}
