use nes_emu_cpu::{Regs, Cpu, Flags};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn zero(regs: Regs) {
    let regs = Regs { pc: 0x00, a: 0b1111_1111, ..regs };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x29, 0b0000_0000];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0b0000_0000,
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::ZERO)
                .difference(Flags::NEGATIVE),
            ..regs
        }
    );
}

#[proptest]
fn positive(regs: Regs) {
    let regs = Regs { pc: 0x00, a: 0b0001_0001, ..regs };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x29, 0b0011_0011];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0b0001_0001,
            pc: 0x02,
            flags: regs
                .flags
                .difference(Flags::NEGATIVE | Flags::ZERO),
            ..regs
        }
    );
}

#[proptest]
fn negative(regs: Regs) {
    let regs = Regs { pc: 0x00, a: 0b1000_0000, ..regs };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x29, 0b1000_0001];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0b1000_0000,
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::NEGATIVE)
                .difference(Flags::ZERO),
            ..regs
        }
    );
}