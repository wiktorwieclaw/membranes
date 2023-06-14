use nes_emu_cpu::Cpu;
use nes_emu_gamepad::Gamepad;

pub struct Nes {
    cpu: Cpu,
    bus: Bus,
}

impl Default for Nes {
    fn default() -> Self {
        Self {
            cpu: Default::default(),
            bus: Bus {
                ram: [0x00; 0x2000],
                rom: [0x00; 0x7FFF],
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

    pub fn load(&mut self, program: &[u8; 0x7FFF]) {
        self.bus.rom.copy_from_slice(&program[..]);
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.next(&mut self.bus);
        }
    }
}

pub struct Bus {
    pub ram: [u8; 0x2000],
    pub rom: [u8; 0x7FFF],
    pub gamepad_1: Gamepad,
    pub gamepad_2: Gamepad,
}

impl nes_emu_cpu::Bus for Bus {
    fn read_u8(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => {
                let address = address & 0b00000111_11111111;
                self.rom[usize::from(address)]
            },
            0x2000..=0x3FFF => {
                let _address = address & 0b00100000_00000111;
                todo!("PPU")
            }
            0x4016 => self.gamepad_1.read_u8(),
            0x4017 => self.gamepad_2.read_u8(),
            0x8000..=0xFFFF => {
                let address = address as usize;
                self.rom[address - 0x8000]
            }
            _ => todo!(),
        }
    }

    fn write_u8(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x1FFF => {
                let address = address & 0b00000111_11111111;
                self.rom[usize::from(address)] = data;
            }
            0x2000..=0x3FFF => {
                let _address = address & 0b00100000_00000111;
                todo!("PPU")
            }
            0x4016 => self.gamepad_1.write_u8(data),
            0x4017 => self.gamepad_2.write_u8(data),
            0x8000..=0xFFFF => todo!(),
            _ => todo!(),
        }
    }
}
