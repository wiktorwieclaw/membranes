use std::num::Wrapping;

use nes_emu_bus::Bus;
use nes_emu_cpu::Cpu;

pub struct Nes {
    cpu: Cpu,
    bus: Bus,
}

impl Default for Nes {
    fn default() -> Self {
        Self {
            cpu: Default::default(),
            bus: Bus {
                memory: [Wrapping(0); 0xFFFF],
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

    pub fn run(&mut self) {
        loop {
            self.cpu.execute_next(&mut self.bus);
        }
    }
}
