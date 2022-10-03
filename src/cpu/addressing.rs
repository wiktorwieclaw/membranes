use std::num::Wrapping;

use super::{Bus, Registers};
use crate::util::WrappingU8Ext;

pub fn operand_immediate(registers: &mut Registers) -> Wrapping<u16> {
    let data = registers.program_counter;
    registers.program_counter += 1;
    data
}

pub fn operand_zero_page(registers: &mut Registers, memory: &mut impl Bus) -> Wrapping<u16> {
    let address = memory.read_u8(registers.program_counter);
    registers.program_counter += 1;
    address.into_wrapping_u16()
}

pub fn operand_zero_page_x(registers: &mut Registers, memory: &mut impl Bus) -> Wrapping<u16> {
    let address = memory.read_u8(registers.program_counter) + registers.x;
    registers.program_counter += 1;
    address.into_wrapping_u16()
}

pub fn operand_zero_page_y(registers: &mut Registers, memory: &mut impl Bus) -> Wrapping<u16> {
    let address = memory.read_u8(registers.program_counter) + registers.y;
    registers.program_counter += 1;
    address.into_wrapping_u16()
}

pub fn operand_absolute(registers: &mut Registers, memory: &mut impl Bus) -> Wrapping<u16> {
    let data = memory.read_u16(registers.program_counter);
    registers.program_counter += 2;
    data
}

pub fn operand_absolute_x(registers: &mut Registers, memory: &mut impl Bus) -> Wrapping<u16> {
    let data = memory.read_u16(registers.program_counter) + registers.x.into_wrapping_u16();
    registers.program_counter += 2;
    data
}

pub fn operand_absolute_y(registers: &mut Registers, memory: &mut impl Bus) -> Wrapping<u16> {
    let data = memory.read_u16(registers.program_counter) + registers.y.into_wrapping_u16();
    registers.program_counter += 2;
    data
}
