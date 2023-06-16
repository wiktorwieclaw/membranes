use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Nes(membranes::Nes);

#[wasm_bindgen]
impl Nes {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Nes(membranes::Nes::new())
    }

    pub fn load(&mut self, program: &[u8]) -> Result<(), String> {
        self.0
            .load(program)
            .map_err(|()| "Program is too long to fit into prg_rom".into())
    }

    pub fn next(&mut self) {
        self.0.next()        
    }
}
