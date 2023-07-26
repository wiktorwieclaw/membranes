use membranes_cpu::Cpu;
use membranes_gamepad::Gamepad;
use wasm_bindgen::prelude::*;

pub use membranes_cpu as cpu;
pub use membranes_gamepad as gamepad;
pub use membranes_rom as rom;

#[wasm_bindgen]
pub struct Nes {
    pub cpu: Cpu,
    #[wasm_bindgen(getter_with_clone)]
    pub bus: Bus,
}

impl Default for Nes {
    fn default() -> Self {
        Self {
            cpu: Cpu::new(),
            bus: Bus {
                ram: vec![0x00; 0x2000],
                prg_rom: vec![0x00; 0x7FFF],
                gamepad_1: Default::default(),
                gamepad_2: Default::default(),
            },
        }
    }
}

#[wasm_bindgen]
impl Nes {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns Err if the program is too long to fit into prg_rom.
    pub fn load(&mut self, rom: &[u8]) -> Result<(), String> {
        let ines = rom::INesV1::parse(rom).map_err(|e| format!("{:?}", e))?;
        let prg = ines.prg_rom();
        let rom = self.bus.prg_rom.get_mut(..prg.len()).unwrap();
        rom.copy_from_slice(prg);
        Ok(())
    }

    pub fn next(&mut self) -> cpu::Effects {
        self.cpu.next(&mut self.bus)
    }

    pub fn ram(&mut self) -> *const u8 {
        self.bus.ram.as_ptr()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Bus {
    #[wasm_bindgen(getter_with_clone)]
    pub ram: Vec<u8>,
    #[wasm_bindgen(getter_with_clone)]
    pub prg_rom: Vec<u8>,
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
                let address = (usize::from(address) - 0x8000) % 0x4000;
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
