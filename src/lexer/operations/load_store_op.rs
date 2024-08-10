use crate::lexer::expression::barrel_shifter::BarrelShifterExpression;
use crate::lexer::expression::ls_imm_index::{IndexMode, LoadStoreImmediateExpression, PreIndex};
use crate::lexer::expression::ls_multiple::LoadStoreMultipleExpression;
use crate::lexer::expression::ls_reg_index::LoadStoreRegisterExpression;
use crate::token::register::Register;
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
            Expression::LoadStoreImmediate(LoadStoreImmediateExpression::new(
                dest.to_owned(),
                base.to_owned(),
                None,
                IndexMode::None,
            ))
        }

        [Token::REGISTER(dest), Token::LPAREN, Token::REGISTER(base), Token::RPAREN, Token::IMMEDIATE(imm)] => {
            Expression::LoadStoreImmediate(LoadStoreImmediateExpression::new(
                dest.to_owned(),
                base.to_owned(),
                Some(imm.clone()),
                IndexMode::Post,
            ))
        }
        [Token::REGISTER(dest), Token::LPAREN, Token::REGISTER(base), Token::IMMEDIATE(imm), Token::RPAREN] => {
            Expression::LoadStoreImmediate(LoadStoreImmediateExpression::new(
                dest.to_owned(),
                base.to_owned(),
                Some(imm.clone()),
                IndexMode::Pre(PreIndex { write_back: false }),
            ))
        }
        [Token::REGISTER(dest), Token::LPAREN, Token::REGISTER(base), Token::IMMEDIATE(imm), Token::RPAREN, Token::BANG] => {
            Expression::LoadStoreImmediate(LoadStoreImmediateExpression::new(
                dest.to_owned(),
                base.to_owned(),
                Some(imm.clone()),
                IndexMode::Pre(PreIndex { write_back: true }),
            ))
        }
        [Token::REGISTER(dest), Token::LPAREN, Token::REGISTER(base), Token::RPAREN, Token::REGISTER(offset)] => {
            Expression::LoadStoreRegister(LoadStoreRegisterExpression::new(
                dest.to_owned(),
                base.to_owned(),
                offset.to_owned(),
                IndexMode::Post,
                None,
            ))
        }
        [Token::REGISTER(dest), Token::LPAREN, Token::REGISTER(base), Token::RPAREN, Token::REGISTER(offset), rest @ ..] =>
        {
            let barrel_shifter = BarrelShifterExpression::new(rest);

            Expression::LoadStoreRegister(LoadStoreRegisterExpression::new(
                dest.to_owned(),
                base.to_owned(),
                offset.to_owned(),
                IndexMode::Post,
                barrel_shifter,
            ))
        }
        // TODO: Support negative index mode
        [Token::REGISTER(dest), Token::LPAREN, Token::REGISTER(base), Token::REGISTER(offset), Token::RPAREN] => {
            Expression::LoadStoreRegister(LoadStoreRegisterExpression::new(
                dest.to_owned(),
                base.to_owned(),
                offset.to_owned(),
                IndexMode::Pre(PreIndex { write_back: false }),
                None,
            ))
        }
        [Token::REGISTER(dest), Token::LPAREN, Token::REGISTER(base), Token::REGISTER(offset), rest @ .., Token::RPAREN, Token::BANG] =>
        {
            let barrel_shifter = BarrelShifterExpression::new(rest);

            Expression::LoadStoreRegister(LoadStoreRegisterExpression::new(
                dest.to_owned(),
                base.to_owned(),
                offset.to_owned(),
                IndexMode::Pre(PreIndex { write_back: true }),
                barrel_shifter,
            ))
        }

        [Token::REGISTER(dest), Token::LPAREN, Token::REGISTER(base), Token::REGISTER(offset), rest @ .., Token::RPAREN] =>
        {
            let barrel_shifter = BarrelShifterExpression::new(rest);

            Expression::LoadStoreRegister(LoadStoreRegisterExpression::new(
                dest.to_owned(),
                base.to_owned(),
                offset.to_owned(),
                IndexMode::Pre(PreIndex { write_back: false }),
                barrel_shifter,
            ))
        }
        _ => panic!("Invalid operands"),
    }
}

fn parse_multiple_op(operands: &[Token]) -> Expression {
    match operands {
        [Token::REGISTER(dest), Token::LBRACE, rest @ .., Token::RBRACE] => {
            let registers = parse_multiple_regs(rest);
            Expression::LoadStoreMultiple(LoadStoreMultipleExpression::new(
                dest.to_owned(),
                registers,
                false,
            ))
        }

        [Token::REGISTER(dest), Token::BANG, Token::LBRACE, rest @ .., Token::RBRACE] => {
            let registers = parse_multiple_regs(rest);
            Expression::LoadStoreMultiple(LoadStoreMultipleExpression::new(
                dest.to_owned(),
                registers,
                true,
            ))
        }
        _ => panic!("Invalid operands"),
    }
}

fn parse_multiple_regs(operands: &[Token]) -> Vec<Register> {
    let mut registers = Vec::new();

    let mut i = 0;
    while i < operands.len() {
        if let Token::REGISTER(reg) = &operands[i] {
            if let Some(Token::MINUS) = operands.get(i + 1) {
                if let Some(Token::REGISTER(end)) = operands.get(i + 2) {
                    let regs = generate_reg_range(reg, end);

                    registers.extend(regs);

                    i += 3;
                    continue;
                }
            }
            registers.push(reg.to_owned());
        }
        i += 1;
    }

    registers
}

fn generate_reg_range(start: &Register, end: &Register) -> Vec<Register> {
    let start = start.to_num();
    let end = end.to_num();
    if start > end {
        panic!("Invalid register range");
    }

    (start..=end)
        .map(|num| Register::from_num(num).unwrap())
        .collect()
}
