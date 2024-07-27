use crate::token::register::Register;

pub enum BarrelShifterOperation {
    LSL,
    LSR,
    ASR,
    ROR,
}

pub enum BarrealShifterShiftAmount {
    Register(Register),
    Number(u8),
}

pub struct BarrelShifterExpression {
    pub operation: BarrelShifterOperation,
    pub shift_amount: BarrealShifterShiftAmount,
}
