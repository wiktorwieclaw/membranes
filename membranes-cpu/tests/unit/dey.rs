use membranes_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn positive(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        y: 0x02,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x88];

    cpu.tick(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x01,
            y: 0x01,
            flags: regs.flags.difference(Flags::NEGATIVE | Flags::ZERO),
            ..regs
        }
    );
}

#[proptest]
fn zero(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        y: 0x01,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x88];

    cpu.tick(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x01,
            y: 0x00,
            flags: regs.flags.union(Flags::ZERO).difference(Flags::NEGATIVE),
            ..regs
        }
    );
}

#[proptest]
fn negative(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        y: 0x00,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x88];

    cpu.tick(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x01,
            y: 0xFF,
            flags: regs.flags.union(Flags::NEGATIVE).difference(Flags::ZERO),
            ..regs
        }
    );
}
