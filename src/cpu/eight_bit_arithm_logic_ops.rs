use crate::cpu::registers::*;
use crate::cpu::CPU;

pub fn add(cpu: &mut CPU, register: Register8bits) {
    let value = cpu.registers.read(&register);
    add_value_in_register_a(cpu, value);
}

pub fn add_hl(cpu: &mut CPU) {
    let value = cpu.value_in_addr(&Register16bits::HL);
    add_value_in_register_a(cpu, value);
}

pub fn add_d8(cpu: &mut CPU) {
    let value = cpu.read_d8();
    add_value_in_register_a(cpu, value);
}

pub fn adc(cpu: &mut CPU, register: Register8bits) {
    let value = cpu.registers.read(&register);
    adc_value_in_register_a(cpu, value);
}

pub fn adc_hl(cpu: &mut CPU) {
    let value = cpu.value_in_addr(&Register16bits::HL);
    adc_value_in_register_a(cpu, value);
}

pub fn adc_d8(cpu: &mut CPU) {
    let value = cpu.read_d8();
    adc_value_in_register_a(cpu, value);
}

pub fn sub(cpu: &mut CPU, register: Register8bits) {
    let value = cpu.registers.read(&register);
    sub_value_in_register_a(cpu, value);
}

pub fn sub_hl(cpu: &mut CPU) {
    let value = cpu.value_in_addr(&Register16bits::HL);
    sub_value_in_register_a(cpu, value);
}

pub fn sub_d8(cpu: &mut CPU) {
    let value = cpu.read_d8();
    sub_value_in_register_a(cpu, value);
}

pub fn and(cpu: &mut CPU, register: Register8bits) {
    let value = cpu.registers.read(&register);
    and_value_in_register_a(cpu, value);
}

pub fn and_hl(cpu: &mut CPU) {
    let value = cpu.value_in_addr(&Register16bits::HL);
    and_value_in_register_a(cpu, value);
}

pub fn and_d8(cpu: &mut CPU) {
    let value = cpu.read_d8();
    and_value_in_register_a(cpu, value);
}

pub fn or(cpu: &mut CPU, register: Register8bits) {
    let value = cpu.registers.read(&register);
    or_value_in_register_a(cpu, value);
}

pub fn or_hl(cpu: &mut CPU) {
    let value = cpu.value_in_addr(&Register16bits::HL);
    or_value_in_register_a(cpu, value);
}

pub fn or_d8(cpu: &mut CPU) {
    let value = cpu.read_d8();
    or_value_in_register_a(cpu, value);
}

pub fn xor(cpu: &mut CPU, register: Register8bits) {
    let value = cpu.registers.read(&register);
    xor_value_in_register_a(cpu, value);
}

pub fn xor_hl(cpu: &mut CPU) {
    let value = cpu.value_in_addr(&Register16bits::HL);
    xor_value_in_register_a(cpu, value);
}

pub fn xor_d8(cpu: &mut CPU) {
    let value = cpu.read_d8();
    xor_value_in_register_a(cpu, value);
}

pub fn cp(cpu: &mut CPU, register: Register8bits) {
    let value = cpu.registers.read(&register);
    cp_value_in_register_a(cpu, value);
}

pub fn cp_hl(cpu: &mut CPU) {
    let value = cpu.value_in_addr(&Register16bits::HL);
    cp_value_in_register_a(cpu, value);
}

pub fn cp_d8(cpu: &mut CPU) {
    let value = cpu.read_d8();
    cp_value_in_register_a(cpu, value);
}

pub fn sbc(cpu: &mut CPU, register: Register8bits) {
    let value = cpu.registers.read(&register);
    sbc_value_in_register_a(cpu, value);
}

pub fn sbc_hl(cpu: &mut CPU) {
    let value = cpu.value_in_addr(&Register16bits::HL);
    sbc_value_in_register_a(cpu, value);
}

pub fn sbc_d8(cpu: &mut CPU) {
    let value = cpu.read_d8();
    sbc_value_in_register_a(cpu, value);
}

pub fn inc(cpu: &mut CPU, register: Register8bits) {
    let initial_val = cpu.registers.read(&register);
    let (new_val, _overflow) = initial_val.overflowing_add(1);

    cpu.registers.write(&register, new_val);

    write_flags_for_inc(cpu, initial_val, new_val)
}

pub fn inc_hl(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);
    let (new_val, _overflow) = initial_val.overflowing_add(1);

    cpu.memory.write_byte(mem_address, new_val);

    write_flags_for_inc(cpu, initial_val, new_val)
}

