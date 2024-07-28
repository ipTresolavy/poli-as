use crate::token::{instruction_name::InstructionName, register::Register, Token};

#[derive(Debug)]
pub enum BarrelShifterOperation {
    LSL,
    LSR,
    ASR,
    ROR,
}

#[derive(Debug)]
pub enum BarrealShifterShiftAmount {
    Register(Register),
    Number(u8),
}

#[derive(Debug)]
pub struct BarrelShifterExpression {
    pub operation: BarrelShifterOperation,
    pub shift_amount: BarrealShifterShiftAmount,
}

impl BarrelShifterExpression {
    pub fn new(tokens: &[Token]) -> Option<Self> {
        if tokens.is_empty() {
            return None;
        }

        let instruction = match &tokens[0] {
            Token::INSTRUCTION(istr) => &istr.value,
            _ => panic!("Expected instruction"),
        };

        let operation = match instruction {
            InstructionName::LSL => BarrelShifterOperation::LSL,
            InstructionName::LSR => BarrelShifterOperation::LSR,
            InstructionName::ASR => BarrelShifterOperation::ASR,
            InstructionName::ROR => BarrelShifterOperation::ROR,
            _ => panic!("Invalid barrel shifter operation"),
        };

        let shift_amount = match &tokens[1] {
            Token::REGISTER(reg) => BarrealShifterShiftAmount::Register(*reg),
            Token::IMMEDIATE(imm) => BarrealShifterShiftAmount::Number(imm.number as u8),
            _ => panic!("Invalid shift amount"),
        };

        Some(Self {
            operation,
            shift_amount,
        })
    }
}
