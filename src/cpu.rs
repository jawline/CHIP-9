use std::num::Wrapping;
use crate::memory::Memory;

pub const REGISTER_MASK: u16 = 0x0F00;
pub const DATA_MASK: u16 = 0x00FF;

pub struct Registers {

    /// The CHIP architecture has 16 8-bit general purpose registers.
    /// Register v[f] also doubles as the carry flag, collision flag, or borrow flag dependent on
    /// the operation.
  pub v: [Wrapping<u8>; 16],

  /// The address register (PC) is named i
  pub i: Wrapping<u16>,
  pub stack: [Wrapping<u8>; 256],

  /// The delay timer counts down to zero at 60hz
  pub delay: Wrapping<u8>,

  /// The sound timer emits a sound if it is not zero.
  /// This timer counts down to zero at 60hz and then stops.
  pub sound: Wrapping<u8>,
}

pub struct Instruction {
    pub desc: String,
    pub execute: fn(registers: &mut Registers, memory: &mut Memory, data: u16),
    pub to_string: fn(data: u16) -> String,
}

impl Instruction {

    /// The zero opcode can be either clear display, ret, or machine call (Call an instruction
    /// written in machine code) depending on parameters. We merge these all into one opcode
    /// execution.
    pub fn mcall_display_or_flow(_registers: &mut Registers, _memory: &mut Memory, data: u16) {
        match data {
            0xE0 => unimplemented!("clear display"),
            0xEE => unimplemented!("ret"),
            _ => panic!("machine code routes are unsupported"),
        }
    }

    pub fn mcall_display_or_flow_to_string(data: u16) -> String {
       match data {
           0xE0 => format!("clear_display"),
           0xEE => format!("return"),
           _ => format!("mcall {:x}", data),
       }
    }
   
    /// Goto changes the I pointer to the fixed location
    pub fn goto(_registers: &mut Registers, _memory: &mut Memory, _data: u16) {
        unimplemented!("goto")
    }

    pub fn goto_to_string(data: u16) -> String {
        format!("goto {:x}", data)
    }

    /// Call pushes a return address and then changes I to the given location
    pub fn call(_registers: &mut Registers, _memory: &mut Memory, _data: u16) {
        unimplemented!("call");
    }

    pub fn call_to_string(data: u16) -> String {
        format!("call {:x}", data)
    }

    /// Extract the register from the opcode when the instruction has the form OXNN
    fn register_from_data(data: u16) -> u8 {
         ((data & REGISTER_MASK) >> 8) as u8
    }

    /// Extract the immediate from the opcode when the instruction has the form OXNN
    pub fn immediate_from_data(data: u16) -> u8 {
        (data & DATA_MASK) as u8
    }

    /// Extract both the register and immediate for instructions in the form OXNN
    pub fn register_and_immediate_from_data(data: u16) -> (u8, u8) {
        (Self::register_from_data(data), Self::immediate_from_data(data))
    }

    /// Checks if a register and an immediate value are equal. If they are equal then we
    /// skip the next instruction, otherwise we run the next instruction.
    pub fn reg_equal(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register, data) = Self::register_and_immediate_from_data(data);
        if registers.v[register as usize] == Wrapping(data) {
            registers.i += Wrapping(2);
        } else {
            registers.i += Wrapping(1);
        }
    }

    pub fn reg_equal_to_string(data: u16) -> String {
        let (register, data) = Self::register_and_immediate_from_data(data);
        format!("eq v{} {}", register, data)
    }

    /// Checks if a register and an immediate are not equal. If they are not equal then skip the
    /// next instruction, otherwise run the next instruction.
    pub fn reg_not_equal(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register, data) = Self::register_and_immediate_from_data(data);
        if registers.v[register as usize] != Wrapping(data) {
            registers.i += Wrapping(2);
        } else {
            registers.i += Wrapping(1);
        }
    }

    pub fn reg_not_equal_to_string(data: u16) -> String {
        let (register, data) = Self::register_and_immediate_from_data(data);
        format!("neq v{} {}", register, data)
    }

    pub fn op_table() -> [Self; 32] {
        let call_instruction = Self {
            desc: format!("call XXX"),
            execute: Self::mcall_display_or_flow,
            to_string: Self::mcall_display_or_flow_to_string
        };

        unimplemented!()
    }
}

pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers {
                v: [Wrapping(0); 16],
                i: Wrapping(0),
                stack: [Wrapping(0); 256],
                delay: Wrapping(0),
                sound: Wrapping(0)
            }
        }
    }

    pub fn step(memory: &mut Memory) {
    }
}
