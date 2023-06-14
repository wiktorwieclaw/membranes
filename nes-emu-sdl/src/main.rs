//! parts of the code taken from https://bugzmanov.github.io/nes_ebook/chapter_3_4.html

use nes_emu_cpu::{Bus, SideEffect};
use rand::Rng;
use sdl2::{event::Event, keyboard::Keycode};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake game", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(sdl2::pixels::PixelFormatEnum::RGB24, 32, 32)
        .unwrap();

    let mut screen_state = [0_u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();

    let rom = std::fs::read("freeware/snake.nes").unwrap();
    let rom = nes_emu_rom::parse_ines(&rom).unwrap();
    let mut nes = nes_emu::Nes::default();
    nes.load(&rom.prg_rom);
    nes.cpu.regs.pc = 0x8600;
    nes.cpu.regs.sp = 0xFF;

    loop {
        handle_user_input(&mut nes.bus, &mut event_pump);
        nes.bus.write_u8(0xFE, rng.gen_range(1, 16));

        if read_screen_state(&mut nes.bus, &mut screen_state) {
            texture.update(None, &screen_state, 32 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        let (_op, effect) = nes.cpu.next(&mut nes.bus);
        if let Some(SideEffect::Break) = effect {
            break;
        }
    }
}

fn handle_user_input(mem: &mut impl Bus, event_pump: &mut sdl2::EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                mem.write_u8(0xff, 0x77);
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                mem.write_u8(0xff, 0x73);
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                mem.write_u8(0xff, 0x61);
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                mem.write_u8(0xff, 0x64);
            }
            _ => { /* do nothing */ }
        }
    }
}

fn color(byte: u8) -> sdl2::pixels::Color {
    match byte {
        0 => sdl2::pixels::Color::BLACK,
        1 => sdl2::pixels::Color::WHITE,
        2 | 9 => sdl2::pixels::Color::GREY,
        3 | 10 => sdl2::pixels::Color::RED,
        4 | 11 => sdl2::pixels::Color::GREEN,
        5 | 12 => sdl2::pixels::Color::BLUE,
        6 | 13 => sdl2::pixels::Color::MAGENTA,
        7 | 14 => sdl2::pixels::Color::YELLOW,
        _ => sdl2::pixels::Color::CYAN,
    }
}

fn read_screen_state(mem: &mut impl Bus, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x600 {
        let color_idx = mem.read_u8(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}
