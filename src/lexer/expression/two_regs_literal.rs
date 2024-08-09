// example: add r0 r1 #0x1234

use crate::{
    token::{immediate::Immediate, register::Register},
    utils::debug_u32,
};

#[derive(Debug, Clone)]
pub struct TwoRegsLiteralExpression {
    pub reg_d: Register,
    pub reg_m: Register,
    pub literal: Immediate,
}

impl TwoRegsLiteralExpression {
    pub fn new(reg_d: Register, reg_m: Register, literal: Immediate) -> Self {
        Self {
            reg_d,
            reg_m,
            literal,
        }
    }

    pub fn to_machine_code(&self) -> u32 {
        let reg_d = self.reg_d.to_num() as u32;
        let reg_m = self.reg_m.to_num() as u32;
        let immediate =
            check_immediate_possible(self.literal.to_num()).expect("Immediate value is too large");
        (reg_d << 12) | (reg_m << 16) | (immediate.0 as u32) << 8 | immediate.1 as u32 | 1 << 25
    }
}

fn check_immediate_possible(immediate: u32) -> Option<(u8, u8)> {
    for rotation in 0..16 {
        let val: u32 = 0xFF_u32.rotate_right(rotation * 2);
        let val = !val;

        if immediate & val == 0 {
            let offset = (immediate & !val).rotate_right((16 - rotation) * 2);
            debug_u32(offset);
            return Some((rotation as u8, offset as u8));
        }

        debug_u32(val);
    }
    None
}
