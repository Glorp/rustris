use game::square::Square;
use game::pos::Pos;
use game::block::BlockRot;
use game::lineclear::LineClear;
use four::Four;

pub struct Field {
    pub w : usize,
    pub h : usize,
    pub f : Vec<Vec<Square>>,
    pub start : Pos,
}

impl Field {

    pub fn is_legal(&self, b : &BlockRot, pos : &Pos) -> bool {
        let halp = | p : Pos | -> bool {
            self.at(&Pos::new(pos.x + p.x, pos.y + p.y)) == Square::Empty
        };

        let sh = &b.shape;
        halp(sh.a) && halp(sh.b) && halp(sh.c) && halp(sh.d)
    }

    pub fn place(self, b : &BlockRot, pos : &Pos) -> (Field, Option<LineClear>) {
        let sq = b.square;
        let halp = | f : Field, p : &Pos, bot : i32 | -> (Field, i32) {
            let newp = Pos::new(pos.x + p.x, pos.y + p.y);
            let newbot = if newp.y > bot { newp.y } else { bot };
            (put(f, &newp, sq), newbot)
        };

        let sh = &b.shape;
        let (f, bot) = halp(self, &sh.a, -1);
        let (f, bot) = halp(f, &sh.b, bot);
        let (f, bot) = halp(f, &sh.c, bot);
        let (f, bot) = halp(f, &sh.d, bot);
        let lc = find_filled(&f, bot);
        (f, lc)
    }

    pub fn at(&self, p : &Pos) -> Square {
        if self.in_bounds(p) {
            self.f[p.y as usize][p.x as usize]
        } else if p.x >= 0 && p.x < (self.w as i32) && p.y < (self.h as i32) {
            Square::Empty
        } else {
            Square::Wall
        }
    }

    pub fn clear_lines(self, lc : &LineClear) -> Field {

        let mut y = lc.bottom;
        let mut by = 0;

        let ls = &lc.lines;

        let copy_line = | fi : Field, from: i32, by : i32 | -> Field {
            let mut f = fi;
            let to = from + by;
            if to < 0 || to >= (f.h as i32) || from == to { return f; }
            if from < 0 {
                for x in 0 .. f.w { f.f[to as usize][x] = Square::Empty };
            } else {
                for x in 0 .. f.w { f.f[to as usize][x] = f.f[from as usize][x] };
            }
            f
        };

        let mut f = self;
        if ls.a { by = by + 1 }
        y = y - 1;
        if ls.b { by = by + 1 } else { f = copy_line(f, y, by); }
        y = y - 1;
        if ls.c { by = by + 1 } else { f = copy_line(f, y, by); }
        y = y - 1;
        if ls.d { by = by + 1 } else { f = copy_line(f, y, by); }

        while y + by > 0 {
            y = y - 1;
            f = copy_line(f, y, by);;
        }
        f
    }

    pub fn in_bounds(&self, p : &Pos) -> bool {
        in_bounds_x(self, p.x) && in_bounds_y(self, p.y)
    }

    pub fn new(w : usize, h : usize) -> Field {
        let mut f = Vec::new();
        for _y in 0 .. h {
            let mut r = Vec::new();
            for _x in 0 .. w { r.push(Square::Empty); }
            f.push(r);
        }

        Field { w: w, h: h, f: f, start: Pos::new((w as i32) / 2, 0) }
    }
}

fn in_bounds_x(f : &Field, x : i32) -> bool {
    x >= 0 && x < (f.w as i32)
}

fn in_bounds_y(f : &Field, y : i32) -> bool {
    y >= 0 && y < (f.h as i32)
}

fn put(fi : Field, p : &Pos, s : Square) -> Field {
    let mut f = fi;
    if f.in_bounds(p) {
        f.f[p.y as usize][p.x as usize] = s;
    }
    f
}

fn find_filled(f : &Field, bottom : i32) -> Option<LineClear> {
    let a =is_filled(f, bottom);
    let b = is_filled(f, bottom - 1);
    let c = is_filled(f, bottom - 2);
    let d = is_filled(f, bottom - 3);
    if a || b || c || d {
        Some(LineClear::new(bottom, a, b, c, d))
    } else {
        None
    }
}

fn is_filled(f : &Field, y : i32) -> bool {
    if !in_bounds_y(f, y) {
        return false;
    }
    for s in &f.f[y as usize] {
        if *s == Square::Empty {
            return false;
        }
    }
    return true;
}
