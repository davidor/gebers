use crate::cpu::registers::*;
use crate::cpu::CPU;

pub fn bit(cpu: &mut CPU, bit: u8, register: Register8bits) {
    let bit_value = cpu.registers.read(&register) & (0x1 << bit);
    write_flags_for_bit(cpu, bit_value)
}

pub fn bit_hl(cpu: &mut CPU, bit: u8) {
    let bit_value = cpu.value_in_addr(&Register16bits::HL) & (0x1 << bit);
    write_flags_for_bit(cpu, bit_value)
}

pub fn res(cpu: &mut CPU, bit: u8, register: Register8bits) {
    let mask = (0x1 << bit) ^ (0xFF);
    let initial_val = cpu.registers.read(&register);
    let new_val = initial_val & mask;

    cpu.registers.write(&register, new_val);
}

pub fn res_hl(cpu: &mut CPU, bit: u8) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);
    let mask = (0x1 << bit) ^ (0xFF);
    let new_val = initial_val & mask;

    cpu.memory.write_byte(mem_address, new_val);
}

pub fn set(cpu: &mut CPU, bit: u8, register: Register8bits) {
    let mask = 0x1 << bit;
    let initial_val = cpu.registers.read(&register);
    let new_val = initial_val | mask;

    cpu.registers.write(&register, new_val);
}

pub fn set_hl(cpu: &mut CPU, bit: u8) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);
    let mask = 0x1 << bit;
    let new_val = initial_val | mask;

    cpu.memory.write_byte(mem_address, new_val);
}

fn write_flags_for_bit(cpu: &mut CPU, bit_value: u8) {
    cpu.registers.write_z_flag(bit_value == 0);
    cpu.registers.write_s_flag(false);
    cpu.registers.write_hc_flag(true);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn bit_is_zero() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10);

        bit(&mut cpu, 2, Register8bits::B);

        assert_eq!(cpu.registers.read_z_flag(), true);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), true);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn bit_is_not_zero() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10);

        bit(&mut cpu, 1, Register8bits::B);

        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), true);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn bit_hl_is_zero() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        bit_hl(&mut cpu, 2);

        assert_eq!(cpu.registers.read_z_flag(), true);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), true);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn bit_hl_is_not_zero() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        bit_hl(&mut cpu, 1);

        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), true);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn res_zero_bit() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10);

        res(&mut cpu, 0, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b10);
    }

    #[test]
    fn res_non_zero_bit() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b11);

        res(&mut cpu, 1, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01);
    }

    #[test]
    fn res_hl_zero_bit() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        res_hl(&mut cpu, 0);

        assert_eq!(mem.read_byte(addr), 0b10);
    }

    #[test]
    fn res_hl_non_zero_bit() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b11);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        res_hl(&mut cpu, 0);

        assert_eq!(mem.read_byte(addr), 0b10);
    }

    #[test]
    fn set_zero_bit() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10);

        set(&mut cpu, 0, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b11);
    }

    #[test]
    fn set_non_zero_bit() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b11);

        set(&mut cpu, 1, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b11);
    }

    #[test]
    fn set_hl_zero_bit() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        set_hl(&mut cpu, 0);

        assert_eq!(mem.read_byte(addr), 0b11);
    }

    #[test]
    fn set_hl_non_zero_bit() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b11);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        set_hl(&mut cpu, 0);

        assert_eq!(mem.read_byte(addr), 0b11);
    }
}
