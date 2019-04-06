pub const IO_PORTS_BEGIN: usize = 0xFF00;
const SERIAL_TRANSFER_DATA: usize = 0xFF01;
const SERIAL_TRANSFER_CONTROL: usize = 0xFF02;

const MEMORY_SIZE: usize = 65_536;

#[derive(Clone)]
pub struct Memory {
    mem: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;

        // Blargg's test roms sent everything that is printed on the screen to
        // the game link port. That allows us to see the result of the tests
        // without implementing the graphics part.
        if address == SERIAL_TRANSFER_CONTROL as u16 {
            print!("{}", self.mem[SERIAL_TRANSFER_DATA] as char)
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = u16::from(self.read_byte(address));
        let high = u16::from(self.read_byte(address + 1));

        (high << 8) | low
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let low = (value & 0x00FF) as u8;
        let high = (value >> 8) as u8;

        self.write_byte(address, low);
        self.write_byte(address + 1, high);
    }
}
