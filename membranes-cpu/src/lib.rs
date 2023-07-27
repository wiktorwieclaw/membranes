use bitflags::bitflags;
use op::Op;
use std::ops::IndexMut;
use wasm_bindgen::prelude::*;

pub mod op;

const STACK_START: u16 = 0x0100;

/// Registers
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[wasm_bindgen]
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
    #[wasm_bindgen]
    pub struct Flags: u8 {
        // Unsigned carry
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL = 0b0000_1000;
        const BREAK_1 = 0b0001_0000;
        const BREAK_2 = 0b0010_0000;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[wasm_bindgen]
pub struct Cpu {
    pub regs: Regs,
}

#[wasm_bindgen]
pub struct Effects {
    pub op: Op,
    pub operand_address: Option<u16>,
    pub operand: Option<u8>,
    pub cycles: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Regs {
                sp: 0xFD,
                flags: Flags::INTERRUPT_DISABLE | Flags::BREAK_2,
                ..Default::default()
            },
        }
    }

    pub fn from_regs(regs: Regs) -> Self {
        Self { regs }
    }

    pub fn regs(&self) -> Regs {
        self.regs
    }

    pub fn next(&mut self, bus: &mut impl Bus) -> Effects {
        let regs = &mut self.regs;

        let opcode = bus.read_u8(regs.pc);
        regs.pc = regs.pc.wrapping_add(1);

        let (op, cycles) =
            Op::parse(opcode).unwrap_or_else(|| panic!("Unsupported opcode: {opcode:x}"));

        let (mnemonic, mode) = (op.mnemonic(), op.mode());

        let operand_address = operand_address(mode, regs, bus);
        let operand = operand_address.map(|a| bus.read_u8(a));
        match (mnemonic, operand_address) {
            (op::Mnemonic::Adc, Some(address)) => adc(address, regs, bus),
            (op::Mnemonic::And, Some(address)) => and(address, regs, bus),
            (op::Mnemonic::Asl, Some(address)) => asl(address, regs, bus),
            (op::Mnemonic::Asl, None) => asl_a(regs),
            (op::Mnemonic::Bcc, Some(address)) => bcc(address, regs, bus),
            (op::Mnemonic::Bcs, Some(address)) => bcs(address, regs, bus),
            (op::Mnemonic::Beq, Some(address)) => beq(address, regs, bus),
            (op::Mnemonic::Bit, Some(address)) => bit(address, regs, bus),
            (op::Mnemonic::Bmi, Some(address)) => bmi(address, regs, bus),
            (op::Mnemonic::Bne, Some(address)) => bne(address, regs, bus),
            (op::Mnemonic::Bpl, Some(address)) => bpl(address, regs, bus),
            (op::Mnemonic::Brk, None) => brk(regs),
            (op::Mnemonic::Bvc, Some(address)) => bvc(address, regs, bus),
            (op::Mnemonic::Bvs, Some(address)) => bvs(address, regs, bus),
            (op::Mnemonic::Clc, None) => clc(regs),
            (op::Mnemonic::Cld, None) => cld(regs),
            (op::Mnemonic::Cli, None) => cli(regs),
            (op::Mnemonic::Clv, None) => clv(regs),
            (op::Mnemonic::Cmp, Some(address)) => cmp(address, regs, bus),
            (op::Mnemonic::Cpx, Some(address)) => cpx(address, regs, bus),
            (op::Mnemonic::Cpy, Some(address)) => cpy(address, regs, bus),
            (op::Mnemonic::Dec, Some(address)) => dec(address, regs, bus),
            (op::Mnemonic::Dex, None) => dex(regs),
            (op::Mnemonic::Dey, None) => dey(regs),
            (op::Mnemonic::Eor, Some(address)) => eor(address, regs, bus),
            (op::Mnemonic::Inc, Some(address)) => inc(address, regs, bus),
            (op::Mnemonic::Inx, None) => inx(regs),
            (op::Mnemonic::Iny, None) => iny(regs),
            (op::Mnemonic::Jmp, Some(address)) => jmp(address, regs),
            (op::Mnemonic::Jsr, Some(address)) => jsr(address, regs, bus),
            (op::Mnemonic::Nop, None) => nop(),
            (op::Mnemonic::Ora, Some(address)) => ora(address, regs, bus),
            (op::Mnemonic::Pha, None) => pha(regs, bus),
            (op::Mnemonic::Php, None) => php(regs, bus),
            (op::Mnemonic::Pla, None) => pla(regs, bus),
            (op::Mnemonic::Plp, None) => plp(regs, bus),
            (op::Mnemonic::Lda, Some(address)) => lda(address, regs, bus),
            (op::Mnemonic::Ldx, Some(address)) => ldx(address, regs, bus),
            (op::Mnemonic::Ldy, Some(address)) => ldy(address, regs, bus),
            (op::Mnemonic::Lsr, Some(address)) => lsr(address, regs, bus),
            (op::Mnemonic::Lsr, None) => lsr_a(regs),
            (op::Mnemonic::Rol, None) => rol_a(regs),
            (op::Mnemonic::Rol, Some(address)) => rol(address, regs, bus),
            (op::Mnemonic::Ror, None) => ror_a(regs),
            (op::Mnemonic::Ror, Some(address)) => ror(address, regs, bus),
            (op::Mnemonic::Rti, None) => rti(regs, bus),
            (op::Mnemonic::Rts, None) => rts(regs, bus),
            (op::Mnemonic::Sbc, Some(address)) => sbc(address, regs, bus),
            (op::Mnemonic::Sec, None) => sec(regs),
            (op::Mnemonic::Sed, None) => sed(regs),
            (op::Mnemonic::Sei, None) => sei(regs),
            (op::Mnemonic::Sta, Some(address)) => sta(address, regs, bus),
            (op::Mnemonic::Stx, Some(address)) => stx(address, regs, bus),
            (op::Mnemonic::Tax, None) => tax(regs),
            (op::Mnemonic::Tay, None) => tay(regs),
            (op::Mnemonic::Tsx, None) => tsx(regs),
            (op::Mnemonic::Txa, None) => txa(regs),
            (op::Mnemonic::Txs, None) => txs(regs),
            (op::Mnemonic::Tya, None) => tya(regs),
            _ => unreachable!("{op:?}"),
        };

        Effects {
            op,
            operand_address,
            operand,
            cycles,
        }
    }
}

