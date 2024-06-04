use membranes_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn no_zero(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        flags: regs.flags.difference(Flags::ZERO),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xD0, 0x05];

    cpu.tick(&mut bus);

    prop_assert_eq!(cpu.regs(), Regs { pc: 0x07, ..regs });
}

#[proptest]
fn zero(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        flags: regs.flags.union(Flags::ZERO),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xD0, 0x05];

    cpu.tick(&mut bus);

    prop_assert_eq!(cpu.regs(), Regs { pc: 0x02, ..regs });
}
