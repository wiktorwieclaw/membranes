use std::num::Wrapping;

use crate::{cpu, joypad::Joypad};

pub struct Bus<'a> {
    pub memory: &'a mut [Wrapping<u8>; 0xFFFF],
    pub joypad_1: &'a mut Option<Joypad>,
    pub joypad_2: &'a mut Option<Joypad>,
}

impl cpu::Bus for Bus<'_> {
    fn read_u8(&mut self, address: Wrapping<u16>) -> Wrapping<u8> {
        match address.0 {
            0x4016 => {
                if let Some(j) = self.joypad_1 {
                    j.read_u8()
                } else {
                    Wrapping(0x00)
                }
            }
            0x4017 => {
                if let Some(j) = self.joypad_2 {
                    j.read_u8()
                } else {
                    Wrapping(0x00)
                }
            }
            _ => self.memory.read_u8(address),
        }
    }

    fn write_u8(&mut self, address: Wrapping<u16>, data: Wrapping<u8>) {
        match address.0 {
            0x4016 => {
                if let Some(j) = self.joypad_1 {
                    j.write_u8(data)
                }
            }
            0x4017 => {
                if let Some(j) = self.joypad_2 {
                    j.write_u8(data)
                }
            }
            _ => self.memory.write_u8(address, data),
        }
    }
}
