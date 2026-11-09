use crate::cpu::{
    Cpu,
    registers::{self, FlagName, Flags, RegisterName},
};

pub fn add_a_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let a = cpu.registers.a();
    let addend = cpu.registers.read_8_bit_reg(register);
    let (value, carry) = a.overflowing_add(addend);
    let half_carry = ((a & 0x0F) + (addend & 0x0F)) > 0x0F;
    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry);

    4
}

pub fn add_a_hl(cpu: &mut Cpu) -> u8 {
    let a = cpu.registers.a();
    let address = cpu.registers.hl();
    let addend = cpu.mmu.read(address);
    let (value, carry) = a.overflowing_add(addend);
    let half_carry = (a & 0x0F) + (addend & 0x0F) > 0x0F;
    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry);

    8
}

pub fn sub_a_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let a = cpu.registers.a();
    let subtrahend = cpu.registers.read_8_bit_reg(register);
    let (value, carry) = a.overflowing_sub(subtrahend);
    let half_carry = (a & 0x0F) < (subtrahend & 0x0F);
    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(true);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry);

    4
}

pub fn sub_a_hl(cpu: &mut Cpu) -> u8 {
    let a = cpu.registers.a();
    let address = cpu.registers.hl();
    let subtrahend = cpu.mmu.read(address);
    let (value, carry) = a.overflowing_sub(subtrahend);
    let half_carry = (a & 0x0F) < (subtrahend & 0x0F);
    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(true);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry);

    4
}

// pub fn and_a_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
//     let a = cpu.registers.a();
//     let operand = match register {
//         RegisterName::HL => cpu.mmu.read(cpu.registers.hl()),
//         _ => cpu.registers.read_8_bit_reg(register),
//     };
//     let value = a & operand;
//     cpu.registers.set_a(value);
//
//     cpu.registers.flags_set_z(value == 0);
//     cpu.registers.flags_set_n(false);
//     cpu.registers.flags_set_c(true);
//     cpu.registers.flags_set_c(false);
//
//     if register == RegisterName::HL { 8 } else { 4 }
// }

pub fn and_a_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let a = cpu.registers.a();
    let operand = cpu.registers.read_8_bit_reg(register);
    let value = a & operand;
    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_c(true);
    cpu.registers.flags_set_c(false);

    4
}

pub fn and_a_hl(cpu: &mut Cpu) -> u8 {
    let a = cpu.registers.a();
    let address = cpu.registers.hl();
    let operand = cpu.mmu.read(address);
    let value = a & operand;
    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_c(true);
    cpu.registers.flags_set_c(false);

    8
}

pub fn xor_a_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let a = cpu.registers.a();
    let operand = cpu.registers.read_8_bit_reg(register);
    let value = a ^ operand;
    cpu.registers.set_a(value);

    cpu.registers.set_exclusive_flag(FlagName::Z, value == 0);

    4
}

pub fn xor_a_hl(cpu: &mut Cpu) -> u8 {
    let a = cpu.registers.a();
    let address = cpu.registers.hl();
    let operand = cpu.mmu.read(address);
    let value = a ^ operand;
    cpu.registers.set_a(value);

    cpu.registers.set_exclusive_flag(FlagName::Z, value == 0);

    8
}
