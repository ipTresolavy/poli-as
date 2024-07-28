use crate::{
    lexer::expression::Expression,
    lexer::expression::{immediate::ImmediateExpression, reg::RegExpression},
    token::{instruction_name::InstructionName, Token},
};

pub fn parse_branch_op(instruction: &InstructionName, operands: &[Token]) -> Expression {
    match instruction {
        InstructionName::B | InstructionName::BL => {
            if operands.len() == 1 {
                if let Token::IMMEDIATE(imm) = &operands[0] {
                    Expression::Immediate(ImmediateExpression::new(imm.clone()))
                } else {
                    panic!("Invalid operands");
                }
            } else {
                panic!("Invalid operands");
            }
        }
        InstructionName::BX => {
            if operands.len() == 1 {
                if let Token::REGISTER(reg) = operands[0] {
                    Expression::Register(RegExpression::new(reg.to_owned()))
                } else {
                    panic!("Invalid operands");
                }
            } else {
                panic!("Invalid operands");
            }
        }
        _ => panic!("Invalid instruction"),
    }
}

pub fn is_branch_op(token: &InstructionName) -> bool {
    matches!(
        token,
        InstructionName::B | InstructionName::BL | InstructionName::BX
    )
}
