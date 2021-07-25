use log::{info, trace};
use std::num::Wrapping;
use crate::memory::Memory;

/// If opcode has the form _XN_ or _XR_ then the first register can be extracted with this mask
pub const REGISTER_MASK: u16 = 0x0F00;

/// If the opcode has the form _XR_ the second register can be extracted with this mask
pub const REGISTER_TWO_MASK: u16 = 0x00F0;

/// If opcodes have the form __II then the immediate value can be extracted with this mask
pub const DATA_MASK: u16 = 0x00FF;

/// If the opcode immediate contaisn only a single nibble of data (the final nibble of the opcode)
/// we extract it with this mask
pub const NIBBLE_DATA_MASK: u16 = 0x000F;

#[derive(Debug)]
pub struct Registers {

    /// The CHIP architecture has 16 8-bit general purpose registers.
    /// Register v[f] also doubles as the carry flag, collision flag, or borrow flag dependent on
    /// the operation.
  pub v: [Wrapping<u8>; 16],
  /// The program counter
  pub pc: Wrapping<u16>,
  /// The address register
  pub i: Wrapping<u16>,
  /// The stack is only used for return
  pub stack: [Wrapping<u8>; 256],

  /// The delay timer counts down to zero at 60hz
  pub delay: Wrapping<u8>,

  /// The sound timer emits a sound if it is not zero.
  /// This timer counts down to zero at 60hz and then stops.
  pub sound: Wrapping<u8>,
}

impl Registers {
    pub fn inc_pc(&mut self, val: u16) {
        self.pc += Wrapping(val);
    }
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

    /// Goto changes the PC pointer to the fixed location
    fn goto(registers: &mut Registers, memory: &mut Memory, data: u16) {
        registers.pc = Wrapping(data);
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
        registers.inc_pc(
        if registers.v[register as usize] == Wrapping(data) {
           2
        } else {
           1
        });
    }

    fn reg_equal_to_string(data: u16) -> String {
        let (register, data) = Self::register_and_immediate_from_data(data);
        format!("eq v{} {}", register, data)
    }

