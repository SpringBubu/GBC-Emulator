use crate::cpu::{Cpu, registers::FlagName, registers::RegisterName};

pub fn rlc_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.r8_value(register);
    let msb = reg & 0x80;
    let value = (reg << 1) | (msb >> 7);

    cpu.set_r8_value(register, value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(false);
    cpu.registers.flags_set_c(msb != 0);

    if register == RegisterName::HL { 16 } else { 8 }
}

pub fn rrc_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.r8_value(register);
    let lsb = reg & 0x01;
    let value = (lsb << 7) | (reg >> 1);

    cpu.set_r8_value(register, value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(false);
    cpu.registers.flags_set_c(lsb != 0);

    if register == RegisterName::HL { 16 } else { 8 }
}

pub fn rl_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.r8_value(register);
    let carry_bit = cpu.registers.flags_c() as u8;
    let value = (reg << 1) | carry_bit;
    let carry = (reg & 0x80) != 0;

    cpu.set_r8_value(register, value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(false);
    cpu.registers.flags_set_c(carry);

    if register == RegisterName::HL { 16 } else { 8 }
}

pub fn rr_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.r8_value(register);
    let carry_bit = cpu.registers.flags_c() as u8;
    let value = (carry_bit << 7) | (reg >> 1);
    let carry = (reg & 0x01) != 0;

    cpu.set_r8_value(register, value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(false);
    cpu.registers.flags_set_c(carry);

    if register == RegisterName::HL { 16 } else { 8 }
}

pub fn sla_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.r8_value(register);
    let msb = reg & 0x80;
    let value = reg << 1;

    cpu.set_r8_value(register, value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(false);
    cpu.registers.flags_set_c(msb != 0);

    if register == RegisterName::HL { 16 } else { 8 }
}

pub fn sra_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.r8_value(register);
    let msb = reg & 0x80;
    let lsb = reg & 0x01;
    let value = msb | (reg >> 1);

    cpu.set_r8_value(register, value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(false);
    cpu.registers.flags_set_c(lsb != 0);

    if register == RegisterName::HL { 16 } else { 8 }
}

pub fn swap_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.r8_value(register);
    let high_nibble = reg << 4;
    let low_nibble = reg >> 4;
    let value = high_nibble | low_nibble;

    cpu.set_r8_value(register, value);

    cpu.registers.set_exclusive_flag(FlagName::Z, value == 0);

    if register == RegisterName::HL { 16 } else { 8 }
}

pub fn srl_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.r8_value(register);
    let lsb = reg & 0x01;
    let value = reg >> 1;

    cpu.set_r8_value(register, value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(false);
    cpu.registers.flags_set_c(lsb != 0);

    if register == RegisterName::HL { 16 } else { 8 }
}

pub fn bit_p_r8(cpu: &mut Cpu, position: u8, register: RegisterName) -> u8 {
    let reg = cpu.r8_value(register);
    let mask = 1 << position;
    let value = reg & mask;
    let compliment = value == 0;

    cpu.registers.flags_set_z(compliment);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(true);

    if register == RegisterName::HL { 12 } else { 8 }
}

pub fn res_p_r8(cpu: &mut Cpu, position: u8, register: RegisterName) -> u8 {
    let reg = cpu.r8_value(register);
    let mask = !(1 << position);
    let value = reg & mask;

    cpu.set_r8_value(register, value);

    if register == RegisterName::HL { 16 } else { 8 }
}

pub fn set_p_r8(cpu: &mut Cpu, position: u8, register: RegisterName) -> u8 {
    let reg = cpu.r8_value(register);
    let mask = 1 << position;
    let value = reg | mask;

    cpu.set_r8_value(register, value);

    if register == RegisterName::HL { 16 } else { 8 }
}
