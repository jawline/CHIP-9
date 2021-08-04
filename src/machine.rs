use crate::cpu::Cpu;
use crate::memory::Memory;
pub const CLOCKS_PER_DELAY: usize = 10;

pub struct Machine {
    pub cpu: Cpu,
    pub memory: Memory,
    clocks_since_delay: usize,
}

impl Machine {

    pub fn of_bytes(data: Vec<u8>) -> Self {
        let mut cpu = Cpu::new();
        cpu.registers.pc.0 = 0x200;
        Self { cpu, memory: Memory::of_bytes(&data), clocks_since_delay: 0 }
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            clocks_since_delay: 0
        }
    }

    pub fn set_key(&mut self, key: u8, state: bool) {
        let current = self.cpu.registers.keys[key as usize];
        if state && !current {
            if let Some(register) = self.cpu.registers.wait_for_key {
                self.cpu.registers.v[register].0 = key;
                self.cpu.registers.wait_for_key = None;
            }
        }

        self.cpu.registers.keys[key as usize] = state;
    }

    pub fn step(&mut self) {

        // Only step the CPU if we are not waiting for a key press
        if let None = self.cpu.registers.wait_for_key {
            self.cpu.step(&mut self.memory);
        }

        // Increment the timers at roughly 1 clock per 10 steps
        self.clocks_since_delay += 1;
        if self.clocks_since_delay >= CLOCKS_PER_DELAY {

            if self.cpu.registers.sound.0 > 0 {
                self.cpu.registers.sound.0 -= 1;
            }

            if self.cpu.registers.delay.0 > 0 {
                self.cpu.registers.delay.0 -= 1;
            }

        }
    }
}
