pub struct MachineCodeBit {
    pub position: u8,
    pub value: bool,
}

pub struct MachineCodeInstruction {
    pub bits: Vec<MachineCodeBit>,
}

impl MachineCodeInstruction {
    pub fn new() -> Self {
        let mut bits = Vec::new();
        for i in 0..32 {
            let bit = MachineCodeBit {
                position: i,
                value: false,
            };
            bits.push(bit);
        }
        MachineCodeInstruction { bits }
    }

    pub fn set(&mut self, position: u8, value: bool) {
        self.bits[position as usize].value = value;
    }

    pub fn push_mask(&mut self, mask: u32, value: u32) {
        for i in 0..32 {
            if mask & (1 << i) != 0 {
                self.set(i as u8, value & (1 << i) != 0);
            }
        }
    }

    pub fn to_debug_string(&self) -> String {
        let mut machine_code = String::new();
        for bit in &self.bits {
            if bit.value {
                machine_code.push('1');
            } else {
                machine_code.push('0');
            }
        }
        machine_code
    }
}

impl Default for MachineCodeInstruction {
    fn default() -> Self {
        Self::new()
    }
}
