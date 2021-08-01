use std::cmp::min;
use std::num::Wrapping;

pub const MEMORY_SIZE: usize = 1024 * 8;

pub struct Memory {
    data: [Wrapping<u8>; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [Wrapping(0); MEMORY_SIZE],
        }
    }

    pub fn of_bytes(data: &[u8]) -> Self {
        let mut new_memory = Self::new();
        for i in 0..min(data.len(), MEMORY_SIZE) {
            new_memory.data[i] = Wrapping(data[i]);
        }
        new_memory
    }

    pub fn get(&self, idx: usize) -> Wrapping<u8> {
        self.data[idx]
    }

    pub fn set(&mut self, idx: usize, val: Wrapping<u8>) {
        self.data[idx] = val;
    }

    pub fn get16(&self, idx: usize) -> Wrapping<u16> {
        let first_part = self.get(idx).0;
        let second_part = self.get(idx + 1).0;
        let fval: Wrapping<u16> = Wrapping(u16::from_be(
            first_part as u16 | ((second_part as u16) << 8),
        ));
        fval
    }
}
