
pub struct Layout {
    pub field : (i32, i32),
    pub next : (i32, i32),
    pub square_size : u32,
}

impl Layout {
    pub fn new(f : (i32, i32), n : (i32, i32), s : u32) -> Layout {
        Layout {
            field: f,
            next: n,
            square_size: s,
        }
    }
}
