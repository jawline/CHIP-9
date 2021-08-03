use std::cmp::min;
use std::num::Wrapping;

pub const MEMORY_SIZE: usize = 1024 * 8;
pub const SCREEN_SIZE: usize = 64 * 32;
pub const SCREEN_WIDTH: usize = 64;

pub struct Memory {
    data: [Wrapping<u8>; MEMORY_SIZE],
    frame_buffer: [u8; SCREEN_SIZE],
}

impl Memory {

    pub fn new() -> Self {
        Self {
            data: [Wrapping(0); MEMORY_SIZE],
            frame_buffer: [0; SCREEN_SIZE]
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

    pub fn draw_sprite(&mut self, x: usize, y: usize, d: usize, i: usize) -> u8 {
        let fb = &mut self.frame_buffer;

        let mut vf_reg = 0;

        for yoff in 0..d {
            let y = y + yoff;
            let sprite = self.data[i + d].0;
            for i in 0..8 {
                // TODO: Return 1 if any pixel touched is already set. Flip it then also
                let xor_value = if sprite & (1 << (7 - i)) != 0 { 1 } else { 0 };
                let current_value = fb[(SCREEN_WIDTH * y) + x + i];
                let new_value = current_value ^ xor_value;

                if current_value == 1 && new_value == 0 {
                    vf_reg = 1;
                }

                fb[(SCREEN_WIDTH * d) + x + i] = new_value;
            }
        }

        vf_reg
    }
}
