// mov r0, r0

use crate::token::register::Register;

pub struct TwoRegsExpression {
    pub reg_d: Register,
    pub reg_m: Register,
}
