use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use game::square::Squares;

pub fn squarestx<'a>(canvas : &mut Canvas<Window>, texture_creator : &'a TextureCreator<WindowContext>, size : u32) -> Squares<Texture<'a>> {
    let mut halp = |col : Color| -> Texture<'a> {
        make_square(canvas, texture_creator, size, col)
    };
    Squares{
        empty: halp(Color::RGB(0, 0, 0)),
        wall: halp(Color::RGB(50, 50, 50)),
        i: halp(Color::RGB(20, 255, 255)),
        j: halp(Color::RGB(0, 0, 255)),
        l: halp(Color::RGB(255, 165, 1)),
        o: halp(Color::RGB(255, 255, 0)),
        s: halp(Color::RGB(25, 255, 2)),
        t: halp(Color::RGB(128, 0, 128)),
        z: halp(Color::RGB(254, 0, 1)),
    }
}

fn make_square<'a>(canvas : &mut Canvas<Window>, texture_creator : &'a TextureCreator<WindowContext>, size : u32, col : Color) -> Texture<'a> {
    let mut square_texture : Texture =
        texture_creator.create_texture_target(None, size, size).unwrap();
    {
        canvas.with_texture_canvas(&mut square_texture, |texture_canvas| {
            texture_canvas.set_draw_color(col);
            texture_canvas.clear();
            /*
            for i in 0..SQUARE_SIZE {
                for j in 0..SQUARE_SIZE {
                    texture_canvas.set_draw_color(Color::RGB(255, 255, 0));
                    texture_canvas.draw_point(Point::new(i as i32, j as i32)).unwrap();
                }
            };
            */
        }).unwrap();
    }
    square_texture
}
