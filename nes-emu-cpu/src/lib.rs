use bitflags::bitflags;
use nes_emu_bits::prelude::*;
use op::Op;
use std::{num::Wrapping, ops::IndexMut};

mod op;

/// Registers
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub struct Regs {
    /// Accumulator
    pub a: Wu8,
    /// Register X
    pub x: Wu8,
    /// Register Y
    pub y: Wu8,
    /// Program counter
    pub pc: Wu16,
    /// Stack pointer
    pub sp: Wu8,
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
    fn read_u8(&mut self, address: Wu16) -> Wu8;

    fn write_u8(&mut self, address: Wu16, data: Wu8);

    fn read_u16(&mut self, address: Wu16) -> Wu16 {
        let lo = self.read_u8(address).into_wu16();
        let hi = self.read_u8(address + Wrapping(1)).into_wu16();
        (hi << 8) | lo
    }

    fn write_u16(&mut self, address: Wu16, data: Wu16) {
        let hi = (data >> 8).cast_wu8();
        let lo = data.cast_wu8();
        self.write_u8(address, lo);
        self.write_u8(address + Wrapping(1), hi);
    }
}

impl<T> Bus for T
where
    T: IndexMut<usize, Output = Wu8>,
{
    fn read_u8(&mut self, address: Wu16) -> Wu8 {
        self[usize::from(address.0)]
    }

    fn write_u8(&mut self, address: Wu16, data: Wu8) {
        self[usize::from(address.0)] = data;
    }
}

#[derive(Default)]
pub struct Cpu {
    regs: Regs,
}

pub enum SideEffect {
    Break,
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

    pub fn execute_next(&mut self, bus: &mut impl Bus) -> Option<SideEffect> {
        let regs = &mut self.regs;

        let opcode = bus.read_u8(regs.pc);
        regs.pc += 1;

        let op = Op::parse(opcode);
        let (mnemonic, mode) = (op.mnemonic(), op.mode());

        let operand = operand_address(mode, regs, bus);

        match (mnemonic, operand) {
            (op::Mnemonic::Brk, None) => {
                regs.flags.set(StatusFlags::B_1, true);
                return Some(SideEffect::Break);
            }
            (op::Mnemonic::Lda, Some(operand)) => lda(operand, regs, bus),
            (op::Mnemonic::Sta, Some(operand)) => sta(operand, regs, bus),
            _ => todo!(),
        };

        None
    }
}

fn operand_address(mode: op::Mode, regs: &mut Regs, bus: &mut impl Bus) -> Option<Wu16> {
    match mode {
        op::Mode::Implicit | op::Mode::Accumulator => None,

        op::Mode::Immediate => {
            let address = regs.pc;
            regs.pc += 1;
            Some(address)
        }

        op::Mode::ZeroPage => {
            let address = bus.read_u8(regs.pc);
            regs.pc += 1;
            Some(address.into_wu16())
        }

        op::Mode::ZeroPageX => {
            let address = bus.read_u8(regs.pc) + regs.x;
            regs.pc += 1;
            Some(address.into_wu16())
        }

        op::Mode::ZeroPageY => {
            let address = bus.read_u8(regs.pc) + regs.y;
            regs.pc += 1;
            Some(address.into_wu16())
        }

        op::Mode::Absolute => {
            let address = bus.read_u16(regs.pc);
            regs.pc += 2;
            Some(address)
        }

        op::Mode::AbsoluteX => {
            let address = bus.read_u16(regs.pc) + regs.x.into_wu16();
            regs.pc += 2;
            Some(address)
        }

        op::Mode::AbsoluteY => {
            let address = bus.read_u16(regs.pc) + regs.y.into_wu16();
            regs.pc += 2;
            Some(address)
        }

        op::Mode::IndirectX => {
            let address = (bus.read_u8(regs.pc) + regs.x).into_wu16();
            let address = bus.read_u16(address);
            regs.pc += 1;
            Some(address)
        }

        op::Mode::IndirectY => {
            let address = bus.read_u8(regs.pc).into_wu16();
            let address = bus.read_u16(address) + regs.y.into_wu16();
            regs.pc += 1;
            Some(address)
        }
    }
}

fn lda(address: Wu16, regs: &mut Regs, bus: &mut impl Bus) {
    regs.a = bus.read_u8(address);

    let is_zero = regs.a == Wrapping(0);
    regs.flags.set(StatusFlags::ZERO, is_zero);

    let is_negative = regs.a & Wrapping(0b1000_0000) != Wrapping(0);
    regs.flags.set(StatusFlags::NEGATIVE, is_negative);
}

fn sta(address: Wu16, regs: &Regs, bus: &mut impl Bus) {
    bus.write_u8(address, regs.a)
}
