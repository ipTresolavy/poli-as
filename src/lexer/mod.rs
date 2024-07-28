use crate::{
    token::{immediate::Immediate, instruction_name::InstructionName, Token},
    tokenizer::Tokenizer,
};

use self::{
    expression::{
        three_regs::ThreeRegsExpression, two_regs_literal::TwoRegsLiteralExpression, Expression,
    },
    symbolizer::Symbolizer,
};

pub mod expression;
pub mod machine_code_builder;
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

    pub fn parse(&mut self) {
        while !self.tokenizer.is_eof() {
            self.parse_line();
        }
    }

    fn parse_line(&mut self) {
        let mut tokens = self.tokenizer.consume_line();

        if tokens.is_empty() {
            return;
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
                    let test = parse_logical_arithmatic_op(operands);
                    println!("{:?}", test);
                } else {
                    panic!("Instruction not supported")
                }
            }
        }
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
        [Token::REGISTER(reg_d), Token::REGISTER(reg_m), Token::IMMEDIATE(imm), rest @ ..] => {
            let barrel_shifter = expression::barrel_shifter::BarrelShifterExpression::new(rest);
            Expression::TwoRegsLiteral(TwoRegsLiteralExpression::new(
                reg_d.to_owned(),
                reg_m.to_owned(),
                imm.clone(),
                barrel_shifter,
            ))
        }
        _ => panic!("Invalid operands"),
    }
}

fn is_logical_arithmatic_op(token: &InstructionName) -> bool {
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
            | InstructionName::CMP
            | InstructionName::CMN
            | InstructionName::ORR
            | InstructionName::MOV
            | InstructionName::BIC
            | InstructionName::MVN
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
