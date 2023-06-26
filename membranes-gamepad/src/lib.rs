use bitflags::bitflags;
use wasm_bindgen::prelude::*;

bitflags! {
    #[derive(Default)]
    #[wasm_bindgen]
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

#[derive(Default, Clone, Copy)]
#[wasm_bindgen]
pub struct Gamepad {
    _is_strobe: bool,
    button_status_flags: ButtonFlags,
}

impl Gamepad {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn press(&mut self, button_flags: ButtonFlags) {
        self.button_status_flags.insert(button_flags)
    }

    pub fn release(&mut self, button_flags: ButtonFlags) {
        self.button_status_flags.remove(button_flags)
    }

    pub fn read_u8(&mut self) -> u8 {
        // TODO
        0
    }

    pub fn write_u8(&mut self, _data: u8) {
        // TODO
    }
}
