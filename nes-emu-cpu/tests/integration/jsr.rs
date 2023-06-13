use nes_emu_cpu::{Cpu, Regs};
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
    bus[0x0001] = 0x00;
    bus[0x0002] = 0x03;
    // destination address
    bus[0x0003] = 0xBE;
    bus[0x0004] = 0xEF;

    cpu.next(&mut bus);

    prop_assert_eq!(
        cpu.regs(),
        Regs {
            pc: 0xBEEF,
            sp: 0xFD,
            ..regs
        }
    );
    prop_assert_eq!(bus[0x01FE], 0x02);
    prop_assert_eq!(bus[0x01FF], 0x00);
}
