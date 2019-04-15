pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

// A tile is a matrix of 8 x 8 bits. The GB can show 4 colors, so we need
// 2 bits to represent them. In total, a tile is 8*8*2 = 128 bits = 16 bytes.
const TILE_SIZE: usize = 16;

const TILES_IN_MEM: usize = 384;

const TILE_RAM_BEGIN: usize = VRAM_BEGIN;
const TILE_RAM_END: usize = 0x97FF;
const TILE_RAM_SIZE: usize = TILE_RAM_END - TILE_RAM_BEGIN;

const TILE_MAP_0_BEGIN: usize = 0x9800;
const TILE_MAP_0_END: usize = 0x9BFF;
const TILE_MAP_1_BEGIN: usize = 0x9C00;
const TILE_MAP_1_END: usize = VRAM_END;
const TILES_PER_MAP: usize = (TILE_MAP_0_END - TILE_MAP_0_BEGIN) / (TILE_SIZE);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

impl From<(u8, u8)> for TilePixelValue {
    fn from(bytes: (u8, u8)) -> TilePixelValue {
        let msb = bytes.0;
        let lsb = bytes.1;

        match (msb != 0, lsb != 0) {
            (false, false) => TilePixelValue::Zero,
            (false, true) => TilePixelValue::One,
            (true, false) => TilePixelValue::Two,
            (true, true) => TilePixelValue::Three,
        }
    }
}

impl From<TilePixelValue> for (u8, u8) {
    fn from(tile_pixel_value: TilePixelValue) -> (u8, u8) {
        match tile_pixel_value {
            TilePixelValue::Zero => (0, 0),
            TilePixelValue::One => (0, 1),
            TilePixelValue::Two => (1, 0),
            TilePixelValue::Three => (1, 1),
        }
    }
}

type Tile = [[TilePixelValue; 8]; 8];

fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

#[derive(Clone)]
pub struct GPU {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; TILES_IN_MEM],
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: [0; VRAM_SIZE],
            tile_set: [empty_tile(); TILES_IN_MEM],
        }
    }

    pub fn read_vram(&self, index: usize) -> u8 {
        self.vram[index]
    }

    pub fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;

        if index < TILE_RAM_SIZE {
            self.store_tile(index);
        } else {
            // TODO
        }
    }

    pub fn get_tile(&self, index: usize) -> &Tile {
        &self.tile_set[index]
    }

    pub fn get_tile_map_0(&self) -> [[&Tile; 32]; 32] {
        let mut res = [[self.get_tile(0); 32]; 32];

        let mut index = 0;
        for addr in TILE_MAP_0_BEGIN..TILE_MAP_0_END {
            let row = index / 32;
            let col = index % 32;

            let tile_index = self.vram[addr - VRAM_BEGIN];
            res[row][col] = &self.get_tile(tile_index.into());
            index += 1;
        }

        res
    }

    fn store_tile(&mut self, index: usize) {
        let (byte1, byte2) = self.bytes_for_tile(index);
        let tile_index = self.tile_index(index);
        let tile_row = self.tile_row(index);

        for pixel_index in 0..8 {
            let value: TilePixelValue = self.pixel_bits(byte1, byte2, pixel_index).into();

            self.tile_set[tile_index][tile_row][pixel_index] = value;
        }
    }

    fn bytes_for_tile(&self, index: usize) -> (u8, u8) {
        let normalized_index = index & 0xFFFE;
        (self.vram[normalized_index], self.vram[normalized_index + 1])
    }

    fn tile_index(&self, index: usize) -> usize {
        index / TILE_SIZE
    }

    fn tile_row(&self, index: usize) -> usize {
        (index % TILE_SIZE) / 2
    }

    fn pixel_bits(&self, byte1: u8, byte2: u8, pixel_index: usize) -> (u8, u8) {
        let mask = 1 << (7 - pixel_index);
        let msb = byte1 & mask;
        let lsb = byte2 & mask;

        (msb, lsb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_vram_to_modify_tile() {
        // Try to modify the 4th row of the 2nd tile with:
        // 01 10 11 01 (msb)
        // 10 00 01 10 (lsb)
        //
        // The value of the pixels should be:
        // 1 2 2 0 2 3 1 2
        // (nth pixel corresponds to nth position _from the left_).

        let tile_index = 2;
        let row_index = 4;
        let addr_byte_1 = 2 * 16 + (4 * 2);
        let addr_byte_2 = addr_byte_1 + 1;
        let mut gpu = GPU::new();

        gpu.write_vram(addr_byte_1, 0b0110_1101);
        gpu.write_vram(addr_byte_2, 0b1000_0110);

        let tile = gpu.get_tile(tile_index);
        let tile_row = tile[row_index];
        assert_eq!(
            tile_row,
            [
                TilePixelValue::One,
                TilePixelValue::Two,
                TilePixelValue::Two,
                TilePixelValue::Zero,
                TilePixelValue::Two,
                TilePixelValue::Three,
                TilePixelValue::One,
                TilePixelValue::Two,
            ]
        )
    }
}
