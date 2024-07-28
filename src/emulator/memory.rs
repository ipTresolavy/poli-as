#[derive(Debug)]
pub struct Heap {
    pub data: [u32; 262144],
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

impl Heap {
    pub fn new() -> Self {
        Self { data: [0; 262144] }
    }

    pub fn read(&self, address: u32) -> u32 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u32, value: u32) {
        self.data[address as usize] = value;
    }
}
