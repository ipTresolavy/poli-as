pub mod barrel_shifter;
pub mod reg_literal;
pub mod three_regs;
pub mod two_regs;
pub mod two_regs_literal;

#[derive(Debug)]
pub enum Expression {
    ThreeRegs(three_regs::ThreeRegsExpression),
    TwoRegs(two_regs::TwoRegsExpression),
    TwoRegsLiteral(two_regs_literal::TwoRegsLiteralExpression),
    RegLiteral(reg_literal::RegLiteralExpression),
}
