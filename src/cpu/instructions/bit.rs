use crate::cpu::{Cpu, registers::RegisterName};

pub fn bit_p_r(cpu: &mut Cpu, position: u8, register: RegisterName) -> u8 {
    let reg = match register {
        RegisterName::HL => cpu.mmu.read(cpu.registers.hl()),
        _ => cpu.registers.read_8_bit_reg(register),
    };
    let mask = 1 << position;
    let value = reg & mask;
    let compliment = value == 0;

    cpu.registers.flags_set_z(compliment);
    cpu.registers.flags_set_n(false);
    cpu.registers.flags_set_h(true);

    // FIXME: Opcode Tables differe here,
    // find out what's correct
    if register == RegisterName::HL {
        12 /* 16 */
    } else {
        8
    }
}

pub fn res_p_r(cpu: &mut Cpu, position: u8, register: RegisterName) -> u8 {
    let reg = match register {
        RegisterName::HL => cpu.mmu.read(cpu.registers.hl()),
        _ => cpu.registers.read_8_bit_reg(register),
    };
    let mask = !(1 << position);
    let value = reg & mask;
    cpu.registers.write_8_bit_reg(register, value);

    if register == RegisterName::HL { 16 } else { 8 }
}

pub fn set_p_r(cpu: &mut Cpu, position: u8, register: RegisterName) -> u8 {
    let reg = match register {
        RegisterName::HL => cpu.mmu.read(cpu.registers.hl()),
        _ => cpu.registers.read_8_bit_reg(register),
    };
    let mask = 1 << position;
    let value = reg | mask;
    cpu.registers.write_8_bit_reg(register, value);

    if register == RegisterName::HL { 16 } else { 8 }
}
