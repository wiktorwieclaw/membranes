use std::num::Wrapping;

use nes_emu_bits::prelude::*;
use nes_emu_cpu::Cpu;
use nes_emu_joypad::Joypad;

const ROM_LEN: usize = 0xFFFF - 0x8000;

pub struct Nes {
    cpu: Cpu,
    bus: Bus,
}

impl Default for Nes {
    fn default() -> Self {
        Self {
            cpu: Default::default(),
            bus: Bus {
                rom: [Wrapping(0); ROM_LEN],
                joypad_1: Default::default(),
                joypad_2: Default::default(),
            },
        }
    }
}

impl Nes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn load(&mut self, program: &[Wu8; ROM_LEN]) {
        self.bus.rom.copy_from_slice(&program[..]);
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.next(&mut self.bus);
        }
    }
}

pub struct Bus {
    pub rom: [Wu8; ROM_LEN],
    pub joypad_1: Joypad,
    pub joypad_2: Joypad,
}

impl nes_emu_cpu::Bus for Bus {
    fn read_u8(&mut self, address: Wu16) -> Wu8 {
        match address.0 {
            0x4016 => self.joypad_1.read_u8(),
            0x4017 => self.joypad_2.read_u8(),
            0x8000..=0xFFFF => {
                let address = address.0 as usize;
                self.rom[address - 0x8000]
            }
            _ => todo!(),
        }
    }

    fn write_u8(&mut self, address: Wu16, data: Wu8) {
        match address.0 {
            0x4016 => self.joypad_1.write_u8(data),
            0x4017 => self.joypad_2.write_u8(data),
            0x8000..=0xFFFF => todo!(),
            _ => todo!(),
        }
    }
}
