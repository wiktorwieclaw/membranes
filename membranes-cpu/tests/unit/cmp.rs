use membranes_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn greater(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0x02,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xC9, 0x01];

    cpu.tick(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::CARRY)
                .difference(Flags::ZERO | Flags::NEGATIVE),
            ..regs
        }
    );
}

#[proptest]
fn equal(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0x01,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xC9, 0x01];

    cpu.tick(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::CARRY | Flags::ZERO)
                .difference(Flags::NEGATIVE),
            ..regs
        }
    );
}

#[proptest]
fn lesser(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0x01,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xC9, 0x02];

    cpu.tick(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::NEGATIVE)
                .difference(Flags::ZERO | Flags::CARRY),
            ..regs
        }
    );
}
