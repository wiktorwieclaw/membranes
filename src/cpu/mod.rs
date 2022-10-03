use bitflags::bitflags;
#[cfg(test)]
use proptest_derive::Arbitrary;
use std::{num::Wrapping, ops::IndexMut};

use crate::util::{WrappingU16Ext, WrappingU8Ext};

mod addressing;
mod instruction;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Registers {
    pub a: Wrapping<u8>,
    pub x: Wrapping<u8>,
    pub y: Wrapping<u8>,
    pub program_counter: Wrapping<u16>,
    pub stack_pointer: Wrapping<u8>,
    pub status_flags: StatusFlags,
}

bitflags! {
    #[derive(Default)]
    #[cfg_attr(test, derive(Arbitrary))]
    pub struct StatusFlags: u8 {
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL = 0b0000_1000;
        const B_1 = 0b0001_0000;
        const B_2 = 0b0010_0000;
        const OVERFLOW = 0b0100_0000;
        const NEGATIVE = 0b1000_0000;
    }
}

pub trait Bus {
    fn read_u8(&mut self, address: Wrapping<u16>) -> Wrapping<u8>;

    fn write_u8(&mut self, address: Wrapping<u16>, data: Wrapping<u8>);

    fn read_u16(&mut self, address: Wrapping<u16>) -> Wrapping<u16> {
        let lo = self.read_u8(address).into_wrapping_u16();
        let hi = self.read_u8(address + Wrapping(1)).into_wrapping_u16();
        (hi << 8) | lo
    }

    fn write_u16(&mut self, address: Wrapping<u16>, data: Wrapping<u16>) {
        let hi = (data >> 8).cast_wrapping_u8();
        let lo = (data & Wrapping(0xff)).cast_wrapping_u8();
        self.write_u8(address, lo);
        self.write_u8(address + Wrapping(1), hi);
    }
}

impl<T> Bus for T
where
    T: IndexMut<usize, Output = Wrapping<u8>>,
{
    fn read_u8(&mut self, address: Wrapping<u16>) -> Wrapping<u8> {
        self[usize::from(address.0)]
    }

    fn write_u8(&mut self, address: Wrapping<u16>, data: Wrapping<u8>) {
        self[usize::from(address.0)] = data;
    }
}

#[derive(Default)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute_next(&mut self, memory: &mut impl Bus) {
        let opcode = memory.read_u8(self.registers.program_counter);
        self.registers.program_counter += 1;

        let registers = &mut self.registers;
        match opcode.0 {
            // LDA Immediate
            0xA9 => {
                let operand = addressing::operand_immediate(registers);
                instruction::lda::lda(operand, registers, memory);
            }
            // LDA Zero Page
            0xA5 => {
                let operand = addressing::operand_zero_page(registers, memory);
                instruction::lda::lda(operand, registers, memory);
            }
            // LDA Zero Page,X
            0xB5 => {
                let operand = addressing::operand_zero_page_x(registers, memory);
                instruction::lda::lda(operand, registers, memory);
            }
            // LDA Absolute
            0xAD => {
                let operand = addressing::operand_absolute(registers, memory);
                instruction::lda::lda(operand, registers, memory);
            }
            // LDA Absolute,X
            0xBD => {
                let operand = addressing::operand_absolute_x(registers, memory);
                instruction::lda::lda(operand, registers, memory);
            }
            // LDA Absolute,Y
            0xB9 => {
                let operand = addressing::operand_absolute_y(registers, memory);
                instruction::lda::lda(operand, registers, memory);
            }
            // LDA Indirect,X
            0xA1 => {
                let operand = todo!();
                instruction::lda::lda(operand, registers, memory);
            }
            // LDA Indirect,Y
            0xB1 => {
                let operand = todo!();
                instruction::lda::lda(operand, registers, memory);
            }
            _ => todo!(),
        };
    }
}
