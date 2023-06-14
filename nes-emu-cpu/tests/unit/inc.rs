use nes_emu_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn positive(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xE6, 0x02, 0x00];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x02,
            flags: regs.flags.difference(Flags::NEGATIVE | Flags::ZERO),
            ..regs
        }
    );
    prop_assert_eq!(bus[0x0002], 0x01);
}

#[proptest]
fn zero(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xE6, 0x02, 0xFF];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x02,
            flags: regs.flags.union(Flags::ZERO).difference(Flags::NEGATIVE),
            ..regs
        }
    );
    prop_assert_eq!(bus[0x0002], 0x00);
}

#[proptest]
fn negative(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xE6, 0x02, 0b1000_0000];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x02,
            flags: regs.flags.union(Flags::NEGATIVE).difference(Flags::ZERO),
            ..regs
        }
    );
    prop_assert_eq!(bus[0x0002], 0b1000_0001);
}
