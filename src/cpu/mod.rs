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
        let src_r8 = Self::map_b3_to_r8(instruction & 0x07);

        // TODO: those two bits can be interpreted in 3 ways depending on operation, find elegant way to map
        // Maybe give op functions the bits and let them interpret themselves?
        let target_r16 = RegisterName::AF; // = Self::map_b2_to_r16((instruction & 0x30) >> 4);

        match instruction {
            0x00 => 1,
            0x01 | 0x11 | 0x21 | 0x31 => load::ld_r16_u16(self, target_r16),
            0x02 | 0x12 => load::ld_r16_mem_r8(self, target_r16, RegisterName::A),
            0x03 | 0x13 | 0x23 | 0x33 => arithmetic::inc_r16(self, target_r16),
            0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C => {
                arithmetic::inc_r8(self, src_r8)
            }
            0x05 | 0x15 | 0x25 | 0x35 | 0x0D | 0x1D | 0x2D | 0x3D => {
                arithmetic::dec_r8(self, src_r8)
            }
            0x06 | 0x16 | 0x26 | 0x36 | 0x0E | 0x1E | 0x2E | 0x3E => {
                let dest_r8 = Self::map_b3_to_r8((instruction & 0x38) >> 3);
                load::ld_r8_u8(self, dest_r8)
            }
            0x07 => bit::rlca(self),
            0x09 | 0x19 | 0x29 | 0x39 => arithmetic::add_hl_r16(self, target_r16),
            0x0A | 0x1A => load::ld_r8_r16_mem(self, RegisterName::A, target_r16),
            0x0B | 0x1B | 0x2B | 0x3B => arithmetic::dec_r16(self, target_r16),
            0x0F => bit::rrca(self),
            0x17 => bit::rla(self),
            0x18 => control::jr_s8(self),
            0x1F => bit::rra(self),
            0x20 | 0x28 | 0x30 | 0x38 => {
                let condition = Self::map_b2_to_jc((instruction & 0x18) >> 3);
                control::jr_cond_s8(self, condition)
            }
            0x22 => load::ld_hl_pm_mem_a(self, true),
            0x27 => arithmetic::daa(self),
            0x2A => load::ld_a_hl_pm_mem(self, true),
            0x2F => arithmetic::cpl(self),
            0x32 => load::ld_hl_pm_mem_a(self, false),
            0x37 => arithmetic::scf(self),
            0x3A => load::ld_a_hl_pm_mem(self, false),
            0x3F => arithmetic::ccf(self),
            0x40..=0x7F => {
                let dest_r8 = Self::map_b3_to_r8((instruction & 0x38) >> 3);

                if (src_r8 == RegisterName::HL) && (dest_r8 == RegisterName::HL) {
                    0 // TODO: 0x76 - halt
                } else {
                    load::ld_r8_r8(self, dest_r8, src_r8)
                }
            }
            0x80..=0x87 => arithmetic::add_a_r8(self, src_r8),
            0x88..=0x8F => arithmetic::adc_a_r8(self, src_r8),
            0x90..=0x97 => arithmetic::sub_a_r8(self, src_r8),
            0x98..=0x9F => arithmetic::sbc_a_r8(self, src_r8),
            0xA0..=0xA7 => arithmetic::and_a_r8(self, src_r8),
            0xA8..=0xAF => arithmetic::xor_a_r8(self, src_r8),
            0xB0..=0xB7 => arithmetic::or_a_r8(self, src_r8),
            0xB8..=0xBF => arithmetic::cp_a_r8(self, src_r8),
            0xC1 | 0xD1 | 0xE1 | 0xF1 => stack::pop_r16(self, target_r16),
            0xC2 | 0xCA | 0xD2 | 0xDA => {
                let condition = Self::map_b2_to_jc((instruction & 0x18) >> 3);
                control::jp_cond_a16(self, condition)
            }
            0xC3 => control::jp_a16(self),
            0xC5 | 0xD5 | 0xE5 | 0xF5 => stack::push_r16(self, target_r16),
            0xC6 => arithmetic::add_a_d8(self),
            0xCB => {
                let instruction = self.fetch_instruction();
                self.prefix_step(instruction)
            }
            0xCE => arithmetic::adc_a_d8(self),
            0xD6 => arithmetic::sub_a_d8(self),
            0xDE => arithmetic::sbc_a_d8(self),
            0xE6 => arithmetic::and_a_d8(self),
            0xE9 => control::jp_hl(self),
            0xEE => arithmetic::xor_a_d8(self),
            0xF9 => arithmetic::or_a_d8(self),
            0xFB => 0, // TODO: learn how interrupts work & implemented accordingly
            0xFE => arithmetic::cp_a_d8(self),
            _ => panic!("Instruction not implemented: {:02X}", instruction),
        }
    }

    fn prefix_step(&mut self, instruction: u8) -> u8 {
        let target = Self::map_b3_to_r8(instruction & 0x07);

        match instruction {
            0x00..=0x07 => bit::rlc_r8(self, target),
            0x08..=0x0F => bit::rrc_r8(self, target),
            0x10..=0x17 => bit::rl_r8(self, target),
            0x18..=0x1F => bit::rr_r8(self, target),
            0x20..=0x27 => bit::sla_r8(self, target),
            0x28..=0x2F => bit::sra_r8(self, target),
            0x30..=0x37 => bit::swap_r8(self, target),
            0x38..=0x3F => bit::srl_r8(self, target),
            0x40..=0x7F => bit::bit_p_r8(self, (instruction & 0x38) >> 3, target),
            0x80..=0xBF => bit::res_p_r8(self, (instruction & 0x38) >> 3, target),
            0xC0..=0xFF => bit::set_p_r8(self, (instruction & 0x38) >> 3, target),
        }
    }

    pub fn test(&mut self, dummy: Vec<u8>) {
        self.dummy_memory = dummy;
        loop {
            let instructions = self.fetch_instruction();
            self.step(instructions);
        }
    }

    fn fetch_instruction(&mut self) -> u8 {
        let pc = self.registers.pc_next();

        // self.mmu.read(pc) // TODO: change this when mmu is implemented
        self.dummy_memory[pc as usize]
    }

    // TODO: refactor(?)
    fn fetch_little_endian_d16(&self, address: u16) -> u16 {
        let low_byte = self.mmu.read(address) as u16;
        let high_byte = self.mmu.read(address.wrapping_add(1)) as u16;

        high_byte << 8 | low_byte
    }

    // TODO: refactor(?)
    fn store_little_endian_16(&mut self, address: u16, value: u16) {
        let high_byte = (value >> 8) as u8;
        let low_byte = value as u8;

        self.mmu.write(address, low_byte);
        self.mmu.write(address.wrapping_add(1), high_byte);
    }

    // TODO: not sure I am happy with these map functions
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

    fn map_b2_to_jc(masked_instr: u8) -> control::JumpCondition {
        use control::*;
        match masked_instr {
            0 => JumpCondition::NZ,
            1 => JumpCondition::Z,
            2 => JumpCondition::NC,
            3 => JumpCondition::C,
            _ => panic!("Invalid target"),
        }
    }

    fn r8_value(&mut self, register: RegisterName) -> u8 {
        match register {
            RegisterName::HL => self.mmu.read(self.registers.hl()),
            _ => self.registers.read_8_bit_reg(register),
        }
    }

    fn set_r8_value(&mut self, register: RegisterName, value: u8) {
        match register {
            RegisterName::HL => self.mmu.write(self.registers.hl(), value),
            _ => self.registers.write_8_bit_reg(register, value),
        };
    }
}
