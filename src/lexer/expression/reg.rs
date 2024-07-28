use crate::token::register::Register;

#[derive(Debug)]
pub struct RegExpression {
    pub register: Register,
}

impl RegExpression {
    pub fn new(register: Register) -> Self {
        Self { register }
    }
}
