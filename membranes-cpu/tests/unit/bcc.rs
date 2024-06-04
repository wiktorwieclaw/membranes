use membranes_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn no_carry(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        flags: regs.flags.difference(Flags::CARRY),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x90, 0x02];

    cpu.tick(&mut bus);

    prop_assert_eq!(cpu.regs(), Regs { pc: 0x04, ..regs });
}

#[proptest]
fn carry(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        flags: regs.flags.union(Flags::CARRY),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x90, 0x05];

    cpu.tick(&mut bus);

    prop_assert_eq!(cpu.regs(), Regs { pc: 0x02, ..regs });
}
