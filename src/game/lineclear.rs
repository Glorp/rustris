use four::Four;

pub struct LineClear {
    pub bottom : i32,
    pub lines : Four<bool>,
}

impl LineClear {
    pub fn new(bottom : i32, a : bool, b : bool, c : bool, d : bool) -> LineClear {
        LineClear {
            bottom: bottom,
            lines: Four::new(a, b, c, d),
        }
    }
}
