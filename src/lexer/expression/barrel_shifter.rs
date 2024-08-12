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

    pub fn to_machine_code(&self) -> u32 {
        let shifter_opcode = match self.shift_amount {
            BarrealShifterShiftAmount::Number(imm) => ((imm & 0x0000001f) as u32) << 7,
            BarrealShifterShiftAmount::Register(reg) => ((reg.to_num() as u32) << 8) | (1 << 4),
        };

        let shift_type_code: u32 = match self.operation {
            BarrelShifterOperation::LSL => 0,
            BarrelShifterOperation::LSR => 1,
            BarrelShifterOperation::ASR => 2,
            BarrelShifterOperation::ROR => 3,
        } << 5;

        shifter_opcode | shift_type_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{
        immediate::Immediate,
        instruction::Instruction,
        register::{Register, RegisterNumbers},
        Token,
    };

    fn create_instruction() -> Token {
        let istr = Instruction::new("lsr", Some("s"), None);
        Token::INSTRUCTION(istr.unwrap())
    }

    fn create_register() -> Token {
        let reg = Register::new(RegisterNumbers::ONE);
        Token::REGISTER(reg)
    }

    fn create_immediate(num: &str) -> Token {
        let imm = Immediate::new(num.to_string());
        Token::IMMEDIATE(imm.expect("should work"))
    }

    #[test]
    fn test_barrel_shifter_expression_to_machine_code_with_immediate() {
        let tokens = vec![create_instruction(), create_immediate("0x5")];

        let expression = BarrelShifterExpression::new(&tokens).unwrap();

        let machine_code = expression.to_machine_code();

        assert_eq!(machine_code, 0b00101010 << 4);
    }

    #[test]
    fn test_barrel_shifter_expression_to_machine_code_with_register() {
        let tokens = vec![create_instruction(), create_register()];

        let expression = BarrelShifterExpression::new(&tokens).unwrap();

        let machine_code = expression.to_machine_code();

        assert_eq!(machine_code, 0b00010011 << 4);
    }
}
