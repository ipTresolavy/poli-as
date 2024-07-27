// Example : mov r0, r1, r2

use crate::token::register::Register;

use super::barrel_shifter::BarrelShifterExpression;

pub struct RegLiteralExpression {
    pub reg_d: Register,
    pub reg_m: Register,
    pub reg_n: Register,
    pub barrel_shifter: Option<BarrelShifterExpression>,
}
