#[derive(Copy, Clone, PartialEq, Eq)]
pub struct  Pos {
    pub x : i32,
    pub y :  i32,
}

impl Pos {

    pub fn new(x: i32, y: i32) -> Pos {
        Pos { x: x, y: y }
    }

    pub fn down(&self) -> Pos {
        Pos::new(self.x, self.y + 1)
    }
    pub fn left(&self) -> Pos {
        Pos::new(self.x - 1, self.y)
    }
    pub fn right(&self) -> Pos {
        Pos::new(self.x + 1, self.y)
    }
}
