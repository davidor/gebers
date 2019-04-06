mod cpu;
mod memory;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let test_rom = "some_path"; // TODO

    let f = File::open(test_rom).unwrap();
    let mut rom = Vec::new();
    for byte in f.bytes() {
        rom.push(byte.unwrap());
    }

    let mut memory = memory::Memory::new();
    for (i, code) in rom.iter().enumerate() {
        memory.write_byte(i as u16, *code);
    }

    let mut cpu = cpu::CPU::new_at_0x100(&mut memory);

    loop {
        cpu.run_next_instruction();
    }
}
