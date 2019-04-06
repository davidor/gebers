use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Register8bits {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

// 16 bit virtual registers.
// These registers allow operations to read and write 16 bits instead of 8.
// There are 4: "af" ("a" and "f" combined), "bc", "de" and "hl".
// The first register represents the most significant byte.
#[derive(Debug, Eq, PartialEq)]
pub enum Register16bits {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlagsRegister {
    // Zero. Set when the result of an instruction is 0.
    z: bool,

    // Add/Sub. Indicates whether the instruction executed was an addition or
    // a subtraction.
    s: bool,

    // Half-carry.
    hc: bool,

    // Carry.
    c: bool,
}

// Each of the 4 most significant bits of the flags register corresponds to a
// flag. The other 4 bits are not used.
impl FlagsRegister {
    const ZERO_BIT: u8 = 7;
    const N_BIT: u8 = 6;
    const HC_BIT: u8 = 5;
    const C_BIT: u8 = 4;

    pub fn new() -> FlagsRegister {
        FlagsRegister {
            z: false,
            s: false,
            hc: false,
            c: false,
        }
    }
}

impl From<FlagsRegister> for u8 {
    fn from(f: FlagsRegister) -> u8 {
        let z_flag = if f.z { 1 } else { 0 };
        let s_flag = if f.s { 1 } else { 0 };
        let hc_flag = if f.hc { 1 } else { 0 };
        let c_flag = if f.c { 1 } else { 0 };

        (z_flag << FlagsRegister::ZERO_BIT)
            | (s_flag << FlagsRegister::N_BIT)
            | (hc_flag << FlagsRegister::HC_BIT)
            | (c_flag << FlagsRegister::C_BIT)
    }
}

impl From<u8> for FlagsRegister {
    fn from(value: u8) -> FlagsRegister {
        let z = value & (1 << FlagsRegister::ZERO_BIT) != 0;
        let s = value & (1 << FlagsRegister::N_BIT) != 0;
        let hc = value & (1 << FlagsRegister::HC_BIT) != 0;
        let c = value & (1 << FlagsRegister::C_BIT) != 0;

        FlagsRegister { z, s, hc, c }
    }
}

#[derive(Debug)]
pub struct ProgramCounter(u16);

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter(0)
    }

    pub fn address(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, address: u16) {
        self.0 = address
    }

    pub fn inc(&mut self, steps: u16) {
        self.0 = self.0.wrapping_add(steps);
    }

    pub fn dec(&mut self, steps: u16) {
        self.0 = self.0.wrapping_sub(steps);
    }
}

#[derive(Debug)]
pub struct StackPointer(u16);

impl StackPointer {
    pub fn new() -> StackPointer {
        StackPointer(0)
    }

    pub fn address(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, address: u16) {
        self.0 = address
    }

    pub fn inc(&mut self, bytes: u16) {
        let current = self.0;
        let (new, _overflow) = current.overflowing_add(bytes);
        self.0 = new
    }

    pub fn dec(&mut self, bytes: u16) {
        let current = self.0;
        let (new, _overflow) = current.overflowing_sub(bytes);
        self.0 = new
    }
}

