mod cpu;
mod machine;
mod memory;

use std::io::{self, Read};
use std::fs::File;
use std::env::args;
use crate::memory::Memory;
use machine::Machine;
use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

fn from_file(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn draw_frame(memory: &Memory, engine: &mut console_engine::ConsoleEngine) {
    engine.clear_screen();

    for y in 0..32 {
        for x in 0..64 {
            if memory.frame_buffer[x + (y * 64)] != 0 {
                engine.set_pxl(x as i32, y as i32, pixel::pxl_fg('*', Color::Cyan));
            }
        }
    }

    engine.draw();
}

fn main() -> io::Result<()> {
    env_logger::init();

    let mut args = args().skip(1);
    let filepath = args.next().unwrap();
    let data = from_file(&filepath)?;
    let mut machine = Machine::of_bytes(data);

    let mut engine = console_engine::ConsoleEngine::init(64, 32, 60).unwrap();

    loop {
        engine.wait_frame();

        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        for i in 0..9 {
            let key_char = ('0' as u8 + i) as char;
            if engine.is_key_pressed(KeyCode::Char(key_char)) {
                machine.set_key(i, true);
            } else {
                machine.set_key(i, false);
            }
        }

        machine.set_key(2, engine.is_key_pressed(KeyCode::Char('w')));
        machine.set_key(8, engine.is_key_pressed(KeyCode::Char('s')));

        machine.set_key(4, engine.is_key_pressed(KeyCode::Char('a')));
        machine.set_key(6, engine.is_key_pressed(KeyCode::Char('d')));

        for _ in 0..10 {
            machine.step();
        }

        if machine.sound() {
            print!("\x07");
        }

        draw_frame(&machine.memory, &mut engine);
    }

    Ok(())
}
