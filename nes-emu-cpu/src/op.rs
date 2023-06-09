#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Op {
    mnemonic: Mnemonic,
    mode: Mode,
    cycles: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
    /// Force Interrupt
    Brk,
    /// Load Accumulator
    Lda,
    /// Store Accumulator
    Sta,
}

/// Addressing Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Implicit,
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
            0x00 => (Mnemonic::Brk, Mode::Implicit, 7),
            0xA9 => (Mnemonic::Lda, Mode::Immediate, 2),
            0xA5 => (Mnemonic::Lda, Mode::ZeroPage, 3),
            0xB5 => (Mnemonic::Lda, Mode::ZeroPageX, 4),
            0xAD => (Mnemonic::Lda, Mode::Absolute, 4),
            0xBD => (Mnemonic::Lda, Mode::AbsoluteX, 4),
            0xB9 => (Mnemonic::Lda, Mode::AbsoluteY, 4),
            0xA1 => (Mnemonic::Lda, Mode::IndirectX, 6),
            0xB1 => (Mnemonic::Lda, Mode::IndirectY, 5),
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
