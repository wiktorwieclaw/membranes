use bitflags::bitflags;
use nes_emu_bits::{WrappingU16Ext, WrappingU8Ext};
use std::{num::Wrapping, ops::IndexMut};

mod op;

/// Registers
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub struct Regs {
    /// Accumulator
    pub a: Wrapping<u8>,
    /// Register X
    pub x: Wrapping<u8>,
    /// Register Y
    pub y: Wrapping<u8>,
    /// Program counter
    pub pc: Wrapping<u16>,
    /// Stack pointer
    pub sp: Wrapping<u8>,
    /// Status flags
    pub flags: StatusFlags,
}

bitflags! {
    #[derive(Default)]
    #[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
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
    regs: Regs,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_regs(regs: Regs) -> Self {
        Self { regs }
    }

    pub fn regs(&self) -> Regs {
        self.regs
    }

    pub fn execute_next(&mut self, bus: &mut impl Bus) {
        let opcode = bus.read_u8(self.regs.pc);
        self.regs.pc += 1;

        let op = op::Op::parse(opcode.0);

        let operand = read_operand(op.mode(), &mut self.regs, bus);
        match (op.mnemonic(), operand) {
            (op::Mnemonic::Lda, Some(operand)) => {
                lda(operand, &mut self.regs, bus);
            }
            _ => todo!(),
        };
    }
}

fn read_operand(mode: op::Mode, regs: &mut Regs, bus: &mut impl Bus) -> Option<Wrapping<u16>> {
    match mode {
        op::Mode::Implied => None,

        op::Mode::Immediate => {
            let data = regs.pc;
            regs.pc += 1;
            Some(data)
        }

        op::Mode::ZeroPage => {
            let address = bus.read_u8(regs.pc);
            regs.pc += 1;
            Some(address.into_wrapping_u16())
        }

        op::Mode::ZeroPageX => {
            let address = bus.read_u8(regs.pc) + regs.x;
            regs.pc += 1;
            Some(address.into_wrapping_u16())
        }

        op::Mode::ZeroPageY => {
            let address = bus.read_u8(regs.pc) + regs.y;
            regs.pc += 1;
            Some(address.into_wrapping_u16())
        }

        op::Mode::Absolute => {
            let data = bus.read_u16(regs.pc);
            regs.pc += 2;
            Some(data)
        }

        op::Mode::AbsoluteX => {
            let data = bus.read_u16(regs.pc) + regs.x.into_wrapping_u16();
            regs.pc += 2;
            Some(data)
        }

        op::Mode::AbsoluteY => {
            let data = bus.read_u16(regs.pc) + regs.y.into_wrapping_u16();
            regs.pc += 2;
            Some(data)
        }

        op::Mode::IndirectX => {
            todo!()
        }

        op::Mode::IndirectY => {
            todo!()
        }
    }
}

pub fn lda(address: Wrapping<u16>, regs: &mut Regs, bus: &mut impl Bus) {
    let value = bus.read_u8(address);
    regs.a = value;

    let is_zero = regs.a == Wrapping(0);
    regs.flags.set(StatusFlags::ZERO, is_zero);

    let is_negative = regs.a & Wrapping(0b1000_0000) != Wrapping(0);
    regs.flags.set(StatusFlags::NEGATIVE, is_negative);
}
