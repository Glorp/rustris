pub type Level = u32;
pub type Gravity = u32;

pub fn gravity(level : Level) -> Gravity {
    match level {
        0 => 48,
        1 => 43,
        2 => 38,
        3 => 33,
        4 => 28,
        5 => 23,
        6 => 18,
        7 => 13,
        8 => 8,
        9 => 6,
        10 ... 12 => 5,
        13 ... 15 => 4,
        16 ... 18 => 3,
        19 ... 28 => 2,
        _ => 1,
    }
}
