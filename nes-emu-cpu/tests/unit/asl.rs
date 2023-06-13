use nes_emu_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn zero(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0x00,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x0A];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0x00,
            pc: 0x01,
            flags: regs
                .flags
                .union(Flags::ZERO)
                .difference(Flags::NEGATIVE | Flags::CARRY),
            ..regs
        }
    );
}

#[proptest]
fn carry_negative(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0b1100_0000,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x0A];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0b1000_0000,
            pc: 0x01,
            flags: regs
                .flags
                .union(Flags::NEGATIVE | Flags::CARRY)
                .difference(Flags::ZERO),
            ..regs
        }
    );
}
