use crate::cpu::{Cpu, registers::RegisterName};

pub fn ld_rr_d16(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let value = cpu.fetch_little_endian();
    cpu.registers.inc_pc(2);
    cpu.registers.write_16_bit_reg(register, value);

    12
}

pub fn ld_rr_mem_a(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let value = cpu.registers.a();
    let address = cpu.registers.read_16_bit_reg(register);
    cpu.mmu.write(address, value);

    8
}

pub fn ld_hl_minus_a(cpu: &mut Cpu) -> u8 {
    let value = cpu.registers.a();
    let address = cpu.registers.hl();
    cpu.mmu.write(address, value);

    let new_hl = address.wrapping_sub(1);
    cpu.registers.write_16_bit_reg(RegisterName::HL, new_hl);

    8
}

pub fn ld_bc_a(cpu: &mut Cpu) -> u8 {
    let value = cpu.registers.a();
    let address = cpu.registers.bc();
    cpu.mmu.write(address, value);

    8
}
