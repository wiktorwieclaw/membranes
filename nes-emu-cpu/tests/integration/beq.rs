use nes_emu_cpu::{Cpu, Flags, Regs};
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
    let mut bus = [0xF0, 0x05];

    cpu.next(&mut bus);

    prop_assert_eq!(cpu.regs(), Regs { pc: 0x02, ..regs });
}

#[proptest]
fn zero(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        flags: regs.flags.union(Flags::ZERO),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xF0, 0x02];

    cpu.next(&mut bus);

    prop_assert_eq!(cpu.regs(), Regs { pc: 0x04, ..regs });
}
