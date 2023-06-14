use bitflags::bitflags;
use op::Op;
use std::ops::IndexMut;

pub mod op;

const STACK_START: u16 = 0x0100;

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
        // Unsigned carry
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL = 0b0000_1000;
        const B_1 = 0b0001_0000;
        const B_2 = 0b0010_0000;
        /// Signed overflow
        const OVERFLOW = 0b0100_0000;
        const NEGATIVE = 0b1000_0000;
    }
}

pub trait Bus {
    fn read_u8(&mut self, address: u16) -> u8;

    fn write_u8(&mut self, address: u16, data: u8);

    /// [AB, CD] -> 0xABCD
    fn read_u16_be(&mut self, address: u16) -> u16 {
        let x = self.read_u8(address);
        let y = self.read_u8(address.wrapping_add(1));
        u16::from_be_bytes([x, y])
    }

    /// 0xABCD -> [AB, CD]
    fn write_u16_be(&mut self, address: u16, data: u16) {
        let [x, y] = data.to_be_bytes();
        self.write_u8(address, x);
        self.write_u8(address.wrapping_add(1), y);
    }

    /// [AB, CD] -> 0xCDAB
    fn read_u16_le(&mut self, address: u16) -> u16 {
        let x = self.read_u8(address);
        let y = self.read_u8(address.wrapping_add(1));
        u16::from_le_bytes([x, y])
    }

