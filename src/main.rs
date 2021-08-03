mod cpu;
mod machine;
mod memory;

use std::io::{self, Read};
use std::fs::File;
use std::env::args;
use machine::Machine;

fn from_file(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() -> io::Result<()> {
    let mut args = args().skip(1);
    let filepath = args.next().unwrap();
    let data = from_file(&filepath)?;
    let mut machine = Machine::of_bytes(data);

    loop {
        machine.step();
    }
}
