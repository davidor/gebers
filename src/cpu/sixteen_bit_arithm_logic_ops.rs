use crate::cpu::registers::*;
use crate::cpu::CPU;

pub fn add16(cpu: &mut CPU, register: Register16bits) {
    let register_val = cpu.registers.read_16b(&register);
    let register_a_val = cpu.registers.read_16b(&Register16bits::HL);
    let (new_val, overflow) = register_a_val.overflowing_add(register_val);

    cpu.registers.write_16b(&Register16bits::HL, new_val);

    cpu.registers.write_s_flag(false);
    cpu.registers
        .write_hc_flag(CPU::half_carry_in_add_16(register_a_val, register_val));
    cpu.registers.write_c_flag(overflow);
}

pub fn dec16(cpu: &mut CPU, register: Register16bits) {
    let initial_val = cpu.registers.read_16b(&register);
    let (new_val, _overflow) = initial_val.overflowing_sub(1);

    cpu.registers.write_16b(&register, new_val);
}

pub fn dec_sp(cpu: &mut CPU) {
    cpu.registers.decrease_sp(1);
}

pub fn inc16(cpu: &mut CPU, register: Register16bits) {
    let initial_val = cpu.registers.read_16b(&register);
    let (new_val, _overflow) = initial_val.overflowing_add(1);

    cpu.registers.write_16b(&register, new_val);
}

pub fn inc_sp(cpu: &mut CPU) {
    cpu.registers.increase_sp(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn add16_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, 1);
        cpu.registers.write_16b(&Register16bits::BC, 1);

        add16(&mut cpu, Register16bits::BC);

        assert_eq!(cpu.registers.read_16b(&Register16bits::HL), 2);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn add16_with_hc() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers
            .write_16b(&Register16bits::HL, 0b0000100000000000);
        cpu.registers
            .write_16b(&Register16bits::BC, 0b0000100000000000);

        add16(&mut cpu, Register16bits::BC);

        assert_eq!(
            cpu.registers.read_16b(&Register16bits::HL),
            0b0001000000000000
        );
        assert_eq!(cpu.registers.read_hc_flag(), true);
    }

    #[test]
    fn add16_with_carry() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers
            .write_16b(&Register16bits::HL, (2_u32.pow(16) - 1) as u16);
        cpu.registers.write_16b(&Register16bits::BC, 1);

        add16(&mut cpu, Register16bits::BC);

        assert_eq!(cpu.registers.read_16b(&Register16bits::HL), 0);
        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn dec16_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::BC, 2);

        dec16(&mut cpu, Register16bits::BC);

        assert_eq!(cpu.registers.read_16b(&Register16bits::BC), 1);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn dec16_with_val_0() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::BC, 0);

        dec16(&mut cpu, Register16bits::BC);

        assert_eq!(cpu.registers.read_16b(&Register16bits::BC), 0xFFFF);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn dec_sp_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_sp(0x8080);

        dec_sp(&mut cpu);

        assert_eq!(cpu.registers.sp(), 0x807F);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn inc16_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        inc16(&mut cpu, Register16bits::BC);

        assert_eq!(cpu.registers.read_16b(&Register16bits::BC), 1);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn inc16_with_overflow() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::BC, 0xFFFF);

        inc16(&mut cpu, Register16bits::BC);

        assert_eq!(cpu.registers.read_16b(&Register16bits::BC), 0);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn inc_sp_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_sp(0x8080);

        inc_sp(&mut cpu);

        assert_eq!(cpu.registers.sp(), 0x8081);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }
}
