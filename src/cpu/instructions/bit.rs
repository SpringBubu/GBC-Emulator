use crate::cpu::{
    Cpu,
    registers::{FlagName, RegisterName},
};

pub fn bit_p_r(cpu: &mut Cpu, position: u8, register: RegisterName) -> u8 {
    let reg = cpu.registers.read_8_bit_reg(register);
    let mask = 1 << position;
    let value = reg & mask;
    let compliment = value == 0;

    cpu.registers.flags_set_z(compliment);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(true);

    8
}

pub fn bit_p_hl(cpu: &mut Cpu, position: u8) -> u8 {
    let address = cpu.registers.hl();
    let memory_value = cpu.mmu.read(address);
    let mask = 1 << position;
    let value = memory_value & mask;
    let compliment = value == 0;

    cpu.registers.flags_set_z(compliment);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(true);

    12 // 16 (?, tables differ here) FIXME: find out what's correct
}

pub fn res_p_r(cpu: &mut Cpu, position: u8, register: RegisterName) -> u8 {
    let reg = cpu.registers.read_8_bit_reg(register);
    let mask = !(1 << position);
    let value = reg & mask;
    cpu.registers.write_8_bit_reg(register, value);

    8
}

pub fn res_p_hl(cpu: &mut Cpu, position: u8) -> u8 {
    let address = cpu.registers.hl();
    let memory_value = cpu.mmu.read(address);
    let mask = !(1 << position);
    let value = memory_value & mask;
    cpu.mmu.write(address, value);

    16
}

pub fn set_p_r(cpu: &mut Cpu, position: u8, register: RegisterName) -> u8 {
    let reg = cpu.registers.read_8_bit_reg(register);
    let mask = 1 << position;
    let value = reg | mask;
    cpu.registers.write_8_bit_reg(register, value);

    8
}

pub fn set_p_hl(cpu: &mut Cpu, position: u8) -> u8 {
    let address = cpu.registers.hl();
    let memory_value = cpu.mmu.read(address);
    let mask = 1 << position;
    let value = memory_value | mask;
    cpu.mmu.write(address, value);

    16
}
