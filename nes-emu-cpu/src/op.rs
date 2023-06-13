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
    Indirect,
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
            0x18 => (Mnemonic::Clc, Mode::Implicit, 2),
            0xD8 => (Mnemonic::Cld, Mode::Implicit, 2),
            0x58 => (Mnemonic::Cli, Mode::Implicit, 2),
            0xB8 => (Mnemonic::Clv, Mode::Implicit, 2),
            0xC9 => (Mnemonic::Cmp, Mode::Immediate, 2),
            0xC5 => (Mnemonic::Cmp, Mode::ZeroPage, 3),
            0xD5 => (Mnemonic::Cmp, Mode::ZeroPageX, 4),
            0xCD => (Mnemonic::Cmp, Mode::Absolute, 4),
            0xDD => (Mnemonic::Cmp, Mode::AbsoluteX, 4),
            0xD9 => (Mnemonic::Cmp, Mode::AbsoluteY, 4),
            0xC1 => (Mnemonic::Cmp, Mode::IndirectX, 6),
            0xD1 => (Mnemonic::Cmp, Mode::IndirectY, 5),
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
