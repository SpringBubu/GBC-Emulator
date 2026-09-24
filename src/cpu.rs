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

const ZERO_FLAG_MASK: u8        = 0b_1000_0000;
const SUBTRACTION_FLAG_MASK: u8 = 0b_0100_0000;
const HALF_CARRY_FLAG_MASK: u8  = 0b_0010_0000;
const CARRY_FLAG_MASK: u8       = 0b_0001_0000;

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
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Register {
    pub fn new(flag_state: bool) -> Self {
        Self {
            a: 0x01,
            f: Flags::new(flag_state),
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

    pub fn get_af(&self) -> u16 {
        let f= u8::from(&self.f);
        (self.a as u16) << 8 | f as u16
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = Flags::from(value as u8);
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    // TODO: Unsure if I want to keep this structure or remove enums and just use specific get-/setters
    pub fn get_16_bit_register(&self, register: RegisterName) -> u16 {
        match register {
            AF => self.get_af(),
            BC => self.get_bc(),
            DE => self.get_de(),
            HL => self.get_hl(),
            _ => panic!("No such 16-bit register: {}", register),
        }
    }
    pub fn set_16_bit_register(&mut self, register: RegisterName, value: u16) {
        match register {
            AF => self.set_af(value),
            BC => self.set_bc(value),
            DE => self.set_de(value),
            HL => self.set_hl(value),
            _ => panic!("No such 16-bit register: {}", register),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Register;
    use super::Flags;

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
