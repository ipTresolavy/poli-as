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
    pub value: u8,
}

impl Number {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
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
