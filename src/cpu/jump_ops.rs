use crate::cpu::instructions::JumpCondition;
use crate::cpu::registers::*;
use crate::cpu::CPU;

pub fn jp(cpu: &mut CPU, condition: JumpCondition) {
    let d16 = cpu.read_d16();

    if condition_is_true(cpu, condition) {
        cpu.registers.write_pc(d16);
    }
}

pub fn jp_hl(cpu: &mut CPU) {
    let hl_val = cpu.registers.read_16b(&Register16bits::HL);

    cpu.registers.write_pc(hl_val);
}

pub fn jr(cpu: &mut CPU, condition: JumpCondition) {
    let jmp = cpu.fetch_byte() as i8;

    if condition_is_true(cpu, condition) {
        let abs_jmp = i8::abs(jmp);

        if jmp > 0 {
            cpu.registers.increase_pc(abs_jmp as u16);
        } else {
            cpu.registers.decrease_pc(abs_jmp as u16);
        }
    }
}

pub fn rst(cpu: &mut CPU, offset: u8) {
    cpu.push_to_stack(cpu.registers.pc());
    cpu.registers.write_pc(u16::from(offset));
}

pub fn ret(cpu: &mut CPU, condition: JumpCondition) {
    if condition_is_true(cpu, condition) {
        let jp_addr = cpu.pop_from_stack();
        cpu.registers.write_pc(jp_addr);
    }
}

pub fn reti(cpu: &mut CPU) {
    let jp_addr = cpu.pop_from_stack();

    cpu.registers.write_pc(jp_addr);

    cpu.interrupts_enabled = true;
}

pub fn call(cpu: &mut CPU, condition: JumpCondition) {
    let a16 = cpu.read_a16();

    if condition_is_true(cpu, condition) {
        cpu.push_to_stack(cpu.registers.pc());
        cpu.registers.write_pc(a16);
    }
}

