use membranes_cpu::{Cpu, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn test(regs: Regs) {
    let regs = Regs {
        pc: 0x00,
        sp: 0xFF,
        ..regs
    };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x00; 0x2000];
    // JSR $0003
    bus[0x0000] = 0x20;
    bus[0x0001] = 0x03;
    bus[0x0002] = 0x00;

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0x0003,
            sp: 0xFD,
            ..regs
        }
    );
    prop_assert_eq!(bus[0x01FE], 0x00);
    prop_assert_eq!(bus[0x01FF], 0x02);
}
