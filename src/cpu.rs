use std::num::Wrapping;
use crate::memory::Memory;

/// If opcode has the form _XN_ or _XR_ then the first register can be extracted with this mask
pub const REGISTER_MASK: u16 = 0x0F00;

/// If the opcode has the form _XR_ the second register can be extracted with this mask
pub const REGISTER_TWO_MASK: u16 = 0x00F0;

/// If opcodes have the form __II then the immediate value can be extracted with this mask
pub const DATA_MASK: u16 = 0x00FF;

pub struct Registers {

    /// The CHIP architecture has 16 8-bit general purpose registers.
    /// Register v[f] also doubles as the carry flag, collision flag, or borrow flag dependent on
    /// the operation.
  pub v: [Wrapping<u8>; 16],

  /// The address register (PC) is named i
  pub i: Wrapping<u16>,
  /// The stack is only used for return
  pub stack: [Wrapping<u8>; 256],

  /// The delay timer counts down to zero at 60hz
  pub delay: Wrapping<u8>,

  /// The sound timer emits a sound if it is not zero.
  /// This timer counts down to zero at 60hz and then stops.
  pub sound: Wrapping<u8>,
}

pub struct Instruction {
    /// Rough description of the opcode from the first byte
    pub desc: String,
    /// Execute the opcode, with the change in state being reflected in registers and memory
    pub execute: fn(registers: &mut Registers, memory: &mut Memory, data: u16),
    /// Granular description of the opcode that requires the opcode data (not just the first byte)
    pub to_string: fn(data: u16) -> String,
}

impl Instruction {

    /// The zero opcode can be either clear display, ret, or machine call (Call an instruction
    /// written in machine code) depending on parameters. We merge these all into one opcode
    /// execution.
    fn mcall_display_or_flow(_registers: &mut Registers, _memory: &mut Memory, data: u16) {
        match data {
            0xE0 => unimplemented!("clear display"),
            0xEE => unimplemented!("ret"),
            _ => panic!("machine code routes are unsupported"),
        }
    }

    fn mcall_display_or_flow_to_string(data: u16) -> String {
       match data {
           0xE0 => format!("clear_display"),
           0xEE => format!("return"),
           _ => format!("mcall {:x}", data),
       }
    }

    /// Goto changes the I pointer to the fixed location
    fn goto(_registers: &mut Registers, _memory: &mut Memory, _data: u16) {
        unimplemented!("goto")
    }

    fn goto_to_string(data: u16) -> String {
        format!("goto {:x}", data)
    }

    /// Call pushes a return address and then changes I to the given location
    fn call(_registers: &mut Registers, _memory: &mut Memory, _data: u16) {
        unimplemented!("call");
    }

    fn call_to_string(data: u16) -> String {
        format!("call {:x}", data)
    }

    /// Extract the register from the opcode when the instruction has the form _R__
    fn register_from_data(data: u16) -> u8 {
         ((data & REGISTER_MASK) >> 8) as u8
    }

    /// Extract the register from the opcode when the register has the form __R_
    fn register_two_from_data(data: u16) -> u8 {
        ((data & REGISTER_TWO_MASK) >> 4) as u8
    }

    /// Extract the immediate from the opcode when the instruction has the form __II
    fn immediate_from_data(data: u16) -> u8 {
        (data & DATA_MASK) as u8
    }

    /// Extract both the register and immediate for instructions in the form _RII
    fn register_and_immediate_from_data(data: u16) -> (usize, u8) {
        (Self::register_from_data(data) as usize, Self::immediate_from_data(data))
    }

    /// Extract two registers from and opcode in the form _RV_
    fn two_registers_from_data(data: u16) -> (usize, usize) {
        (Self::register_from_data(data) as usize, Self::register_two_from_data(data) as usize)
    }

    /// Checks if a register and an immediate value are equal. If they are equal then we
    /// skip the next instruction, otherwise we run the next instruction.
    fn reg_equal(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register, data) = Self::register_and_immediate_from_data(data);
        if registers.v[register as usize] == Wrapping(data) {
            registers.i += Wrapping(2);
        } else {
            registers.i += Wrapping(1);
        }
    }

    fn reg_equal_to_string(data: u16) -> String {
        let (register, data) = Self::register_and_immediate_from_data(data);
        format!("eq v{} {}", register, data)
    }

    /// Checks if a register and an immediate are not equal. If they are not equal then skip the
    /// next instruction, otherwise run the next instruction.
    fn reg_not_equal(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register, data) = Self::register_and_immediate_from_data(data);
        if registers.v[register as usize] != Wrapping(data) {
            registers.i += Wrapping(2);
        } else {
            registers.i += Wrapping(1);
        }
    }

    fn reg_not_equal_to_string(data: u16) -> String {
        let (register, data) = Self::register_and_immediate_from_data(data);
        format!("neq v{} {}", register, data)
    }

    /// Checks if two registers are equal. If they are then skip the next instruction, otherwise
    /// run it.
    fn two_reg_equal(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register1, register2) = Self::two_registers_from_data(data);
        if registers.v[register1] == registers.v[register2] {
            registers.i += Wrapping(2);
        } else {
            registers.i += Wrapping(1);
        }
    }

    fn two_reg_equal_to_string(data: u16) -> String {
        let (register1, register2) = Self::two_registers_from_data(data);
        format!("eq v{} v{}", register1, register2)
    }

    /// Load an immediate into a register
    pub fn load_immediate(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register, data) = Self::register_and_immediate_from_data(data);
        registers.v[register] = Wrapping(data);
    }

    pub fn load_immediate_to_string(data: u16) -> String {
        let (register, data) = Self::register_and_immediate_from_data(data);
        format!("ld v{} {}", register, data)
    }

    /// Same as load immediate but add it to the register rather than add
    pub fn add_immediate(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register, data) = Self::register_and_immediate_from_data(data);
        registers.v[register] = registers.v[register] + Wrapping(data);
    }

    pub fn add_immediate_to_string(data: u16) -> String {
        let (register, data) = Self::register_and_immediate_from_data(data);
        format!("add v{} {}", register, data)
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
