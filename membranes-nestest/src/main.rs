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
        println!("{log}");
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
        op, operand_info, ..
    } = effects;

    let mnemonic = op.mnemonic();
    let mode = op.mode();

    let hex_0 = bus.read_u8(pc);
    let hex_1 = bus.read_u8(pc.wrapping_add(1));
    let hex_2 = bus.read_u8(pc.wrapping_add(2));

    let (hex, argument) = match mode {
        op::Mode::Implied => (format!("{:02X}", hex_0), String::new()),
        op::Mode::Accumulator => (format!("{:02X}", hex_0), String::from("A")),
        op::Mode::Relative => {
            let operand_info = operand_info.unwrap();
            let offset = bus.read_u8(operand_info.address) as i8;
            let address = pc.wrapping_add(2).wrapping_add_signed(offset.into());
            (
                format!("{:02X} {:02X}", hex_0, hex_1),
                format!("${:02X}", address),
            )
        }
        op::Mode::Immediate => (
            format!("{:02X} {:02X}", hex_0, hex_1),
            format!("#${:02X}", hex_1),
        ),
        op::Mode::ZeroPage => {
            let operand_info = operand_info.unwrap();
            let hex = format!("{:02X} {:02X}", hex_0, hex_1);
            let argument = format!("${:02X} = {:02X}", hex_1, operand_info.value);
            (hex, argument)
        }
        op::Mode::ZeroPageX => {
            let operand_info = operand_info.unwrap();
            (
                format!("{:02X} {:02X}", hex_0, hex_1),
                format!(
                    "${:02X},X @ {:02X} = {:02X}",
                    operand_info.raw_address, operand_info.address, operand_info.value
                ),
            )
        }
        op::Mode::ZeroPageY => {
            let operand_info = operand_info.unwrap();
            (
                format!("{:02X} {:02X}", hex_0, hex_1),
                format!(
                    "${:02X},Y @ {:02X} = {:02X}",
                    operand_info.raw_address, operand_info.address, operand_info.value
                ),
            )
        }
        op::Mode::Absolute => {
            let operand_info = operand_info.unwrap();
            (
                format!("{:02X} {:02X} {:02X}", hex_0, hex_1, hex_2),
                if matches!(mnemonic, op::Mnemonic::Jmp | op::Mnemonic::Jsr) {
                    format!("${:02X}{:02X}", hex_2, hex_1)
                } else {
                    format!("${:02X}{:02X} = {:02X}", hex_2, hex_1, operand_info.value)
                },
            )
        }
        op::Mode::AbsoluteX => {
            let operand_info = operand_info.unwrap();
            (
                format!("{:02X} {:02X} {:02X}", hex_0, hex_1, hex_2),
                format!(
                    "${:02X}{:02X},X @ {:04X} = {:02X}",
                    hex_2, hex_1, operand_info.address, operand_info.value
                ),
            )
        }
        op::Mode::AbsoluteY => {
            let operand_info = operand_info.unwrap();
            (
                format!("{:02X} {:02X} {:02X}", hex_0, hex_1, hex_2),
                format!(
                    "${:02X}{:02X},Y @ {:04X} = {:02X}",
                    hex_2, hex_1, operand_info.address, operand_info.value
                ),
            )
        }
        op::Mode::Indirect => {
            let operand_info = operand_info.unwrap();
            (
                format!("{:02X} {:02X} {:02X}", hex_0, hex_1, hex_2),
                format!(
                    "(${:02X}{:02X}) = {:04X}",
                    hex_2, hex_1, operand_info.address
                ),
            )
        }
        op::Mode::IndirectX => {
            let operand_info = operand_info.unwrap();
            (
                format!("{:02X} {:02X}", hex_0, hex_1),
                format!(
                    "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                    hex_1,
                    hex_1.wrapping_add(x),
                    operand_info.address,
                    operand_info.value
                ),
            )
        }
        op::Mode::IndirectY => {
            let operand_info = operand_info.unwrap();
            (
                format!("{:02X} {:02X}", hex_0, hex_1),
                format!(
                    "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                    hex_1,
                    u16::from_le_bytes([
                        bus.read_u8(hex_1.into()),
                        bus.read_u8(hex_1.wrapping_add(1).into())
                    ]),
                    operand_info.address,
                    operand_info.value
                ),
            )
        }
    };
    let asm = format!("{mnemonic} {argument}");
    format!("{pc:04X}  {hex:9} {asm:31} A:{a:02X} X:{x:02X} Y:{y:02X} P:{flags:02X} SP:{sp:02X}")
}