fn operand_address(mode: op::Mode, regs: &mut Regs, bus: &mut impl Bus) -> Option<u16> {
    match mode {
        op::Mode::Implied | op::Mode::Accumulator => None,

        op::Mode::Immediate | op::Mode::Relative => {
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
            let raw = bus.read_u8(regs.pc);
            regs.pc = regs.pc.wrapping_add(1);
            let address = raw.wrapping_add(regs.x);
            Some(address.into())
        }

        op::Mode::ZeroPageY => {
            let raw = bus.read_u8(regs.pc);
            regs.pc = regs.pc.wrapping_add(1);
            let address = raw.wrapping_add(regs.y);
            Some(address.into())
        }

        op::Mode::Absolute => {
            let address = bus.read_u16_le(regs.pc);
            regs.pc = regs.pc.wrapping_add(2);
            Some(address)
        }

        op::Mode::AbsoluteX => {
            let raw = bus.read_u16_le(regs.pc);
            regs.pc = regs.pc.wrapping_add(2);
            let address = raw.wrapping_add(regs.x.into());
            Some(address)
        }

        op::Mode::AbsoluteY => {
            let raw = bus.read_u16_le(regs.pc);
            regs.pc = regs.pc.wrapping_add(2);
            let address = raw.wrapping_add(regs.y.into());
            Some(address)
        }

        op::Mode::Indirect => {
            let raw = bus.read_u16_le(regs.pc);
            regs.pc = regs.pc.wrapping_add(1);
            let address = bus.read_u16_le(raw);
            Some(address)
        }

        op::Mode::IndirectX => {
            let raw = bus.read_u8(regs.pc);
            regs.pc = regs.pc.wrapping_add(1);
            let address = raw.wrapping_add(regs.x).into();
            let address = bus.read_u16_le(address);
            Some(address)
        }

        op::Mode::IndirectY => {
            let raw = bus.read_u8(regs.pc);
            regs.pc = regs.pc.wrapping_add(1);
            let address = bus.read_u16_le(raw.into()).wrapping_add(regs.y.into());
            Some(address)
        }
    }
}

