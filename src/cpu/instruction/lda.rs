use std::num::Wrapping;

use crate::cpu::{Bus, Registers, StatusFlags};

pub fn lda(address: Wrapping<u16>, registers: &mut Registers, memory: &mut impl Bus) {
    let value = memory.read_u8(address);
    registers.a = value;

    let is_zero = registers.a == Wrapping(0);
    registers.status_flags.set(StatusFlags::ZERO, is_zero);

    let is_negative = registers.a & Wrapping(0b1000_0000) != Wrapping(0);
    registers
        .status_flags
        .set(StatusFlags::NEGATIVE, is_negative);
}

#[cfg(test)]
mod test {
    use proptest::{prelude::any, strategy::Strategy};
    use test_strategy::proptest;

    use super::*;

    fn positive_byte() -> impl Strategy<Value = Wrapping<u8>> {
        (0x01..0x7F_u8).prop_map(Wrapping)
    }

    fn negative_byte() -> impl Strategy<Value = Wrapping<u8>> {
        (0x80..0xFF_u8).prop_map(Wrapping)
    }

    #[proptest]
    fn lda_sets_zero_flag_when_value_is_0(
        #[strategy(any::<Registers>())] mut registers: Registers,
    ) {
        // Arrange
        let registers_before = registers.clone();
        let mut memory = [Wrapping(0x00)];

        // Act
        lda(Wrapping(0), &mut registers, &mut memory);

        // Assert
        assert_eq!(registers.a, Wrapping(0x00));
        assert_eq!(registers.x, registers_before.x);
        assert_eq!(registers.y, registers_before.y);
        assert_eq!(registers.program_counter, registers_before.program_counter);
        assert_eq!(registers.stack_pointer, registers_before.stack_pointer);
        assert_eq!(
            registers.status_flags,
            registers_before
                .status_flags
                .union(StatusFlags::ZERO)
                .difference(StatusFlags::NEGATIVE)
        );
    }

    #[proptest]
    fn lda_sets_register_a(
        #[strategy(any::<Registers>())] mut registers: Registers,
        #[strategy(positive_byte())] value: Wrapping<u8>,
    ) {
        // Arrange
        let registers_before = registers.clone();
        let mut memory = [value];

        // Act
        lda(Wrapping(0x00), &mut registers, &mut memory);

        // Assert
        assert_eq!(registers.a, value);
        assert_eq!(registers.x, registers_before.x);
        assert_eq!(registers.y, registers_before.y);
        assert_eq!(registers.program_counter, registers_before.program_counter);
        assert_eq!(registers.stack_pointer, registers_before.stack_pointer);
        assert_eq!(
            registers.status_flags,
            registers_before
                .status_flags
                .difference(StatusFlags::ZERO | StatusFlags::NEGATIVE),
        );
    }

    #[proptest]
    fn lda_loads_and_sets_negative_flag(
        #[strategy(any::<Registers>())] mut registers: Registers,
        #[strategy(negative_byte())] value: Wrapping<u8>,
    ) {
        // Arrange
        let registers_before = registers.clone();
        let mut memory = [value];

        // Act
        lda(Wrapping(0x00), &mut registers, &mut memory);

        // Assert
        assert_eq!(registers.a, value);
        assert_eq!(registers.x, registers_before.x);
        assert_eq!(registers.y, registers_before.y);
        assert_eq!(registers.program_counter, registers_before.program_counter);
        assert_eq!(registers.stack_pointer, registers_before.stack_pointer);
        assert_eq!(
            registers.status_flags,
            registers_before
                .status_flags
                .difference(StatusFlags::ZERO)
                .union(StatusFlags::NEGATIVE)
        );
    }
}
