use crate::token::instruction::Instruction;

use super::expression::Expression;

#[derive(Debug)]
pub struct CpuOperation {
    pub instruction: Instruction,
    pub expression: Expression,
}

impl CpuOperation {
    pub fn new(instruction: Instruction, expression: Expression) -> Self {
        CpuOperation {
            instruction,
            expression,
        }
    }
}
