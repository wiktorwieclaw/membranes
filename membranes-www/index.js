import init, { Nes } from "./pkg/membranes_wasm.js";

await init("./pkg/membranes_wasm_bg.wasm");

let nes = new Nes();
console.log(nes);