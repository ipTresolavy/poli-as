pub mod barrel_shifter;
pub mod immediate;
pub mod ls_imm_index;
pub mod ls_reg_index;
pub mod reg;
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
    Immediate(immediate::ImmediateExpression),
    Register(reg::RegExpression),
    LoadStoreImmediate(ls_imm_index::LoadStoreImmediateExpression),
    LoadStoreRegister(ls_reg_index::LoadStoreRegisterExpression),
}
