
pub struct Four<T> {
    pub a : T,
    pub b : T,
    pub c : T,
    pub d : T,
}

impl<T> Four<T> {
    pub fn new(a : T, b : T, c : T, d : T) -> Four<T> {
        Four { a: a, b: b, c: c, d: d, }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FourLabel {
    A,
    B,
    C,
    D,
}

pub const A : FourLabel = FourLabel::A;
pub const B : FourLabel = FourLabel::B;
pub const C : FourLabel = FourLabel::C;
pub const D : FourLabel = FourLabel::D;

impl<T> Four<T> {

    pub fn pick<'a>(&'a self, fl : FourLabel) -> &'a T {
        match fl {
            A => &self.a,
            B => &self.b,
            C => &self.c,
            D => &self.d,
        }
    }
}

impl FourLabel {
    pub fn left(&self) -> FourLabel {
        match self {
            &A => D,
            &B => A,
            &C => B,
            &D => C,
        }
    }

    pub fn right(&self) -> FourLabel {
        match self {
            &A => B,
            &B => C,
            &C => D,
            &D => A,
        }
    }
}