    /// Checks if a register and an immediate are not equal. If they are not equal then skip the
    /// next instruction, otherwise run the next instruction.
    fn reg_not_equal(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register, data) = Self::register_and_immediate_from_data(data);
        registers.inc_pc(
        if registers.v[register as usize] != Wrapping(data) {
           2
        } else {
           1
        });
    }

    fn reg_not_equal_to_string(data: u16) -> String {
        let (register, data) = Self::register_and_immediate_from_data(data);
        format!("neq v{} {}", register, data)
    }

    /// Checks if two registers are equal. If they are then skip the next instruction, otherwise
    /// run it.
    fn two_reg_equal(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register1, register2) = Self::two_registers_from_data(data);
        registers.inc_pc(if registers.v[register1] != registers.v[register2] { 2 } else { 1 });
    }

    fn two_reg_equal_to_string(data: u16) -> String {
        let (register1, register2) = Self::two_registers_from_data(data);
        format!("eq v{} v{}", register1, register2)
    }

    /// Load an immediate into a register
    fn load_immediate(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register, data) = Self::register_and_immediate_from_data(data);
        registers.v[register] = Wrapping(data);
    }

    fn load_immediate_to_string(data: u16) -> String {
        let (register, data) = Self::register_and_immediate_from_data(data);
        format!("ld v{} {}", register, data)
    }

    /// Same as load immediate but add it to the register rather than add
    fn add_immediate(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register, data) = Self::register_and_immediate_from_data(data);
        registers.v[register] = registers.v[register] + Wrapping(data);
        registers.inc_pc(1);
    }

    fn add_immediate_to_string(data: u16) -> String {
        let (register, data) = Self::register_and_immediate_from_data(data);
        format!("add v{} {}", register, data)
    }

    fn math_or_bitop(registers: &mut Registers, memory: &mut Memory, data: u16) {
        unimplemented!("math or binop row")
    }

    fn math_or_bitop_to_string(data: u16) -> String {
        unimplemented!("math or binop tostring")
    }

    fn two_registers_not_equal(registers: &mut Registers, memory: &mut Memory, data: u16) {
        let (register1, register2) = Self::two_registers_from_data(data);
        registers.inc_pc(if registers.v[register1] != registers.v[register2] { 2 } else { 1 });
    }

    fn two_registers_not_equal_to_string(data: u16) -> String {
        let (register1, register2) = Self::two_registers_from_data(data);
        format!("neq v{} v{}", register1, register2)
    }

    fn set_i(registers: &mut Registers, memory: &mut Memory, data: u16) {
        registers.i = Wrapping(data);
        registers.inc_pc(1);
    }

    fn set_i_to_string(data: u16) -> String {
        format!("ld i {:x}", data)
    }

    fn jump_immediate_plus_register(registers: &mut Registers, memory: &mut Memory, data: u16) {
        registers.pc = Wrapping(registers.v[0].0 as u16) + Wrapping(data);
    }

    fn jump_immediate_plus_register_to_string(data: u16) -> String {
        format!("jump v0 + {}", data)
    }

    /// The masked random instruction generates a random value between 0 and 255, masks it with an
    /// immediate (& imm) and then places it in a specified register.
    fn masked_random(registers: &mut Registers, memory: &mut Memory, data: u16) {
        println!("TODO: Rand");
        let (register, mask) = Self::register_and_immediate_from_data(data);
        registers.v[register] = Wrapping(45 & mask);
        registers.inc_pc(1);
    }

    fn masked_random_to_string(data: u16) -> String {
        let (register, mask) = Self::register_and_immediate_from_data(data);
        format!("rand v{} {}", register, mask)
    }

    fn draw_sprite(registers: &mut Registers, memory: &mut Memory, data: u16) {
        unimplemented!();
    }

    fn draw_sprite_to_string(data: u16) -> String {
        let (register1, register2) = Self::two_registers_from_data(data);
        let imm = data & NIBBLE_DATA_MASK;
        format!("draw v{} v{} {}", register1, register2, imm)
    }

    fn key_op(registers: &mut Registers, memory: &mut Memory, data: u16) {
        unimplemented!();
    }

    fn key_op_to_string(data: u16) -> String {
        unimplemented!();
    }

    fn load_or_store(registers: &mut Registers, memory: &mut Memory, data: u16) {
        unimplemented!();
    }

    fn load_or_store_to_string(data: u16) -> String {
        unimplemented!();
    }

    pub fn main_op_table() -> [Self; 15] {

        let call_instruction = Self {
            desc: format!("call XXX"),
            execute: Self::mcall_display_or_flow,
            to_string: Self::mcall_display_or_flow_to_string
        };

        let goto_instruction = Self {
            desc: format!("goto NNN"),
            execute: Self::goto,
            to_string: Self::goto_to_string,
        };

        let call_instruction = Self {
            desc: format!("call NNN"),
            execute: Self::call,
            to_string: Self::call_to_string,
        };

        let reg_eq = Self {
            desc: format!("eq vX II"),
            execute: Self::reg_equal,
            to_string: Self::reg_equal_to_string,
        };

        let reg_neq = Self {
            desc: format!("neq vX II"),
            execute: Self::reg_not_equal,
            to_string: Self::reg_not_equal_to_string,
        };

        let two_reg_eq = Self {
            desc: format!("eq Vx Vy"),
            execute: Self::two_reg_equal,
            to_string: Self::two_reg_equal_to_string,
        };

        let load_immediate = Self {
            desc: format!("ld Vx II"),
            execute: Self::load_immediate,
            to_string: Self::load_immediate_to_string,
        };

        let add_immediate = Self {
            desc: format!("add Vx II"),
            execute: Self::add_immediate,
            to_string: Self::add_immediate_to_string,
        };

        let math_or_bitop = Self {
            desc: format!("math or bitop"),
            execute: Self::math_or_bitop,
            to_string: Self::math_or_bitop_to_string,
        };

        let two_reg_not_equal = Self {
            desc: format!("neq Vx Vy"),
            execute: Self::two_registers_not_equal,
            to_string: Self::two_registers_not_equal_to_string,
        };

        let set_i = Self {
            desc: format!("ld I, NNN"),
            execute: Self::set_i,
            to_string: Self::set_i_to_string,
        };

        let jump_imm_plus_register = Self {
            desc: format!("jmp III + Vx"),
            execute: Self::jump_immediate_plus_register,
            to_string: Self::jump_immediate_plus_register_to_string,
        };

        let masked_random = Self {
            desc: format!("rand Vx & II"),
            execute: Self::masked_random,
            to_string: Self::masked_random_to_string,
        };

        let draw_sprite = Self {
            desc: format!("draw_sprite"),
            execute: Self::draw_sprite,
            to_string: Self::draw_sprite_to_string,
        };

        let key_op = Self {
            desc: format!("key"),
            execute: Self::key_op,
            to_string: Self::key_op_to_string,
        };

        let load_or_store = Self {
            desc: format!("load or store"),
            execute: Self::load_or_store,
            to_string: Self::load_or_store_to_string,
        };

        [call_instruction, goto_instruction, reg_eq, reg_neq, two_reg_eq, load_immediate, add_immediate, math_or_bitop, two_reg_not_equal, set_i, jump_imm_plus_register, masked_random, draw_sprite, key_op, load_or_store]
    }
}

pub struct Cpu {
    pub registers: Registers,
    pub main_op_table: [Instruction; 15],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers {
                pc: Wrapping(0),
                v: [Wrapping(0); 16],
                i: Wrapping(0),
                stack: [Wrapping(0); 256],
                delay: Wrapping(0),
                sound: Wrapping(0)
            },
            main_op_table: Instruction::main_op_table(),
        }
    }

    pub fn step(&mut self, memory: &mut Memory) {
        let next_opcode = memory.get16(self.registers.pc.0 as usize).0;
        let op_id = ((next_opcode & 0xF000) >> 12) as usize;
        trace!("ID: {:x} DATA: {:x}", op_id, next_opcode & 0x0FFF);
        (self.main_op_table[op_id].execute)(&mut self.registers, memory, next_opcode & 0x0FFF); 
    }
}

#[cfg(test)]
mod instruction_tests {
    use log::info;
    use crate::cpu::Cpu;
    use crate::cpu::Memory;
    use std::num::Wrapping;

	#[ctor::ctor]
	fn init() {
		let _ = env_logger::builder().is_test(true).try_init();
    }

    fn prepare_cpu() -> Cpu {
        let cpu = Cpu::new();
        cpu
    }

    fn assemble_goto(data: &mut [u8], address: u16) {
        data[0] = (1 << 4) | ((address >> 8) & 0x0F) as u8;
        data[1] = (address & 0x00FF) as u8;
    }

    #[test]
    fn goto() {
		let mut program = [0; 256];
		assemble_goto(&mut program, 0xAF);
		let mut memory = Memory::of_bytes(&program);
		let mut cpu = prepare_cpu();
        cpu.step(&mut memory);
		info!("{:?}", cpu.registers);
		assert!(cpu.registers.pc == Wrapping(0x00AF));
    }
}
