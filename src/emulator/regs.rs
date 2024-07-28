use super::cpu::Flags;

#[derive(Debug)]
pub struct CpuRegisters {
    pub r: [u32; 16],
    pub cpsr: u32,
}

impl Default for CpuRegisters {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuRegisters {
    pub fn new() -> Self {
        Self {
            r: [0; 16],
            cpsr: 0,
        }
    }

    pub fn set(&mut self, reg: u8, value: u32) {
        self.r[reg as usize] = value;
    }

    pub fn get(&self, reg: u8) -> u32 {
        self.r[reg as usize]
    }

    pub fn update_flags(&mut self, result: u32, flags: &Flags) {
        self.cpsr = 0;
        self.cpsr |= (result == 0) as u32;
        self.cpsr |= result & (1 << 31);
        self.cpsr |= (flags.carry as u32) << 29;
        self.cpsr |= (flags.overflow as u32) << 28;
    }
}
