#![allow(dead_code, unused_variables, unused_imports)]
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator};

mod sdl;
use sdl::squares::squarestx;
use sdl::draw::{draw_statefull};
use sdl::layout::Layout;

mod game;
use game::pad::{Pad, Dir, Rot};
use game::state::{StateFull, StateAfter, state};
use game::game::step;
use game::rando::Rando;

mod four;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    const SQUARE_SIZE : u32 = 8;
    const SCREEN_SCALE : u32 = 4;
    let lo = Layout::new((20, 20), (140, 30), SQUARE_SIZE);

    let window = video_subsystem
        .window("rustris", 320 * SCREEN_SCALE, 200 * SCREEN_SCALE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build().unwrap();

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);

    match canvas.set_scale(SCREEN_SCALE as f32, SCREEN_SCALE as f32) {
        Result::Err(_) => panic!(),
        _ => {},
    };
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    canvas.clear();
    canvas.present();

    let texture_creator : TextureCreator<_> = canvas.texture_creator();
    let textures = squarestx(&mut canvas, &texture_creator, SQUARE_SIZE);

    let mut blocks = Rando::new(1234);

    let first = blocks.block;
    blocks = blocks.next();
    let second = blocks.block;
    blocks = blocks.next();

    let mut state : StateFull = state(first, second, 0);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut pad : Pad = Pad::new();

    'running: loop {

        pad = pad.next();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    pad = pad.dir_down(Dir::Left);
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    pad = pad.dir_up(Dir::Left);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    pad = pad.dir_down(Dir::Right);
                },
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    pad = pad.dir_up(Dir::Right);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    pad = pad.down_down();
                },
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    pad = pad.down_up();
                },
                Event::KeyDown { keycode: Some(Keycode::Z), repeat: false, .. } => {
                    pad = pad.rot_down(Rot::Left);
                },
                Event::KeyDown { keycode: Some(Keycode::X), repeat: false, .. } => {
                    pad = pad.rot_down(Rot::Right);
                },
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } => {},
                _ => {}
            }
        }

        state = match step(state, &pad) {
            StateAfter::Full(st) => st,
            StateAfter::Missing(st) => {
                let next = blocks.block;
                blocks = blocks.next();
                StateFull { state: st, next: next }
            },
            _ => panic!(),
        };

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        draw_statefull(&mut canvas, &textures, &state, &lo);

        canvas.present();
    }
}
