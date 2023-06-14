use nes_emu_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn overflow_into_zero(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0x01,
        flags: regs.flags.union(Flags::CARRY),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xE9, 0x01];

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
fn overflow_into_negative(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0x00,
        flags: regs.flags.difference(Flags::CARRY),
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xE9, 0x00];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0xFF,
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::OVERFLOW | Flags::NEGATIVE)
                .difference(Flags::CARRY | Flags::ZERO),
            ..regs
        }
    );
}
