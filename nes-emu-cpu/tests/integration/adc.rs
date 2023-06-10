use nes_emu_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn signed_wrapping_into_zero(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0xFF,
        flags: regs.flags.difference(Flags::CARRY),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x69, 0x01];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0x00,
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::ZERO | Flags::CARRY)
                .difference(Flags::NEGATIVE | Flags::OVERFLOW),
            ..regs
        }
    );
}

#[proptest]
fn signed_wrapping_into_positive(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0xFF,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x69, 0x02];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0x01 + regs.flags.contains(Flags::CARRY) as u8,
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::CARRY)
                .difference(Flags::OVERFLOW | Flags::ZERO | Flags::NEGATIVE),
            ..regs
        }
    );
}

#[proptest]
fn unsigned_overflow_into_zero(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0x80,
        flags: regs.flags.difference(Flags::CARRY),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x69, 0x80];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0x00,
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::OVERFLOW | Flags::CARRY | Flags::ZERO)
                .difference(Flags::NEGATIVE),
            ..regs
        }
    );
}

#[proptest]
fn unsigned_overflow_into_positive(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0x81,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x69, 0x81];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0x02 + regs.flags.contains(Flags::CARRY) as u8,
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::OVERFLOW | Flags::CARRY)
                .difference(Flags::NEGATIVE | Flags::ZERO),
            ..regs
        }
    );
}

#[proptest]
fn unsigned_overflow_into_negative(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0x50,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x69, 0x50];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0xA0 + regs.flags.contains(Flags::CARRY) as u8,
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::OVERFLOW | Flags::NEGATIVE)
                .difference(Flags::CARRY | Flags::ZERO),
            ..regs
        }
    );
}
