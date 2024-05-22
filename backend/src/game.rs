pub(crate) type Position = (u32, u32);

#[derive(Debug, Default, Clone)]
pub(crate) struct Board {
    tiles: Vec<Tile>,
    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl Board {
    pub(crate) fn new(width: u16, height: u16) -> Self {
        Board {
            tiles: vec![Tile::Empty; usize::from(width) * usize::from(height)],
            width,
            height,
        }
    }

    pub(crate) fn place(&mut self, x: u16, y: u16, id: u8) {
        let x = usize::from(x.min(self.width));
        let y = usize::from(y.min(self.height));
        self.tiles[x + usize::from(self.width) * y] = Tile::Player(id);
    }

    pub(crate) fn serialize(&self) -> String {
        self.tiles.iter().map(Tile::to_char).collect()
    }
}

pub enum GoError {
    OutOfBounds,
    Suicide,
}

#[derive(Debug, Clone, Copy, Default)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Player(u8),
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Wall => '/',
            Tile::Player(c) => *c as char,
        }
    }
}
