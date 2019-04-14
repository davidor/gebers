use crate::cpu::bit_ops::*;
use crate::cpu::control_ops::*;
use crate::cpu::eight_bit_arithm_logic_ops::*;
use crate::cpu::eight_bit_load_ops::*;
use crate::cpu::instructions::*;
use crate::cpu::jump_ops::*;
use crate::cpu::registers::*;
use crate::cpu::rotate_ops::*;
use crate::cpu::sixteen_bit_arithm_logic_ops::*;
use crate::cpu::sixteen_bit_load_ops::*;
use crate::memory::Memory;

mod bit_ops;
mod control_ops;
mod eight_bit_arithm_logic_ops;
mod eight_bit_load_ops;
mod jump_ops;
mod rotate_ops;
mod sixteen_bit_arithm_logic_ops;
mod sixteen_bit_load_ops;

mod instructions;
mod registers;

pub struct CPU<'memory> {
    registers: Registers,
    memory: &'memory mut Memory,
    interrupts_enabled: bool,
}

impl<'memory> CPU<'memory> {
    pub fn new(memory: &'memory mut Memory) -> Self {
        CPU {
            registers: Registers::new(),
            memory,
            interrupts_enabled: true,
        }
    }

    /// ROMs start running at PC = 0x100 after the bootloader.
    /// This method creates a new CPU setting the registers and memory with the
    /// values when PC = 0x100 I observed when debugging test ROMs using the
    /// BGB emulator.
    pub fn new_at_0x100(memory: &'memory mut Memory) -> Self {
        let mut registers = Registers::new();

        registers.write_pc(0x100);
        registers.write_sp(0xFFFE);

        registers.write_16b(&Register16bits::AF, 0x1180);
        registers.write_16b(&Register16bits::BC, 0x0000);
        registers.write_16b(&Register16bits::DE, 0xFF56);
        registers.write_16b(&Register16bits::HL, 0x000D);

        registers.write_flags(true, false, false, false);

        memory.write_byte(0xFF0F, 0xE1);

        CPU {
            registers,
            memory,
            interrupts_enabled: false,
        }
    }

