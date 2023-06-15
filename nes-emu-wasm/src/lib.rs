use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Nes(nes_emu::Nes);

#[wasm_bindgen]
impl Nes {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Nes(nes_emu::Nes::new())
    }

    #[wasm_bindgen]
    pub fn load(&mut self, program: &[u8]) -> Result<(), String> {
        self.0
            .load(program)
            .map_err(|()| "Program is too long to fit into prg_rom".into())
    }

    #[wasm_bindgen]
    pub fn next(&mut self) {
        self.0.next()        
    }
}