fn adc(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let a = regs.a;
    let m = bus.read_u8(address);
    let c = regs.flags.contains(Flags::CARRY) as u8;
    let (result, is_overflow1) = regs.a.overflowing_add(m);
    let (result, is_overflow2) = result.overflowing_add(c);

    let is_overflow = is_overflow1 | is_overflow2;
    regs.flags.set(Flags::CARRY, is_overflow);
    regs.flags.set(Flags::ZERO, is_zero(result));
    regs.flags.set(
        Flags::OVERFLOW,
        (a ^ m) & 0x80 == 0 && (a ^ result) & 0x80 != 0,
    );
    regs.flags.set(Flags::NEGATIVE, is_negative(result));
    regs.a = result;
}

fn and(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let operand = bus.read_u8(address);
    regs.a &= operand;
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn asl_a(regs: &mut Regs) {
    regs.flags.set(Flags::CARRY, (regs.a >> 7) == 1);

    regs.a <<= 1;
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
    regs.flags.set(Flags::OVERFLOW, m & (1 << 6) != 0);
    regs.flags.set(Flags::NEGATIVE, m & (1 << 7) != 0);
}

fn bmi(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    if regs.flags.contains(Flags::NEGATIVE) {
        let offset = bus.read_u8(address) as i8;
        regs.pc = regs.pc.wrapping_add_signed(offset.into());
    }
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

fn brk(regs: &mut Regs) {
    regs.flags.set(Flags::BREAK_1, true);
}

fn bvc(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    if !regs.flags.contains(Flags::OVERFLOW) {
        let offset = bus.read_u8(address) as i8;
        regs.pc = regs.pc.wrapping_add_signed(offset.into());
    }
}

fn bvs(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    if regs.flags.contains(Flags::OVERFLOW) {
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

fn eor(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let operand = bus.read_u8(address);
    regs.a ^= operand;
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
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
    bus.write_u16_le(
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

    regs.a >>= 1;
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

fn ora(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let operand = bus.read_u8(address);
    regs.a |= operand;
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn pha(regs: &mut Regs, bus: &mut impl Bus) {
    bus.write_u8(STACK_START.wrapping_add(regs.sp.into()), regs.a);
    regs.sp = regs.sp.wrapping_sub(1);
}

fn php(regs: &mut Regs, bus: &mut impl Bus) {
    let flags = regs.flags.union(Flags::BREAK_1 | Flags::BREAK_2);
    bus.write_u8(STACK_START.wrapping_add(regs.sp.into()), flags.bits());
    regs.sp = regs.sp.wrapping_sub(1);
}

fn pla(regs: &mut Regs, bus: &mut impl Bus) {
    regs.sp = regs.sp.wrapping_add(1);
    regs.a = bus.read_u8(STACK_START.wrapping_add(regs.sp.into()));
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn plp(regs: &mut Regs, bus: &mut impl Bus) {
    regs.sp = regs.sp.wrapping_add(1);
    regs.flags = Flags::from_bits_truncate(bus.read_u8(STACK_START.wrapping_add(regs.sp.into())));
    regs.flags.remove(Flags::BREAK_1);
    regs.flags.insert(Flags::BREAK_2);
}

fn rol_a(regs: &mut Regs) {
    let new_carry = regs.a & (1 << 7) != 0;
    regs.a = regs.a << 1 | regs.flags.contains(Flags::CARRY) as u8;
    regs.flags.set(Flags::CARRY, new_carry);
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn rol(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let m = bus.read_u8(address);
    let new_carry = m & (1 << 7) != 0;
    let result = m << 1 | regs.flags.contains(Flags::CARRY) as u8;
    regs.flags.set(Flags::CARRY, new_carry);
    regs.flags.set(Flags::ZERO, is_zero(m));
    regs.flags.set(Flags::NEGATIVE, is_negative(m));
    bus.write_u8(address, result);
}

fn ror_a(regs: &mut Regs) {
    let new_carry = regs.a & (1 << 0) != 0;
    regs.a = regs.a >> 1 | (regs.flags.contains(Flags::CARRY) as u8) << 7;
    regs.flags.set(Flags::CARRY, new_carry);
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn ror(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let m = bus.read_u8(address);
    let new_carry = m & (1 << 0) != 0;
    let result = m >> 1 | regs.flags.contains(Flags::CARRY) as u8;
    regs.flags.set(Flags::CARRY, new_carry);
    regs.flags.set(Flags::ZERO, is_zero(m));
    regs.flags.set(Flags::NEGATIVE, is_negative(m));
    bus.write_u8(address, result);
}

fn rti(regs: &mut Regs, bus: &mut impl Bus) {
    regs.sp = regs.sp.wrapping_add(1);
    regs.flags = Flags::from_bits_truncate(bus.read_u8(STACK_START.wrapping_add(regs.sp.into())));
    regs.flags.remove(Flags::BREAK_1);
    regs.flags.insert(Flags::BREAK_2);

    regs.pc = bus.read_u16_le(STACK_START.wrapping_add(regs.sp.wrapping_add(1).into()));
    regs.sp = regs.sp.wrapping_add(2);
}

fn rts(regs: &mut Regs, bus: &mut impl Bus) {
    regs.pc = bus
        .read_u16_le(STACK_START.wrapping_add(regs.sp.wrapping_add(1).into()))
        .wrapping_add(1);
    regs.sp = regs.sp.wrapping_add(2);
}

fn sbc(address: u16, regs: &mut Regs, bus: &mut impl Bus) {
    let a = regs.a;
    let m = bus.read_u8(address);
    let c = regs.flags.contains(Flags::CARRY) as u8;
    let (result, is_overflow1) = a.overflowing_sub(m);
    let (result, is_overflow2) = result.overflowing_sub(1 - c);
    let is_overflow = is_overflow1 | is_overflow2;
    regs.flags.set(Flags::CARRY, !is_overflow);
    regs.flags.set(Flags::ZERO, result == 0);
    regs.flags.set(
        Flags::OVERFLOW,
        (a ^ m) & 0x80 != 0 && (a ^ result) & 0x80 != 0,
    );
    regs.flags.set(Flags::NEGATIVE, is_negative(result));
    regs.a = result;
}

fn sec(regs: &mut Regs) {
    regs.flags.insert(Flags::CARRY)
}

fn sed(regs: &mut Regs) {
    regs.flags.insert(Flags::DECIMAL)
}

fn sei(regs: &mut Regs) {
    regs.flags.insert(Flags::INTERRUPT_DISABLE)
}

fn sta(address: u16, regs: &Regs, bus: &mut impl Bus) {
    bus.write_u8(address, regs.a)
}

fn stx(address: u16, regs: &Regs, bus: &mut impl Bus) {
    bus.write_u8(address, regs.x)
}

fn tax(regs: &mut Regs) {
    regs.x = regs.a;
    regs.flags.set(Flags::ZERO, is_zero(regs.x));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.x));
}

fn tay(regs: &mut Regs) {
    regs.y = regs.a;
    regs.flags.set(Flags::ZERO, is_zero(regs.y));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.y));
}

fn tsx(regs: &mut Regs) {
    regs.x = regs.sp;
    regs.flags.set(Flags::ZERO, is_zero(regs.x));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.x));
}

fn txa(regs: &mut Regs) {
    regs.a = regs.x;
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn txs(regs: &mut Regs) {
    regs.sp = regs.x;
}

fn tya(regs: &mut Regs) {
    regs.a = regs.y;
    regs.flags.set(Flags::ZERO, is_zero(regs.a));
    regs.flags.set(Flags::NEGATIVE, is_negative(regs.a));
}

fn is_zero(n: u8) -> bool {
    n == 0x00
}

fn is_negative(n: u8) -> bool {
    n & 0b1000_0000 != 0
}
