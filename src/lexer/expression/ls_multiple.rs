use crate::token::register::Register;

#[derive(Debug)]
pub struct LoadStoreMultipleExpression {
    pub base: Register,
    pub registers: Vec<Register>,
    pub write_back: bool,
}

impl LoadStoreMultipleExpression {
    pub fn new(base: Register, registers: Vec<Register>, write_back: bool) -> Self {
        Self {
            base,
            registers,
            write_back,
        }
    }
}
