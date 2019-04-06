use crate::cpu::registers::*;
use crate::cpu::CPU;

pub fn rl(cpu: &mut CPU, register: Register8bits) {
    let initial_val = cpu.registers.read(&register);
    let initial_bit_7 = initial_val >> 7;
    let initial_c = cpu.registers.read_c_flag();
    let new_bit_0 = if initial_c { 1 } else { 0 };
    let new_val = initial_val << 1 | new_bit_0;

    cpu.registers.write(&register, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_7 == 1);
}

pub fn rla(cpu: &mut CPU) {
    let initial_val = cpu.registers.read(&Register8bits::A);
    let initial_bit_7 = initial_val >> 7;
    let initial_c = cpu.registers.read_c_flag();
    let new_bit_0 = if initial_c { 1 } else { 0 };
    let new_val = initial_val << 1 | new_bit_0;

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers
        .write_flags(false, false, false, initial_bit_7 == 1);
}

pub fn rl_hl(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);
    let initial_bit_7 = initial_val >> 7;
    let initial_c = cpu.registers.read_c_flag();
    let new_bit_0 = if initial_c { 1 } else { 0 };
    let new_val = initial_val << 1 | new_bit_0;

    cpu.memory.write_byte(mem_address, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_7 == 1);
}

pub fn rlc(cpu: &mut CPU, register: Register8bits) {
    let initial_val = cpu.registers.read(&register);
    let new_val = initial_val.rotate_left(1);

    cpu.registers.write(&register, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, new_val & 1 == 1);
}

pub fn rlca(cpu: &mut CPU) {
    let initial_val = cpu.registers.read(&Register8bits::A);
    let new_val = initial_val.rotate_left(1);

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers
        .write_flags(false, false, false, new_val & 1 == 1);
}

pub fn rlc_hl(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);
    let new_val = initial_val.rotate_left(1);

    cpu.memory.write_byte(mem_address, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, new_val & 1 == 1);
}

pub fn rr(cpu: &mut CPU, register: Register8bits) {
    let initial_val = cpu.registers.read(&register);
    let initial_bit_0 = initial_val & 1;
    let initial_c = cpu.registers.read_c_flag();
    let new_bit_7 = if initial_c { 1 } else { 0 };
    let new_val = (new_bit_7 << 7) | (initial_val >> 1);

    cpu.registers.write(&register, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_0 == 1);
}

pub fn rra(cpu: &mut CPU) {
    let initial_val = cpu.registers.read(&Register8bits::A);
    let initial_bit_0 = initial_val & 1;
    let initial_c = cpu.registers.read_c_flag();
    let new_bit_7 = if initial_c { 1 } else { 0 };
    let new_val = (new_bit_7 << 7) | (initial_val >> 1);

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers
        .write_flags(false, false, false, initial_bit_0 == 1);
}

pub fn rr_hl(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);
    let initial_bit_0 = initial_val & 1;
    let initial_c = cpu.registers.read_c_flag();
    let new_bit_7 = if initial_c { 1 } else { 0 };
    let new_val = (new_bit_7 << 7) | (initial_val >> 1);

    cpu.memory.write_byte(mem_address, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_0 == 1);
}

pub fn rrc(cpu: &mut CPU, register: Register8bits) {
    let initial_val = cpu.registers.read(&register);
    let initial_bit_0 = initial_val & 1;
    let new_val = initial_val.rotate_right(1);

    cpu.registers.write(&register, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_0 == 1);
}

pub fn rrca(cpu: &mut CPU) {
    let initial_val = cpu.registers.read(&Register8bits::A);
    let initial_bit_0 = initial_val & 1;
    let new_val = initial_val.rotate_right(1);

    cpu.registers.write(&Register8bits::A, new_val);

    cpu.registers
        .write_flags(false, false, false, initial_bit_0 == 1);
}

pub fn rrc_hl(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);
    let initial_bit_0 = initial_val & 1;
    let new_val = initial_val.rotate_right(1);

    cpu.memory.write_byte(mem_address, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_0 == 1);
}

pub fn sla(cpu: &mut CPU, register: Register8bits) {
    let initial_val = cpu.registers.read(&register);
    let initial_bit_7 = initial_val >> 7;
    let new_val = initial_val << 1;

    cpu.registers.write(&register, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_7 == 1);
}

