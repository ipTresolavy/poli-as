use std::num::ParseIntError;

use crate::lexer::expression::ls_index::{IndexMode, LoadStoreExpression};
use crate::{
    lexer::expression::Expression,
    token::{instruction_name::InstructionName, Token},
};

pub fn is_load_store_op(token: &InstructionName) -> bool {
    matches!(
        token,
        InstructionName::STR
            | InstructionName::LDR
            | InstructionName::STM
            | InstructionName::STMDB
            | InstructionName::STMDA
            | InstructionName::STMIB
            | InstructionName::STMIA
            | InstructionName::LDM
            | InstructionName::LDMDB
            | InstructionName::LDMDA
            | InstructionName::LDMIB
            | InstructionName::LDMIA
    )
}

pub fn parse_load_store_op(instruction: &InstructionName, operands: &[Token]) -> Expression {
    match instruction {
        InstructionName::STR | InstructionName::LDR => parse_single_op(operands),
        InstructionName::STM
        | InstructionName::STMDB
        | InstructionName::STMDA
        | InstructionName::STMIB
        | InstructionName::STMIA
        | InstructionName::LDM
        | InstructionName::LDMDB
        | InstructionName::LDMDA
        | InstructionName::LDMIB
        | InstructionName::LDMIA => parse_multiple_op(operands),
        _ => panic!("Invalid instruction"),
    }
}

fn parse_single_op(operands: &[Token]) -> Expression {
    match operands {
        [Token::REGISTER(dest), Token::LPAREN, Token::REGISTER(base), Token::RPAREN] => {
            Expression::LoadStore(LoadStoreExpression::new(
                dest.to_owned(),
                base.to_owned(),
                None,
                IndexMode::None,
            ))
        }

        [Token::REGISTER(dest), Token::LPAREN, Token::REGISTER(base), Token::RPAREN] => {
            Expression::LoadStore(LoadStoreExpression::new(
                dest.to_owned(),
                base.to_owned(),
                None,
                IndexMode::None,
            ))
        }
        _ => todo!(),
    }
}

fn parse_multiple_op(operands: &[Token]) -> Expression {
    todo!()
}
