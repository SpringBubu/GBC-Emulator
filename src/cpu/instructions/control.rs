use crate::cpu::{
    Cpu,
    registers::{FlagName, Flags, RegisterName},
};

pub enum JumpCondition {
    Z,
    NZ,
    C,
    NC,
}

pub fn jr_cond_r8(cpu: &mut Cpu, condition: JumpCondition) -> u8 {
    let address = cpu.registers.pc();
    cpu.registers.inc_pc(1);

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