pub fn sla_hl(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);
    let initial_bit_7 = initial_val >> 7;
    let new_val = initial_val << 1;

    cpu.memory.write_byte(mem_address, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_7 == 1);
}

pub fn sra(cpu: &mut CPU, register: Register8bits) {
    let initial_val = cpu.registers.read(&register);
    let initial_bit_7 = initial_val >> 7;
    let initial_bit_0 = initial_val & 1;
    let new_val = (initial_val >> 1) | (initial_bit_7 << 7);

    cpu.registers.write(&register, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_0 == 1);
}

pub fn sra_hl(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);

    let initial_bit_7 = initial_val >> 7;
    let initial_bit_0 = initial_val & 1;
    let new_val = (initial_val >> 1) | (initial_bit_7 << 7);

    cpu.memory.write_byte(mem_address, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_0 == 1);
}

pub fn srl(cpu: &mut CPU, register: Register8bits) {
    let initial_val = cpu.registers.read(&register);
    let initial_bit_0 = initial_val & 1;
    let new_val = initial_val >> 1;

    cpu.registers.write(&register, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_0 == 1);
}

pub fn srl_hl(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);
    let initial_bit_0 = initial_val & 1;
    let new_val = initial_val >> 1;

    cpu.memory.write_byte(mem_address, new_val);

    cpu.registers
        .write_flags(new_val == 0, false, false, initial_bit_0 == 1);
}

pub fn swap(cpu: &mut CPU, register: Register8bits) {
    let initial_val = cpu.registers.read(&register);
    let high = initial_val & 0xF0;
    let low = initial_val & 0x0F;
    let new_val = (low << 4) | (high >> 4);

    cpu.registers.write(&register, new_val);

    cpu.registers.write_flags(new_val == 0, false, false, false);
}

