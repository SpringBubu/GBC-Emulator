use crate::cpu::{Cpu, registers::{self, RegisterName}};

pub fn pop_r16(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let lower_byte = cpu.mmu.read(cpu.registers.sp()) as u16;
    cpu.registers.inc_sp(1);

    let upper_byte = cpu.mmu.read(cpu.registers.sp()) as u16;
    cpu.registers.inc_sp(1);

    let value = (upper_byte << 8) | lower_byte;
    cpu.registers.write_16_bit_reg(register, value);

    12
}

pub fn push_r16(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let value = cpu.registers.read_16_bit_reg(register);
    let upper_byte = (value >> 8) as u8;
    let lower_byte = value as u8;

    cpu.registers.dec_sp(1);
    cpu.mmu.write(cpu.registers.sp(), upper_byte);

    cpu.registers.dec_sp(1);
    cpu.mmu.write(cpu.registers.sp(), lower_byte);

    16
}
