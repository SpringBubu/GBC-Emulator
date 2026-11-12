use crate::cpu::{
    Cpu,
    registers::{FlagName, RegisterName},
};

fn get_operands(cpu: &mut Cpu, register: RegisterName) -> (u8, u8) {
    let lhs = cpu.registers.a();
    let rhs = match register {
        RegisterName::HL => cpu.mmu.read(cpu.registers.hl()),
        _ => cpu.registers.read_8_bit_reg(register),
    };

    (lhs, rhs)
}

pub fn inc_rr(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.registers.read_16_bit_reg(register);
    let value = reg.wrapping_add(1);
    cpu.registers.write_16_bit_reg(register, value);

    8
}

pub fn inc_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let (_, value) = get_operands(cpu, register);
    let inc_val = value.wrapping_add(1);
    let half_carry = ((value & 0x0F) + 1) > 0x0F;

    cpu.registers.flags_set_z(inc_val == 0);
    cpu.registers.flags_set_z(false);
    cpu.registers.flags_set_z(half_carry);

    if register == RegisterName::HL {
        let address = cpu.registers.hl();
        cpu.mmu.write(address, inc_val);

        12
    } else {
        cpu.registers.write_8_bit_reg(register, inc_val);

        4
    }
}

pub fn dec_rr(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.registers.read_16_bit_reg(register);
    let value = reg.wrapping_sub(reg);
    cpu.registers.write_16_bit_reg(register, value);

    8
}

pub fn dec_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let (_, value) = get_operands(cpu, register);
    let inc_val = value.wrapping_sub(1);
    let half_carry = (value & 0x0F) < 1;

    cpu.registers.flags_set_z(inc_val == 0);
    cpu.registers.flags_set_z(true);
    cpu.registers.flags_set_z(half_carry);

    if register == RegisterName::HL {
        let address = cpu.registers.hl();
        cpu.mmu.write(address, inc_val);

        12
    } else {
        cpu.registers.write_8_bit_reg(register, inc_val);

        4
    }
}

pub fn cpl(cpu: &mut Cpu) -> u8 {
    let a = cpu.registers.a();
    cpu.registers.set_a(!a);

    cpu.registers.flags_set_n(true);
    cpu.registers.flags_set_h(true);

    4
}

pub fn scf(cpu: &mut Cpu) -> u8 {
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(false);
    cpu.registers.flags_set_c(true);

    4
}

pub fn ccf(cpu: &mut Cpu) -> u8 {
    let carry = cpu.registers.flags_c();

    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(false);
    cpu.registers.flags_set_c(!carry);

    4
}

pub fn add_a_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let (a, addend) = get_operands(cpu, register);
    let (value, carry) = a.overflowing_add(addend);
    let half_carry = ((a & 0x0F) + (addend & 0x0F)) > 0x0F;
    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn sub_a_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let (a, subtrahend) = get_operands(cpu, register);
    let (value, carry) = a.overflowing_sub(subtrahend);
    let half_carry = (a & 0x0F) < (subtrahend & 0x0F);
    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(true);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn and_a_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let (a, operand) = get_operands(cpu, register);
    let value = a & operand;
    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(true);
    cpu.registers.flags_set_c(false);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn xor_a_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let a = cpu.registers.a();
    let operand = match register {
        RegisterName::HL => cpu.mmu.read(cpu.registers.hl()),
        _ => cpu.registers.read_8_bit_reg(register),
    };

    let value = a ^ operand;
    cpu.registers.set_a(value);

    cpu.registers.set_exclusive_flag(FlagName::Z, value == 0);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn or_a_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let (a, operand) = get_operands(cpu, register);
    let value = a | operand;
    cpu.registers.set_a(value);

    cpu.registers.set_exclusive_flag(FlagName::Z, value == 0);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn cp_a_r(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let (a, operand) = get_operands(cpu, register);
    let (value, carry) = a.overflowing_sub(operand);
    let half_carry = (a & 0x0F) < (operand & 0x0F);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(true);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry);

    if register == RegisterName::HL { 8 } else { 4 }
}