    /// 0xABCD -> [CD, AB]
    fn write_u16_le(&mut self, address: u16, data: u16) {
        let [x, y] = data.to_le_bytes();
        self.write_u8(address, x);
        self.write_u8(address.wrapping_add(1), y);
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
    pub regs: Regs,
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

    pub fn next(&mut self, bus: &mut impl Bus) -> (op::Op, Option<SideEffect>) {
        let regs = &mut self.regs;

        let opcode = bus.read_u8(regs.pc);
        regs.pc = regs.pc.wrapping_add(1);

        let op = Op::parse(opcode).expect(&format!("Unsupported opcode: {opcode:x}"));

        let (mnemonic, mode) = (op.mnemonic(), op.mode());

        let address = operand_address(mode, regs, bus);
        match (mnemonic, address) {
            (op::Mnemonic::Adc, Some(address)) => adc(address, regs, bus),
            (op::Mnemonic::Adc, None) => unreachable!(),
            (op::Mnemonic::And, Some(address)) => and(address, regs, bus),
            (op::Mnemonic::And, None) => unreachable!(),
            (op::Mnemonic::Asl, Some(address)) => asl(address, regs, bus),
            (op::Mnemonic::Asl, None) => asl_a(regs),
            (op::Mnemonic::Bcc, Some(address)) => bcc(address, regs, bus),
            (op::Mnemonic::Bcc, None) => unreachable!(),
            (op::Mnemonic::Bcs, Some(address)) => bcs(address, regs, bus),
            (op::Mnemonic::Bcs, None) => unreachable!(),
            (op::Mnemonic::Beq, Some(address)) => beq(address, regs, bus),
            (op::Mnemonic::Beq, None) => unreachable!(),
            (op::Mnemonic::Bit, Some(address)) => bit(address, regs, bus),
            (op::Mnemonic::Bit, None) => unreachable!(),
            (op::Mnemonic::Bne, Some(address)) => bne(address, regs, bus),
            (op::Mnemonic::Bne, None) => unreachable!(),
            (op::Mnemonic::Bpl, Some(address)) => bpl(address, regs, bus),
            (op::Mnemonic::Bpl, None) => unreachable!(),
            (op::Mnemonic::Brk, Some(_)) => unreachable!(),
            (op::Mnemonic::Brk, None) => {
                regs.flags.set(Flags::B_1, true);
                return (op, Some(SideEffect::Break));
            }
            (op::Mnemonic::Clc, Some(_)) => unreachable!(),
            (op::Mnemonic::Clc, None) => clc(regs),
            (op::Mnemonic::Cld, Some(_)) => unreachable!(),
            (op::Mnemonic::Cld, None) => cld(regs),
            (op::Mnemonic::Cli, Some(_)) => unreachable!(),
            (op::Mnemonic::Cli, None) => cli(regs),
            (op::Mnemonic::Clv, Some(_)) => unreachable!(),
            (op::Mnemonic::Clv, None) => clv(regs),
            (op::Mnemonic::Cmp, Some(address)) => cmp(address, regs, bus),
            (op::Mnemonic::Cmp, None) => unreachable!(),
            (op::Mnemonic::Cpx, Some(address)) => cpx(address, regs, bus),
            (op::Mnemonic::Cpx, None) => unreachable!(),
            (op::Mnemonic::Cpy, Some(address)) => cpy(address, regs, bus),
            (op::Mnemonic::Cpy, None) => unreachable!(),
            (op::Mnemonic::Dec, Some(address)) => dec(address, regs, bus),
            (op::Mnemonic::Dec, None) => unreachable!(),
            (op::Mnemonic::Dex, Some(_)) => unreachable!(),
            (op::Mnemonic::Dex, None) => dex(regs),
            (op::Mnemonic::Dey, Some(_)) => unreachable!(),
            (op::Mnemonic::Dey, None) => dey(regs),
            (op::Mnemonic::Inc, Some(address)) => inc(address, regs, bus),
            (op::Mnemonic::Inc, None) => unreachable!(),
            (op::Mnemonic::Inx, Some(_)) => unreachable!(),
            (op::Mnemonic::Inx, None) => inx(regs),
            (op::Mnemonic::Iny, Some(_)) => unreachable!(),
            (op::Mnemonic::Iny, None) => iny(regs),
            (op::Mnemonic::Jmp, Some(address)) => jmp(address, regs),
            (op::Mnemonic::Jmp, None) => unreachable!(),
            (op::Mnemonic::Jsr, Some(address)) => jsr(address, regs, bus),
            (op::Mnemonic::Jsr, None) => unreachable!(),
            (op::Mnemonic::Nop, Some(_)) => unreachable!(),
            (op::Mnemonic::Nop, None) => nop(),
            (op::Mnemonic::Lda, Some(address)) => lda(address, regs, bus),
            (op::Mnemonic::Lda, None) => unreachable!(),
            (op::Mnemonic::Ldx, Some(address)) => ldx(address, regs, bus),
            (op::Mnemonic::Ldx, None) => unreachable!(),
            (op::Mnemonic::Ldy, Some(address)) => ldy(address, regs, bus),
            (op::Mnemonic::Ldy, None) => unreachable!(),
            (op::Mnemonic::Lsr, Some(address)) => lsr(address, regs, bus),
            (op::Mnemonic::Lsr, None) => lsr_a(regs),
            (op::Mnemonic::Rts, Some(_)) => unreachable!(),
            (op::Mnemonic::Rts, None) => rts(regs, bus),
            (op::Mnemonic::Sbc, Some(address)) => sbc(address, regs, bus),
            (op::Mnemonic::Sbc, None) => unreachable!(),
            (op::Mnemonic::Sec, Some(_)) => unreachable!(),
            (op::Mnemonic::Sec, None) => sec(regs),
            (op::Mnemonic::Sta, Some(address)) => sta(address, regs, bus),
            (op::Mnemonic::Sta, None) => unreachable!(),
            (op::Mnemonic::Txa, Some(_)) => unreachable!(),
            (op::Mnemonic::Txa, None) => txa(regs),
            _ => todo!("{op:?}"),
        };

        (op, None)
    }
}

fn operand_address(mode: op::Mode, regs: &mut Regs, bus: &mut impl Bus) -> Option<u16> {
    match mode {
        op::Mode::Implied | op::Mode::Accumulator => None,

        op::Mode::Immediate | op::Mode::Relative => {
            let operand = regs.pc;
            regs.pc = regs.pc.wrapping_add(1);
            Some(operand)
        }

        op::Mode::ZeroPage => {
            let operand = bus.read_u8(regs.pc);
            regs.pc = regs.pc.wrapping_add(1);
            Some(operand.into())
        }

        op::Mode::ZeroPageX => {
            let operand = bus.read_u8(regs.pc).wrapping_add(regs.x);
            regs.pc = regs.pc.wrapping_add(1);
            Some(operand.into())
        }

        op::Mode::ZeroPageY => {
            let operand = bus.read_u8(regs.pc).wrapping_add(regs.y);
            regs.pc = regs.pc.wrapping_add(1);
            Some(operand.into())
        }

        op::Mode::Absolute => {
            let operand = bus.read_u16_le(regs.pc);
            regs.pc = regs.pc.wrapping_add(2);
            Some(operand)
        }

        op::Mode::AbsoluteX => {
            let operand = bus.read_u16_le(regs.pc).wrapping_add(regs.x.into());
            regs.pc = regs.pc.wrapping_add(2);
            Some(operand)
        }

        op::Mode::AbsoluteY => {
            let operand = bus.read_u16_le(regs.pc).wrapping_add(regs.y.into());
            regs.pc = regs.pc.wrapping_add(2);
            Some(operand)
        }

        op::Mode::Indirect => {
            let address = bus.read_u16_le(regs.pc);
            let operand = bus.read_u16_le(address);
            regs.pc = regs.pc.wrapping_add(1);
            Some(operand)
        }

        op::Mode::IndirectX => {
            let address = bus.read_u8(regs.pc).wrapping_add(regs.x).into();
            let operand = bus.read_u16_le(address);
            regs.pc = regs.pc.wrapping_add(1);
            Some(operand)
        }

        op::Mode::IndirectY => {
            let address = bus.read_u8(regs.pc).into();
            let operand = bus.read_u16_le(address).wrapping_add(regs.y.into());
            regs.pc = regs.pc.wrapping_add(1);
            Some(operand)
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
    regs.flags
        .set(Flags::OVERFLOW, is_signed_overflow(operand, regs.a, sum));
    regs.flags.set(Flags::NEGATIVE, is_negative(sum));

    regs.a = sum;
}

fn and(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let operand = bus.read_u8(address);
    regs.a &= operand;
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn asl_a(regs: &mut Regs) {
    regs.flags.set(Flags::CARRY, (regs.a >> 7) == 1);

    regs.a = regs.a << 1;
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn asl(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let m = bus.read_u8(address);
    regs.flags.set(Flags::CARRY, (m >> 7) == 1);

    let m = m << 1;
    regs.flags.set(Flags::ZERO, is_zero(m));
    regs.flags.set(Flags::NEGATIVE, is_negative(m));
    bus.write_u8(address, m);
}

fn bcc(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    if !regs.flags.contains(Flags::CARRY) {
        let offset = bus.read_u8(address) as i8;
        regs.pc = regs.pc.wrapping_add_signed(offset.into());
    }
}

fn bcs(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    if regs.flags.contains(Flags::CARRY) {
        let offset = bus.read_u8(address) as i8;
        regs.pc = regs.pc.wrapping_add_signed(offset.into());
    }
}

fn beq(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    if regs.flags.contains(Flags::ZERO) {
        let offset = bus.read_u8(address) as i8;
        regs.pc = regs.pc.wrapping_add_signed(offset.into());
    }
}

fn bit(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let m = bus.read_u8(address);
    let result = m & regs.a;
    regs.flags.set(Flags::ZERO, result == 0);
    regs.flags.set(Flags::OVERFLOW, result & (1 << 6) != 0);
    regs.flags.set(Flags::NEGATIVE, result & (1 << 7) != 0);
}

fn bne(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    if !regs.flags.contains(Flags::ZERO) {
        let offset = bus.read_u8(address) as i8;
        regs.pc = regs.pc.wrapping_add_signed(offset.into());
    }
}

fn bpl(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    if !regs.flags.contains(Flags::NEGATIVE) {
        let offset = bus.read_u8(address) as i8;
        regs.pc = regs.pc.wrapping_add_signed(offset.into());
    }
}

fn clc(regs: &mut Regs) {
    regs.flags.remove(Flags::CARRY);
}

fn cld(regs: &mut Regs) {
    regs.flags.remove(Flags::DECIMAL);
}

fn cli(regs: &mut Regs) {
    regs.flags.remove(Flags::INTERRUPT_DISABLE);
}

fn clv(regs: &mut Regs) {
    regs.flags.remove(Flags::OVERFLOW);
}

fn cmp(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let m = bus.read_u8(address);
    regs.flags.set(Flags::CARRY, regs.a >= m);
    regs.flags.set(Flags::ZERO, regs.a == m);
    regs.flags
        .set(Flags::NEGATIVE, is_negative(regs.a.wrapping_sub(m)));
}

fn cpx(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let m = bus.read_u8(address);
    regs.flags.set(Flags::CARRY, regs.x >= m);
    regs.flags.set(Flags::ZERO, regs.x == m);
    regs.flags
        .set(Flags::NEGATIVE, is_negative(regs.x.wrapping_sub(m)));
}

fn cpy(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let m = bus.read_u8(address);
    regs.flags.set(Flags::CARRY, regs.y >= m);
    regs.flags.set(Flags::ZERO, regs.y == m);
    regs.flags
        .set(Flags::NEGATIVE, is_negative(regs.y.wrapping_sub(m)));
}

fn dec(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let m = bus.read_u8(address);
    let m = m.wrapping_sub(1);
    regs.flags.set(Flags::ZERO, is_zero(m));
    regs.flags.set(Flags::NEGATIVE, is_negative(m));
    bus.write_u8(address, m);
}

fn dex(regs: &mut Regs) {
    regs.x = regs.x.wrapping_sub(1);
    regs.flags.set(Flags::ZERO, is_zero(regs.x));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.x));
}

fn dey(regs: &mut Regs) {
    regs.y = regs.y.wrapping_sub(1);
    regs.flags.set(Flags::ZERO, is_zero(regs.y));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.y));
}

fn inc(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let m = bus.read_u8(address);
    let m = m.wrapping_add(1);
    regs.flags.set(Flags::ZERO, is_zero(m));
    regs.flags.set(Flags::NEGATIVE, is_negative(m));
    bus.write_u8(address, m);
}

fn inx(regs: &mut Regs) {
    regs.x = regs.x.wrapping_add(1);
    regs.flags.set(Flags::ZERO, is_zero(regs.x));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.x));
}

