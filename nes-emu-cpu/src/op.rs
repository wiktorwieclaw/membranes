use std::num::Wrapping;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Op {
    mnemonic: Mnemonic,
    mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
    Brk,
    Lda,
}

/// Addressing Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Implied,
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
    pub fn parse(opcode: Wrapping<u8>) -> Self {
        let (mnemonic, mode) = match opcode.0 {
            0x00 => (Mnemonic::Brk, Mode::Implied),
            0xA9 => (Mnemonic::Lda, Mode::Immediate),
            0xA5 => (Mnemonic::Lda, Mode::ZeroPage),
            0xB5 => (Mnemonic::Lda, Mode::ZeroPageX),
            0xAD => (Mnemonic::Lda, Mode::Absolute),
            0xBD => (Mnemonic::Lda, Mode::AbsoluteX),
            0xB9 => (Mnemonic::Lda, Mode::AbsoluteY),
            0xA1 => (Mnemonic::Lda, Mode::IndirectX),
            0xB1 => (Mnemonic::Lda, Mode::IndirectY),
            _ => panic!(),
        };
        Self { mnemonic, mode }
    }

    pub fn mnemonic(&self) -> Mnemonic {
        self.mnemonic
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }
}
