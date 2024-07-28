use crate::{
    emulator::regs::CpuRegisters,
    token::{instruction_name::InstructionName, register::Register, Token},
};

#[derive(Debug, Copy, Clone)]
pub enum BarrelShifterOperation {
    LSL,
    LSR,
    ASR,
    ROR,
}

#[derive(Debug, Copy, Clone)]
pub enum BarrealShifterShiftAmount {
    Register(Register),
    Number(u8),
}

#[derive(Debug, Copy, Clone)]
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

    pub fn apply(&self, value: u32, regs: &CpuRegisters) -> u32 {
        match self.shift_amount {
            BarrealShifterShiftAmount::Number(imm) => match self.operation {
                BarrelShifterOperation::LSL => value << imm,
                BarrelShifterOperation::LSR => value >> imm,
                BarrelShifterOperation::ASR => ((value as i32) >> imm) as u32,
                BarrelShifterOperation::ROR => value.rotate_right(imm as u32),
            },
            BarrealShifterShiftAmount::Register(num) => {
                let shift_amount = regs.get(num.to_num()) & 0xFF;
                match self.operation {
                    BarrelShifterOperation::LSL => value << shift_amount,
                    BarrelShifterOperation::LSR => value >> shift_amount,
                    BarrelShifterOperation::ASR => ((value as i32) >> shift_amount) as u32,
                    BarrelShifterOperation::ROR => value.rotate_right(shift_amount),
                }
            }
        }
    }
}