fn iny(regs: &mut Regs) {
    regs.y = regs.y.wrapping_add(1);
    regs.flags.set(Flags::ZERO, is_zero(regs.y));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.y));
}

fn jmp(address: u16, regs: &mut Regs) {
    regs.pc = address
}

fn jsr(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    bus.write_u16_be(
        STACK_START.wrapping_add(regs.sp.wrapping_sub(1).into()),
        regs.pc.wrapping_sub(1),
    );
    regs.sp = regs.sp.wrapping_sub(2);
    regs.pc = address;
}

fn lda(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    regs.a = bus.read_u8(address);
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn ldx(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    regs.x = bus.read_u8(address);
    regs.flags.set(Flags::ZERO, is_zero(regs.x));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.x));
}

fn ldy(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    regs.y = bus.read_u8(address);
    regs.flags.set(Flags::ZERO, is_zero(regs.y));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.y));
}

fn lsr_a(regs: &mut Regs) {
    regs.flags.set(Flags::CARRY, regs.a & (1 << 0) != 0);

    regs.a = regs.a >> 1;
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn lsr(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let m = bus.read_u8(address);
    regs.flags.set(Flags::CARRY, m & (1 << 0) != 0);

    let m = m >> 1;
    regs.flags.set(Flags::ZERO, is_zero(m));
    regs.flags.set(Flags::NEGATIVE, is_negative(m));
    bus.write_u8(address, m);
}

fn nop() {}

fn rts(regs: &mut Regs, bus: &mut impl Bus) {
    regs.pc = bus
        .read_u16_be(STACK_START.wrapping_add(regs.sp.wrapping_add(1).into()))
        .wrapping_add(1);
    regs.sp = regs.sp.wrapping_add(2);
}

fn sbc(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let m = bus.read_u8(address);
    let c = !regs.flags.contains(Flags::CARRY) as u8;
    let result = regs.a.wrapping_sub(m).wrapping_sub(c);
    let is_overflow = is_signed_overflow(m, regs.a, result);
    regs.flags.set(Flags::CARRY, !is_overflow);
    regs.flags.set(Flags::ZERO, is_zero(result));
    regs.flags.set(Flags::OVERFLOW, is_overflow);
    regs.flags.set(Flags::NEGATIVE, is_negative(result));
    regs.a = result;
}

fn sec(regs: &mut Regs) {
    regs.flags.insert(Flags::CARRY)
}

fn sta(address: u16, regs: &Regs, bus: &mut impl Bus) {
    bus.write_u8(address, regs.a)
}

fn txa(regs: &mut Regs) {
    regs.a = regs.x;
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn is_zero(n: u8) -> bool {
    n == 0x00
}

fn is_negative(n: u8) -> bool {
    n & 0b1000_0000 != 0
}

fn is_signed_overflow(n: u8, m: u8, result: u8) -> bool {
    (n ^ result) & (m ^ result) & 0x80 != 0
}
