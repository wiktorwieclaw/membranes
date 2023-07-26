use std::error::Error;

use membranes::cpu::{op, Bus, Effects, Regs};

const NESTEST_ROM: &[u8] = include_bytes!("nestest.nes");
const NESTEST_LOG: &str = include_str!("nestest.log");

fn main() -> Result<(), Box<dyn Error>> {
    let mut nes = membranes::Nes::new();
    nes.load(NESTEST_ROM)?;
    nes.cpu.regs.pc = 0xC000;

    for (i, expected) in NESTEST_LOG.lines().enumerate() {
        let regs = nes.cpu.regs;
        let effects = nes.next();

        let log = format_log(regs, &mut nes.bus, effects);

        // FIXME:, remove split when PPU is implemented
        let expected = expected.split_at(73).0;
        assert_eq!(log, expected, "line {i}");
    }

    Ok(())
}

fn format_log(regs: Regs, bus: &mut membranes::Bus, effects: Effects) -> String {
    let membranes::cpu::Regs {
        a,
        x,
        y,
        flags,
        sp,
        pc,
    } = regs;
    let flags = flags.bits();

    let Effects {
        op,
        opcode,
        address,
        ..
    } = effects;

    let mut hex = std::iter::once(&opcode)
        .chain(&address.raw)
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<String>>()
        .join(" ");

    let mnemonic = op.mnemonic().to_string();
    let mode = op.mode();
    let argument = match mode {
        op::Mode::Implied | op::Mode::Accumulator => String::new(),
        op::Mode::Relative => format!("${:02X}", address.raw[0]),
        op::Mode::Immediate => {
            let operand = bus.read_u8(address.effective.unwrap());
            hex += &format!(" {operand:02X}");
            format!("#${:02X}", operand)
        }
        op::Mode::ZeroPage => format!("${:02X}", address.raw[0]),
        op::Mode::ZeroPageX => format!("${:02X},X", address.raw[0]),
        op::Mode::ZeroPageY => format!("${:02X},Y", address.raw[0]),
        op::Mode::Absolute => format!("${:02X}{:02X}", address.raw[1], address.raw[0]),
        op::Mode::AbsoluteX => format!("${:02X}{:02X},X", address.raw[1], address.raw[0]),
        op::Mode::AbsoluteY => format!("${:02X}{:02X},Y", address.raw[1], address.raw[0]),
        op::Mode::Indirect => format!("(${:02X}{:02X})", address.raw[1], address.raw[0]),
        op::Mode::IndirectX => format!("(${:02X},X)", address.raw[0]),
        op::Mode::IndirectY => format!("(${:02X}),Y", address.raw[0]),
    };
    let asm = format!("{mnemonic} {argument}");

    format!("{pc:04X}  {hex:9} {asm:31} A:{a:02X} X:{x:02X} Y:{y:02X} P:{flags:02X} SP:{sp:02X}")
}
