use crate::cpu::Cpu;

pub enum JumpCondition {
    Z,
    NZ,
    C,
    NC,
}

pub fn jr_s8(cpu: &mut Cpu) -> u8 {
    let address = cpu.registers.pc_next();
    let value = cpu.mmu.read(address) as i8;
    let pc_after_jump = address.wrapping_add_signed(value.into());

    cpu.registers.set_pc(pc_after_jump);

    12
}

pub fn jr_cond_s8(cpu: &mut Cpu, condition: JumpCondition) -> u8 {
    let address = cpu.registers.pc_next();
    let value = cpu.mmu.read(address) as i8;
    let jump = match condition {
        JumpCondition::Z => cpu.registers.flags_z(),
        JumpCondition::NZ => !cpu.registers.flags_z(),
        JumpCondition::C => cpu.registers.flags_c(),
        JumpCondition::NC => !cpu.registers.flags_c(),
    };

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
    let pc_after_jump = cpu.fetch_little_endian_d16(pc);

    cpu.registers.set_pc(pc_after_jump);

    16
}

pub fn jp_cond_a16(cpu: &mut Cpu, condition: JumpCondition) -> u8 {
    let pc = cpu.registers.pc_next();
    let jump = match condition {
        JumpCondition::Z => cpu.registers.flags_z(),
        JumpCondition::NZ => !cpu.registers.flags_z(),
        JumpCondition::C => cpu.registers.flags_c(),
        JumpCondition::NC => !cpu.registers.flags_c(),
    };

    if jump {
        let pc_after_jump = cpu.fetch_little_endian_d16(pc);

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
