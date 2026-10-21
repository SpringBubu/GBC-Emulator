mod instructions;
mod registers;

use crate::cpu::{instructions::*, registers::Register};

pub struct Cpu {
    registers: Register,
    dummy_memory: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Register::new(false),
            dummy_memory: Vec::new(),
        }
    }

    fn step(&mut self, instruction: u8) -> u8 {
        match instruction {
            0x00 => 1,
            0x21 => load::ld_hl_d16(self),
            0x31 => load::ld_sp_d16(self),
            0x32 => 1, // TODO: needs memory implementation
            0xAF => arithmetic::xor_a(self),
            0xCB => self.prefix_step(instruction),
            _ => panic!("Instruction not implemented: {:02X}", instruction),
        }
    }

    fn prefix_step(&mut self, instruction: u8) -> u8 {
        match instruction {
            0x7C => 1, // TODO
            _ => panic!("Prefixed instruction not implemented: {:02X}", instruction),
        }
    }

    pub fn test(&mut self, dummy: Vec<u8>) {
        self.dummy_memory = dummy;
        loop {
            let pc = self.registers.pc() as usize;
            self.registers.inc_pc(1);
            let instruction = self.dummy_memory[pc];
            self.step(instruction);
        }
    }

    fn fetch_little_endian(&self) -> u16 {
        let pc = self.registers.pc() as usize;
        let low_byte = self.dummy_memory[pc] as u16;
        let high_byte = self.dummy_memory[pc + 1] as u16;
        high_byte << 8 | low_byte
    }
}
