use crate::cpu::{
    Cpu,
    registers::{FlagName, RegisterName},
};

pub fn set_p_r(cpu: &mut Cpu, position: u8, register: RegisterName) -> u8 {
    let reg = cpu.registers.read_8_bit_reg(register);
    let mask = 1 << position;
    let value = reg | mask;
    cpu.registers.write_8_bit_reg(register, value);

    8
}
