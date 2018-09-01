use sdl::layout::Layout;

use game::field::{Field};
use game::square::{Squares};
use sdl2::pixels::Color;
use game::block::{BlockRot, block};
use game::pos::Pos;
use game::state::{StateFull, State, Mode};
use game::lineclear::LineClear;
use four::A;

use sdl2::video::{Window};
use sdl2::render::{Canvas, Texture};
use sdl2::rect::{Rect};


pub fn draw_statefull<'a>(canvas : &mut Canvas<Window>, txs : &Squares<Texture<'a>>, st : &StateFull, l : &Layout) {
    draw_state(canvas, txs, &st.state, l);
    draw_block(canvas, l.square_size, txs, &block(st.next, A), &Pos::new(0, 0), l.next);
}

pub fn draw_state<'a>(canvas : &mut Canvas<Window>, txs : &Squares<Texture<'a>>, st : &State, l : &Layout) {
    draw_field(canvas, l.square_size, txs, &st.field, l.field);
    match &st.mode {
        Mode::Block(bs) => draw_block(canvas, l.square_size, txs, &bs.blockrot(), &bs.pos, l.field),
        Mode::Clear(lc) => draw_clear(canvas, l.square_size, lc, st.field.w, l.field, st.frames),
    }
}

pub fn draw_clear<'a>(canvas : &mut Canvas<Window>, size : u32, lc : &LineClear, w : usize, off : (i32, i32), fr: u32) {
    let wpx = size * (w as u32);
    let rgb : u8 = 240 - ((fr as u8)  * 12);
    let a : u8 = 15 + ((fr as u8)  * 12);
    canvas.set_draw_color(Color::RGBA(rgb, rgb, rgb, a));

    let halp = | c : &mut Canvas<Window>, y : i32 | {
        match c.fill_rect(Rect::new(off.0, off.1 + (y * (size as i32)), wpx, size)) {
            Err(_) => panic!(),
            _ => {},
        };
    };

    let ls = &lc.lines;
    if ls.a { halp(canvas, lc.bottom); }
    if ls.b { halp(canvas, lc.bottom - 1); }
    if ls.c { halp(canvas, lc.bottom - 2); }
    if ls.d { halp(canvas, lc.bottom - 3); }

}

fn draw_field<'a>(canvas : &mut Canvas<Window>, size : u32, txs : &Squares<Texture<'a>>, f : &Field, off : (i32, i32)) {
    for y in 0 .. (f.h as i32) +1 {
        let pixy = off.1 + (y * (size as i32));
        for x in -1 .. (f.w as i32) + 1 {
            let tx = txs.pick(f.at(&Pos::new(x, y)));
            let pixx = off.0 + (x * (size as i32));
            canvas.copy(tx, None, Rect::new(pixx, pixy, size, size)).unwrap();
        }
    }
}

fn draw_block<'a>(canvas : &mut Canvas<Window>, size : u32, txs : &Squares<Texture<'a>>, b : &BlockRot, pos : &Pos, off : (i32, i32)) {
    let tx = txs.pick(b.square);

    let mut halp = | p : &Pos | {
        let x = pos.x + p.x;
        let y = pos.y + p.y;
        canvas.copy(tx,
                    None,
                    Rect::new(off.0 + (x * (size as i32)),
                              off.1 + (y * (size as i32)),
                              size,
                              size)).unwrap();
    };
    halp(&b.shape.a);
    halp(&b.shape.b);
    halp(&b.shape.c);
    halp(&b.shape.d);
}
