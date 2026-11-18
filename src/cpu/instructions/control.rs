use crate::cpu::Cpu;

pub enum JumpCondition {
    Z,
    NZ,
    C,
    NC,
}

fn check_jp_cond(cpu: &mut Cpu, condition: JumpCondition) -> bool {
    match condition {
        JumpCondition::Z => cpu.registers.flags_z(),
        JumpCondition::NZ => !cpu.registers.flags_z(),
        JumpCondition::C => cpu.registers.flags_c(),
        JumpCondition::NC => !cpu.registers.flags_c(),
    }
}

pub fn jr_i8(cpu: &mut Cpu) -> u8 {
    let address = cpu.registers.pc_next();
    let value = cpu.mmu.read(address) as i8;
    let pc_after_jump = address.wrapping_add_signed(value.into());

    cpu.registers.set_pc(pc_after_jump);

    12
}

pub fn jr_cond_i8(cpu: &mut Cpu, condition: JumpCondition) -> u8 {
    let address = cpu.registers.pc_next();
    let value = cpu.mmu.read(address) as i8;
    let jump = check_jp_cond(cpu, condition);

    if jump {
        let pc_after_jp = address.wrapping_add_signed(value.into());

        cpu.registers.set_pc(pc_after_jp);

        12
    } else {
        8
    }
}

pub fn jp_a16(cpu: &mut Cpu) -> u8 {
    let pc = cpu.registers.pc_next();
    let pc_after_jump = cpu.fetch_little_endian_u16(pc);

    cpu.registers.set_pc(pc_after_jump);

    16
}

pub fn jp_cond_a16(cpu: &mut Cpu, condition: JumpCondition) -> u8 {
    let pc = cpu.registers.pc_next();
    let jump = check_jp_cond(cpu, condition);

    if jump {
        let pc_after_jump = cpu.fetch_little_endian_u16(pc);

        cpu.registers.set_pc(pc_after_jump);

        16
    } else {
        12
    }
}

pub fn jp_hl(cpu: &mut Cpu) -> u8 {
    let hl = cpu.registers.hl();

    cpu.registers.set_pc(hl);

    4
}

pub fn ret(cpu: &mut Cpu) -> u8 {
    let address = cpu.registers.sp();
    let value = cpu.fetch_little_endian_u16(address);
    cpu.registers.inc_sp(2);

    cpu.registers.set_pc(value);

    16
}

pub fn ret_cond(cpu: &mut Cpu, condition: JumpCondition) -> u8 {
    let jump = check_jp_cond(cpu, condition);

    if jump { ret(cpu) + 4 } else { 8 }
}

// FIXME: this might be wrong
pub fn call_a16(cpu: &mut Cpu) -> u8 {
    let value = cpu.registers.pc_next();
    let address= cpu.registers.sp();

    cpu.store_little_endian_u16(address, value);
    cpu.registers.dec_sp(2);

    let new_pc = cpu.fetch_little_endian_u16(address);
    cpu.registers.set_pc(new_pc);

    24
}
