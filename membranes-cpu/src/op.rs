#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Op {
    mnemonic: Mnemonic,
    mode: Mode,
    cycles: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
    /// Add with Carry
    Adc,
    /// Logical AND
    And,
    /// Arithmetic Shift Left
    Asl,
    /// Branch if Carry Clear
    Bcc,
    /// Branch if Carry Set
    Bcs,
    /// Branch if Equal
    Beq,
    /// Bit Test
    Bit,
    /// Branch if Minus
    Bmi,
    /// Branch if Not Equal
    Bne,
    /// Branch if Positive
    Bpl,
    /// Force Interrupt
    Brk,
    /// Branch if Overflow Clear
    Bvc,
    /// Branch if Overflow Set
    Bvs,
    /// Clear Carry Flag
    Clc,
    /// Clear Decimal Mode
    Cld,
    /// Clear Interrupt Disable
    Cli,
    /// Clear Overflow Flag
    Clv,
    /// Compare
    Cmp,
    /// Compare X Register
    Cpx,
    /// Compare Y Register
    Cpy,
    /// Decrement Memory
    Dec,
    /// Decrement X Register
    Dex,
    /// Decrement Y Register
    Dey,
    /// Exclusive OR
    Eor,
    /// Increment Memory
    Inc,
    /// Increment X Register
    Inx,
    /// Increment Y Register
    Iny,
    /// Jump
    Jmp,
    /// Jump to Subroutine
    Jsr,
    /// Load Accumulator
    Lda,
    /// Load X Register
    Ldx,
    /// Load Y Register
    Ldy,
    /// Logical Shift Right
    Lsr,
    /// No Operation
    Nop,
    /// Logical Inclusive OR
    Ora,
    /// Push Accumulator
    Pha,
    /// Push Processor Status
    Php,
    /// Pull Accumulator
    Pla,
    /// Pull Processor Status
    Plp,
    /// Rotate Left
    Rol,
    /// Rotate Right
    Ror,
    /// Return from Interrupt
    Rti,
    /// Return from Subroutine
    Rts,
    /// Subtract with Carry
    Sbc,
    /// Set Carry Flag
    Sec,
    /// Set Decimal Flag
    Sed,
    /// Set Interrupt Disable
    Sei,
    /// Store Accumulator
    Sta,
    /// Store X Register
    Stx,
    /// Store Y Register
    Sty,
    /// Transfer Accumulator to X
    Tax,
    /// Transfer Accumulator to Y
    Tay,
    /// Transfer Stack Pointer to X
    Tsx,
    /// Transfer X to Accumulator
    Txa,
    /// Transfer X to Stack Pointer
    Txs,
    /// Transfer Y to Accumulator
    Tya,
}

/// Addressing Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Implied,
    Relative,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

