use crate::registers::Register;

pub struct Cpu {
    registers: Register,
}


impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Register::new(false),
        }
    }

    fn step(instruction: u8) {
        match instruction {
            0x00 => {},
            _ => panic!("Instruction not implemented: {:02X}", instruction),
        }
    }

    pub fn test(dummy: Vec<u8>) {
        for instruction in dummy {
            Self::step(instruction);
        }
    }
}
