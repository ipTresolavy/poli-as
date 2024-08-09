use crate::{
    token::{immediate::Immediate, instruction_name::InstructionName, Token},
    tokenizer::Tokenizer,
};

use self::{
    cpu_op::CpuOperation,
    expression::{
        reg_literal::RegLiteralExpression, three_regs::ThreeRegsExpression,
        two_regs_literal::TwoRegsLiteralExpression, Expression,
    },
    operations::{
        branch_op::{is_branch_op, parse_branch_op},
        load_store_op::{is_load_store_op, parse_load_store_op},
    },
    symbolizer::Symbolizer,
};

pub mod cpu_op;
pub mod expression;
pub mod machine_code_builder;
pub mod operations;
pub mod symbolizer;

pub struct Lexer {
    tokenizer: Tokenizer,
    symbol_table: Symbolizer,
}

impl Lexer {
    pub fn new(tokenizer: Tokenizer, symbol_table: Symbolizer) -> Self {
        Lexer {
            tokenizer,
            symbol_table,
        }
    }

    pub fn parse(&mut self) -> Vec<CpuOperation> {
        let mut operations = Vec::new();
        while !self.tokenizer.is_eof() {
            let op = self.parse_line();

            if let Some(op) = op {
                operations.push(op);
            }
        }
        operations
    }

    fn parse_line(&mut self) -> Option<CpuOperation> {
        let mut tokens = self.tokenizer.consume_line();

        if tokens.is_empty() {
            return None;
        }

        // first replace any labels with their addresses
        replace_label_ref(&mut tokens, &self.symbol_table);

        let index = tokens
            .iter()
            .position(|token| matches!(token, Token::INSTRUCTION(_)));

        if let Some(index) = index {
            let (instruction, operands) = tokens.split_at_mut(index + 1);
            let instruction = instruction.last().unwrap();
            if let Token::INSTRUCTION(instruction) = instruction {
                if is_logical_arithmatic_op(&instruction.value) {
                    let expr = parse_logical_arithmatic_op(operands);
                    return Some(CpuOperation::new(*instruction, expr));
                } else if is_move_op(&instruction.value) {
                    let expr = parse_move_op(operands);
                    return Some(CpuOperation::new(*instruction, expr));
                } else if is_branch_op(&instruction.value) {
                    let expr = parse_branch_op(&instruction.value, operands);
                    return Some(CpuOperation::new(*instruction, expr));
                } else if is_load_store_op(&instruction.value) {
                    let expr = parse_load_store_op(&instruction.value, operands);
                    return Some(CpuOperation::new(*instruction, expr));
                } else {
                    panic!("Instruction not supported")
                }
            }
        }
        None
    }
}

// Keep in mind we make a copy of the expressions in memory
fn parse_logical_arithmatic_op(operands: &[Token]) -> Expression {
    match operands {
        [Token::REGISTER(reg_d), Token::REGISTER(reg_m), Token::REGISTER(reg_n), rest @ ..] => {
            let barrel_shifter = expression::barrel_shifter::BarrelShifterExpression::new(rest);
            Expression::ThreeRegs(ThreeRegsExpression::new(
                reg_d.to_owned(),
                reg_m.to_owned(),
                reg_n.to_owned(),
                barrel_shifter,
            ))
        }
        [Token::REGISTER(reg_d), Token::REGISTER(reg_m), Token::IMMEDIATE(imm)] => {
            Expression::TwoRegsLiteral(TwoRegsLiteralExpression::new(
                reg_d.to_owned(),
                reg_m.to_owned(),
                imm.clone(),
            ))
        }
        _ => panic!("Invalid operands"),
    }
}

fn parse_move_op(operands: &[Token]) -> Expression {
    match operands {
        [Token::REGISTER(reg_d), Token::REGISTER(reg_m), rest @ ..] => {
            let barrel_shifter = expression::barrel_shifter::BarrelShifterExpression::new(rest);
            Expression::TwoRegs(expression::two_regs::TwoRegsExpression::new(
                reg_d.to_owned(),
                reg_m.to_owned(),
                barrel_shifter,
            ))
        }
        [Token::REGISTER(reg_d), Token::IMMEDIATE(imm)] => {
            Expression::RegLiteral(RegLiteralExpression::new(reg_d.to_owned(), imm.clone()))
        }
        _ => panic!("Invalid operands"),
    }
}

pub fn is_logical_arithmatic_op(token: &InstructionName) -> bool {
    matches!(
        token,
        InstructionName::AND
            | InstructionName::EOR
            | InstructionName::SUB
            | InstructionName::RSB
            | InstructionName::ADD
            | InstructionName::ADC
            | InstructionName::SBC
            | InstructionName::RSC
            | InstructionName::TST
            | InstructionName::TEQ
            | InstructionName::ORR
            | InstructionName::BIC
    )
}

pub fn is_move_op(token: &InstructionName) -> bool {
    matches!(
        token,
        InstructionName::MOV | InstructionName::MVN | InstructionName::CMP | InstructionName::CMN
    )
}

fn replace_label_ref(tokens: &mut [Token], symbol_table: &Symbolizer) {
    for token in tokens.iter_mut() {
        if let Token::LABELREF(label) = token {
            let address = symbol_table.get_address(label);
            if address.is_none() {
                panic!("Label {} not found in symbol table", label);
            }

            let immediate = Immediate::new(address.unwrap().value.to_string()).unwrap();

            *token = Token::IMMEDIATE(immediate);
        }
    }
}
