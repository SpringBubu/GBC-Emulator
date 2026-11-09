use crate::cpu::{Cpu, registers::RegisterName};

pub fn ld_rr_d16(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let value = cpu.fetch_little_endian();
    cpu.registers.inc_pc(2);
    cpu.registers.write_16_bit_reg(register, value);

    12
}

pub fn ld_rr_mem_r(cpu: &mut Cpu, destination: RegisterName, source: RegisterName) -> u8 {
    let address = cpu.registers.read_16_bit_reg(destination);
    let value = cpu.registers.read_8_bit_reg(source);
    cpu.mmu.write(address, value);

    8
}

pub fn ld_hl_minus_a(cpu: &mut Cpu) -> u8 {
    let address = cpu.registers.hl();
    let value = cpu.registers.a();
    cpu.mmu.write(address, value);

    let new_hl = address.wrapping_sub(1);
    cpu.registers.write_16_bit_reg(RegisterName::HL, new_hl);

    8
}
pub fn ld_hl_plus_a(cpu: &mut Cpu) -> u8 {
    let address = cpu.registers.hl();
    let value = cpu.registers.a();
    cpu.mmu.write(address, value);

    let new_hl = address.wrapping_add(1);
    cpu.registers.write_16_bit_reg(RegisterName::HL, new_hl);

    8
}

pub fn ld_r_r(cpu: &mut Cpu, destination: RegisterName, source: RegisterName) -> u8 {
    let value = cpu.registers.read_8_bit_reg(source);
    cpu.registers.write_8_bit_reg(destination, value);

    4
}

pub fn ld_r_rr_mem(cpu: &mut Cpu, destination: RegisterName, source: RegisterName) -> u8 {
    let address = cpu.registers.read_16_bit_reg(source);
    let value = cpu.mmu.read(address);
    cpu.registers.write_8_bit_reg(destination, value);

    8
}
