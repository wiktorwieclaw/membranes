use nes_emu_cpu::{Cpu, Flags, Regs};
use proptest::prelude::*;

prop_compose! {
    fn positive_byte()(v in 0b0000_0001..0b0111_1111u8) -> u8 {
        v
    }
}

prop_compose! {
    fn negative_byte()(v in 0b1000_0000..0b1111_1111u8) -> u8 {
        v
    }
}

prop_compose! {
    fn regs_with_pc(pc: u16)(mut regs: Regs) -> Regs {
        regs.pc = pc;
        regs
    }
}

proptest! {
    #[test]
    fn zero(regs in regs_with_pc(0x00)) {
        let mut cpu = Cpu::from_regs(regs);
        let mut bus = [0xA9, 0x00];

        cpu.next(&mut bus);

        assert_eq!(cpu.regs(), Regs {
            a: 0x00,
            pc: 0x02,
            flags: regs
                .flags
                .union(Flags::ZERO)
                .difference(Flags::NEGATIVE),
            ..regs
        });
    }

    #[test]
    fn positive(regs in regs_with_pc(0x00), operand in positive_byte()) {
        let mut cpu = Cpu::from_regs(regs);
        let mut bus = [0xA9, operand];

        cpu.next(&mut bus);

        assert_eq!(cpu.regs(), Regs {
            a: operand,
            pc: 0x02,
            flags: regs
                .flags
                .difference(Flags::ZERO | Flags::NEGATIVE),
            ..regs
        });
    }

    #[test]
    fn negative(regs in regs_with_pc(0x00), operand in negative_byte()) {
        let mut cpu = Cpu::from_regs(regs);
        let mut bus = [0xA9, operand];

        cpu.next(&mut bus);

        assert_eq!(cpu.regs(), Regs {
            a: operand,
            pc: 0x02,
            flags: regs
                .flags
                .difference(Flags::ZERO)
                .union(Flags::NEGATIVE),
            ..regs
        });
    }
}
