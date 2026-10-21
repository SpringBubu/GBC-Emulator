use crate::cpu::{Cpu, registers::RegisterName};

pub fn ld_sp_d16(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch_little_endian();
    cpu.registers.inc_pc(2);
    cpu.registers.write_16_bit_reg(RegisterName::SP, value);
    3
}

pub fn ld_hl_d16(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch_little_endian();
    cpu.registers.inc_pc(2);
    cpu.registers.write_16_bit_reg(RegisterName::HL, value);
    3
}
