// Example : mov r0, #0x1234

use crate::token::{immediate::Immediate, register::Register};

#[derive(Debug, Clone)]
pub struct RegLiteralExpression {
    pub register: Register,
    pub literal: Immediate,
}

impl RegLiteralExpression {
    pub fn new(register: Register, literal: Immediate) -> Self {
        Self { register, literal }
    }

    pub fn to_machine_code(&self) -> u32 {
        let register = self.register.to_num() as u32;
        let literal = self.literal.to_num();

        let (rotation, lower_byte) =
            check_immediate_possible(literal).expect("Impossível representar o valor imediato");

        (register << 12) | (rotation as u32) << 8 | lower_byte as u32 | 1 << 25
    }
}

fn check_immediate_possible(immediate: u32) -> Option<(u8, u8)> {
    for rotation in 0..16 {
        let val: u32 = 0xFF_u32.rotate_right(rotation * 2);
        let val = !val;

        if immediate & val == 0 {
            let offset = (immediate & !val).rotate_right((16 - rotation) * 2);
            return Some((rotation as u8, offset as u8));
        }
    }
    None
}
