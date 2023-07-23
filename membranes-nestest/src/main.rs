use std::error::Error;

const NESTEST_ROM: &[u8] = include_bytes!("nestest.nes");
const NESTEST_LOG: &str = include_str!("nestest.log");

fn main() -> Result<(), Box<dyn Error>> {
    let mut nes = membranes::Nes::new();
    nes.load(NESTEST_ROM)?;
    nes.cpu.regs.pc = 0xC000;

    for _ in 0..3 {
        let membranes::cpu::Regs {
            a,
            x,
            y,
            pc,
            flags,
            sp,
        } = nes.cpu.regs;
        let flags = flags.bits();
        println!("{pc:04X}  A:{a:02X} X:{x:02X} Y:{y:02X} P:{flags:02X} SP:{sp:02X}");
        nes.next();
    }

    Ok(())
}
