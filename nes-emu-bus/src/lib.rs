use nes_emu_joypad::Joypad;
use std::num::Wrapping;

pub struct Bus {
    pub memory: [Wrapping<u8>; 0xFFFF],
    pub joypad_1: Option<Joypad>,
    pub joypad_2: Option<Joypad>,
}

impl nes_emu_cpu::Bus for Bus {
    fn read_u8(&mut self, address: Wrapping<u16>) -> Wrapping<u8> {
        match address.0 {
            0x4016 => {
                if let Some(j) = &mut self.joypad_1 {
                    j.read_u8()
                } else {
                    Wrapping(0x00)
                }
            }
            0x4017 => {
                if let Some(j) = &mut self.joypad_2 {
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
                if let Some(j) = &mut self.joypad_1 {
                    j.write_u8(data)
                }
            }
            0x4017 => {
                if let Some(j) = &mut self.joypad_2 {
                    j.write_u8(data)
                }
            }
            _ => self.memory.write_u8(address, data),
        }
    }
}
