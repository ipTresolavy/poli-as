// Example : mov r0, r1, r2

use crate::token::register::Register;

pub struct RegLiteralExpression {
    pub reg_d: Register,
    pub reg_m: Register,
    pub reg_n: Register,
}
