use std::num::Wrapping;

use crate::{Bus, Registers, StatusFlags};

pub fn lda(address: Wrapping<u16>, registers: &mut Registers, bus: &mut impl Bus) {
    let value = bus.read_u8(address);
    registers.a = value;

    let is_zero = registers.a == Wrapping(0);
    registers.status_flags.set(StatusFlags::ZERO, is_zero);

    let is_negative = registers.a & Wrapping(0b1000_0000) != Wrapping(0);
    registers
        .status_flags
        .set(StatusFlags::NEGATIVE, is_negative);
}
