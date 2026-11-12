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
        let src_reg = Self::map_b3_to_r8(instruction & 0x07);
        let dest_reg = Self::map_b3_to_r8((instruction & 0x38) >> 3);

        match instruction {
            0x00 => 1,
            0x01 => load::ld_rr_d16(self, RegisterName::BC),
            0x02 => load::ld_rr_mem_r(self, RegisterName::BC, RegisterName::A),
            0x03 => arithmetic::inc_rr(self, RegisterName::BC),
            0x04 => arithmetic::inc_r(self, RegisterName::B),
            0x05 => arithmetic::dec_r(self, RegisterName::B),
            0x0B => arithmetic::inc_rr(self, RegisterName::BC),
            0x0A => load::ld_r_rr_mem(self, RegisterName::A, RegisterName::BC),
            0x0C => arithmetic::inc_r(self, RegisterName::C),
            0x0D => arithmetic::dec_r(self, RegisterName::C),
            0x11 => load::ld_rr_d16(self, RegisterName::DE),
            0x12 => load::ld_rr_mem_r(self, RegisterName::DE, RegisterName::A),
            0x13 => arithmetic::inc_rr(self, RegisterName::DE),
            0x14 => arithmetic::inc_r(self, RegisterName::D),
            0x15 => arithmetic::dec_r(self, RegisterName::D),
            0x1A => load::ld_r_rr_mem(self, RegisterName::A, RegisterName::DE),
            0x1B => arithmetic::inc_rr(self, RegisterName::DE),
            0x1C => arithmetic::inc_r(self, RegisterName::E),
            0x1D => arithmetic::dec_r(self, RegisterName::E),
            0x20 => control::jr_cond_r8(self, control::JumpCondition::NZ),
            0x21 => load::ld_rr_d16(self, RegisterName::HL),
            0x22 => load::ld_hl_mem_a(self, true),
            0x23 => arithmetic::inc_rr(self, RegisterName::HL),
            0x24 => arithmetic::inc_r(self, RegisterName::H),
            0x25 => arithmetic::dec_r(self, RegisterName::H),
            0x2B => arithmetic::inc_rr(self, RegisterName::HL),
            0x2C => arithmetic::inc_r(self, RegisterName::L),
            0x2D => arithmetic::dec_r(self, RegisterName::L),
            0x2F => arithmetic::cpl(self),
            0x31 => load::ld_rr_d16(self, RegisterName::SP),
            0x32 => load::ld_hl_mem_a(self, false),
            0x33 => arithmetic::inc_rr(self, RegisterName::SP),
            0x34 => arithmetic::inc_r(self, RegisterName::HL),
            0x35 => arithmetic::dec_r(self, RegisterName::HL),
            0x37 => arithmetic::scf(self),
            0x3B => arithmetic::inc_rr(self, RegisterName::SP),
            0x3C => arithmetic::inc_r(self, RegisterName::A),
            0x3D => arithmetic::dec_r(self, RegisterName::A),
            0x3F => arithmetic::ccf(self),
            0x40..=0x7F => {
                if src_reg == RegisterName::HL {
                    load::ld_r_rr_mem(self, dest_reg, src_reg)
                } else if dest_reg == RegisterName::HL {
                    load::ld_rr_mem_r(self, dest_reg, src_reg)
                } else if (src_reg == RegisterName::HL) && (dest_reg == RegisterName::HL) {
                    0 // TODO: 0x76 - halt
                } else {
                    load::ld_r_r(self, dest_reg, src_reg)
                }
            }
            0x80..=0x87 => arithmetic::add_a_r(self, src_reg),
            0x90..=0x97 => arithmetic::sub_a_r(self, src_reg),
            0xA0..=0xA7 => arithmetic::and_a_r(self, src_reg),
            0xA8..=0xAF => arithmetic::xor_a_r(self, src_reg),
            0xB0..=0xB7 => arithmetic::or_a_r(self, src_reg),
            0xB8..=0xBF => arithmetic::cp_a_r(self, src_reg),
            0xCB => {
                // TODO: make this more elegant
                let pc = self.registers.pc() as usize;
                self.registers.inc_pc(1);
                self.prefix_step(self.dummy_memory[pc])
            }
            0xFB => 0, // TODO: learn how interrupts work & implemented accordingly
            _ => panic!("Instruction not implemented: {:02X}", instruction),
        }
    }

    fn prefix_step(&mut self, instruction: u8) -> u8 {
        // evil bit pattern level hacking
        let target = Self::map_b3_to_r8(instruction & 0x07);
        let bit_pos = (instruction & 0x38) >> 3;

        match instruction {
            0x40..=0x7F => bit::bit_p_r(self, bit_pos, target),
            0x80..=0xBF => bit::res_p_r(self, bit_pos, target),
            0xC0..=0xFF => bit::set_p_r(self, bit_pos, target),
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

    // TODO: switch dummy_memory to mmu once implemented
    fn fetch_little_endian(&self) -> u16 {
        let pc = self.registers.pc() as usize;
        let low_byte = self.dummy_memory[pc] as u16;
        let high_byte = self.dummy_memory[pc + 1] as u16;

        high_byte << 8 | low_byte
    }

    fn map_b3_to_r8(masked_instr: u8) -> RegisterName {
        match masked_instr {
            0 => RegisterName::B,
            1 => RegisterName::C,
            2 => RegisterName::D,
            3 => RegisterName::E,
            4 => RegisterName::H,
            5 => RegisterName::L,
            6 => RegisterName::HL,
            7 => RegisterName::A,
            _ => panic!("Invalid target"),
        }
    }
}
