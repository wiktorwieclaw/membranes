use std::num::Wrapping;

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct ButtonFlags: u8 {
        const RIGHT = 0b10000000;
        const LEFT = 0b01000000;
        const DOWN = 0b00100000;
        const UP = 0b00010000;
        const START = 0b00001000;
        const SELECT = 0b00000100;
        const B = 0b00000010;
        const A = 0b00000001;
    }
}

#[derive(Default)]
pub struct Joypad {
    is_strobe: bool,
    button_status_flags: ButtonFlags,
}

impl Joypad {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn press(&mut self, button_flags: ButtonFlags) {
        self.button_status_flags.insert(button_flags)
    }

    pub fn release(&mut self, button_flags: ButtonFlags) {
        self.button_status_flags.remove(button_flags)
    }

    pub fn read_u8(&mut self) -> Wrapping<u8> {
        // TODO
        Wrapping(0)
    }

    pub fn write_u8(&mut self, data: Wrapping<u8>) {
        // TODO
    }
}
