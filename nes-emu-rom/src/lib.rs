pub struct Header;

pub struct Rom {
    pub prg_rom: [u8; 0x7FFF],
}

pub fn parse(bytes: &[u8]) -> Rom {
    let mut rom = Rom {
        prg_rom: [0x00; 0x7FFF],
    };

    // todo: use iter instead of indexing

    let prg_rom_pages = usize::from(bytes[4]);
    let prg_page_size = 16384;
    let prg_rom_size = prg_rom_pages * prg_page_size;

    let skip_trainer = bytes[6] & 0b100 != 0;

    let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
    rom.prg_rom
        .copy_from_slice(&bytes[prg_rom_start..][..(prg_rom_size - 1)]);
    rom
}
