use nes_emu_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn accumulator_zero(regs: Regs) {
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
fn accumulator_carry(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        a: 0b1000_0001,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x0A];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            a: 0b0000_0010,
            pc: 0x01,
            flags: regs
                .flags
                .union(Flags::CARRY)
                .difference(Flags::ZERO | Flags::NEGATIVE),
            ..regs
        }
    );
}

#[proptest]
fn memory_carry(regs: Regs) {
    let regs = Regs { pc: 0x00, ..regs };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x06, 0x02, 0b1000_0001];

    cpu.next(&mut bus);

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
    prop_assert_eq!(bus[0x0002], 0b0000_0010);
}
