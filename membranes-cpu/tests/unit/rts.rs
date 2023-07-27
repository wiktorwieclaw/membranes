use membranes_cpu::{Cpu, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn test(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        sp: 0xFD,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x00; 0x2000];
    // prg rom
    bus[0x0000] = 0x60;
    // stack
    bus[0x01FE] = 0xEE;
    bus[0x01FF] = 0xBE;

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0xBEEF,
            sp: 0xFF,
            ..regs
        }
    );
}
