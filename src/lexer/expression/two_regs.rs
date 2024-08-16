// mov r0, r0

use crate::token::instruction_name::InstructionName;
use crate::token::register::Register;

use crate::lexer::expression::barrel_shifter::BarrelShifterExpression;

#[derive(Debug, Copy, Clone)]
pub struct TwoRegsExpression {
    pub reg_d: Register,
    pub reg_m: Register,
    pub barrel_shifter: Option<BarrelShifterExpression>,
}

impl TwoRegsExpression {
    pub fn new(
        reg_d: Register,
        reg_m: Register,
        barrel_shifter: Option<BarrelShifterExpression>,
    ) -> Self {
        Self {
            reg_d,
            reg_m,
            barrel_shifter,
        }
    }

    // For some god forsaken reason, even though they have the same expression, cmp is different
    pub fn to_machine_code(&self, name: &InstructionName) -> u32 {
        let reg_d = self.reg_d.to_num() as u32;
        let reg_m = self.reg_m.to_num() as u32;
        let barrel_shifter = self
            .barrel_shifter
            .map(|bs| bs.to_machine_code())
            .unwrap_or(0);

        if matches!(name, InstructionName::CMP | InstructionName::CMN) {
            return (reg_d << 16) | (reg_m) | (barrel_shifter & !0b1111);
        }

        (reg_d << 12) | (reg_m) | (barrel_shifter << 4)
    }
}
