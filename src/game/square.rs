use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Square {
    Empty,
    Wall,
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Square::Empty => 'E',
            Square::Wall => 'W',
            Square::I  => 'I',
            Square::J  => 'J',
            Square::L  => 'L',
            Square::O  => 'O',
            Square::S  => 'S',
            Square::T  => 'T',
            Square::Z  => 'Z',
        };
        write!(f, "{}", printable)
    }
}

pub struct Squares<T> {
    pub empty : T,
    pub wall : T,
    pub i : T,
    pub j : T,
    pub l : T,
    pub o : T,
    pub s : T,
    pub t : T,
    pub z : T,
}

impl<T> Squares<T> {
    pub fn pick<'a>(&'a self, sq : Square) -> &'a T {
        match sq {
            Square::Empty => &self.empty,
            Square::Wall => &self.wall,
            Square::I => &self.i,
            Square::J => &self.j,
            Square::L => &self.l,
            Square::O => &self.o,
            Square::S => &self.s,
            Square::T => &self.t,
            Square::Z => &self.z,
        }
    }
}
