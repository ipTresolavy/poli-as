use immediate::ImmediateBase;

use self::{immediate::Immediate, instruction::Instruction, register::Register};

pub mod immediate;
pub mod instruction;
pub mod instruction_name;
pub mod register;

#[derive(Debug)]
pub enum Token {
    REGISTER(Register),
    INSTRUCTION(Instruction),
    IMMEDIATE(Immediate),
    LABEL(Label),
    LABELREF(String),
    DIRECTIVE(Directive),
    NUMBER(Number),
    MINUS,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    EQUAL,
    COMMA,
    BANG,
    ILLEGAL,
    EOF,
}

impl Token {
    pub fn is_instruction(&self) -> bool {
        matches!(self, Token::INSTRUCTION(_))
    }

    pub fn is_directive(&self) -> bool {
        matches!(self, Token::DIRECTIVE(_))
    }

    pub fn extract_directive(&self) -> Option<&Directive> {
        match self {
            Token::DIRECTIVE(directive) => Some(directive),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Token::NUMBER(_))
    }

    pub fn extract_number(&self) -> Option<&Number> {
        match self {
            Token::NUMBER(number) => Some(number),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Label {
    pub value: String,
}
impl Label {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct Number {
    pub value: u32,
}

impl Number {
    pub fn new(value: &str) -> Option<Self> {
        let base = determine_base(&value)?;

        let number: u32 = match base {
            ImmediateBase::HEX => u32::from_str_radix(value.trim_start_matches("0x"), 16).ok()?,
            ImmediateBase::DEC => (value.parse::<i32>().ok()?) as u32,
            ImmediateBase::OCT => u32::from_str_radix(value.trim_start_matches("0o"), 8).ok()?,
            ImmediateBase::BIN => u32::from_str_radix(value.trim_start_matches("0b"), 2).ok()?,
        };

        Some(Number { value: number })
    }
}

fn determine_base(val: &str) -> Option<ImmediateBase> {
    if val.starts_with("0x") {
        return Some(ImmediateBase::HEX);
    }

    if val.starts_with("0b") {
        return Some(ImmediateBase::BIN);
    }

    if val.starts_with("0o") {
        return Some(ImmediateBase::OCT);
    }

    Some(ImmediateBase::DEC)
}

#[derive(Debug)]
pub struct Directive {
    pub value: String,
}

impl Directive {
    pub fn new(value: String) -> Directive {
        Directive { value }
    }
}
