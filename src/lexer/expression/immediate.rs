use crate::token::immediate::Immediate;

#[derive(Debug, Clone)]
pub struct ImmediateExpression {
    pub literal: Immediate,
}

impl ImmediateExpression {
    pub fn new(literal: Immediate) -> Self {
        Self { literal }
    }
}