impl Op {
    pub fn parse(opcode: u8) -> Option<Self> {
        let (mnemonic, mode, cycles) = match opcode {
            0x69 => (Mnemonic::Adc, Mode::Immediate, 2),
            0x65 => (Mnemonic::Adc, Mode::ZeroPage, 3),
            0x75 => (Mnemonic::Adc, Mode::ZeroPageX, 4),
            0x6D => (Mnemonic::Adc, Mode::Absolute, 4),
            0x7D => (Mnemonic::Adc, Mode::AbsoluteX, 4),
            0x79 => (Mnemonic::Adc, Mode::AbsoluteY, 4),
            0x61 => (Mnemonic::Adc, Mode::IndirectX, 6),
            0x71 => (Mnemonic::Adc, Mode::IndirectY, 5),
            0x29 => (Mnemonic::And, Mode::Immediate, 2),
            0x25 => (Mnemonic::And, Mode::ZeroPage, 3),
            0x35 => (Mnemonic::And, Mode::ZeroPageX, 4),
            0x2D => (Mnemonic::And, Mode::Absolute, 4),
            0x3D => (Mnemonic::And, Mode::AbsoluteX, 4),
            0x39 => (Mnemonic::And, Mode::AbsoluteY, 4),
            0x21 => (Mnemonic::And, Mode::IndirectX, 6),
            0x31 => (Mnemonic::And, Mode::IndirectY, 5),
            0x0A => (Mnemonic::Asl, Mode::Accumulator, 2),
            0x06 => (Mnemonic::Asl, Mode::ZeroPage, 5),
            0x16 => (Mnemonic::Asl, Mode::ZeroPageX, 6),
            0x0E => (Mnemonic::Asl, Mode::Absolute, 6),
            0x1E => (Mnemonic::Asl, Mode::AbsoluteX, 7),
            0x90 => (Mnemonic::Bcc, Mode::Relative, 2),
            0xB0 => (Mnemonic::Bcs, Mode::Relative, 2),
            0xF0 => (Mnemonic::Beq, Mode::Relative, 2),
            0x24 => (Mnemonic::Bit, Mode::ZeroPage, 3),
            0x2C => (Mnemonic::Bit, Mode::Absolute, 4),
            0x30 => (Mnemonic::Bmi, Mode::Relative, 2),
            0xD0 => (Mnemonic::Bne, Mode::Relative, 2),
            0x10 => (Mnemonic::Bpl, Mode::Relative, 2),
            0x00 => (Mnemonic::Brk, Mode::Implied, 7),
            0x50 => (Mnemonic::Bvc, Mode::Relative, 2),
            0x70 => (Mnemonic::Bvs, Mode::Relative, 2),
            0x18 => (Mnemonic::Clc, Mode::Implied, 2),
            0xD8 => (Mnemonic::Cld, Mode::Implied, 2),
            0x58 => (Mnemonic::Cli, Mode::Implied, 2),
            0xB8 => (Mnemonic::Clv, Mode::Implied, 2),
            0xC9 => (Mnemonic::Cmp, Mode::Immediate, 2),
            0xC5 => (Mnemonic::Cmp, Mode::ZeroPage, 3),
            0xD5 => (Mnemonic::Cmp, Mode::ZeroPageX, 4),
            0xCD => (Mnemonic::Cmp, Mode::Absolute, 4),
            0xDD => (Mnemonic::Cmp, Mode::AbsoluteX, 4),
            0xD9 => (Mnemonic::Cmp, Mode::AbsoluteY, 4),
            0xC1 => (Mnemonic::Cmp, Mode::IndirectX, 6),
            0xD1 => (Mnemonic::Cmp, Mode::IndirectY, 5),
            0xE0 => (Mnemonic::Cpx, Mode::Immediate, 2),
            0xE4 => (Mnemonic::Cpx, Mode::ZeroPage, 3),
            0xEC => (Mnemonic::Cpx, Mode::Absolute, 4),
            0xC0 => (Mnemonic::Cpy, Mode::Immediate, 2),
            0xC4 => (Mnemonic::Cpy, Mode::ZeroPage, 3),
            0xCC => (Mnemonic::Cpy, Mode::Absolute, 4),
            0xC6 => (Mnemonic::Dec, Mode::ZeroPage, 5),
            0xD6 => (Mnemonic::Dec, Mode::ZeroPageX, 6),
            0xCE => (Mnemonic::Dec, Mode::Absolute, 6),
            0xDE => (Mnemonic::Dec, Mode::AbsoluteX, 7),
            0xCA => (Mnemonic::Dex, Mode::Implied, 2),
            0x88 => (Mnemonic::Dey, Mode::Implied, 2),
            0x49 => (Mnemonic::Eor, Mode::Immediate, 2),
            0x45 => (Mnemonic::Eor, Mode::ZeroPage, 3),
            0x55 => (Mnemonic::Eor, Mode::ZeroPageX, 4),
            0x4D => (Mnemonic::Eor, Mode::Absolute, 4),
            0x5D => (Mnemonic::Eor, Mode::AbsoluteX, 4),
            0x59 => (Mnemonic::Eor, Mode::AbsoluteY, 4),
            0x41 => (Mnemonic::Eor, Mode::IndirectX, 6),
            0x51 => (Mnemonic::Eor, Mode::IndirectY, 5),
            0xE6 => (Mnemonic::Inc, Mode::ZeroPage, 5),
            0xF6 => (Mnemonic::Inc, Mode::ZeroPage, 6),
            0xEE => (Mnemonic::Inc, Mode::Absolute, 6),
            0xFE => (Mnemonic::Inc, Mode::AbsoluteX, 7),
            0xE8 => (Mnemonic::Inx, Mode::Implied, 2),
            0xC8 => (Mnemonic::Iny, Mode::Implied, 2),
            0x4C => (Mnemonic::Jmp, Mode::Absolute, 3),
            0x6C => (Mnemonic::Jmp, Mode::Indirect, 5),
            0x20 => (Mnemonic::Jsr, Mode::Absolute, 6),
            0xA9 => (Mnemonic::Lda, Mode::Immediate, 2),
            0xA5 => (Mnemonic::Lda, Mode::ZeroPage, 3),
            0xB5 => (Mnemonic::Lda, Mode::ZeroPageX, 4),
            0xAD => (Mnemonic::Lda, Mode::Absolute, 4),
            0xBD => (Mnemonic::Lda, Mode::AbsoluteX, 4),
            0xB9 => (Mnemonic::Lda, Mode::AbsoluteY, 4),
            0xA1 => (Mnemonic::Lda, Mode::IndirectX, 6),
            0xB1 => (Mnemonic::Lda, Mode::IndirectY, 5),
            0xA2 => (Mnemonic::Ldx, Mode::Immediate, 2),
            0xA6 => (Mnemonic::Ldx, Mode::ZeroPage, 3),
            0xB6 => (Mnemonic::Ldx, Mode::ZeroPageY, 4),
            0xAE => (Mnemonic::Ldx, Mode::Absolute, 4),
            0xBE => (Mnemonic::Ldx, Mode::AbsoluteY, 4),
            0xA0 => (Mnemonic::Ldy, Mode::Immediate, 2),
            0xA4 => (Mnemonic::Ldy, Mode::ZeroPage, 3),
            0xB4 => (Mnemonic::Ldy, Mode::ZeroPageX, 4),
            0xAC => (Mnemonic::Ldy, Mode::Absolute, 4),
            0xBC => (Mnemonic::Ldy, Mode::AbsoluteX, 4),
            0x4A => (Mnemonic::Lsr, Mode::Accumulator, 2),
            0x46 => (Mnemonic::Lsr, Mode::ZeroPage, 5),
            0x56 => (Mnemonic::Lsr, Mode::ZeroPageX, 6),
            0x4E => (Mnemonic::Lsr, Mode::Absolute, 6),
            0x5E => (Mnemonic::Lsr, Mode::AbsoluteX, 7),
            0xEA => (Mnemonic::Nop, Mode::Implied, 2),
            0x09 => (Mnemonic::Ora, Mode::Immediate, 2),
            0x05 => (Mnemonic::Ora, Mode::ZeroPage, 3),
            0x15 => (Mnemonic::Ora, Mode::ZeroPageX, 4),
            0x0D => (Mnemonic::Ora, Mode::Absolute, 4),
            0x1D => (Mnemonic::Ora, Mode::AbsoluteX, 4),
            0x19 => (Mnemonic::Ora, Mode::AbsoluteY, 4),
            0x01 => (Mnemonic::Ora, Mode::IndirectX, 6),
            0x11 => (Mnemonic::Ora, Mode::IndirectY, 5),
            0x48 => (Mnemonic::Pha, Mode::Implied, 3),
            0x08 => (Mnemonic::Php, Mode::Implied, 3),
            0x68 => (Mnemonic::Pla, Mode::Implied, 4),
            0x28 => (Mnemonic::Plp, Mode::Implied, 4),
            0x2A => (Mnemonic::Rol, Mode::Accumulator, 2),
            0x26 => (Mnemonic::Rol, Mode::ZeroPage, 5),
            0x36 => (Mnemonic::Rol, Mode::ZeroPageX, 6),
            0x2E => (Mnemonic::Rol, Mode::Absolute, 6),
            0x3E => (Mnemonic::Rol, Mode::AbsoluteX, 7),
            0x6A => (Mnemonic::Ror, Mode::Accumulator, 2),
            0x66 => (Mnemonic::Ror, Mode::ZeroPage, 5),
            0x76 => (Mnemonic::Ror, Mode::ZeroPageX, 6),
            0x6E => (Mnemonic::Ror, Mode::Absolute, 6),
            0x7E => (Mnemonic::Ror, Mode::AbsoluteX, 7),
            0x40 => (Mnemonic::Rti, Mode::Implied, 6),
            0x60 => (Mnemonic::Rts, Mode::Implied, 6),
            0xE9 => (Mnemonic::Sbc, Mode::Immediate, 2),
            0xE5 => (Mnemonic::Sbc, Mode::ZeroPage, 3),
            0xF5 => (Mnemonic::Sbc, Mode::ZeroPageX, 4),
            0xED => (Mnemonic::Sbc, Mode::Absolute, 4),
            0xFD => (Mnemonic::Sbc, Mode::AbsoluteX, 4),
            0xF9 => (Mnemonic::Sbc, Mode::AbsoluteY, 4),
            0xE1 => (Mnemonic::Sbc, Mode::IndirectX, 6),
            0xF1 => (Mnemonic::Sbc, Mode::IndirectY, 5),
            0x38 => (Mnemonic::Sec, Mode::Implied, 2),
            0xF8 => (Mnemonic::Sec, Mode::Implied, 2),
            0x78 => (Mnemonic::Sec, Mode::Implied, 2),
            0x85 => (Mnemonic::Sta, Mode::ZeroPage, 3),
            0x95 => (Mnemonic::Sta, Mode::ZeroPageX, 4),
            0x8D => (Mnemonic::Sta, Mode::Absolute, 4),
            0x9D => (Mnemonic::Sta, Mode::AbsoluteX, 5),
            0x99 => (Mnemonic::Sta, Mode::AbsoluteY, 5),
            0x81 => (Mnemonic::Sta, Mode::IndirectX, 6),
            0x91 => (Mnemonic::Sta, Mode::IndirectY, 6),
            0x86 => (Mnemonic::Stx, Mode::ZeroPage, 3),
            0x96 => (Mnemonic::Stx, Mode::ZeroPageY, 4),
            0x8E => (Mnemonic::Stx, Mode::Absolute, 4),
            0x84 => (Mnemonic::Sty, Mode::ZeroPage, 3),
            0x94 => (Mnemonic::Sty, Mode::ZeroPageX, 4),
            0x8C => (Mnemonic::Sty, Mode::Absolute, 4),
            0xAA => (Mnemonic::Tax, Mode::Implied, 2),
            0xA8 => (Mnemonic::Tay, Mode::Implied, 2),
            0xBA => (Mnemonic::Tsx, Mode::Implied, 2),
            0x8A => (Mnemonic::Txa, Mode::Implied, 2),
            0x9A => (Mnemonic::Txs, Mode::Implied, 2),
            0x98 => (Mnemonic::Tya, Mode::Implied, 2),
            _ => return None,
        };
        Some(Self {
            mnemonic,
            mode,
            cycles,
        })
    }

    pub fn mnemonic(&self) -> Mnemonic {
        self.mnemonic
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }
}
