mod cpu;
mod gpu;
mod interrupts;
mod memory;

extern crate minifb;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn main() {
    /*
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    */

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
    /*
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let gpu = memory.gpu();

        let tile_map = gpu.get_tile_map_0();

        //        for i in buffer.iter_mut() {
        //            *i = 0; // write something more funny here!
        //        }

        //32*32 tiles; each one has 8 x 8 bits.
        for row in 0..256 {
            for col in 0..256 {
                let tile_row = row / 32;
                let tile_col = col / 32;

                let bit_row = row % 8;
                let bit_col = col % 8;

                let pixel_value = tile_map[tile_row][tile_col][bit_row][bit_col];

                let color = match pixel_value {
                    gpu::TilePixelValue::Zero => 0x0f380f,
                    gpu::TilePixelValue::One => 0x306230,
                    gpu::TilePixelValue::Two => 0x8bac0f,
                    gpu::TilePixelValue::Three => 0x9bbc0f,
                };

                buffer[row * 256 + col] = color
            }
        }

        window.update_with_buffer(&buffer).unwrap();

        //cpu.run_next_instruction();
    }
    */
}
