use membranes_cpu::Cpu;
use membranes_gamepad::Gamepad;

pub struct Nes {
    pub cpu: Cpu,
    pub bus: Bus,
}

impl Default for Nes {
    fn default() -> Self {
        Self {
            cpu: Default::default(),
            bus: Bus {
                ram: [0x00; 0x2000],
                prg_rom: [0x00; 0x7FFF],
                gamepad_1: Default::default(),
                gamepad_2: Default::default(),
            },
        }
    }
}

impl Nes {
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns Err if the program is too long to fit into prg_rom.
    pub fn load(&mut self, program: &[u8]) -> Result<(), ()> {
        let rom = self.bus.prg_rom.get_mut(..program.len()).ok_or(())?;
        rom.copy_from_slice(&program[..]);
        Ok(())
    }

    pub fn next(&mut self) {
        self.cpu.next(&mut self.bus);
    }
}

pub struct Bus {
    pub ram: [u8; 0x2000],
    pub prg_rom: [u8; 0x7FFF],
    pub gamepad_1: Gamepad,
    pub gamepad_2: Gamepad,
}

impl membranes_cpu::Bus for Bus {
    fn read_u8(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => {
                let address = address & 0b00000111_11111111;
                self.ram[usize::from(address)]
            }
            0x2000..=0x3FFF => {
                let _address = address & 0b00100000_00000111;
                // todo: remap address
                todo!("PPU")
            }
            0x4016 => self.gamepad_1.read_u8(),
            0x4017 => self.gamepad_2.read_u8(),
            0x8000..=0xFFFF => {
                let address = usize::from(address - 0x8000);
                self.prg_rom[address]
            }
            _ => todo!(),
        }
    }

    fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => {
                let address = usize::from(address & 0b00000111_11111111);
                self.ram[address] = data;
            }
            0x2000..=0x3FFF => {
                let _address = address & 0b00100000_00000111;
                // todo: remap address
                todo!("PPU")
            }
            0x4016 => self.gamepad_1.write_u8(data),
            0x4017 => self.gamepad_2.write_u8(data),
            0x8000..=0xFFFF => panic!("Cannot write to ROM"),
            _ => todo!(),
        }
    }
}