pub fn dec(cpu: &mut CPU, register: Register8bits) {
    let initial_val = cpu.registers.read(&register);
    let (new_val, _overflow) = initial_val.overflowing_sub(1);

    cpu.registers.write(&register, new_val);

    write_flags_for_dec(cpu, initial_val, new_val)
}

pub fn dec_hl(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);
    let (new_val, _overflow) = initial_val.overflowing_sub(1);

    cpu.memory.write_byte(mem_address, new_val);

    write_flags_for_dec(cpu, initial_val, new_val)
}

pub fn cpl(cpu: &mut CPU) {
    let initial_val = cpu.registers.read(&Register8bits::A);
    let new_val = initial_val ^ 0xFF;

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers.write_s_flag(true);
    cpu.registers.write_hc_flag(true);
}

pub fn daa(cpu: &mut CPU) {
    // Ref: https://ehaskins.com/2018-01-30%20Z80%20DAA/

    let value = cpu.registers.read(&Register8bits::A);

    let mut correction = 0;

    let mut set_c = false;

    if cpu.registers.read_hc_flag() || (!cpu.registers.read_s_flag() && (value & 0xf) > 9) {
        correction |= 0x6;
    }

    if cpu.registers.read_c_flag() || (!cpu.registers.read_s_flag() && value > 0x99) {
        correction |= 0x60;
        set_c = true;
    }

    let new_val = if cpu.registers.read_s_flag() {
        value.wrapping_sub(correction)
    } else {
        value.wrapping_add(correction)
    };

    cpu.registers.write(&Register8bits::A, new_val);

    let c = if set_c {
        true
    } else {
        cpu.registers.read_c_flag()
    };

    cpu.registers
        .write_flags(new_val == 0, cpu.registers.read_s_flag(), false, c);
}

fn add_value_in_register_a(cpu: &mut CPU, value: u8) {
    let register_a_val = cpu.registers.read(&Register8bits::A);
    let (new_val, overflow) = register_a_val.overflowing_add(value);

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers.write_flags(
        new_val == 0,
        false,
        half_carry_in_add(&[register_a_val, value]),
        overflow,
    );
}

fn adc_value_in_register_a(cpu: &mut CPU, value: u8) {
    let register_a_val = cpu.registers.read(&Register8bits::A);
    let (new_val, overflow1) = register_a_val.overflowing_add(value);
    let carry = if cpu.registers.read_c_flag() { 1 } else { 0 };
    let (new_val, overflow2) = new_val.overflowing_add(carry);

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers.write_flags(
        new_val == 0,
        false,
        half_carry_in_add(&[register_a_val, value, carry]),
        overflow1 || overflow2,
    );
}

fn sub_value_in_register_a(cpu: &mut CPU, value: u8) {
    let register_a_val = cpu.registers.read(&Register8bits::A);
    let (new_val, overflow) = register_a_val.overflowing_sub(value);

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers.write_flags(
        new_val == 0,
        true,
        half_carry_in_sub(&[register_a_val, value]),
        overflow,
    );
}

fn and_value_in_register_a(cpu: &mut CPU, value: u8) {
    let register_a_val = cpu.registers.read(&Register8bits::A);
    let new_val = register_a_val & value;

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers.write_flags(new_val == 0, false, true, false);
}

fn or_value_in_register_a(cpu: &mut CPU, value: u8) {
    let register_a_val = cpu.registers.read(&Register8bits::A);
    let new_val = register_a_val | value;

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers.write_flags(new_val == 0, false, false, false);
}

fn xor_value_in_register_a(cpu: &mut CPU, value: u8) {
    let register_a_val = cpu.registers.read(&Register8bits::A);
    let new_val = register_a_val ^ value;

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers.write_flags(new_val == 0, false, false, false);
}

fn cp_value_in_register_a(cpu: &mut CPU, value: u8) {
    let register_a_val = cpu.registers.read(&Register8bits::A);
    let (new_val, overflow) = register_a_val.overflowing_sub(value);

    cpu.registers.write_flags(
        new_val == 0,
        true,
        half_carry_in_sub(&[register_a_val, value]),
        overflow,
    );
}

fn sbc_value_in_register_a(cpu: &mut CPU, value: u8) {
    let register_a_val = cpu.registers.read(&Register8bits::A);
    let (new_val, overflow1) = register_a_val.overflowing_sub(value);
    let carry = if cpu.registers.read_c_flag() { 1 } else { 0 };
    let (new_val, overflow2) = new_val.overflowing_sub(carry);

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers.write_flags(
        new_val == 0,
        true,
        half_carry_in_sub(&[register_a_val, value, carry]),
        overflow1 || overflow2,
    );
}

