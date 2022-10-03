use std::{cell::RefCell, num::Wrapping};

use bus::Bus;
use cpu::Cpu;
use joypad::Joypad;

mod bus;
mod cpu;
mod joypad;
mod util;

pub struct Nes {
    cpu: Cpu,
    memory: [Wrapping<u8>; 0xFFFF],
    joypad_1: Option<Joypad>,
    joypad_2: Option<Joypad>,
}

impl Default for Nes {
    fn default() -> Self {
        Self {
            cpu: Default::default(),
            memory: [Wrapping(0); 0xFFFF],
            joypad_1: Default::default(),
            joypad_2: Default::default(),
        }
    }
}

impl Nes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.execute_next(&mut Bus {
                memory: &mut self.memory,
                joypad_1: &mut self.joypad_1,
                joypad_2: &mut self.joypad_2,
            });
        }
    }
}
