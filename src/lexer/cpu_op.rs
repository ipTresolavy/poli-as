use crate::token::instruction::Instruction;

use super::{expression::Expression, machine_code_builder::MachineCodeInstruction};

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

    pub fn to_machine_code(&self) -> MachineCodeInstruction {
        let mut code = MachineCodeInstruction::new();
        let condition_mask = self.instruction.condition.to_machine_code();
        code.push_mask(16 << 28, condition_mask);

        println!("{}", code.to_debug_string());

        code
    }
}
