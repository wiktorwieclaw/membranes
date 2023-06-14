use nes_emu_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn no_negative(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        flags: regs.flags.difference(Flags::NEGATIVE),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x10, 0x05];

    cpu.next(&mut bus);

    prop_assert_eq!(cpu.regs(), Regs { pc: 0x07, ..regs });
}

#[proptest]
fn no_negative_2(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        flags: regs.flags.difference(Flags::NEGATIVE),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x10, 0xFF];

    cpu.next(&mut bus);

    prop_assert_eq!(cpu.regs(), Regs { pc: 0x01, ..regs });
}

#[proptest]
fn negative(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        flags: regs.flags.union(Flags::NEGATIVE),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x10, 0x05];

    cpu.next(&mut bus);

    prop_assert_eq!(cpu.regs(), Regs { pc: 0x02, ..regs });
}
