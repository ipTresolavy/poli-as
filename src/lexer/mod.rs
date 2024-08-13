use symbolizer::SymbolTable;

use crate::token::{
    immediate::Immediate,
    instruction::Instruction,
    instruction_name::InstructionName,
    register::{Register, RegisterNumbers},
    Token,
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
};

pub mod cpu_op;
pub mod expression;
pub mod machine_code_builder;
pub mod operations;
pub mod symbolizer;

pub struct Lexer {
    symbol_table: SymbolTable,
    addr: u32,
}

impl Lexer {
    pub fn new(symbol_table: SymbolTable) -> Self {
        Lexer {
            symbol_table,
            addr: 0,
        }
    }

    pub fn increment_addr(&mut self, addr: u32) {
        self.addr += addr;
    }

    pub fn parse_line(&mut self, mut tokens: Vec<Token>) -> Option<CpuOperation> {
        if tokens.is_empty() {
            return None;
        }

        // first replace any labels with their addresses
        replace_label_ref(&mut tokens, &self.symbol_table, &mut self.addr);
        let mut tokens = replace_pseudo_ops(tokens);

        let index = tokens
            .iter()
            .position(|token| matches!(token, Token::INSTRUCTION(_)));

        self.increment_addr(4);

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
                    panic!("Instruction {:?} not supported", instruction.value)
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

fn replace_label_ref(tokens: &mut [Token], symbol_table: &SymbolTable, current_addr: &mut u32) {
    for token in tokens.iter_mut() {
        if let Token::LABELREF(label) = token {
            let address = symbol_table.get_address(label);
            if address.is_none() {
                panic!("Label {} not found in symbol table", label);
            }

            let immediate = Immediate::new(
                (address.unwrap().value as i32 - current_addr.to_owned() as i32 - 8).to_string(),
            )
            .unwrap();

            *token = Token::IMMEDIATE(immediate);
        }
    }
}

fn replace_pseudo_ops(mut tokens: Vec<Token>) -> Vec<Token> {
    let index = tokens
        .iter()
        .position(|token| matches!(token, Token::INSTRUCTION(_)));

    if let Some(index) = index {
        let (instruction, _) = tokens.split_at_mut(index + 1);
        let instruction = instruction.last().unwrap();
        if let Token::INSTRUCTION(instruction) = instruction {
            if is_pseudo_istr(&instruction.value) {
                match instruction.value {
                    InstructionName::PUSH => {
                        let istr = Instruction::new(
                            "stmdb",
                            None,
                            Some(instruction.condition.to_string()),
                        )
                        .unwrap();

                        tokens[index] = Token::INSTRUCTION(istr);
                        tokens.insert(
                            index + 1,
                            Token::REGISTER(Register::new(RegisterNumbers::THIRTEEN)),
                        );

                        return tokens;
                    }
                    InstructionName::POP => {
                        // let mut new_tokens = vec![Token::INSTRUCTION(InstructionName::LDMIA)];
                        // new_tokens.extend(operands);
                        // new_tokens
                        let istr = Instruction::new(
                            "ldmia",
                            None,
                            Some(instruction.condition.to_string()),
                        )
                        .unwrap();

                        tokens[index] = Token::INSTRUCTION(istr);

                        tokens.insert(
                            index + 1,
                            Token::REGISTER(Register::new(RegisterNumbers::THIRTEEN)),
                        );

                        return tokens;
                    }
                    InstructionName::LSL
                    | InstructionName::LSR
                    | InstructionName::ROR
                    | InstructionName::ASR => {
                        let mov =
                            Instruction::new("mov", None, Some(instruction.condition.to_string()))
                                .unwrap();

                        let istr = *instruction;

                        tokens[index] = Token::INSTRUCTION(mov);
                        tokens.insert(tokens.len() - 1, Token::INSTRUCTION(istr));

                        return tokens;
                    }
                    _ => panic!("Invalid instruction"),
                };
            }
        }
    }

    tokens
}

fn is_pseudo_istr(token: &InstructionName) -> bool {
    matches!(
        token,
        InstructionName::PUSH
            | InstructionName::POP
            | InstructionName::LSL
            | InstructionName::LSR
            | InstructionName::ROR
            | InstructionName::ASR
    )
}
