use bitflags::bitflags;
use op::Op;
use std::ops::IndexMut;

mod op;

/// Registers
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub struct Regs {
    /// Accumulator
    pub a: u8,
    /// Register X
    pub x: u8,
    /// Register Y
    pub y: u8,
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u8,
    /// Status flags
    pub flags: Flags,
}

bitflags! {
    #[derive(Default)]
    #[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
    pub struct Flags: u8 {
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
    fn read_u8(&mut self, address: u16) -> u8;

    fn write_u8(&mut self, address: u16, data: u8);

    fn read_u16(&mut self, address: u16) -> u16 {
        let lo: u16 = self.read_u8(address).into();
        let hi: u16 = self.read_u8(address.wrapping_add(1)).into();
        (hi << 8) | lo
    }

    fn write_u16(&mut self, address: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = data as u8;
        self.write_u8(address, lo);
        self.write_u8(address.wrapping_add(1), hi);
    }
}

impl<T> Bus for T
where
    T: IndexMut<usize, Output = u8>,
{
    fn read_u8(&mut self, address: u16) -> u8 {
        self[usize::from(address)]
    }

    fn write_u8(&mut self, address: u16, data: u8) {
        self[usize::from(address)] = data;
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

    pub fn next(&mut self, bus: &mut impl Bus) -> Option<SideEffect> {
        let regs = &mut self.regs;

        let opcode = bus.read_u8(regs.pc);
        regs.pc = regs.pc.wrapping_add(1);

        let op = Op::parse(opcode);
        let (mnemonic, mode) = (op.mnemonic(), op.mode());

        let address = operand_address(mode, regs, bus);

        match (mnemonic, address) {
            (op::Mnemonic::Brk, None) => {
                regs.flags.set(Flags::B_1, true);
                return Some(SideEffect::Break);
            }
            (op::Mnemonic::Lda, Some(address)) => lda(address, regs, bus),
            (op::Mnemonic::Lda, None) => unreachable!(),
            (op::Mnemonic::Sta, Some(address)) => sta(address, regs, bus),
            (op::Mnemonic::Sta, None) => unreachable!(),
            _ => todo!(),
        };

        None
    }
}

fn operand_address(mode: op::Mode, regs: &mut Regs, bus: &mut impl Bus) -> Option<u16> {
    match mode {
        op::Mode::Implicit | op::Mode::Accumulator => None,

        op::Mode::Immediate => {
            let address = regs.pc;
            regs.pc = regs.pc.wrapping_add(1);
            Some(address)
        }

        op::Mode::ZeroPage => {
            let address = bus.read_u8(regs.pc);
            regs.pc = regs.pc.wrapping_add(1);
            Some(address.into())
        }

        op::Mode::ZeroPageX => {
            let address = bus.read_u8(regs.pc).wrapping_add(regs.x);
            regs.pc = regs.pc.wrapping_add(1);
            Some(address.into())
        }

        op::Mode::ZeroPageY => {
            let address = bus.read_u8(regs.pc).wrapping_add(regs.y);
            regs.pc = regs.pc.wrapping_add(1);
            Some(address.into())
        }

        op::Mode::Absolute => {
            let address = bus.read_u16(regs.pc);
            regs.pc = regs.pc.wrapping_add(2);
            Some(address)
        }

        op::Mode::AbsoluteX => {
            let address = bus.read_u16(regs.pc).wrapping_add(regs.x.into());
            regs.pc = regs.pc.wrapping_add(2);
            Some(address)
        }

        op::Mode::AbsoluteY => {
            let address = bus.read_u16(regs.pc).wrapping_add(regs.y.into());
            regs.pc = regs.pc.wrapping_add(2);
            Some(address)
        }

        op::Mode::IndirectX => {
            let address = bus.read_u8(regs.pc).wrapping_add(regs.x).into();
            let address = bus.read_u16(address);
            regs.pc = regs.pc.wrapping_add(1);
            Some(address)
        }

        op::Mode::IndirectY => {
            let address = bus.read_u8(regs.pc).into();
            let address = bus.read_u16(address).wrapping_add(regs.y.into());
            regs.pc = regs.pc.wrapping_add(1);
            Some(address)
        }
    }
}

fn adc(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let operand = bus.read_u8(address);
    let (sum, new_carry1) = regs.a.overflowing_add(operand);

    let old_carry = regs.flags.contains(Flags::CARRY) as u8;
    let (sum, new_carry2) = sum.overflowing_add(old_carry);
    
    regs.flags.set(Flags::CARRY, new_carry1 | new_carry2);
    regs.flags.set(Flags::ZERO, is_zero(sum));
    regs.flags.set(Flags::OVERFLOW, is_signed_overflow(operand, regs.a, sum));
    regs.flags.set(Flags::NEGATIVE, is_negative(sum));

    regs.a = sum;
}

fn lda(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    regs.a = bus.read_u8(address);
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn sta(address: u16, regs: &Regs, bus: &mut impl Bus) {
    bus.write_u8(address, regs.a)
}

fn is_zero(n: u8) -> bool {
    n == 0x00
}

fn is_negative(n: u8) -> bool {
    n & 0b1000_0000 != 0
}

fn is_signed_overflow(n: u8, m: u8, result: u8) -> bool {
    (n ^ result) & (m & result) & 0x80 != 0
}