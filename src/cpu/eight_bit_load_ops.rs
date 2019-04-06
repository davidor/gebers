use crate::cpu::registers::*;
use crate::cpu::CPU;

use crate::memory::IO_PORTS_BEGIN;

pub fn ld_r8_r8(cpu: &mut CPU, r1: Register8bits, r2: Register8bits) {
    let val_src = cpu.registers.read(&r2);

    cpu.registers.write(&r1, val_src);
}

pub fn ld_r8_d8(cpu: &mut CPU, register: Register8bits) {
    let data = cpu.read_d8();

    cpu.registers.write(&register, data);
}

pub fn ld_hl_d8(cpu: &mut CPU) {
    let data = cpu.read_d8();
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);

    cpu.memory.write_byte(mem_address, data);
}

pub fn ld_r8_addr(cpu: &mut CPU, r1: Register8bits, r2: Register8bits) {
    let mem_address = IO_PORTS_BEGIN as u16 + (u16::from(cpu.registers.read(&r2)));
    let data = cpu.memory.read_byte(mem_address);

    cpu.registers.write(&r1, data);
}

pub fn ld_addr_r8(cpu: &mut CPU, r1: Register8bits, r2: Register8bits) {
    let mem_address = IO_PORTS_BEGIN as u16 + (u16::from(cpu.registers.read(&r1)));
    let data = cpu.registers.read(&r2);

    cpu.memory.write_byte(mem_address, data);
}

pub fn ld_a_hli(cpu: &mut CPU) {
    let data = cpu.value_in_addr(&Register16bits::HL);

    cpu.registers.write(&Register8bits::A, data);

    let initial_val = cpu.registers.read_16b(&Register16bits::HL);
    let (new_val, _overflow) = initial_val.overflowing_add(1);

    cpu.registers.write_16b(&Register16bits::HL, new_val);
}

pub fn ld_a_hld(cpu: &mut CPU) {
    let data = cpu.value_in_addr(&Register16bits::HL);

    cpu.registers.write(&Register8bits::A, data);

    let initial_val = cpu.registers.read_16b(&Register16bits::HL);
    let (new_val, _overflow) = initial_val.overflowing_sub(1);

    cpu.registers.write_16b(&Register16bits::HL, new_val);
}

pub fn ld_hli_a(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let data = cpu.registers.read(&Register8bits::A);

    cpu.memory.write_byte(mem_address, data);

    let initial_val = cpu.registers.read_16b(&Register16bits::HL);
    let (new_val, _overflow) = initial_val.overflowing_add(1);

    cpu.registers.write_16b(&Register16bits::HL, new_val);
}

pub fn ld_hld_a(cpu: &mut CPU) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let data = cpu.registers.read(&Register8bits::A);

    cpu.memory.write_byte(mem_address, data);

    let initial_val = cpu.registers.read_16b(&Register16bits::HL);
    let (new_val, _overflow) = initial_val.overflowing_sub(1);

    cpu.registers.write_16b(&Register16bits::HL, new_val);
}

pub fn ld_hl_r8(cpu: &mut CPU, register: Register8bits) {
    let mem_address = cpu.registers.read_16b(&Register16bits::HL);
    let data = cpu.registers.read(&register);

    cpu.memory.write_byte(mem_address, data);
}

pub fn ld_r8_hl(cpu: &mut CPU, register: Register8bits) {
    let data = cpu.value_in_addr(&Register16bits::HL);

    cpu.registers.write(&register, data);
}

pub fn ld_r16_r8(cpu: &mut CPU, r16: Register16bits, r8: Register8bits) {
    let mem_address = cpu.registers.read_16b(&r16);
    let data = cpu.registers.read(&r8);

    cpu.memory.write_byte(mem_address, data);
}

pub fn ld_r8_r16(cpu: &mut CPU, r8: Register8bits, r16: Register16bits) {
    cpu.registers.write(&r8, cpu.value_in_addr(&r16));
}

pub fn ld_a8_a(cpu: &mut CPU) {
    let mem_address = (IO_PORTS_BEGIN as u16) + u16::from(cpu.fetch_byte());
    let data = cpu.registers.read(&Register8bits::A);

    cpu.memory.write_byte(mem_address, data);
}

pub fn ld_a_a8(cpu: &mut CPU) {
    let mem_address = (IO_PORTS_BEGIN as u16) + u16::from(cpu.fetch_byte());
    let data = cpu.memory.read_byte(mem_address);

    cpu.registers.write(&Register8bits::A, data);
}

pub fn ld_a16_a(cpu: &mut CPU) {
    let a16 = cpu.read_a16();
    let data = cpu.registers.read(&Register8bits::A);

    cpu.memory.write_byte(a16, data);
}

