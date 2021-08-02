use crate::cpu::Cpu;
use crate::memory::Memory;

pub struct Machine {
    pub cpu: Cpu,
    pub memory: Memory,
}

impl Machine {

    pub fn of_bytes(data: Vec<u8>) -> Self {
        Self { cpu: Cpu::new(), memory: Memory::of_bytes(&data) }
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
        }
    }
}