fn condition_is_true(cpu: &CPU, condition: JumpCondition) -> bool {
    match condition {
        JumpCondition::Always => true,
        JumpCondition::Z => cpu.registers.read_z_flag(),
        JumpCondition::NZ => !cpu.registers.read_z_flag(),
        JumpCondition::C => cpu.registers.read_c_flag(),
        JumpCondition::NC => !cpu.registers.read_c_flag(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn jp_unconditional() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);

        jp(&mut cpu, JumpCondition::Always);

        assert_eq!(cpu.registers.pc(), 0x0104);
    }

    #[test]
    fn jp_z_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_flags(true, false, false, false);

        jp(&mut cpu, JumpCondition::Z);

        assert_eq!(cpu.registers.pc(), 0x104);
    }

    #[test]
    fn jp_z_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_flags(false, false, false, false);

        jp(&mut cpu, JumpCondition::Z);

        assert_eq!(cpu.registers.pc(), initial_pc + 2);
    }

    #[test]
    fn jp_nz_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_flags(false, false, false, false);

        jp(&mut cpu, JumpCondition::NZ);

        assert_eq!(cpu.registers.pc(), 0x0104);
    }

    #[test]
    fn jp_nz_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_flags(true, false, false, false);

        jp(&mut cpu, JumpCondition::NZ);

        assert_eq!(cpu.registers.pc(), initial_pc + 2);
    }

    #[test]
    fn jp_c_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_flags(false, false, false, true);

        jp(&mut cpu, JumpCondition::C);

        assert_eq!(cpu.registers.pc(), 0x0104);
    }

    #[test]
    fn jp_c_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_flags(false, false, false, false);

        jp(&mut cpu, JumpCondition::C);

        assert_eq!(cpu.registers.pc(), initial_pc + 2);
    }

    #[test]
    fn jp_nc_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_flags(false, false, false, false);

        jp(&mut cpu, JumpCondition::NC);

        assert_eq!(cpu.registers.pc(), 0x0104);
    }

    #[test]
    fn jp_nc_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write_flags(false, false, false, true);

        jp(&mut cpu, JumpCondition::NC);

        assert_eq!(cpu.registers.pc(), initial_pc + 2);
    }

    #[test]
    fn jp_hl_op() {
        let mut mem = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        let hl_val = 0x8000;
        cpu.registers.write_16b(&Register16bits::HL, hl_val);

        jp_hl(&mut cpu);

        assert_eq!(cpu.registers.pc(), hl_val);
    }

    #[test]
    fn jr_unconditional() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        let jmp_val = 8;
        mem.write_byte(initial_pc, jmp_val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);

        jr(&mut cpu, JumpCondition::Always);

        assert_eq!(cpu.registers.pc(), initial_pc + 1 + jmp_val as u16);
    }

    #[test]
    fn jr_z_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        let jmp_val = 8;
        mem.write_byte(initial_pc, jmp_val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(true, false, false, false);
        cpu.registers.write_pc(initial_pc);

        jr(&mut cpu, JumpCondition::Z);

        assert_eq!(cpu.registers.pc(), initial_pc + 1 + jmp_val as u16);
    }

    #[test]
    fn jr_z_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        let jmp_val = 8;
        mem.write_byte(initial_pc, jmp_val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, false);
        cpu.registers.write_pc(initial_pc);

        jr(&mut cpu, JumpCondition::Z);

        assert_eq!(cpu.registers.pc(), initial_pc + 1);
    }

    #[test]
    fn jr_nz_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        let jmp_val = 8;
        mem.write_byte(initial_pc, jmp_val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, false);
        cpu.registers.write_pc(initial_pc);

        jr(&mut cpu, JumpCondition::NZ);

        assert_eq!(cpu.registers.pc(), initial_pc + 1 + jmp_val as u16);
    }

    #[test]
    fn jr_nz_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        let jmp_val = 8;
        mem.write_byte(initial_pc, jmp_val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(true, false, false, false);
        cpu.registers.write_pc(initial_pc);

        jr(&mut cpu, JumpCondition::NZ);

        assert_eq!(cpu.registers.pc(), initial_pc + 1);
    }

    #[test]
    fn jr_c_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        let jmp_val = 8;
        mem.write_byte(initial_pc, jmp_val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write_pc(initial_pc);

        jr(&mut cpu, JumpCondition::C);

        assert_eq!(cpu.registers.pc(), initial_pc + 1 + jmp_val as u16);
    }

    #[test]
    fn jr_c_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        let jmp_val = 8;
        mem.write_byte(initial_pc, jmp_val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, false);
        cpu.registers.write_pc(initial_pc);

        jr(&mut cpu, JumpCondition::C);

        assert_eq!(cpu.registers.pc(), initial_pc + 1);
    }

    #[test]
    fn jr_nc_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        let jmp_val = 8;
        mem.write_byte(initial_pc, jmp_val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, false);
        cpu.registers.write_pc(initial_pc);

        jr(&mut cpu, JumpCondition::NC);

        assert_eq!(cpu.registers.pc(), initial_pc + 1 + jmp_val as u16);
    }

    #[test]
    fn jr_nc_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        let jmp_val = 8;
        mem.write_byte(initial_pc, jmp_val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write_pc(initial_pc);

        jr(&mut cpu, JumpCondition::NC);

        assert_eq!(cpu.registers.pc(), initial_pc + 1);
    }

    #[test]
    fn rst_op() {
        let mut mem = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_pc = 0x1000;
        let initial_sp = 0x8082;
        cpu.registers.write_sp(initial_sp);
        cpu.registers.write_pc(initial_pc);

        rst(&mut cpu, 0x08);

        let final_sp = cpu.registers.sp();
        assert_eq!(final_sp, initial_sp - 2);
        assert_eq!(cpu.registers.pc(), 0x08);
        assert_eq!(mem.read_word(final_sp), initial_pc);
    }

    #[test]
    fn ret_unconditional() {
        let mut mem = Memory::new();
        let initial_sp = 0x8080;
        let jp_addr = 0x10;
        mem.write_byte(initial_sp, jp_addr);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_sp(initial_sp);

        ret(&mut cpu, JumpCondition::Always);

        assert_eq!(cpu.registers.sp(), initial_sp + 2);
        assert_eq!(cpu.registers.pc(), jp_addr as u16);
    }

    #[test]
    fn ret_z_true() {
        let mut mem = Memory::new();
        let initial_sp = 0x8080;
        let jp_addr = 0x10;
        mem.write_byte(initial_sp, jp_addr);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(true, false, false, false);
        cpu.registers.write_sp(initial_sp);

        ret(&mut cpu, JumpCondition::Z);

        assert_eq!(cpu.registers.sp(), initial_sp + 2);
        assert_eq!(cpu.registers.pc(), jp_addr as u16);
    }

    #[test]
    fn ret_z_false() {
        let mut mem = Memory::new();
        let initial_sp = 0x8080;
        let jp_addr = 0x10;
        mem.write_byte(initial_sp, jp_addr);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_pc = cpu.registers.pc();
        cpu.registers.write_flags(false, false, false, false);
        cpu.registers.write_sp(initial_sp);

        ret(&mut cpu, JumpCondition::Z);

        assert_eq!(cpu.registers.sp(), initial_sp);
        assert_eq!(cpu.registers.pc(), initial_pc);
    }

    #[test]
    fn ret_nz_true() {
        let mut mem = Memory::new();
        let initial_sp = 0x8080;
        let jp_addr = 0x10;
        mem.write_byte(initial_sp, jp_addr);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, false);
        cpu.registers.write_sp(initial_sp);

        ret(&mut cpu, JumpCondition::NZ);

        assert_eq!(cpu.registers.sp(), initial_sp + 2);
        assert_eq!(cpu.registers.pc(), jp_addr as u16);
    }

    #[test]
    fn ret_nz_false() {
        let mut mem = Memory::new();
        let initial_sp = 0x8080;
        let jp_addr = 0x10;
        mem.write_byte(initial_sp, jp_addr);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_pc = cpu.registers.pc();
        cpu.registers.write_flags(true, false, false, false);
        cpu.registers.write_sp(initial_sp);

        ret(&mut cpu, JumpCondition::NZ);

        assert_eq!(cpu.registers.sp(), initial_sp);
        assert_eq!(cpu.registers.pc(), initial_pc);
    }

    #[test]
    fn ret_c_true() {
        let mut mem = Memory::new();
        let initial_sp = 0x8080;
        let jp_addr = 0x10;
        mem.write_byte(initial_sp, jp_addr);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write_sp(initial_sp);

        ret(&mut cpu, JumpCondition::C);

        assert_eq!(cpu.registers.sp(), initial_sp + 2);
        assert_eq!(cpu.registers.pc(), jp_addr as u16);
    }

    #[test]
    fn ret_c_false() {
        let mut mem = Memory::new();
        let initial_sp = 0x8080;
        let jp_addr = 0x10;
        mem.write_byte(initial_sp, jp_addr);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_pc = cpu.registers.pc();
        cpu.registers.write_flags(false, false, false, false);
        cpu.registers.write_sp(initial_sp);

        ret(&mut cpu, JumpCondition::C);

        assert_eq!(cpu.registers.sp(), initial_sp);
        assert_eq!(cpu.registers.pc(), initial_pc);
    }

    #[test]
    fn ret_nc_true() {
        let mut mem = Memory::new();
        let initial_sp = 0x8080;
        let jp_addr = 0x10;
        mem.write_byte(initial_sp, jp_addr);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, false);
        cpu.registers.write_sp(initial_sp);

        ret(&mut cpu, JumpCondition::NC);

        assert_eq!(cpu.registers.sp(), initial_sp + 2);
        assert_eq!(cpu.registers.pc(), jp_addr as u16);
    }

    #[test]
    fn ret_nc_false() {
        let mut mem = Memory::new();
        let initial_sp = 0x8080;
        let jp_addr = 0x10;
        mem.write_byte(initial_sp, jp_addr);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_pc = cpu.registers.pc();
        cpu.registers.write_flags(false, false, false, true);
        cpu.registers.write_sp(initial_sp);

        ret(&mut cpu, JumpCondition::NC);

        assert_eq!(cpu.registers.sp(), initial_sp);
        assert_eq!(cpu.registers.pc(), initial_pc);
    }

    #[test]
    fn reti_op() {
        let mut mem = Memory::new();
        let initial_sp = 0x8080;
        let jp_addr = 0x10;
        mem.write_byte(initial_sp, jp_addr);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_sp(initial_sp);

        reti(&mut cpu);

        assert_eq!(cpu.registers.sp(), initial_sp + 2);
        assert_eq!(cpu.registers.pc(), jp_addr as u16);

        // TODO: test enable interrupts
    }

    #[test]
    fn call_unconditional() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_sp = 0x8000;
        cpu.registers.write_sp(initial_sp);
        cpu.registers.write_pc(initial_pc);

        call(&mut cpu, JumpCondition::Always);

        let final_sp = cpu.registers.sp();
        assert_eq!(cpu.registers.pc(), 0x0104);
        assert_eq!(final_sp, initial_sp - 2);
        assert_eq!(mem.read_word(final_sp), initial_pc + 2)
    }

    #[test]
    fn call_z_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_sp = 0x8000;
        cpu.registers.write_sp(initial_sp);
        cpu.registers.write_pc(initial_pc);

        cpu.registers.write_flags(true, false, false, false);

        call(&mut cpu, JumpCondition::Z);

        let final_sp = cpu.registers.sp();
        assert_eq!(cpu.registers.pc(), 0x0104);
        assert_eq!(final_sp, initial_sp - 2);
        assert_eq!(mem.read_word(final_sp), initial_pc + 2)
    }

    #[test]
    fn call_z_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_sp = 0x8000;
        cpu.registers.write_sp(initial_sp);
        cpu.registers.write_pc(initial_pc);

        cpu.registers.write_flags(false, false, false, false);

        call(&mut cpu, JumpCondition::Z);

        assert_eq!(cpu.registers.pc(), initial_pc + 2);
        assert_eq!(cpu.registers.sp(), initial_sp);
    }

    #[test]
    fn call_nz_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_sp = 0x8000;
        cpu.registers.write_sp(initial_sp);
        cpu.registers.write_pc(initial_pc);

        cpu.registers.write_flags(false, false, false, false);

        call(&mut cpu, JumpCondition::NZ);

        let final_sp = cpu.registers.sp();
        assert_eq!(cpu.registers.pc(), 0x0104);
        assert_eq!(final_sp, initial_sp - 2);
        assert_eq!(mem.read_word(final_sp), initial_pc + 2)
    }

    #[test]
    fn call_nz_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_sp = 0x8000;
        cpu.registers.write_sp(initial_sp);
        cpu.registers.write_pc(initial_pc);

        cpu.registers.write_flags(true, false, false, false);

        call(&mut cpu, JumpCondition::NZ);

        assert_eq!(cpu.registers.pc(), initial_pc + 2);
        assert_eq!(cpu.registers.sp(), initial_sp);
    }

    #[test]
    fn call_c_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_sp = 0x8000;
        cpu.registers.write_sp(initial_sp);
        cpu.registers.write_pc(initial_pc);

        cpu.registers.write_flags(false, false, false, true);

        call(&mut cpu, JumpCondition::C);

        let final_sp = cpu.registers.sp();
        assert_eq!(cpu.registers.pc(), 0x0104);
        assert_eq!(final_sp, initial_sp - 2);
        assert_eq!(mem.read_word(final_sp), initial_pc + 2)
    }

    #[test]
    fn call_c_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_sp = 0x8000;
        cpu.registers.write_sp(initial_sp);
        cpu.registers.write_pc(initial_pc);

        cpu.registers.write_flags(false, false, false, false);

        call(&mut cpu, JumpCondition::C);

        assert_eq!(cpu.registers.pc(), initial_pc + 2);
        assert_eq!(cpu.registers.sp(), initial_sp);
    }

    #[test]
    fn call_nc_true() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_sp = 0x8000;
        cpu.registers.write_sp(initial_sp);
        cpu.registers.write_pc(initial_pc);

        cpu.registers.write_flags(false, false, false, false);

        call(&mut cpu, JumpCondition::NC);

        let final_sp = cpu.registers.sp();
        assert_eq!(cpu.registers.pc(), 0x0104);
        assert_eq!(final_sp, initial_sp - 2);
        assert_eq!(mem.read_word(final_sp), initial_pc + 2)
    }

    #[test]
    fn call_nc_false() {
        let mut mem = Memory::new();
        let initial_pc = 0x10;
        mem.write_word(initial_pc, 0x104);
        let mut cpu: CPU = CPU::new(&mut mem);
        let initial_sp = 0x8000;
        cpu.registers.write_sp(initial_sp);
        cpu.registers.write_pc(initial_pc);

        cpu.registers.write_flags(false, false, false, true);

        call(&mut cpu, JumpCondition::NC);

        assert_eq!(cpu.registers.pc(), initial_pc + 2);
        assert_eq!(cpu.registers.sp(), initial_sp);
    }
}
