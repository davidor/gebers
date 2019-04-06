use crate::cpu::CPU;

pub fn ccf(cpu: &mut CPU) {
    cpu.registers.write_s_flag(false);
    cpu.registers.write_hc_flag(false);
    cpu.registers.write_c_flag(!cpu.registers.read_c_flag());
}

pub fn scf(cpu: &mut CPU) {
    cpu.registers.write_s_flag(false);
    cpu.registers.write_hc_flag(false);
    cpu.registers.write_c_flag(true);
}

pub fn nop() {}

pub fn halt() {
    // TODO
}

pub fn stop() {
    // TODO
}

pub fn di(cpu: &mut CPU) {
    cpu.interrupts_enabled = false;
}

pub fn ei(cpu: &mut CPU) {
    cpu.interrupts_enabled = true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn ccf_with_carry_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);

        ccf(&mut cpu);

        assert_eq!(cpu.registers.read_c_flag(), false);
    }

    #[test]
    fn ccf_with_carry_not_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        ccf(&mut cpu);

        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn scf_with_carry_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_flags(false, false, false, true);

        scf(&mut cpu);

        assert_eq!(cpu.registers.read_c_flag(), true);
    }

    #[test]
    fn scf_with_carry_not_set() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);

        scf(&mut cpu);

        assert_eq!(cpu.registers.read_c_flag(), true);
    }
}
