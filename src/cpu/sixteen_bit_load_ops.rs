use crate::cpu::registers::*;
use crate::cpu::CPU;

pub fn ld_r16_d16(cpu: &mut CPU, register: Register16bits) {
    let d16 = cpu.read_d16();

    cpu.registers.write_16b(&register, d16);
}

pub fn ld_sp_d16(cpu: &mut CPU) {
    let d16 = cpu.read_d16();

    cpu.registers.write_sp(d16);
}

pub fn ld_a16_sp(cpu: &mut CPU) {
    let a16 = cpu.read_a16();

    cpu.memory.write_word(a16, cpu.registers.sp());
}

pub fn add_hl_sp(cpu: &mut CPU) {
    let sp = cpu.registers.sp();
    let register_hl_val = cpu.registers.read_16b(&Register16bits::HL);
    let (new_val, overflow) = register_hl_val.overflowing_add(sp);

    cpu.registers.write_16b(&Register16bits::HL, new_val);

    cpu.registers.write_s_flag(false);
    cpu.registers
        .write_hc_flag(CPU::half_carry_in_add_16(register_hl_val, sp));
    cpu.registers.write_c_flag(overflow);
}

pub fn ld_sp_hl(cpu: &mut CPU) {
    let register_hl_val = cpu.registers.read_16b(&Register16bits::HL);

    cpu.registers.write_sp(register_hl_val);
}

pub fn add_sp_r8(cpu: &mut CPU) {
    let sp_val = i32::from(cpu.registers.sp());
    let r8 = i32::from(cpu.fetch_byte() as i8);
    let new_val = sp_val.wrapping_add(r8);

    cpu.registers.write_sp(new_val as u16);

    cpu.registers.write_flags(
        false,
        false,
        (sp_val ^ r8 ^ new_val) & 0x10 != 0,
        (sp_val ^ r8 ^ new_val) & 0x100 != 0,
    );
}

pub fn ld_hl_sp_r8(cpu: &mut CPU) {
    let sp_val = i32::from(cpu.registers.sp());
    let r8 = i32::from(cpu.fetch_byte() as i8);
    let new_val = sp_val.wrapping_add(r8);

    cpu.registers.write_16b(&Register16bits::HL, new_val as u16);

    cpu.registers.write_flags(
        false,
        false,
        (sp_val ^ r8 ^ new_val) & 0x10 != 0,
        (sp_val ^ r8 ^ new_val) & 0x100 != 0,
    );
}

pub fn push(cpu: &mut CPU, register: Register16bits) {
    let data = cpu.registers.read_16b(&register);

    cpu.push_to_stack(data);
}

pub fn pop(cpu: &mut CPU, register: Register16bits) {
    let data = cpu.pop_from_stack();

    cpu.registers.write_16b(&register, data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn ld_r16_d16_op() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        let d16 = 0x8040;
        mem.write_word(initial_pc, d16);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);

        ld_r16_d16(&mut cpu, Register16bits::BC);

        assert_eq!(cpu.registers.read_16b(&Register16bits::BC), d16);
    }

    #[test]
    fn ld_sp_d16_op() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        let d16 = 0x8040;
        mem.write_word(initial_pc, d16);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);

        ld_sp_d16(&mut cpu);

        assert_eq!(cpu.registers.sp(), d16);
    }

    #[test]
    fn ld_a16_sp_op() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        let a16 = 0x0104;
        mem.write_word(initial_pc, a16);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_sp(2);

        ld_a16_sp(&mut cpu);

        assert_eq!(mem.read_byte(a16), 2);
    }

    #[test]
    fn add_hl_sp_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, 2);
        cpu.registers.write_sp(4);

        add_hl_sp(&mut cpu);

        assert_eq!(cpu.registers.read_16b(&Register16bits::HL), 6);
        assert_eq!(cpu.registers.read_z_flag(), false);
        assert_eq!(cpu.registers.read_s_flag(), false);
        assert_eq!(cpu.registers.read_hc_flag(), false);
        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn ld_sp_hl_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, 2);

        ld_sp_hl(&mut cpu);

        assert_eq!(cpu.registers.sp(), 2);
    }

    #[test]
    fn add_sp_r8_op() {
        let mut mem: Memory = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_sp(0x8000);

        add_sp_r8(&mut cpu);

        assert_eq!(cpu.registers.sp(), 0x8002)
    }

    #[test]
    fn add_sp_r8_negative() {
        let mut mem: Memory = Memory::new();
        let initial_pc = 0x200;
        let r8: i8 = -1;
        let initial_sp = 0x8000;
        mem.write_byte(initial_pc, r8.to_le_bytes()[0]);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_sp(initial_sp);

        add_sp_r8(&mut cpu);

        assert_eq!(cpu.registers.sp(), initial_sp - (i8::abs(r8) as u16));
    }

    #[test]
    fn ld_hl_sp_r8_op() {
        let mut mem: Memory = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_sp(0x8000);

        ld_hl_sp_r8(&mut cpu);

        assert_eq!(cpu.registers.read_16b(&Register16bits::HL), 0x8002);
    }

    #[test]
    fn push_op() {
        let mut mem = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_sp(0x8080);
        cpu.registers.write_16b(&Register16bits::BC, 0x4020);

        push(&mut cpu, Register16bits::BC);

        assert_eq!(cpu.registers.sp(), 0x807E);
        assert_eq!(mem.read_word(0x807E), 0x4020);
    }

    #[test]
    fn pop_op() {
        let mut mem = Memory::new();
        mem.write_word(0x8080, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_sp(0x8080);

        pop(&mut cpu, Register16bits::BC);

        assert_eq!(cpu.registers.sp(), 0x8082);
        assert_eq!(cpu.registers.read_16b(&Register16bits::BC), 0x104);
    }
}
