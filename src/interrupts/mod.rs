use std::collections::HashMap;

pub const ENABLED_INTERRUPTS_ADDR: usize = 0xFFFF;
pub const PENDING_INTERRUPTS_ADDR: usize = 0xFF0F;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum InterruptKind {
    VBLANK,
    LCDSTAT,
    TIMER,
    SERIAL,
    JOYPAD,
}

// When there are several pending interrupts to be run, some kinds have
// precedence over others.
const SORTED_INTERRUPT_KINDS: [InterruptKind; 5] = [
    InterruptKind::VBLANK,
    InterruptKind::LCDSTAT,
    InterruptKind::TIMER,
    InterruptKind::SERIAL,
    InterruptKind::JOYPAD,
];

#[derive(Clone)]
pub struct InterruptRegister {
    values: HashMap<InterruptKind, bool>,
}

impl InterruptRegister {
    pub fn new() -> InterruptRegister {
        let mut values: HashMap<InterruptKind, bool> = HashMap::new();

        for kind in SORTED_INTERRUPT_KINDS.iter() {
            values.insert(kind.clone(), false);
        }

        InterruptRegister { values }
    }

    pub fn set(&mut self, kind: &InterruptKind, value: bool) {
        self.values.insert(kind.clone(), value).unwrap();
    }

    pub fn get(&self, kind: &InterruptKind) -> bool {
        self.values[kind]
    }

    fn bit_in_mem_addr(kind: &InterruptKind) -> u8 {
        match *kind {
            InterruptKind::VBLANK => 0,
            InterruptKind::LCDSTAT => 1,
            InterruptKind::TIMER => 2,
            InterruptKind::SERIAL => 3,
            InterruptKind::JOYPAD => 4,
        }
    }

    fn bit_enabled_for_kind(kind: &InterruptKind, value: u8) -> bool {
        value & (1 << InterruptRegister::bit_in_mem_addr(&kind) as u8) != 0
    }
}

impl From<InterruptRegister> for u8 {
    fn from(register: InterruptRegister) -> u8 {
        register
            .values
            .iter()
            .map(|(kind, &set)| {
                if set {
                    2_u8.pow(InterruptRegister::bit_in_mem_addr(kind).into())
                } else {
                    0
                }
            })
            .sum()
    }
}

impl From<u8> for InterruptRegister {
    fn from(value: u8) -> InterruptRegister {
        let mut res = InterruptRegister::new();

        for kind in SORTED_INTERRUPT_KINDS.iter() {
            res.values.insert(
                kind.clone(),
                InterruptRegister::bit_enabled_for_kind(kind, value),
            );
        }

        res
    }
}

#[derive(Clone)]
pub struct Interrupts {
    enabled: InterruptRegister,
    pending: InterruptRegister,
}

impl Interrupts {
    pub fn new() -> Interrupts {
        Interrupts {
            enabled: InterruptRegister::new(),
            pending: InterruptRegister::new(),
        }
    }

    fn isr_address(kind: &InterruptKind) -> u16 {
        match *kind {
            InterruptKind::VBLANK => 0x40,
            InterruptKind::LCDSTAT => 0x48,
            InterruptKind::TIMER => 0x50,
            InterruptKind::SERIAL => 0x58,
            InterruptKind::JOYPAD => 0x60,
        }
    }

    pub fn ie_value(&self) -> u8 {
        self.enabled.clone().into()
    }

    pub fn if_value(&self) -> u8 {
        self.pending.clone().into()
    }

    pub fn enable_or_disable_interrupts(&mut self, value: u8) {
        self.enabled = InterruptRegister::from(value);
    }

    pub fn add_interrupts(&mut self, value: u8) {
        self.pending = InterruptRegister::from(value);
    }

    pub fn isr_of_first_pending(&mut self) -> Option<u16> {
        for kind in SORTED_INTERRUPT_KINDS.iter() {
            if self.is_enabled(kind) && self.is_pending(kind) {
                self.pending.set(kind, false);
                return Some(Interrupts::isr_address(kind));
            }
        }

        None
    }

    fn is_pending(&self, kind: &InterruptKind) -> bool {
        self.pending.get(&kind)
    }

    fn is_enabled(&self, kind: &InterruptKind) -> bool {
        self.enabled.get(&kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enable_and_disable_interrupts() {
        let mut interrupts = Interrupts::new();
        let value = 0b00011010;

        interrupts.enable_or_disable_interrupts(value);

        assert_eq!(interrupts.ie_value(), value)
    }

    #[test]
    fn add_interrupts() {
        let mut interrupts = Interrupts::new();
        let value = 0b00011010;

        interrupts.add_interrupts(value);

        assert_eq!(interrupts.if_value(), value)
    }

    #[test]
    fn isr_of_first_pending() {
        // We are going to add interrupts for "lcdstat" and "serial",
        // and enable "timer" and "serial".
        // The first pending should be "serial". "lcdstat" has precedence but
        // it is not enabled.
        let mut interrupts = Interrupts::new();
        let value = 0b00001010;
        interrupts.add_interrupts(value);
        interrupts.enable_or_disable_interrupts(0b00001100);

        let isr = interrupts.isr_of_first_pending().unwrap();

        assert_eq!(isr, 0x58)
    }

    #[test]
    fn is_of_first_pending_when_no_pending() {
        let mut interrupts = Interrupts::new();

        assert_eq!(interrupts.isr_of_first_pending(), None)
    }
}
