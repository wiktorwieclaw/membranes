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
    let mut bus = [0x24, 0x02, 0xFF];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::ZERO)
                .difference(Flags::NEGATIVE | Flags::OVERFLOW),
            ..regs
        }
    );
}

#[proptest]
fn overflow(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0b0100_0000,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x24, 0x02, 0xFF];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::OVERFLOW)
                .difference(Flags::NEGATIVE | Flags::ZERO),
            ..regs
        }
    );
}

#[proptest]
fn negative(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0b1000_0000,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x24, 0x02, 0xFF];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::NEGATIVE)
                .difference(Flags::ZERO | Flags::OVERFLOW),
            ..regs
        }
    );
}
