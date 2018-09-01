use game::block::{Block, BlockRot, Rotation, block};
use game::field::{Field};
use game::pos::Pos;
use game::level::{Level, Gravity, gravity};
use four::{A};
use game::lineclear::LineClear;

pub struct BlockState {
    pub block : Block,
    pub rotation : Rotation,
    pub pos : Pos,
}

impl BlockState {
    pub fn new(b : Block, r : Rotation, p : Pos) -> BlockState {
        BlockState { block: b, rotation: r, pos: p }
    }
    pub fn blockrot(&self) -> BlockRot {
        block(self.block, self.rotation)
    }
}



pub enum Mode {
    Block(BlockState),
    Clear(LineClear),
}

pub struct Count {
    pub level : Level,
    pub lines : u64,
    pub score : u64,
}

impl Count {
    pub fn new(level : Level, lines : u64, score : u64) -> Count {
        Count { level: level, lines: lines, score: score }
    }
}

pub struct State {
    pub field : Field,
    pub mode : Mode,
    pub count : Count,
    pub frames : u32,
}

impl State {
    pub fn block(f : Field, bs : BlockState, c : Count, fr : u32) -> State {
        State {
            field: f,
            mode: Mode::Block(bs),
            count: c,
            frames: fr,
        }
    }
    pub fn clear(f : Field, lc : LineClear, c : Count, fr : u32) -> State {
        State {
            field: f,
            mode: Mode::Clear(lc),
            count: c,
            frames: fr,
        }
    }
}

pub struct StateFull {
    pub state : State,
    pub next : Block,
}

impl StateFull {
    pub fn block(f : Field, bs : BlockState, c : Count, fr : u32, n: Block) -> StateFull {
        StateFull {
            state: State::block(f, bs, c, fr),
            next: n,
        }
    }

    pub fn clear(f : Field, lc : LineClear, c : Count, fr : u32, n: Block) -> StateFull {
        StateFull {
            state: State::clear(f, lc, c, fr),
            next: n,
        }
    }
}

pub fn state(block : Block, next : Block, level : Level) -> StateFull {
    let f = Field::new(10, 20);
    let c = Count::new(0, 0, 0);
    let bs = BlockState::new(block, A, f.start);
    StateFull::block(f, bs, c, 0, next)
}

pub enum StateAfter {
    Full(StateFull),
    Missing(State),
    Ohno(State),
}
