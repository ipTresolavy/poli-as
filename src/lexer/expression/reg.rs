use crate::token::register::Register;

#[derive(Debug, Copy, Clone)]
pub struct RegExpression {
    pub register: Register,
}

impl RegExpression {
    pub fn new(register: Register) -> Self {
        Self { register }
    }
}
