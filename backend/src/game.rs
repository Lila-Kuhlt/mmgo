use self::uf::UnionFind;

pub(crate) type Position = (u16, u16);

mod uf;

#[derive(Debug, Default, Clone)]
pub(crate) struct Board {
    tiles: Vec<Tile>,
    pub(crate) width: u16,
    pub(crate) height: u16,
    uf: UnionFind,
}

impl Board {
    pub(crate) fn new(width: u16, height: u16) -> Self {
        Board {
            tiles: vec![Tile::Empty; usize::from(width) * usize::from(height)],
            width,
            height,
            uf: UnionFind::new(usize::from(width * height)),
        }
    }

    fn index(&self, x: u16, y: u16) -> usize {
        let x = usize::from(x.min(self.width));
        let y = usize::from(y.min(self.height));
        x + usize::from(self.width) * y
    }
    fn tile_mut(&mut self, x: u16, y: u16) -> Option<&mut Tile> {
        let index = self.index(x, y);
        self.tiles.get_mut(index)
    }

    pub(crate) fn resolve_conflict(&mut self, x: u16, y: u16) {
        let Some(tile) = self.tile_mut(x, y) else { return };
        *tile = match *tile {
            Tile::TryPlace(id) => Tile::Player(id),
            Tile::Contested => Tile::Empty,
            t => t,
        };
    }

    pub(crate) fn try_place(&mut self, x: u16, y: u16, id: u8) {
        let Some(tile) = self.tile_mut(x, y) else { return };
        *tile = match *tile {
            Tile::Empty => Tile::TryPlace(id),
            Tile::TryPlace(_) | Tile::Contested => Tile::Contested,
            t => t,
        };
    }

    pub(crate) fn serialize(&self) -> String {
        self.tiles.iter().map(Tile::to_char).collect()
    }

    fn remove_group(&mut self, x: u16, y: u16) {
        let tile = *self.tile_mut(x, y).expect("Tried to remove non board space");
        let mut stack = vec![(x, y)];
        while let Some((x, y)) = stack.pop() {
            if self.tile_mut(x, y).as_deref() == Some(&tile) {
                self.uf.reset_node(self.index(x, y));
                self.adjacent_tiles(x, y, &mut stack)
            }
        }
    }

    fn adjacent_tiles(&self, x: u16, y: u16, stack: &mut Vec<(u16, u16)>) {
        for i in (x.max(1) - 1)..=(x.min(u16::from(self.width) - 1) + 1) {
            for j in (y.max(1) - 1)..=(y.min(u16::from(self.height) - 1) + 1) {
                stack.push((i, j))
            }
        }
    }
}

pub enum GoError {
    OutOfBounds,
    Suicide,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Player(u8),
    TryPlace(u8),
    Contested,
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Wall => '/',
            Tile::Player(c) => *c as char,
            _ => unreachable!("Forgot to clean up intermediate board state"),
        }
    }
}
