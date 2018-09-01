use game::block::Block;

pub struct Rando {
    x : u32,
    pub block : Block,
}

impl Rando {
    pub fn new(x : u32) -> Rando {
        gen(x)
    }

    fn priv_new(seed : u32, b: Block) -> Rando {
        Rando { x: seed, block: b }
    }

    pub fn next(&self) -> Rando {
        let current = self.block;
        let res = gen(self.x);
        if current != res.block {
            res
        } else {
            gen(res.x)
        }
    }
}

fn gen(x : u32) -> Rando {
    let mut res = 0;
    let mut x = x;
    while res == 0 {
        x = x ^ (x << 13);
        x = x ^ (x >> 13);
        x = x ^ (x << 6);
        res = x % 8;
    };
    let block =
        match res {
            1 => Block::I,
            2 => Block::J,
            3 => Block::L,
            4 => Block::O,
            5 => Block::S,
            6 => Block::T,
            7 => Block::Z,
            _ => panic!(),
        };
    Rando::priv_new(x, block)
}