fn write_flags_for_inc(cpu: &mut CPU, initial_val: u8, new_val: u8) {
    cpu.registers.write_z_flag(new_val == 0);
    cpu.registers.write_s_flag(false);
    cpu.registers
        .write_hc_flag(half_carry_in_add(&[initial_val, 1]));
}

fn write_flags_for_dec(cpu: &mut CPU, initial_val: u8, new_val: u8) {
    cpu.registers.write_z_flag(new_val == 0);
    cpu.registers.write_s_flag(true);
    cpu.registers
        .write_hc_flag(half_carry_in_sub(&[initial_val, 1]));
}

fn half_carry_in_add(values: &[u8]) -> bool {
    let sum: u8 = values.iter().map(|val: &u8| val & 0xF).sum();
    sum > 0xF
}

fn half_carry_in_sub(values: &[u8]) -> bool {
    let first = values[0];
    let sum_rest = values.iter().skip(1).map(|val: &u8| val & 0xF).sum();
    first & 0xF < sum_rest
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn add_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 2);
        cpu.registers.write(&Register8bits::A, 1);

        add(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 3);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn add_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        add(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_z_flag(), true)
    }

    #[test]
    fn add_with_hc() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 15);
        cpu.registers.write(&Register8bits::B, 1);

        add(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 16);
        assert_eq!(cpu.registers.read_hc_flag(), true)
    }

    #[test]
    fn add_with_carry() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 255);
        cpu.registers.write(&Register8bits::B, 2);

        add(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 1);
        assert_eq!(cpu.registers.read_c_flag(), true)
    }

    #[test]
    fn add_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);
        cpu.registers.write(&Register8bits::A, 1);

        add_hl(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 3);
    }

    #[test]
    fn add_d8_op() {
        let mut mem: Memory = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write(&Register8bits::A, 1);

        add_d8(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 3);
    }

    #[test]
    fn inc_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        inc(&mut cpu, Register8bits::C);
        assert_eq!(cpu.registers.read(&Register8bits::C), 1);
    }

    #[test]
    fn inc_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 255);

        inc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0);
        assert_eq!(cpu.registers.read_z_flag(), true)
    }

    #[test]
    fn inc_with_hc() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 15);

        inc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 16);
        assert_eq!(cpu.registers.read_hc_flag(), true)
    }

    #[test]
    fn inc_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        inc_hl(&mut cpu);

        assert_eq!(mem.read_byte(addr), 3);
    }

    #[test]
    fn adc_with_carry_flag_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write(&Register8bits::A, 1);
        cpu.registers.write(&Register8bits::B, 2);

        adc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 4);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn adc_without_carry_flag_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, false);
        cpu.registers.write(&Register8bits::A, 1);
        cpu.registers.write(&Register8bits::B, 2);

        adc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 3);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn adc_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        adc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_z_flag(), true)
    }

    #[test]
    fn adc_with_hc() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 15);
        cpu.registers.write(&Register8bits::B, 1);

        adc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 16);
        assert_eq!(cpu.registers.read_hc_flag(), true);
    }

    #[test]
    fn adc_with_carry() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 255);
        cpu.registers.write(&Register8bits::B, 2);

        adc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 1);
        assert_eq!(cpu.registers.read_c_flag(), true)
    }

    #[test]
    fn adc_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write_16b(&Register16bits::HL, addr);
        cpu.registers.write(&Register8bits::A, 1);

        adc_hl(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 4);
    }

    #[test]
    fn adc_d8_op() {
        let mut mem: Memory = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write(&Register8bits::A, 1);

        adc_d8(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 3);
    }

    #[test]
    fn sub_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 2);
        cpu.registers.write(&Register8bits::B, 1);

        sub(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 1);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), true);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn sub_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        sub(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_z_flag(), true)
    }

    #[test]
    fn sub_with_hc() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b10000100);
        cpu.registers.write(&Register8bits::B, 0b00001000);

        sub(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b01111100);
        assert_eq!(cpu.registers.read_hc_flag(), true);
    }

    #[test]
    fn sub_with_carry() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 1);

        sub(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 255);
        assert_eq!(cpu.registers.read_c_flag(), true)
    }

    #[test]
    fn sub_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);
        cpu.registers.write(&Register8bits::A, 10);

        sub_hl(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 8);
    }

    #[test]
    fn sub_d8_op() {
        let mut mem: Memory = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 3);
        cpu.registers.write_pc(initial_pc);

        sub_d8(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 1);
    }

    #[test]
    fn and_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b100);
        cpu.registers.write(&Register8bits::B, 0b101);

        and(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b100);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), true);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn and_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b100);
        cpu.registers.write(&Register8bits::B, 0b001);

        and(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_z_flag(), true);
    }

    #[test]
    fn and_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b101);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);
        cpu.registers.write(&Register8bits::A, 0b110);

        and_hl(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b100);
    }

    #[test]
    fn and_d8_op() {
        let mut mem: Memory = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 0b101);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write(&Register8bits::A, 0b110);

        and_d8(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b100);
    }

    #[test]
    fn or_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b100);
        cpu.registers.write(&Register8bits::B, 0b001);

        or(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b101);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn or_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        or(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_z_flag(), true);
    }

    #[test]
    fn or_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b101);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);
        cpu.registers.write(&Register8bits::A, 0b110);

        or_hl(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b111);
    }

    #[test]
    fn or_d8_op() {
        let mut mem: Memory = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 0b101);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b110);
        cpu.registers.write_pc(initial_pc);

        or_d8(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b111);
    }

    #[test]
    fn xor_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b11);
        cpu.registers.write(&Register8bits::B, 0b01);

        xor(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b10);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn xor_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b101);
        cpu.registers.write(&Register8bits::B, 0b101);

        xor(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_z_flag(), true);
    }

    #[test]
    fn xor_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b101);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);
        cpu.registers.write(&Register8bits::A, 0b110);

        xor_hl(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b011);
    }

    #[test]
    fn xor_d8_op() {
        let mut mem: Memory = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 0b101);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write(&Register8bits::A, 0b110);

        xor_d8(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b011);
    }

    #[test]
    fn cp_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 2);
        cpu.registers.write(&Register8bits::B, 1);

        cp(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), true);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn cp_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        cp(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read_z_flag(), true)
    }

    #[test]
    fn cp_with_hc() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b10000100);
        cpu.registers.write(&Register8bits::B, 0b00001000);

        cp(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b10000100);
        assert_eq!(cpu.registers.read_hc_flag(), true);
    }

    #[test]
    fn cp_with_carry() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 1);

        cp(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_c_flag(), true)
    }

    #[test]
    fn cp_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);
        cpu.registers.write(&Register8bits::A, 3);

        cp_hl(&mut cpu);

        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), true);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn cp_d8_op() {
        let mut mem: Memory = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 3);

        cp_d8(&mut cpu);

        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), true);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn dec_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::C, 2);

        dec(&mut cpu, Register8bits::C);

        assert_eq!(cpu.registers.read(&Register8bits::C), 1);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), true);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn dec_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 1);

        dec(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0);
        assert_eq!(cpu.registers.read_z_flag(), true)
    }

    #[test]
    fn dec_with_hc() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10000);

        dec(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b1111);
        assert_eq!(cpu.registers.read_hc_flag(), true)
    }

    #[test]
    fn dec_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        dec_hl(&mut cpu);

        assert_eq!(mem.read_byte(addr), 1);
    }

    #[test]
    fn sbc_with_carry_flag_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write(&Register8bits::A, 3);
        cpu.registers.write(&Register8bits::B, 1);

        sbc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 1);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), true);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn sbc_without_carry_flag_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, false);
        cpu.registers.write(&Register8bits::A, 3);
        cpu.registers.write(&Register8bits::B, 1);

        sbc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 2);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), true);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn sbc_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        sbc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_z_flag(), true)
    }

    #[test]
    fn sbc_with_hc() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b10000);
        cpu.registers.write(&Register8bits::B, 0b1000);

        sbc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b1000);
        assert_eq!(cpu.registers.read_hc_flag(), true);
    }

    #[test]
    fn sbc_with_carry() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 1);
        cpu.registers.write(&Register8bits::B, 2);

        sbc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::A), 255);
        assert_eq!(cpu.registers.read_c_flag(), true)
    }

    #[test]
    fn sbc_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 1);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write_16b(&Register16bits::HL, addr);
        cpu.registers.write(&Register8bits::A, 3);

        sbc_hl(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 1);
    }

    #[test]
    fn sbc_d8_op() {
        let mut mem: Memory = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 1);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write(&Register8bits::A, 3);

        sbc_d8(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 1);
    }

    #[test]
    fn cpl_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b10101010);

        cpl(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b01010101);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), true);
        assert_eq!(cpu.registers.read_hc_flag(), true);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }
}
