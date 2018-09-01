use game::square::Square;
use game::pos::Pos;
use four::{Four, FourLabel, A, B, C, D};

pub struct BlockRot {
    pub square : Square,
    pub shape : Four<Pos>,
}

impl BlockRot {
    pub fn new(s : Square, a : (i32, i32), b : (i32, i32), c : (i32, i32), d : (i32, i32)) -> BlockRot {
        BlockRot{
            square: s,
            shape: Four {
                a: Pos::new(a.0, a.1),
                b: Pos::new(b.0, b.1),
                c: Pos::new(c.0, c.1),
                d: Pos::new(d.0, d.1),
            }
        }
    }
}

pub type BlockRots = Four<BlockRot>;

pub type Rotation = FourLabel;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Block {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

pub struct Blocks<T> {
    pub i : T,
    pub j : T,
    pub l : T,
    pub o : T,
    pub s : T,
    pub t : T,
    pub z : T,
}

impl<T> Blocks<T> {
    pub fn pick<'a>(&'a self, b : &Block) -> &'a T {
        match b {
            Block::I => &self.i,
            Block::J => &self.j,
            Block::L => &self.l,
            Block::O => &self.o,
            Block::S => &self.s,
            Block::T => &self.t,
            Block::Z => &self.z,
        }
    }
}

const I : Block = Block::I;
const J : Block = Block::J;
const L : Block = Block::L;
const O : Block = Block::O;
const S : Block = Block::S;
const T : Block = Block::T;
const Z : Block = Block::Z;

pub fn block(b : Block, r : Rotation) -> BlockRot {
    match (b, r) {
        (I, A) | (I, C) => BlockRot::new(Square::I, (-2, 0), (-1, 0), (0, 0), (1, 0)),
        (I, B) | (I, D) => BlockRot::new(Square::I, (0, -1), (0, 0), (0, 1), (0, 2)),
        (J, A) => BlockRot::new(Square::J, (-1, 0), (0, 0), (1, 0), (1, 1)),
        (J, B) => BlockRot::new(Square::J, (0, -1), (0, 0), (0, 1), (-1, 1)),
        (J, C) => BlockRot::new(Square::J, (-1, -1), (-1, 0), (0, 0), (1, 0)),
        (J, D) => BlockRot::new(Square::J, (1, -1), (0, -1), (0, 0), (0, 1)),
        (L, A) => BlockRot::new(Square::L, (-1, 1), (-1, 0), (0, 0), (1, 0)),
        (L, B) => BlockRot::new(Square::L, (-1, -1), (0, -1), (0, 0), (0, 1)),
        (L, C) => BlockRot::new(Square::L, (-1, 0), (0, 0), (1, 0), (1, -1)),
        (L, D) => BlockRot::new(Square::L, (0, -1), (0, 0), (0, 1), (1, 1)),
        (O, _) => BlockRot::new(Square::O, (0, 0), (0, 1), (1, 1), (1, 0)),
        (S, A) | (S, C) => BlockRot::new(Square::S, (-1, 1), (0, 1), (0, 0), (1, 0)),
        (S, B) | (S, D) => BlockRot::new(Square::S, (0, -1), (0, 0), (1, 0), (1, 1)),
        (T, A) => BlockRot::new(Square::T, (-1, 0), (0, 0), (1, 0), (0, 1)),
        (T, B) => BlockRot::new(Square::T, (0, -1), (0, 0), (0, 1), (-1, 0)),
        (T, C) => BlockRot::new(Square::T, (-1, 0), (0, 0), (1, 0), (0, -1)),
        (T, D) => BlockRot::new(Square::T, (0, -1), (0, 0), (0, 1), (1, 0)),
        (Z, A) | (Z, C) => BlockRot::new(Square::Z, (-1, 0), (0, 0), (0, 1), (1, 1)),
        (Z, B) | (Z, D) => BlockRot::new(Square::Z, (1, -1), (1, 0), (0, 0), (0, 1)),
    }
}
