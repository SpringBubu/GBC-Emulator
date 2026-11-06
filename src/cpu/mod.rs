mod instructions;
mod registers;

use crate::cpu::{instructions::*, registers::*};
use crate::memory::Mmu;

pub struct Cpu {
    registers: Register,
    dummy_memory: Vec<u8>,
    mmu: Mmu,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Register::new(false),
            dummy_memory: Vec::new(),
            mmu: Mmu::new(),
        }
    }

    fn step(&mut self, instruction: u8) -> u8 {
        match instruction {
            0x00 => 1,
            0x01 => load::ld_rr_d16(self, RegisterName::BC),
            0x02 => load::ld_bc_a(self),
            0x11 => load::ld_rr_d16(self, RegisterName::DE),
            0x20 => 0,
            0x21 => load::ld_rr_d16(self, RegisterName::HL),
            0x31 => load::ld_rr_d16(self, RegisterName::SP),
            0x32 => load::ld_hl_minus_a(self),
            0xAF => arithmetic::xor_a(self),
            0xCB => {
                // TODO: make this more elegant
                let pc = self.registers.pc() as usize;
                self.registers.inc_pc(1);
                self.prefix_step(self.dummy_memory[pc])
            }
            _ => panic!("Instruction not implemented: {:02X}", instruction),
        }
    }

    fn prefix_step(&mut self, instruction: u8) -> u8 {
        // take & check last 3 bits
        let target = match instruction & 0x07 {
            0 => RegisterName::B,
            1 => RegisterName::C,
            2 => RegisterName::D,
            3 => RegisterName::E,
            4 => RegisterName::H,
            5 => RegisterName::L,
            6 => RegisterName::HL, // TODO: think about how to do this properly for HL since it's a little different
            7 => RegisterName::A,
            _ => panic!("Invalid target"),
        };

        let bit_pos = (instruction & 0x38) >> 3; // evil bit pattern level hacking

        match instruction {
            0x40..=0x7F => {
                if target == RegisterName::HL {
                    bit::bit_p_hl(self, bit_pos)
                } else {
                    bit::bit_p_r(self, bit_pos, target)
                }
            }
            0x80..=0xBF => {
                if target == RegisterName::HL {
                    bit::res_p_hl(self, bit_pos)
                } else {
                    bit::res_p_r(self, bit_pos, target)
                }
            }
            0xC0..=0xFF => {
                if target == RegisterName::HL {
                    bit::set_p_hl(self, bit_pos)
                } else {
                    bit::set_p_r(self, bit_pos, target)
                }
            }
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
