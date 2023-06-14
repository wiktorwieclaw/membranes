use crate::strategy::*;
use nes_emu_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn zero(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        x: 0,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x8A];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0x00,
            pc: 0x01,
            flags: regs.flags.union(Flags::ZERO).difference(Flags::NEGATIVE),
            ..regs
        }
    );
}

#[proptest]
fn positive(regs: Regs, #[strategy(positive_byte())] x: u8) {
    let regs = Regs {
        pc: 0x00,
        x,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x8A];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: regs.x,
            pc: 0x01,
            flags: regs.flags.difference(Flags::ZERO | Flags::NEGATIVE),
            ..regs
        }
    );
}

#[proptest]
fn negative(regs: Regs, #[strategy(negative_byte())] x: u8) {
    let regs = Regs {
        pc: 0x00,
        x,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x8A];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: regs.x,
            pc: 0x01,
            flags: regs.flags.difference(Flags::ZERO).union(Flags::NEGATIVE),
            ..regs
        }
    );
}
