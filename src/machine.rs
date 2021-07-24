use crate::cpu::Cpu;
use crate::memory::Memory;

pub struct Machine {
    pub cpu: Cpu,
    pub memory: Memory,
}

impl Machine {
    pub fn new() -> Self {
        Self {
          cpu: Cpu::new(),
          memory: Memory::new()
        }
    }
}
