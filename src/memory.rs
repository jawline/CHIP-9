use log::trace;
use std::cmp::min;
use std::num::Wrapping;

pub const MEMORY_SIZE: usize = 1024 * 8;
pub const SCREEN_SIZE: usize = 64 * 32;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const SPRITE_MEM: [u8; 5 * 16] = [0xF0_u8, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80];

pub struct Memory {
    data: [Wrapping<u8>; MEMORY_SIZE],
    pub frame_buffer: [u8; SCREEN_SIZE],
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
            new_memory.data[0x200 + i] = Wrapping(data[i]);
        }
        new_memory
    }

    pub fn get(&self, idx: usize) -> Wrapping<u8> {
        if idx < 0x4000 {
            self.data[idx]
        } else {
            Wrapping(SPRITE_MEM[idx - 0x4000])
        }
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

    pub fn clear_display(&mut self) {
        for i in 0..SCREEN_SIZE {
            self.frame_buffer[i] = 0;
        }
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, n: usize, i: usize) -> u8 {

        let mut vf_reg = 0;

        for yoff in 0..n {

            let y = (y + yoff) % SCREEN_HEIGHT;
            let sprite = self.get(i + yoff).0;

            for xoff in 0..8 {
                let x = (x + xoff) % SCREEN_WIDTH;

                let fb = &mut self.frame_buffer;
                let fb_idx = (y * SCREEN_WIDTH) + x;
                // TODO: Return 1 if any pixel touched is already set. Flip it then also
                let xor_value = if sprite & (1 << (7 - xoff)) != 0 { 1 } else { 0 };
                let current_value = fb[fb_idx];
                let new_value = current_value ^ xor_value;
                trace!("{} {} {} {} {}", x, y, new_value, sprite, i + yoff);

                if current_value == 1 && new_value == 0 {
                    vf_reg = 1;
                }

                fb[fb_idx] = new_value;
            }
        }

        vf_reg
    }
}
