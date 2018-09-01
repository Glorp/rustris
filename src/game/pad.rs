#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Rot {
    Left,
    Right,
}

pub struct Pad {
    dir : Option<Dir>,
    das : Option<(Dir, i32)>,
    down : Option<bool>,
    rot : Option<Rot>,
}

impl Pad {
    pub fn new() -> Pad {
        Pad::nw(None, None, None, None)
    }


    fn nw(dir : Option<Dir>, das : Option<(Dir, i32)>, down : Option<bool>, rot : Option<Rot>) -> Pad {
        Pad { dir: dir, das: das, rot: rot, down: down }
    }

    pub fn dir_down(&self, d : Dir) -> Pad {
        Pad::nw(Some(d), Some((d, 17)), self.down, self.rot)
    }
    pub fn dir_up(&self, d : Dir) -> Pad {
        let new_das =
            match self.das {
                Some((old, _)) => if d == old { None } else { self.das },
                _ => None,
            };
        Pad::nw(self.dir, new_das, self.down, self.rot)

    }
    pub fn down_down(&self) -> Pad {
        Pad::nw(self.dir, self.das, Some(true), self.rot)
    }
    pub fn down_up(&self) -> Pad {
        let new_down =
            match self.down {
                None => None,
                _ => Some(false),
            };
        Pad::nw(self.dir, self.das, new_down, self.rot)
    }
    pub fn rot_down(&self, r : Rot) -> Pad {
        Pad::nw(self.dir, self.das, self.down, Some(r))
    }
    pub fn next(&self) -> Pad {
        let new_down =
            match self.down {
                Some(true) => Some(true),
                _ => None,
            };
        let new_das =
            match self.das {
                Some((d, t)) => if t > 0 { Some((d, t - 1)) } else { Some((d, 6)) },
                None => None,
            };
        Pad::nw(None, new_das, new_down, None)
    }

    pub fn dir(&self) -> Option<Dir> {
        match (self.dir, self.das) {
            (Some(d), _) => Some(d),
            (_, Some((d, 0))) => Some(d),
            _ => None,
        }
    }
    pub fn down(&self) -> bool {
        self.down.is_some()
    }
    pub fn rot(&self) -> Option<Rot> {
        self.rot
    }
}
