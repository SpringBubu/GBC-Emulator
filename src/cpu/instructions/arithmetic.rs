use crate::cpu::{
    Cpu,
    registers::{FlagName, RegisterName},
};

pub fn xor_a(cpu: &mut Cpu) -> u8 {
    let mut value = cpu.registers.a();
    value ^= value;
    cpu.registers.write_8_bit_reg(RegisterName::A, value);
    cpu.registers.set_exclusive_flag(FlagName::Z, value == 0);
    1
}
