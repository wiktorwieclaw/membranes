use crate::strategy::*;
use membranes_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn zero(regs: Regs) {
    let regs = Regs { pc: 0x00, ..regs };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xA0, 0x00];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            y: 0x00,
            pc: 0x02,
            flags: regs.flags.union(Flags::ZERO).difference(Flags::NEGATIVE),
            ..regs
        }
    );
}

#[proptest]
fn positive(regs: Regs, #[strategy(positive_byte())] operand: u8) {
    let regs = Regs { pc: 0x00, ..regs };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xA0, operand];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            y: operand,
            pc: 0x02,
            flags: regs.flags.difference(Flags::ZERO | Flags::NEGATIVE),
            ..regs
        }
    );
}

#[proptest]
fn negative(regs: Regs, #[strategy(negative_byte())] operand: u8) {
    let regs = Regs { pc: 0x00, ..regs };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0xA0, operand];

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            y: operand,
            pc: 0x02,
            flags: regs.flags.difference(Flags::ZERO).union(Flags::NEGATIVE),
            ..regs
        }
    );
}
