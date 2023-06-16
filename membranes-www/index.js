import init, * as membranes from "./pkg/membranes_wasm.js";

await init("./pkg/membranes_wasm_bg.wasm");

let nes = new membranes.Nes();
console.log(nes);