use std::error::Error;

use membranes::{
    cpu::{op, Bus as _, Effects, Regs},
    Bus, Nes,
};

const NESTEST_ROM: &[u8] = include_bytes!("nestest.nes");
const NESTEST_LOG: &str = include_str!("nestest.log");

fn main() -> Result<(), Box<dyn Error>> {
    let mut nes = Nes::new();
    nes.load(NESTEST_ROM)?;
    nes.cpu.regs.pc = 0xC000;

    for (i, expected) in NESTEST_LOG.lines().enumerate() {
        let regs = nes.cpu.regs;
        let effects = nes.next_op();

        let log = format_log(regs, &mut nes.bus, effects);
        // FIXME: remove split when PPU is implemented
        let expected = expected.split_at(73).0;
        assert_eq!(log, expected, "line {}", i + 1);
    }

    Ok(())
}

fn format_log(regs: Regs, bus: &mut Bus, effects: Effects) -> String {
    let Regs {
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
        operand_address,
        operand,
        ..
    } = effects;

    let mnemonic = op.mnemonic();
    let mode = op.mode();

    let hex = [
        bus.read_u8(pc),
        bus.read_u8(pc.wrapping_add(1)),
        bus.read_u8(pc.wrapping_add(2)),
    ];

    let (hex, argument) = match mode {
        op::Mode::Implied => (format!("{:02X}", hex[0]), String::new()),
        op::Mode::Accumulator => (format!("{:02X}", hex[0]), String::from("A")),
        op::Mode::Relative => {
            let offset = bus.read_u8(operand_address.unwrap()) as i8;
            let address = pc.wrapping_add(2).wrapping_add_signed(offset.into());
            (
                format!("{:02X} {:02X}", hex[0], hex[1]),
                format!("${:02X}", address),
            )
        }
        op::Mode::Immediate => (
            format!("{:02X} {:02X}", hex[0], hex[1]),
            format!("#${:02X}", hex[1]),
        ),
        op::Mode::ZeroPage => (
            format!("{:02X} {:02X}", hex[0], hex[1]),
            format!("${:02X} = {:02X}", hex[1], operand.unwrap()),
        ),
        op::Mode::ZeroPageX => (
            format!("{:02X} {:02X}", hex[0], hex[1]),
            format!("${:02X},X", hex[1]),
        ),
        op::Mode::ZeroPageY => (
            format!("{:02X} {:02X}", hex[0], hex[1]),
            format!("${:02X},Y", hex[1]),
        ),
        op::Mode::Absolute => (
            format!("{:02X} {:02X} {:02X}", hex[0], hex[1], hex[2]),
            if matches!(mnemonic, op::Mnemonic::Jmp | op::Mnemonic::Jsr) {
                format!("${:02X}{:02X}", hex[2], hex[1])
            } else {
                format!("${:02X}{:02X} = {:02X}", hex[2], hex[1], operand.unwrap())
            },
        ),
        op::Mode::AbsoluteX => (
            format!("{:02X} {:02X} {:02X}", hex[0], hex[1], hex[2]),
            format!("${:02X}{:02X},X", hex[2], hex[1]),
        ),
        op::Mode::AbsoluteY => (
            format!("{:02X} {:02X} {:02X}", hex[0], hex[1], hex[2]),
            format!("${:02X}{:02X},Y", hex[2], hex[1]),
        ),
        op::Mode::Indirect => (
            format!("{:02X} {:02X} {:02X}", hex[0], hex[1], hex[2]),
            format!(
                "(${:02X}{:02X}) = {:04X}",
                hex[2],
                hex[1],
                bus.read_u16_le(u16::from_be_bytes([hex[2], hex[1]]))
            ),
        ),
        op::Mode::IndirectX => (
            format!("{:02X} {:02X}", hex[0], hex[1]),
            format!(
                "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                hex[1],
                hex[1].wrapping_add(x),
                operand_address.unwrap(),
                operand.unwrap()
            ),
        ),
        op::Mode::IndirectY => (format!("{:02X} {:02X}", hex[0], hex[1]), {
            let operand_address = operand_address.unwrap();
            format!(
                "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                hex[1],
                u16::from_le_bytes([
                    bus.read_u8(hex[1].into()),
                    bus.read_u8(hex[1].wrapping_add(1).into())
                ]),
                operand_address,
                operand.unwrap()
            )
        }),
    };
    let asm = format!("{mnemonic} {argument}");
    format!("{pc:04X}  {hex:9} {asm:31} A:{a:02X} X:{x:02X} Y:{y:02X} P:{flags:02X} SP:{sp:02X}")
}