    pub fn run_next_instruction(&mut self) {
        if self.attend_pending_interrupt() {
            return;
        }

        let mut instruction_byte = self.fetch_byte();

        let prefixed = instruction_byte == instructions::PREFIX_INSTR_CODE;

        let instruction = if prefixed {
            instruction_byte = self.fetch_byte();
            Instruction::decode_prefixed(instruction_byte)
        } else {
            Instruction::decode(instruction_byte)
        };

        self.execute(instruction);
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            //  8-bit arithmetic and logic
            Instruction::ADD(register) => add(self, register),
            Instruction::ADDHL => add_hl(self),
            Instruction::ADDD8 => add_d8(self),
            Instruction::INC(register) => inc(self, register),
            Instruction::INCHL => inc_hl(self),
            Instruction::ADC(register) => adc(self, register),
            Instruction::ADCHL => adc_hl(self),
            Instruction::ADCD8 => adc_d8(self),
            Instruction::SUB(register) => sub(self, register),
            Instruction::SUBHL => sub_hl(self),
            Instruction::SUBD8 => sub_d8(self),
            Instruction::AND(register) => and(self, register),
            Instruction::ANDHL => and_hl(self),
            Instruction::ANDD8 => and_d8(self),
            Instruction::OR(register) => or(self, register),
            Instruction::ORHL => or_hl(self),
            Instruction::ORD8 => or_d8(self),
            Instruction::XOR(register) => xor(self, register),
            Instruction::XORHL => xor_hl(self),
            Instruction::XORD8 => xor_d8(self),
            Instruction::CP(register) => cp(self, register),
            Instruction::CPHL => cp_hl(self),
            Instruction::CPD8 => cp_d8(self),
            Instruction::DEC(register) => dec(self, register),
            Instruction::DECHL => dec_hl(self),
            Instruction::SBC(register) => sbc(self, register),
            Instruction::SBCHL => sbc_hl(self),
            Instruction::SBCD8 => sbc_d8(self),
            Instruction::CPL => cpl(self),
            Instruction::DAA => daa(self),

            // 16-bit arithmetic and logic
            Instruction::ADD16(register) => add16(self, register),
            Instruction::DEC16(register) => dec16(self, register),
            Instruction::DECSP => dec_sp(self),
            Instruction::INC16(register) => inc16(self, register),
            Instruction::INCSP => inc_sp(self),

            // Bit operations
            Instruction::BIT(bit_n, register) => bit(self, bit_n, register),
            Instruction::BITHL(bit) => bit_hl(self, bit),
            Instruction::RES(bit, register) => res(self, bit, register),
            Instruction::RESHL(bit) => res_hl(self, bit),
            Instruction::SET(bit, register) => set(self, bit, register),
            Instruction::SETHL(bit) => set_hl(self, bit),

            // Rotates and shifts
            Instruction::RL(register) => rl(self, register),
            Instruction::RLA => rla(self),
            Instruction::RLHL => rl_hl(self),
            Instruction::RLC(register) => rlc(self, register),
            Instruction::RLCA => rlca(self),
            Instruction::RLCHL => rlc_hl(self),
            Instruction::RR(register) => rr(self, register),
            Instruction::RRA => rra(self),
            Instruction::RRHL => rr_hl(self),
            Instruction::RRC(register) => rrc(self, register),
            Instruction::RRCA => rrca(self),
            Instruction::RRCHL => rrc_hl(self),
            Instruction::SLA(register) => sla(self, register),
            Instruction::SLAHL => sla_hl(self),
            Instruction::SRA(register) => sra(self, register),
            Instruction::SRAHL => sra_hl(self),
            Instruction::SRL(register) => srl(self, register),
            Instruction::SRLHL => srl_hl(self),
            Instruction::SWAP(register) => swap(self, register),
            Instruction::SWAPHL => swap_hl(self),

            // 8-bit load
            Instruction::LDR8R8(r1, r2) => ld_r8_r8(self, r1, r2),
            Instruction::LDR8D8(register) => ld_r8_d8(self, register),
            Instruction::LDHLD8 => ld_hl_d8(self),
            Instruction::LDR8ADDR(r1, r2) => ld_r8_addr(self, r1, r2),
            Instruction::LDADDRR8(r1, r2) => ld_addr_r8(self, r1, r2),
            Instruction::LDAHLI => ld_a_hli(self),
            Instruction::LDAHLD => ld_a_hld(self),
            Instruction::LDHLIA => ld_hli_a(self),
            Instruction::LDHLDA => ld_hld_a(self),
            Instruction::LDHLR8(register) => ld_hl_r8(self, register),
            Instruction::LDR8HL(register) => ld_r8_hl(self, register),
            Instruction::LDR16R8(r16, r8) => ld_r16_r8(self, r16, r8),
            Instruction::LDR8R16(r8, r16) => ld_r8_r16(self, r8, r16),
            Instruction::LDA8A => ld_a8_a(self),
            Instruction::LDAA8 => ld_a_a8(self),
            Instruction::LDA16A => ld_a16_a(self),
            Instruction::LDAA16 => ld_a_a16(self),

            // 16-bit load
            Instruction::LDR16D16(register) => ld_r16_d16(self, register),
            Instruction::LDSPD16 => ld_sp_d16(self),
            Instruction::LDA16SP => ld_a16_sp(self),
            Instruction::ADDHLSP => add_hl_sp(self),
            Instruction::LDSPHL => ld_sp_hl(self),
            Instruction::ADDSPr8 => add_sp_r8(self),
            Instruction::LDHLSPr8 => ld_hl_sp_r8(self),
            Instruction::PUSH(register) => push(self, register),
            Instruction::POP(register) => pop(self, register),

            // Control
            Instruction::CCF => ccf(self),
            Instruction::NOP => nop(),
            Instruction::SCF => scf(self),
            Instruction::HALT => halt(),
            Instruction::STOP => stop(),
            Instruction::DI => di(self),
            Instruction::EI => ei(self),
            Instruction::PREFIX => (),

            // Jumps
            Instruction::JP(condition) => jp(self, condition),
            Instruction::JPHL => jp_hl(self),
            Instruction::JR(condition) => jr(self, condition),
            Instruction::RST(offset) => rst(self, offset),
            Instruction::RET(condition) => ret(self, condition),
            Instruction::RETI => reti(self),
            Instruction::CALL(condition) => call(self, condition),

            Instruction::UNUSED => panic!("Tried to run unknown op code"),
        }
    }

    fn half_carry_in_add_16(x: u16, y: u16) -> bool {
        (x & 0xFFF) + (y & 0xFFF) > 0xFFF
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory.read_byte(self.registers.pc());
        self.registers.increase_pc(1);
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let low = u16::from(self.fetch_byte());
        let hi = u16::from(self.fetch_byte());
        (hi << 8) | low
    }

    fn read_d8(&mut self) -> u8 {
        self.fetch_byte()
    }

    fn read_d16(&mut self) -> u16 {
        self.fetch_word()
    }

    fn read_a16(&mut self) -> u16 {
        self.fetch_word()
    }

    fn push_to_stack(&mut self, value: u16) {
        self.registers.decrease_sp(2);
        self.memory.write_word(self.registers.sp(), value);
    }

    fn pop_from_stack(&mut self) -> u16 {
        let value = self.memory.read_word(self.registers.sp());
        self.registers.increase_sp(2);
        value
    }

    // Uses the value stored in a 16 bit register as an address and returns the
    // byte stored in that memory address.
    fn value_in_addr(&self, register: &Register16bits) -> u8 {
        let register_val = self.registers.read_16b(register);
        self.memory.read_byte(register_val)
    }

    fn attend_pending_interrupt(&mut self) -> bool {
        if self.interrupts_enabled {
            let isr_addr = self.memory.interrupts.isr_of_first_pending();

            match isr_addr {
                Some(addr) => {
                    self.interrupts_enabled = false;
                    self.push_to_stack(self.registers.pc());
                    self.registers.write_pc(addr);
                    return true;
                }
                None => return false,
            }
        }

        false
    }
}
