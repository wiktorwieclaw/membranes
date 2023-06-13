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
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    /// Load Accumulator
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    /// Store Accumulator
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}

/// Addressing Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Implicit,
    Relative,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl Op {
    pub fn parse(opcode: u8) -> Self {
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
            0x00 => (Mnemonic::Brk, Mode::Implicit, 7),
            0x20 => (Mnemonic::Jsr, Mode::Absolute, 6),
            0xA9 => (Mnemonic::Lda, Mode::Immediate, 2),
            0xA5 => (Mnemonic::Lda, Mode::ZeroPage, 3),
            0xB5 => (Mnemonic::Lda, Mode::ZeroPageX, 4),
            0xAD => (Mnemonic::Lda, Mode::Absolute, 4),
            0xBD => (Mnemonic::Lda, Mode::AbsoluteX, 4),
            0xB9 => (Mnemonic::Lda, Mode::AbsoluteY, 4),
            0xA1 => (Mnemonic::Lda, Mode::IndirectX, 6),
            0xB1 => (Mnemonic::Lda, Mode::IndirectY, 5),
            0x60 => (Mnemonic::Rts, Mode::Implicit, 6),
            0x85 => (Mnemonic::Sta, Mode::ZeroPage, 3),
            0x95 => (Mnemonic::Sta, Mode::ZeroPageX, 4),
            0x8D => (Mnemonic::Sta, Mode::Absolute, 4),
            0x9D => (Mnemonic::Sta, Mode::AbsoluteX, 5),
            0x99 => (Mnemonic::Sta, Mode::AbsoluteY, 5),
            0x81 => (Mnemonic::Sta, Mode::IndirectX, 6),
            0x91 => (Mnemonic::Sta, Mode::IndirectY, 6),
            _ => panic!(),
        };
        Self {
            mnemonic,
            mode,
            cycles,
        }
    }

    pub fn mnemonic(&self) -> Mnemonic {
        self.mnemonic
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }
}
