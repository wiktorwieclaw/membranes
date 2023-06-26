import init, * as membranes from "./pkg/membranes.js";
const wasm = await init("./pkg/membranes_bg.wasm");
let nes = new membranes.Nes();

const color = (byte) => {
    switch (byte) {
        case 0: 
            return "black";
        case 1:
            return "white"
        case 2:
        case 9:
            return "grey";
        default:
            return "cyan";
    }
}

const run = (rom) => {
    console.log(rom)
    nes.load(rom);

    const canvas = document.getElementById("screen");
    const ntiles = 32;
    const ctx = canvas.getContext("2d");
    ctx.scale(10, 10);

    let ram = new Uint8Array(wasm.memory.buffer, nes.ram(), 0x2000);

    const loop = () => {
        for(let i = 0; i < 128; ++i) {
            ram[0xFE] = randGenRange(1, 16);
            nes.next();
        }

        for (let i = 0; i < ntiles; i++) {
            for (let j = 0; j < ntiles; j++) {
                ctx.fillStyle = color(ram[0x0200 + i * ntiles + j]);
                ctx.fillRect(j, i, 1, 1);
            }
        }

        window.requestAnimationFrame(loop)
    }
    loop()
}

const randGenRange = (min, max) => {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min) + min);
}

const main = async () => {
    const fileSelector = document.getElementById('file-selector');

    document.addEventListener(
        'keydown',
        (e) => {
            let ram = new Uint8Array(wasm.memory.buffer, nes.ram(), 0x2000);
            switch (e.key) {
                case "w":
                    ram[0xff] = 0x77;
                    break;
                case "s":
                    ram[0xff] = 0x73;
                    break;
                case "a":
                    ram[0xff] = 0x61;
                    break;
                case "d":
                    ram[0xff] = 0x64;
                    break;
                case "r":
                    ram.fill(0x00);
                    nes.reset();
                    break;
                default:
                    break;
            }
        },
        false
    );

    // fileSelector.addEventListener('change', (event) => {
    //     const file = event.target.files[0];
    //     const reader = new FileReader();
    //     reader.addEventListener("load", () => {;
    //         const bytes = new Uint8Array(reader.result);
    //         run(bytes)
    //     });
    //     reader.readAsArrayBuffer(file);
    // });

    fetch("./snake.nes")
        .then((res) => res.arrayBuffer())
        .then((buf) => {
            const bytes = new Uint8Array(buf);
            run(bytes)
        })
}

await main();