#[derive(Debug)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    flags: FlagsRegister,

    pc: ProgramCounter,

    sp: StackPointer,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            flags: FlagsRegister::new(),

            pc: ProgramCounter::new(),

            sp: StackPointer::new(),
        }
    }

    pub fn read(&self, register: &Register8bits) -> u8 {
        match register {
            Register8bits::A => self.a,
            Register8bits::B => self.b,
            Register8bits::C => self.c,
            Register8bits::D => self.d,
            Register8bits::E => self.e,
            Register8bits::H => self.h,
            Register8bits::L => self.l,
        }
    }

    pub fn write(&mut self, register: &Register8bits, value: u8) {
        match register {
            Register8bits::A => self.a = value,
            Register8bits::B => self.b = value,
            Register8bits::C => self.c = value,
            Register8bits::D => self.d = value,
            Register8bits::E => self.e = value,
            Register8bits::H => self.h = value,
            Register8bits::L => self.l = value,
        }
    }

    pub fn read_16b(&self, register: &Register16bits) -> u16 {
        match register {
            Register16bits::AF => {
                let f: FlagsRegister = self.flags.clone();
                Registers::read_16b_from(self.a, f.into())
            }
            Register16bits::BC => Registers::read_16b_from(self.b, self.c),
            Register16bits::DE => Registers::read_16b_from(self.d, self.e),
            Register16bits::HL => Registers::read_16b_from(self.h, self.l),
        }
    }

    pub fn write_16b(&mut self, register: &Register16bits, value: u16) {
        let (high, low) = Registers::split_16b(value);

        match register {
            Register16bits::AF => {
                self.a = high;
                self.flags = low.into();
            }

            Register16bits::BC => {
                self.b = high;
                self.c = low;
            }

            Register16bits::DE => {
                self.d = high;
                self.e = low;
            }

            Register16bits::HL => {
                self.h = high;
                self.l = low;
            }
        }
    }

    pub fn write_flags(&mut self, z: bool, s: bool, hc: bool, c: bool) {
        self.flags.z = z;
        self.flags.s = s;
        self.flags.hc = hc;
        self.flags.c = c;
    }

    pub fn read_z_flag(&self) -> bool {
        self.flags.z
    }

    pub fn write_z_flag(&mut self, z: bool) {
        self.flags.z = z
    }

    pub fn read_s_flag(&self) -> bool {
        self.flags.s
    }

    pub fn write_s_flag(&mut self, s: bool) {
        self.flags.s = s
    }

    pub fn read_hc_flag(&self) -> bool {
        self.flags.hc
    }

    pub fn write_hc_flag(&mut self, hc: bool) {
        self.flags.hc = hc
    }

    pub fn read_c_flag(&self) -> bool {
        self.flags.c
    }

    pub fn write_c_flag(&mut self, c: bool) {
        self.flags.c = c
    }

    pub fn increase_pc(&mut self, steps: u16) {
        self.pc.inc(steps);
    }

    pub fn decrease_pc(&mut self, steps: u16) {
        self.pc.dec(steps);
    }

    pub fn write_pc(&mut self, value: u16) {
        self.pc.set(value);
    }

    pub fn increase_sp(&mut self, bytes: u16) {
        self.sp.inc(bytes);
    }

    pub fn decrease_sp(&mut self, bytes: u16) {
        self.sp.dec(bytes);
    }

    pub fn write_sp(&mut self, value: u16) {
        self.sp.set(value);
    }

    pub fn sp(&self) -> u16 {
        self.sp.address()
    }

    pub fn pc(&self) -> u16 {
        self.pc.address()
    }

    fn read_16b_from(r1: u8, r2: u8) -> u16 {
        (u16::from(r1)) << 8 | u16::from(r2)
    }

    fn split_16b(value: u16) -> (u8, u8) {
        let high = ((value & 0xFF00) >> 8) as u8;
        let low = (value & 0xFF) as u8;

        (high, low)
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "AF: {:X}; BC: {:X}; DE: {:X}; HL: {:X}\nSP: {:X}\nPC: {:X}",
            self.read_16b(&Register16bits::AF),
            self.read_16b(&Register16bits::BC),
            self.read_16b(&Register16bits::DE),
            self.read_16b(&Register16bits::HL),
            self.sp(),
            self.pc(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_register_from_u8() {
        assert_eq!(
            FlagsRegister {
                z: true,
                s: false,
                hc: false,
                c: false
            },
            FlagsRegister::from(0b1000_0000)
        );

        assert_eq!(
            FlagsRegister {
                z: false,
                s: true,
                hc: false,
                c: false
            },
            FlagsRegister::from(0b0100_0000)
        );

        assert_eq!(
            FlagsRegister {
                z: false,
                s: false,
                hc: true,
                c: false
            },
            FlagsRegister::from(0b0010_0000)
        );

        assert_eq!(
            FlagsRegister {
                z: false,
                s: false,
                hc: false,
                c: true
            },
            FlagsRegister::from(0b0001_0000)
        );

        assert_eq!(
            FlagsRegister {
                z: true,
                s: true,
                hc: false,
                c: false
            },
            FlagsRegister::from(0b1100_0000)
        );
    }

    #[test]
    fn u8_from_flags_register() {
        let val: u8 = FlagsRegister {
            z: true,
            s: false,
            hc: false,
            c: false,
        }
        .into();

        assert_eq!(val, 0x80);

        let val: u8 = FlagsRegister {
            z: false,
            s: true,
            hc: false,
            c: false,
        }
        .into();

        assert_eq!(val, 0x40);

        let val: u8 = FlagsRegister {
            z: false,
            s: false,
            hc: true,
            c: false,
        }
        .into();

        assert_eq!(val, 0x20);

        let val: u8 = FlagsRegister {
            z: false,
            s: false,
            hc: false,
            c: true,
        }
        .into();

        assert_eq!(val, 0x10);

        let val: u8 = FlagsRegister {
            z: false,
            s: false,
            hc: false,
            c: false,
        }
        .into();

        assert_eq!(val, 0);
    }
}
