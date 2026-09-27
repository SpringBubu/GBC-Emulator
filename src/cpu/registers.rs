use self::RegisterName::*;

pub enum RegisterName {
    // 8-bit registers
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,

    // 16-bit registers
    AF,
    BC,
    DE,
    HL,

    SP,
    PC,
}

impl std::fmt::Display for RegisterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let register = match self {
            A => "A",
            F => "F",
            B => "B",
            C => "C",
            D => "D",
            E => "E",
            H => "H",
            L => "L",

            AF => "AF",
            BC => "BC",
            DE => "DE",
            HL => "HL",

            SP => "SP",
            PC => "PC",
        };
        write!(f, "{}", register)
    }
}

// TODO: refactor Flags struct away and just use a u8
enum FlagName {
    Z,
    N,
    H,
    C,
}

#[derive(Debug, PartialEq)]
pub struct Flags {
    zero: bool,
    subtraction: bool,
    half_carry: bool,
    carry: bool,
}

const ZERO_FLAG_MASK: u8 = 0b_1000_0000;
const SUBTRACTION_FLAG_MASK: u8 = 0b_0100_0000;
const HALF_CARRY_FLAG_MASK: u8 = 0b_0010_0000;
const CARRY_FLAG_MASK: u8 = 0b_0001_0000;

impl Flags {
    pub fn new(flag_state: bool) -> Self {
        Self {
            zero: true,
            subtraction: false,
            half_carry: flag_state,
            carry: flag_state,
        }
    }
}

#[rustfmt::skip]
impl std::convert::From<&Flags> for u8 {
    fn from(flags: &Flags) -> u8 {
        (if flags.zero { ZERO_FLAG_MASK } else { 0 })
        | (if flags.subtraction { SUBTRACTION_FLAG_MASK } else { 0 })
        | (if flags.half_carry { HALF_CARRY_FLAG_MASK } else { 0 })
        | (if flags.carry { CARRY_FLAG_MASK } else { 0 })
    }
}

impl std::convert::From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self {
            zero: value & ZERO_FLAG_MASK != 0,
            subtraction: value & SUBTRACTION_FLAG_MASK != 0,
            half_carry: value & HALF_CARRY_FLAG_MASK != 0,
            carry: value & CARRY_FLAG_MASK != 0,
        }
    }
}

pub struct Register {
    a: u8,
    pub f: Flags, // TODO: refactor flag
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Register {
    pub fn new(flag_state: bool) -> Self {
        Self {
            a: 0,
            f: Flags::new(flag_state),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn post_boot() -> Self {
        Self {
            a: 0x01,
            f: Flags::new(false),
            b: 0x00,
            c: 0x12,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFEE,
            pc: 0x0100,
        }
    }

    pub fn a(&self) -> u8 {
        self.a
    }
    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn c(&self) -> u8 {
        self.c
    }
    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    pub fn d(&self) -> u8 {
        self.d
    }
    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn e(&self) -> u8 {
        self.e
    }
    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn h(&self) -> u8 {
        self.h
    }
    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn l(&self) -> u8 {
        self.l
    }
    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    pub fn af(&self) -> u16 {
        let f = u8::from(&self.f);
        (self.a as u16) << 8 | f as u16
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = Flags::from(value as u8);
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    // TODO: Unsure if I want to keep this structure or remove enums and just use specific get-/setters
    pub fn read_16_bit_reg(&self, register: RegisterName) -> u16 {
        match register {
            AF => self.af(),
            BC => self.bc(),
            DE => self.de(),
            HL => self.hl(),
            SP => self.sp(),
            PC => self.pc(),
            _ => panic!("No such 16-bit register: {}", register),
        }
    }
    pub fn write_16_bit_reg(&mut self, register: RegisterName, value: u16) {
        match register {
            AF => self.set_af(value),
            BC => self.set_bc(value),
            DE => self.set_de(value),
            HL => self.set_hl(value),
            SP => self.set_sp(value),
            PC => self.set_pc(value),
            _ => panic!("No such 16-bit register: {}", register),
        }
    }

    pub fn read_8_bit_reg(&self, register: RegisterName) -> u8 {
        match register {
            A => self.a(),
            F => unimplemented!("Needs refactor"),
            B => self.b(),
            C => self.c(),
            D => self.d(),
            E => self.e(),
            H => self.h(),
            L => self.l(),
            _ => panic!("No such 8-bit register: {}", register),
        }
    }
    pub fn write_8_bit_reg(&mut self, register: RegisterName, value: u8) {
        match register {
            A => self.set_a(value),
            F => unimplemented!("Needs refactor"),
            B => self.set_b(value),
            C => self.set_c(value),
            D => self.set_d(value),
            E => self.set_e(value),
            H => self.set_h(value),
            L => self.set_l(value),
            _ => panic!("No such 8-bit register: {}", register),
        }
    }

    pub fn sp(&self) -> u16 {
        self.sp
    }
    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }
    pub fn inc_sp(&mut self, value: u16) {
        self.sp = self.sp.wrapping_add(value);
    }
    pub fn dec_sp(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(value);
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }
    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }
    pub fn inc_pc(&mut self, value: u16) {
        self.pc = self.pc.wrapping_add(value);
    }
}

#[cfg(test)]
mod test {
    use super::Flags;
    use super::Register;

    #[test]
    fn flags() {
        let flags = Flags::new(true);
        assert!(flags.half_carry);
        assert!(flags.carry);
        assert_eq!(u8::from(&flags), 0b_1011_0000);
        assert_eq!(Flags::from(0b_1011_0000), flags);
    }

    #[test]
    fn registers_16_bit() {
        let mut register = Register::new(true);

        register.set_af(0b_1010_1010_1111_0000);
        register.set_bc(0b_0101_0101_0000_0001);
        register.set_de(0b_0011_0011_0000_0010);
        register.set_hl(0b_1100_1100_0000_0100);

        assert_eq!(register.a, 0b_1010_1010);
        assert_eq!(register.f, Flags::from(0b_1111_0000));

        assert_eq!(register.b, 0b_0101_0101);
        assert_eq!(register.c, 0b_0000_0001);

        assert_eq!(register.d, 0b_0011_0011);
        assert_eq!(register.e, 0b_0000_0010);

        assert_eq!(register.h, 0b_1100_1100);
        assert_eq!(register.l, 0b_0000_0100);
    }
}
