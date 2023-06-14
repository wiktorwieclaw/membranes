pub struct Rom {
    pub prg_rom: [u8; 0x7FFF],
}

#[derive(Debug)]
pub struct ParseError;

pub fn parse_ines(bytes: &[u8]) -> Result<Rom, ParseError> {
    let mut rom = Rom {
        prg_rom: [0x00; 0x7FFF],
    };

    let nes_tag = bytes.get(..4).ok_or(ParseError)?;
    if nes_tag != &[b'N', b'E', b'S', 0x1A] {
        return Err(ParseError);
    }
    let len_prg_rom = bytes.get(4).ok_or(ParseError)?;
    let len_chr_rom = bytes.get(5).ok_or(ParseError)?;
    let flags6 = bytes.get(6).ok_or(ParseError)?;
    let flags7 = bytes.get(7).ok_or(ParseError)?;
    let len_prg_ram = bytes.get(8).ok_or(ParseError)?;
    let flags9 = bytes.get(9).ok_or(ParseError)?;
    let flags10 = bytes.get(10).ok_or(ParseError)?;
    let reserved = bytes.get(11..16).ok_or(ParseError)?;

    let prg_rom_pages = usize::from(*len_prg_rom);
    let prg_page_len = 16384;
    let prg_rom_len = prg_rom_pages * prg_page_len;

    let skip_trainer = bytes[6] & 0b100 != 0;

    let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
    rom.prg_rom
        .copy_from_slice(&bytes[prg_rom_start..][..(prg_rom_len - 1)]);
    Ok(rom)
}
