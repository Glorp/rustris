use game::state::{StateFull, State, BlockState, Mode, StateAfter, Count};
use game::lineclear::LineClear;
use game::pad::{Pad, Dir, Rot};
use game::field::{Field};
use game::block::{Block};
use game::pos::{Pos};
use game::level::{Level, Gravity, gravity};

use std::cmp::{min};

use four::{Four, FourLabel};

pub fn step(stf : StateFull, pad : &Pad) -> StateAfter {

    let st = stf.state;
    let f = st.field;
    let m = st.mode;
    let c = st.count;
    let fr = st.frames;

    match m {
        Mode::Block(bs) => block_step(bs, f, c, fr, stf.next, pad),
        Mode::Clear(lc) => clear_step(lc, f, c, fr, stf.next),
    }
}

fn clear_step(lc : LineClear, f : Field, c : Count, fr : u32, n : Block) -> StateAfter {
    if fr < 20 {
        StateAfter::Full(StateFull::clear(f, lc, c, fr + 1, n))
    } else {
        let newbs = BlockState::new(n, FourLabel::A, f.start);
        let newf = f.clear_lines(&lc);
        StateAfter::Missing(State::block(newf, newbs, c, fr + 1))
    }
}

fn block_step(bs : BlockState, f : Field, c : Count, fr : u32, n : Block, pad : &Pad) -> StateAfter {

    let moved = try_move(bs, &f, pad.dir());
    let roted = try_rot(moved, &f, pad.rot());

    let levelgrav = gravity(c.level);
    let grav = if pad.down() { min(levelgrav, 2) } else { levelgrav };

    let newfr = fr + 1;
    if fr >= grav {
        step_down(roted, f, c, n)
    } else {
        StateAfter::Full(StateFull::block(f, roted, c, newfr, n))
    }
}

fn step_down(bs : BlockState, f : Field, c : Count, n : Block) -> StateAfter {

    let blockrot = &bs.blockrot();
    let p = bs.pos;
    let newp = p.down();

    if f.is_legal(&blockrot, &newp) {
        let newbs = BlockState::new(bs.block, bs.rotation, newp);
        StateAfter::Full(StateFull::block(f, newbs, c, 0, n))
    } else {
        let (newf, lco) = f.place(&blockrot, &p);
        match lco {
            Some(lc) => StateAfter::Full(StateFull::clear(newf, lc, c, 0, n)),
            None => {
                let newp = newf.start;
                let newbs = BlockState::new(n, FourLabel::A, newp);
                StateAfter::Missing(State::block(newf, newbs, c, 0))
            },
        }
    }
}


fn legal_bs(bs : BlockState, f: &Field, orelse : BlockState) -> BlockState {
    let blockrot = &bs.blockrot();
    if f.is_legal(&blockrot, &bs.pos) {
      bs
    } else {
      orelse
    }
}

fn try_move(bs : BlockState, f: &Field, d : Option<Dir>) -> BlockState {
    let p = bs.pos;
    let newp = match d {
        Some(Dir::Left) => p.left(),
        Some(Dir::Right) => p.right(),
        _ => return bs,
    };
    let newbs = BlockState::new(bs.block,  bs.rotation, newp);
    legal_bs(newbs, &f, bs)
}

fn try_rot(bs : BlockState, f: &Field, r : Option<Rot>) -> BlockState {
    let br = bs.rotation;
    let newr = match r {
        Some(Rot::Left) => br.left(),
        Some(Rot::Right) => br.right(),
        _ => return bs,
    };
    let newbs = BlockState::new(bs.block, newr, bs.pos);
    legal_bs(newbs, &f, bs)
}
