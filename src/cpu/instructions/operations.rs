use crate::cpu::{Cpu, registers::RegisterName};

pub fn xor_a(cpu: &mut Cpu) -> u8 {
    let mut value = cpu.registers.a();
    value ^= value;
    cpu.registers.write_8_bit_reg(RegisterName::A, value);
    // TODO: set flags to Z000
    1
}
