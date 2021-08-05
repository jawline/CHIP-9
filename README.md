### CHIP-9

This project implements as CHIP-8 emulator / interpreter in Rust and use the CLI and the input and output device, allowing you to play CHIP-8 games entirely in the terminal. The emulator is written in Rust with a custom emulation core and uses the console_engine library for IO.

![Screenshott](/screenshots/screen1.jpg?raw=true "Screenshot 1")
![Screenshott](/screenshots/screen2.jpg?raw=true "Screenshot 2")
![Screenshott](/screenshots/screen3.jpg?raw=true "Screenshot 3")
![Screenshott](/screenshots/screen4.jpg?raw=true "Screenshot 4")

#### General Structure

The CHIP-8 virtual machine is very straightforward, lacking virtual memory, allocation, or display synchronization and driving video out and sound through dedicated instructions. Following this, the design of our emulator is straightforward, with most of the implementation focused on emulating the individual opcodes. We split the design of our emulator into four data structures: CPU, Registers, Memory and Machine. The CPU holds handles to the registers and memory. The registers contain the current state of the stack, scratch registers, PC, I, and the sound and delay registers. The memory structure emulates the RAM of the machine, including the 4kb of R/W RAM and the additional ROM for text sprites. The machine joins each of these pieces together, and handles input conversion and the two CHIP-8 clocks (delay and sound).

#### CPU | ISA

CHIP-8 programs use fixed width two byte opcodes. The leading nibble in each opcode identifies the base instruction but there are 32 opcodes and only 16 possible assignments to a nibble so for some base opcodes other part of the opcode may be used to decide the final instruction. Annoyingly, this is not always the second nibble, so each of the extended instructions contains it's own op table.

To model this we maintain several different opcode tables. One for the base table, then one more for each opcode specific instruction. Instructions implementations are fetched by following the tables until we reach a base implementation.

The instructions are all stored big-endian and are generally straightforward in implementation. The exception to this is the mcall instruction which is meant to execute code in the host machines assembly. To avoid nesting machine specific emulators we do not treat this case, though it is generally unused in ROM's so it doesn't cause too many issues.

#### Memory

A CHIP-8 machine has 4kb of user addressable R/W RAM which is used for program code and data. It also has a small region of read only memory for storing sprites of the characters 0 through F. Memory is addressed through the 16-bit register I which is positioned using dedicated opcodes. There is also a 64x32 1-bit frame buffer which can only be interacted with through the clear display and draw sprite instructions.

#### Display

CHIP-8 systems use a 64 by 32 black and white display. The display is one bit, and is unable to show shades of gray. Internally this is represented through a boolean frame buffer with space for 64x32 boolean values. There is no vertical synchronization or double buffering logic in CHIP-8, instead the screen can be redrawn after every frame buffer operation. This can lead to visual artifacting but generally games design around this.

#### Sound

CHIP-8 can only play a sound through it's sound register. A sound while play whenever the value in the register is not zero. While the register is not zero it will tick down at a frequency of 60hz.
