use crate::token::{instruction::Instruction, instruction_name::InstructionName};

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
        code.push_mask((15 << 28) as u32, condition_mask);

        code.push_mask(0x0fffffff, self.generate_proc());

        code
    }

    fn generate_bx(&self) -> u32 {
        let base: u32 = 0x012fff10;

        let reg = match self.expression {
            Expression::Register(ref reg) => reg.to_machine_code(),
            _ => panic!("Expected register"),
        };

        base | reg
    }

    fn generate_b(&self) -> u32 {
        let base: u32 = 0x0e000000;

        let link: u32 = match self.instruction.value {
            InstructionName::BL => 0x01000000,
            InstructionName::B => 0,
            _ => panic!("Expected branch instruction"),
        };

        let base = base | link;

        let imm = match self.expression {
            Expression::Immediate(ref imm) => imm.to_machine_code(),
            _ => panic!("Expected register"),
        };

        base | imm
    }

    fn generate_proc(&self) -> u32 {
        let base: u32 = 0x00000000;
        let save = match self.instruction.save_register {
            true => 1 << 20,
            false => 0,
        };

        let base = base | save;

        let proc_opcode = get_proc_opcode(&self.instruction.value);

        let base = base | (proc_opcode << 21);

        let expression = get_proc_expression(&self.expression);

        base | expression
    }
}

fn get_proc_opcode(operation: &InstructionName) -> u32 {
    match operation {
        InstructionName::AND => 0,
        InstructionName::EOR => 1,
        InstructionName::SUB => 2,
        InstructionName::RSB => 3,
        InstructionName::ADD => 4,
        InstructionName::ADC => 5,
        InstructionName::SBC => 6,
        InstructionName::RSC => 7,
        InstructionName::TST => 8,
        InstructionName::TEQ => 9,
        InstructionName::CMP => 10,
        InstructionName::CMN => 11,
        InstructionName::ORR => 12,
        InstructionName::MOV => 13,
        InstructionName::BIC => 14,
        InstructionName::MVN => 15,
        _ => panic!("Invalid operation"),
    }
}

fn get_proc_expression(expression: &Expression) -> u32 {
    match expression {
        Expression::ThreeRegs(expr) => expr.to_machine_code(),
        Expression::TwoRegs(expr) => expr.to_machine_code(),
        Expression::TwoRegsLiteral(expr) => expr.to_machine_code(),
        Expression::RegLiteral(expr) => expr.to_machine_code(),
        _ => panic!("Invalid expression"),
    }
}
