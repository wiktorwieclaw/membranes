use nes_emu_cpu::{Cpu, Regs, StatusFlags};
use proptest::prelude::*;
use std::num::Wrapping;

prop_compose! {
    fn positive_byte()(v in 0b0000_0001..0b0111_1111u8) -> Wrapping<u8> {
        Wrapping(v)
    }
}

prop_compose! {
    fn negative_byte()(v in 0b1000_0000..0b1111_1111u8) -> Wrapping<u8> {
        Wrapping(v)
    }
}

prop_compose! {
    fn regs_with_pc(pc: u16)(mut regs: Regs) -> Regs {
        regs.pc = Wrapping(pc);
        regs
    }
}

proptest! {
    #[test]
    fn lda_updates_a_and_status_when_operand_is_0(regs in regs_with_pc(0x00)) {
        let mut cpu = Cpu::from_regs(regs);
        let mut bus = [Wrapping(0xA9), Wrapping(0)];

        cpu.execute_next(&mut bus);

        assert_eq!(cpu.regs(), Regs {
            a: Wrapping(0x00),
            pc: Wrapping(0x02),
            flags: regs
                .flags
                .union(StatusFlags::ZERO)
                .difference(StatusFlags::NEGATIVE),
            ..regs
        });
    }

    #[test]
    fn lda_updates_a_and_status_when_operand_is_positive(
        regs in regs_with_pc(0x00),
        operand in positive_byte(),
    ) {
        let mut cpu = Cpu::from_regs(regs);
        let mut bus = [Wrapping(0xA9), operand];

        cpu.execute_next(&mut bus);

        assert_eq!(cpu.regs(), Regs {
            a: operand,
            pc: Wrapping(0x02),
            flags: regs
                .flags
                .difference(StatusFlags::ZERO | StatusFlags::NEGATIVE),
            ..regs
        });
    }

    #[test]
    fn lda_updates_a_and_status_when_operand_is_negative(
        regs in regs_with_pc(0x00),
        operand in negative_byte(),
    ) {
        let mut cpu = Cpu::from_regs(regs);
        let mut bus = [Wrapping(0xA9), operand];

        cpu.execute_next(&mut bus);

        assert_eq!(cpu.regs(), Regs {
            a: operand,
            pc: Wrapping(0x02),
            flags: regs
                .flags
                .difference(StatusFlags::ZERO)
                .union(StatusFlags::NEGATIVE),
            ..regs
        });
    }
}
