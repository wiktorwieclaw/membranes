use wasm_bindgen::prelude::*;

pub const PRG_ROM_PAGE_LEN: usize = 16384;

pub struct INesV1<'a> {
    bytes: &'a [u8],
}

impl INesV1<'_> {
    pub fn parse(bytes: &[u8]) -> Result<INesV1, ParseError> {
        if bytes.len() < 16 {
            return Err(ParseError::Header);
        }

        if &bytes[..4] != b"NES\x1A" {
            return Err(ParseError::Header);
        }

        // TODO: "assert length"

        Ok(INesV1 { bytes })
    }

    pub fn prg_rom_npages(&self) -> u8 {
        self.bytes[4]
    }

    pub fn has_trainer(&self) -> bool {
        self.bytes[6] & 0b100 != 0
    }

    pub fn prg_rom(&self) -> &[u8] {
        let npages: usize = self.prg_rom_npages().into();
        let start = 16 + if self.has_trainer() { 512 } else { 0 };
        let len = npages * PRG_ROM_PAGE_LEN;
        &self.bytes[start..(start + len - 1)]
    }
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum ParseError {
    Header,
}
