use nes_emu_cpu::{Cpu, Registers, StatusFlags};
use proptest::{prelude::any, strategy::Strategy};
use std::num::Wrapping;
use test_strategy::proptest;

fn positive_byte() -> impl Strategy<Value = u8> {
    0b0000_0001..0b0111_1111u8
}

fn negative_byte() -> impl Strategy<Value = u8> {
    0b1000_0000..0b1111_1111u8
}

fn any_registers() -> impl Strategy<Value = Registers> {
    any::<(u8, u8, u8, u16, u8, u8)>().prop_map(|(a, x, y, pc, sp, status)| Registers {
        a: Wrapping(a),
        x: Wrapping(x),
        y: Wrapping(y),
        program_counter: Wrapping(pc),
        stack_pointer: Wrapping(sp),
        status_flags: StatusFlags::from_bits(status).unwrap(),
    })
}

#[proptest]
fn lda_updates_a_and_status_when_operand_is_0(
    #[strategy(any_registers())] mut registers: Registers,
) {
    // Arrange
    registers.program_counter = Wrapping(0x0000);
    let mut cpu = Cpu::new().with_regs(registers);
    let mut memory = [0xa9, 0x00].map(Wrapping); // LDA #$00

    // Act
    cpu.execute_next(&mut memory);

    // Assert
    let expected = Registers {
        a: Wrapping(0x00),
        program_counter: Wrapping(0x0002),
        status_flags: registers
            .status_flags
            .union(StatusFlags::ZERO)
            .difference(StatusFlags::NEGATIVE),
        ..registers
    };
    assert_eq!(cpu.regs(), expected);
}

#[proptest]
fn lda_updates_a_and_status_when_operand_is_positive(
    #[strategy(any_registers())] mut registers: Registers,
    #[strategy(positive_byte())] operand: u8,
) {
    // Arrange
    registers.program_counter = Wrapping(0x0000);
    let mut cpu = Cpu::new().with_regs(registers);
    let mut memory = [0xa9, operand].map(Wrapping); // LDA #operand

    // Act
    cpu.execute_next(&mut memory);

    // Assert
    let expected = Registers {
        a: Wrapping(operand),
        program_counter: Wrapping(0x0002),
        status_flags: registers
            .status_flags
            .difference(StatusFlags::ZERO | StatusFlags::NEGATIVE),
        ..registers
    };
    assert_eq!(cpu.regs(), expected);
}

#[proptest]
fn lda_updates_a_and_status_when_operand_is_negative(
    #[strategy(any_registers())] mut registers: Registers,
    #[strategy(negative_byte())] operand: u8,
) {
    // Arrange
    registers.program_counter = Wrapping(0x0000);
    let mut cpu = Cpu::new().with_regs(registers);
    let mut memory = [0xa9, operand].map(Wrapping); // LDA #operand

    // Act
    cpu.execute_next(&mut memory);

    // Assert
    let expected = Registers {
        a: Wrapping(operand),
        program_counter: Wrapping(0x0002),
        status_flags: registers
            .status_flags
            .difference(StatusFlags::ZERO)
            .union(StatusFlags::NEGATIVE),
        ..registers
    };
    assert_eq!(cpu.regs(), expected);
}