pub fn swap_hl(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let initial_val = cpu.memory.read_byte(mem_address);
    let high = initial_val & 0xF0;
    let low = initial_val & 0x0F;
    let new_val = (low << 4) | (high >> 4);

    cpu.memory.write_byte(mem_address, new_val);

    cpu.registers.write_flags(new_val == 0, false, false, false);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn rl_with_carry_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write(&Register8bits::B, 0b00101111);

        rl(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01011111);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rl_with_carry_not_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b00101111);

        rl(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01011110);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rl_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        rl(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0);
        assert_eq!(cpu.registers.read_z_flag(), true);
    }

    #[test]
    fn rl_old_bit_7_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b00101111);

        rl(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01011110);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rl_old_bit_7_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10101111);

        rl(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01011110);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn rla_with_carry_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write(&Register8bits::A, 0b00101111);

        rla(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b01011111);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rla_with_carry_not_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b00101111);

        rla(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b01011110);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rla_old_bit_7_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b00101111);

        rla(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b01011110);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rla_old_bit_7_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b10101111);

        rla(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b01011110);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn rla_result_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        rla(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rl_hl_with_carry_set() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b00101111);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        rl_hl(&mut cpu);

        assert_eq!(mem.read_byte(addr), 0b01011111);
    }

    #[test]
    fn rl_hl_with_carry_not_set() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b00101111);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        rl_hl(&mut cpu);

        assert_eq!(mem.read_byte(addr), 0b01011110);
    }

    #[test]
    fn rlc_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b00101111);

        rlc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01011110);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rlc_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        rlc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0);
        assert_eq!(cpu.registers.read_z_flag(), true);
    }

    #[test]
    fn rlc_old_bit_7_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b00101100);

        rlc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01011000);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rlc_old_bit_7_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10101111);

        rlc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01011111);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn rlca_old_bit_7_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b00101100);

        rlca(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b01011000);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rlca_old_bit_7_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b10101111);

        rlca(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b01011111);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn rlca_result_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        rlca(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rlc_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10101111);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        rlc_hl(&mut cpu);

        assert_eq!(mem.read_byte(addr), 0b01011111)
    }

    #[test]
    fn rr_with_carry_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write(&Register8bits::B, 0b00101110);

        rr(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b10010111);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rr_with_carry_not_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b00101110);

        rr(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b00010111);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rr_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        rr(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0);
        assert_eq!(cpu.registers.read_z_flag(), true);
    }

    #[test]
    fn rr_old_bit_0_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b00100000);

        rr(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b00010000);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rr_old_bit_0_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b00101111);

        rr(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b00010111);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn rra_with_carry_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write(&Register8bits::A, 0b00101110);

        rra(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b10010111);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rra_with_carry_not_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b00101110);

        rra(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b00010111);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rra_old_bit_0_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b00100000);

        rra(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b00010000);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rra_old_bit_0_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b00101111);

        rra(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b00010111);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn rra_result_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        rra(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rr_hl_with_carry_set() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b00101110);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        rr_hl(&mut cpu);

        assert_eq!(mem.read_byte(addr), 0b10010111);
    }

    #[test]
    fn rr_hl_with_carry_not_set() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b00101110);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        rr_hl(&mut cpu);

        assert_eq!(mem.read_byte(addr), 0b00010111);
    }

    #[test]
    fn rrc_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10101110);

        rrc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01010111);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rrc_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        rrc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0);
        assert_eq!(cpu.registers.read_z_flag(), true);
    }

    #[test]
    fn rrc_old_bit_0_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10000000);

        rrc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01000000);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rrc_old_bit_0_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10000001);

        rrc(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b11000000);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn rrca_old_bit_0_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b10000000);

        rrca(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b01000000);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rrca_old_bit_0_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, 0b10000001);

        rrca(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0b11000000);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn rrca_result_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        rrca(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 0);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn rrc_hl_old_bit_0_is_0() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10000000);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        rrc_hl(&mut cpu);

        assert_eq!(cpu.registers.read_c_flag(), false);
        assert_eq!(mem.read_byte(addr), 0b01000000);
    }

    #[test]
    fn rrc_hl_old_bit_0_is_1() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10000001);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        rrc_hl(&mut cpu);

        assert_eq!(cpu.registers.read_c_flag(), true);
        assert_eq!(mem.read_byte(addr), 0b11000000);
    }

    #[test]
    fn sla_old_bit_7_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b00101110);

        sla(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01011100);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn sla_old_bit_7_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10101110);

        sla(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01011100);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn sla_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        sla(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0);
        assert_eq!(cpu.registers.read_z_flag(), true);
    }

    #[test]
    fn sla_hl_old_bit_7_is_0() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b00101110);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        sla_hl(&mut cpu);

        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
        assert_eq!(mem.read_byte(addr), 0b01011100);
    }

    #[test]
    fn sla_hl_old_bit_7_is_1() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10101110);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        sla_hl(&mut cpu);

        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), true);
        assert_eq!(mem.read_byte(addr), 0b01011100);
    }

    #[test]
    fn sra_old_bit_7_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b00101110);

        sra(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b00010111);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn sra_old_bit_7_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10101110);

        sra(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b11010111);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn sra_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        sra(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0);
        assert_eq!(cpu.registers.read_z_flag(), true);
    }

    #[test]
    fn sra_old_bit_0_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b00101110);

        sra(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b00010111);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn sra_old_bit_0_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b00101111);

        sra(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b00010111);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn sra_hl_old_bit_7_is_0() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b00101110);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        sra_hl(&mut cpu);

        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
        assert_eq!(mem.read_byte(addr), 0b00010111);
    }

    #[test]
    fn sra_hl_old_bit_7_is_1() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10101110);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        sra_hl(&mut cpu);

        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
        assert_eq!(mem.read_byte(addr), 0b11010111);
    }

    #[test]
    fn srl_old_bit_0_is_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10101110);

        srl(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01010111);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn srl_old_bit_0_is_1() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10101111);

        srl(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01010111);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn srl_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        srl(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0);
        assert_eq!(cpu.registers.read_z_flag(), true);
    }

    #[test]
    fn srl_hl_old_bit_0_is_0() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10101110);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        srl_hl(&mut cpu);

        assert_eq!(cpu.registers.read_c_flag(), false);
        assert_eq!(mem.read_byte(addr), 0b01010111);
    }

    #[test]
    fn srl_hl_old_bit_0_is_1() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10101111);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        srl_hl(&mut cpu);

        assert_eq!(cpu.registers.read_c_flag(), true);
        assert_eq!(mem.read_byte(addr), 0b01010111);
    }

    #[test]
    fn swap_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 0b10100101);

        swap(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0b01011010);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn swap_equals_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        swap(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 0);
        assert_eq!(cpu.registers.read_z_flag(), true);
    }

    #[test]
    fn swap_hl_op() {
        let mut mem: Memory = Memory::new();
        let addr = 1;
        mem.write_byte(addr, 0b10100101);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        swap_hl(&mut cpu);

        assert_eq!(mem.read_byte(addr), 0b01011010);
    }
}
