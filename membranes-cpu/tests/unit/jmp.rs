use membranes_cpu::{Cpu, Regs};
use proptest::prelude::*;
use test_strategy::proptest;

#[proptest]
fn test(regs: Regs) {
    let regs = Regs { pc: 0x00, ..regs };
    let mut cpu = Cpu::from_regs(regs);
    let mut bus = [0x00; 0xFFFF];
    bus[0x0000] = 0x6C;
    bus[0x0001] = 0xFC;
    bus[0x0002] = 0xFF;
    bus[0xFFFC] = 0xFC;
    bus[0xFFFD] = 0xBA;

    cpu.tick(&mut bus);

    prop_assert_eq!(cpu.regs(), Regs { pc: 0xBAFC, ..regs });
}
