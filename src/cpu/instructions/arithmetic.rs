use crate::cpu::{
    Cpu,
    registers::{FlagName, RegisterName},
};

pub fn inc_r16(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.registers.read_16_bit_reg(register);
    let value = reg.wrapping_add(1);

    cpu.registers.write_16_bit_reg(register, value);

    8
}

pub fn inc_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let value = cpu.r8_value(register);
    let inc_val = value.wrapping_add(1);
    let half_carry = ((value & 0x0F) + 1) > 0x0F;

    cpu.registers.flags_set_z(inc_val == 0);
    cpu.registers.flags_set_z(false);
    cpu.registers.flags_set_z(half_carry);

    cpu.set_r8_value(register, value);

    if register == RegisterName::HL { 12 } else { 4 }
}

pub fn dec_r16(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let reg = cpu.registers.read_16_bit_reg(register);
    let value = reg.wrapping_sub(reg);

    cpu.registers.write_16_bit_reg(register, value);

    8
}

pub fn dec_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let value = cpu.r8_value(register);
    let inc_val = value.wrapping_sub(1);
    let half_carry = (value & 0x0F) < 1;

    cpu.registers.flags_set_z(inc_val == 0);
    cpu.registers.flags_set_z(true);
    cpu.registers.flags_set_z(half_carry);

    cpu.set_r8_value(register, value);

    if register == RegisterName::HL { 12 } else { 4 }
}

pub fn daa(cpu: &mut Cpu) -> u8 {
    let mut offset: u8 = 0;

    let a = cpu.registers.a();
    let flags = cpu.registers.flags();

    if flags.half_carry || (!flags.subtraction && ((a & 0x0F) > 0x09)) {
        offset |= 0x06;
    }

    if flags.carry || (!flags.subtraction && (a > 0x99)) {
        offset |= 0x60;
        cpu.registers.flags_set_c(true);
    }

    let value = if flags.subtraction {
        a.wrapping_sub(offset)
    } else {
        a.wrapping_add(offset)
    };

    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_h(false);

    4
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

pub fn add_hl_r16(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let hl = cpu.registers.hl();
    let addend = cpu.registers.read_16_bit_reg(register);
    let (value, carry) = hl.overflowing_add(addend);
    let half_carry = ((hl & 0x0FFF) + (addend & 0x0FFF)) > 0x0FFF;

    cpu.registers.set_hl(value);

    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry);

    8
}

fn add_a(cpu: &mut Cpu, addend: u8) {
    let a = cpu.registers.a();
    let (value, carry) = a.overflowing_add(addend);
    let half_carry = ((a & 0x0F) + (addend & 0x0F)) > 0x0F;

    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry);
}

pub fn add_a_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let addend = cpu.r8_value(register);

    add_a(cpu, addend);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn add_a_d8(cpu: &mut Cpu) -> u8 {
    let addend = cpu.mmu.read(cpu.registers.pc_next());

    add_a(cpu, addend);

    8
}

fn adc_a(cpu: &mut Cpu, addend: u8) {
    let a = cpu.registers.a();
    let addend_carry = cpu.registers.flags_c() as u8;
    let (addend, carry_1) = addend.overflowing_add(addend_carry);
    let (value, carry_2) = a.overflowing_add(addend);
    let half_carry = ((a & 0x0F) + (addend & 0x0F) + addend_carry) > 0x0F;

    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry_1 || carry_2);
}

pub fn adc_a_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let addend = cpu.r8_value(register);

    adc_a(cpu, addend);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn adc_a_d8(cpu: &mut Cpu) -> u8 {
    let addend = cpu.mmu.read(cpu.registers.pc_next());

    adc_a(cpu, addend);

    8
}

fn sub_a(cpu: &mut Cpu, subtrahend: u8) {
    let a = cpu.registers.a();
    let (value, carry) = a.overflowing_sub(subtrahend);
    let half_carry = (a & 0x0F) < (subtrahend & 0x0F);

    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(true);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry);
}

pub fn sub_a_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let subtrahend = cpu.r8_value(register);

    sub_a(cpu, subtrahend);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn sub_a_d8(cpu: &mut Cpu) -> u8 {
    let subtrahend = cpu.mmu.read(cpu.registers.pc_next());

    sub_a(cpu, subtrahend);

    8
}

fn sbc_a(cpu: &mut Cpu, subtrahend: u8) {
    let a = cpu.registers.a();
    let subtrahend_carry = cpu.registers.flags_c() as u8;
    let (subtrahend, carry_1) = subtrahend.overflowing_sub(subtrahend_carry);
    let (value, carry_2) = a.overflowing_sub(subtrahend);
    let half_carry = ((a & 0x0F) + (subtrahend & 0x0F) + subtrahend_carry) > 0x0F;

    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(true);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry_1 || carry_2);
}

pub fn sbc_a_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let subtrahend = cpu.r8_value(register);

    sbc_a(cpu, subtrahend);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn sbc_a_d8(cpu: &mut Cpu) -> u8 {
    let subtrahend = cpu.mmu.read(cpu.registers.pc_next());

    sbc_a(cpu, subtrahend);

    8
}

fn and_a(cpu: &mut Cpu, operand: u8) {
    let a = cpu.registers.a();
    let value = a & operand;

    cpu.registers.set_a(value);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(true);
    cpu.registers.flags_set_c(false);
}

pub fn and_a_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let operand = cpu.r8_value(register);

    and_a(cpu, operand);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn and_a_d8(cpu: &mut Cpu) -> u8 {
    let operand = cpu.mmu.read(cpu.registers.pc_next());

    and_a(cpu, operand);

    8
}

fn xor_a(cpu: &mut Cpu, operand: u8) {
    let a = cpu.registers.a();
    let value = a ^ operand;

    cpu.registers.set_a(value);

    cpu.registers.set_exclusive_flag(FlagName::Z, value == 0);
}

pub fn xor_a_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let operand = cpu.r8_value(register);

    xor_a(cpu, operand);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn xor_a_d8(cpu: &mut Cpu) -> u8 {
    let operand = cpu.mmu.read(cpu.registers.pc_next());

    xor_a(cpu, operand);

    8
}

fn or_a(cpu: &mut Cpu, operand: u8) {
    let a = cpu.registers.a();
    let value = a | operand;

    cpu.registers.set_a(value);

    cpu.registers.set_exclusive_flag(FlagName::Z, value == 0);
}

pub fn or_a_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let operand = cpu.r8_value(register);

    or_a(cpu, operand);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn or_a_d8(cpu: &mut Cpu) -> u8 {
    let operand = cpu.mmu.read(cpu.registers.pc_next());

    or_a(cpu, operand);

    8
}

fn cp_a(cpu: &mut Cpu, operand: u8) {
    let a = cpu.registers.a();
    let (value, carry) = a.overflowing_sub(operand);
    let half_carry = (a & 0x0F) < (operand & 0x0F);

    cpu.registers.flags_set_z(value == 0);
    cpu.registers.flags_set_n(true);
    cpu.registers.flags_set_h(half_carry);
    cpu.registers.flags_set_c(carry);
}

pub fn cp_a_r8(cpu: &mut Cpu, register: RegisterName) -> u8 {
    let operand = cpu.r8_value(register);

    cp_a(cpu, operand);

    if register == RegisterName::HL { 8 } else { 4 }
}

pub fn cp_a_d8(cpu: &mut Cpu) -> u8 {
    let operand = cpu.mmu.read(cpu.registers.pc_next());

    cp_a(cpu, operand);

    8
}