pub fn ld_a_a16(cpu: &mut CPU) {
    let a16 = cpu.read_a16();
    let data = cpu.memory.read_byte(a16);

    cpu.registers.write(&Register8bits::A, data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Memory;

    #[test]
    fn ld_r8_r8_op() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::C, 4);

        ld_r8_r8(&mut cpu, Register8bits::B, Register8bits::C);

        assert_eq!(cpu.registers.read(&Register8bits::B), 4);
    }

    #[test]
    fn ld_r8_r8_same_register() {
        let mut mem: Memory = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, 4);

        ld_r8_r8(&mut cpu, Register8bits::B, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 4);
    }

    #[test]
    fn ld_r8_d8_op() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 4);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);

        ld_r8_d8(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), 4);
    }

    #[test]
    fn ld_hl_d8_op() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 4);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, 8);
        cpu.registers.write_pc(initial_pc);

        ld_hl_d8(&mut cpu);

        assert_eq!(cpu.memory.read_byte(8), 4);
    }

    #[test]
    fn ld_r8_addr_op() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(IO_PORTS_BEGIN as u16 + 2, 4);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write(&Register8bits::C, 2);

        ld_r8_addr(&mut cpu, Register8bits::A, Register8bits::C);

        assert_eq!(cpu.registers.read(&Register8bits::A), 4);
    }

    #[test]
    fn ld_addr_r8_op() {
        let mut mem = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::C, 2);
        cpu.registers.write(&Register8bits::A, 4);

        ld_addr_r8(&mut cpu, Register8bits::C, Register8bits::A);

        assert_eq!(mem.read_byte(IO_PORTS_BEGIN as u16 + 2), 4);
    }

    #[test]
    fn ld_a_hli_op() {
        let addr = 4;
        let val = 2;
        let mut mem = Memory::new();
        mem.write_byte(addr, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        ld_a_hli(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), val);
        assert_eq!(cpu.registers.read_16b(&Register16bits::HL), addr + 1);
    }

    #[test]
    fn ld_a_hld_op() {
        let addr = 4;
        let val = 2;
        let mut mem = Memory::new();
        mem.write_byte(addr, val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        ld_a_hld(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), val);
        assert_eq!(cpu.registers.read_16b(&Register16bits::HL), addr - 1);
    }

    #[test]
    fn ld_hli_a_op() {
        let addr = 4;
        let val = 2;
        let mut mem = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, val);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        ld_hli_a(&mut cpu);

        assert_eq!(cpu.registers.read_16b(&Register16bits::HL), addr + 1);
        assert_eq!(mem.read_byte(addr), val)
    }

    #[test]
    fn ld_hld_a_op() {
        let addr = 4;
        let val = 2;
        let mut mem = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, val);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        ld_hld_a(&mut cpu);

        assert_eq!(cpu.registers.read_16b(&Register16bits::HL), addr - 1);
        assert_eq!(mem.read_byte(addr), val)
    }

    #[test]
    fn ld_hl_r8_op() {
        let addr = 4;
        let val = 2;
        let mut mem = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::B, val);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        ld_hl_r8(&mut cpu, Register8bits::B);

        assert_eq!(mem.read_byte(addr), val)
    }

    #[test]
    fn ld_r8_hl_op() {
        let addr = 4;
        let val = 2;
        let mut mem = Memory::new();
        mem.write_byte(addr, val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::HL, addr);

        ld_r8_hl(&mut cpu, Register8bits::B);

        assert_eq!(cpu.registers.read(&Register8bits::B), val)
    }

    #[test]
    fn ld_r16_r8_op() {
        let addr = 4;
        let val = 2;
        let mut mem = Memory::new();
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write(&Register8bits::A, val);
        cpu.registers.write_16b(&Register16bits::BC, addr);

        ld_r16_r8(&mut cpu, Register16bits::BC, Register8bits::A);

        assert_eq!(mem.read_byte(addr), val)
    }

    #[test]
    fn ld_r8_r16_op() {
        let addr = 4;
        let val = 2;
        let mut mem = Memory::new();
        mem.write_byte(addr, val);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_16b(&Register16bits::BC, addr);

        ld_r8_r16(&mut cpu, Register8bits::A, Register16bits::BC);

        assert_eq!(cpu.registers.read(&Register8bits::A), val)
    }

    #[test]
    fn ld_a8_a_op() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 4); // a8
        let addr = IO_PORTS_BEGIN as u16 + 4;
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write(&Register8bits::A, 2);

        ld_a8_a(&mut cpu);

        assert_eq!(mem.read_byte(addr), 2);
    }

    #[test]
    fn ld_a_a8_op() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        mem.write_byte(initial_pc, 4); // a8
        let addr = IO_PORTS_BEGIN as u16 + 4;
        mem.write_byte(addr, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);

        ld_a_a8(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 2);
    }

    #[test]
    fn ld_a16_a_op() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        let addr = 0x0104;
        mem.write_word(initial_pc, addr);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);
        cpu.registers.write(&Register8bits::A, 2);

        ld_a16_a(&mut cpu);

        assert_eq!(mem.read_byte(addr), 2);
    }

    #[test]
    fn ld_a_a16_op() {
        let mut mem = Memory::new();
        let initial_pc = 0x200;
        let addr = 0x0104;
        mem.write_word(initial_pc, addr);
        mem.write_byte(addr, 2);
        let mut cpu: CPU = CPU::new(&mut mem);
        cpu.registers.write_pc(initial_pc);

        ld_a_a16(&mut cpu);

        assert_eq!(cpu.registers.read(&Register8bits::A), 2);
    }
}
