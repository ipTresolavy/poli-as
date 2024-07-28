use crate::lexer::cpu_op::CpuOperation;

#[derive(Debug)]
pub struct InstructionMemory {
    pub data: Vec<CpuOperation>,
}

impl InstructionMemory {
    pub fn new(data: Vec<CpuOperation>) -> Self {
        Self { data }
    }

    pub fn read(&self, address: usize) -> &CpuOperation {
        &self.data[address / 4]
    }
}
