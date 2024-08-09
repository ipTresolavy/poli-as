use crate::token::immediate::Immediate;

#[derive(Debug, Clone)]
pub struct ImmediateExpression {
    pub literal: Immediate,
}

impl ImmediateExpression {
    pub fn new(literal: Immediate) -> Self {
        Self { literal }
    }

    pub fn to_machine_code(&self) -> u32 {
        self.literal.to_num()
    }
}
