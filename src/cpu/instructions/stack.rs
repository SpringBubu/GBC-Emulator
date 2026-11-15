use crate::cpu::{
    Cpu,
    registers::{self, RegisterName},
};

pub fn pop_r16(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let address = cpu.registers.sp();
    let value = cpu.fetch_little_endian_d16(address);

    cpu.registers.inc_sp(2);

    cpu.registers.write_16_bit_reg(register, value);

    12
}

pub fn push_r16(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let address = cpu.registers.sp();
    let value = cpu.registers.read_16_bit_reg(register);

    cpu.registers.dec_sp(2);

    cpu.store_little_endian_16(address, value);

    16
}
