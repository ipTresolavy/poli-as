pub struct Stack {
    pub data: [u32; 8192],
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self { data: [0; 8192] }
    }
    pub fn read(&self, address: u32) -> u32 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u32, value: u32) {
        self.data[address as usize] = value;
    }
}
