use std::time::{Duration, SystemTime};

use self::uf::UnionFind;

pub(crate) type Position = (u16, u16);

mod uf;

#[derive(Debug, Clone)]
pub(crate) struct Board {
    tiles: Vec<Tile>,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) start: SystemTime,
    uf: UnionFind,
}

impl Board {
    pub(crate) fn new(width: u16, height: u16) -> Self {
        Board {
            tiles: vec![Tile::Empty; usize::from(width) * usize::from(height)],
            width,
            height,
            start: SystemTime::now(),
            uf: UnionFind::new(width as usize, height as usize),
        }
    }

    fn index(&self, x: u16, y: u16) -> usize {
        let x = usize::from(x.min(self.width));
        let y = usize::from(y.min(self.height));
        x + usize::from(self.width) * y
    }

    fn is_suicide(&mut self, x: u16, y: u16, id: u8) -> bool {
        let empty_tiles = self.adjacent_filter(x, y, Tile::Empty);
        let mut other_players = self.adjacent_tiles(x, y).filter_map(|(_, _, t)| match t {
            Tile::Player(id) => Some((self.uf.get_liberties(self.index(x, y)), id)),
            _ => None,
        });
        (empty_tiles.count() == 0) && other_players.all(|(libs, oid)| if oid == id { libs < 2 } else { libs > 1 })
    }

    fn tile_mut(&mut self, x: u16, y: u16) -> Option<&mut Tile> {
        let index = self.index(x, y);
        self.tiles.get_mut(index)
    }

    fn try_tile(&self, x: i16, y: i16) -> Option<(u16, u16, Tile)> {
        if x < 0 || y < 0 {
            return None;
        }
        self.tile(x as u16, y as u16).map(|t| (x as u16, y as u16, t))
    }

    fn tile(&self, x: u16, y: u16) -> Option<Tile> {
        let index = self.index(x, y);
        self.tiles.get(index).copied()
    }

    pub(crate) fn resolve_conflict(&mut self, x: u16, y: u16) {
        let index = self.index(x, y);
        let Some(mut tile) = self.tile(x, y) else { return };
        tile = match tile {
            Tile::TryPlace(id) => {
                self.adjacent_tiles(x, y)
                    .for_each(|(x, y, _)| self.uf.subtract_liberty(self.index(x, y)));
                self.adjacent_filter(x, y, Tile::Player(id))
                    .for_each(|(x, y)| self.uf.union(index, self.index(x, y)));

                Tile::Player(id)
            }
            Tile::Contested => Tile::Empty,
            t => t,
        };
        self.tiles[index] = tile;
    }

    pub(crate) fn try_place(&mut self, x: u16, y: u16, id: u8) {
        if self.is_suicide(x, y, id) {
            return;
        }
        let Some(tile) = self.tile_mut(x, y) else { return };
        *tile = match *tile {
            Tile::Empty => Tile::TryPlace(id),
            Tile::TryPlace(_) | Tile::Contested => Tile::Contested,
            t => t,
        };
    }

    pub(crate) fn kill_neighbors(&mut self, x: u16, y: u16) {
        for (x, y, _) in self.adjacent_tiles(x, y) {
            let index = self.index(x, y);
            if self.uf.get_liberties(index) == 0 {
                self.remove_group(x, y);
            }
        }
    }

    pub(crate) fn serialize(&self) -> String {
        self.tiles.iter().map(Tile::to_char).collect()
    }

    fn remove_group(&mut self, x: u16, y: u16) {
        dbg!("removing tiles", x, y);
        let tile = *self.tile_mut(x, y).expect("Tried to remove non board space");
        let mut stack = vec![(x, y, tile)];
        while let Some((x, y, t)) = stack.pop() {
            if t == tile {
                self.uf.reset_node(self.index(x, y));
                self.adjacent_tiles(x, y).for_each(|d| stack.push(d));
                *self.tile_mut(x, y).unwrap() = Tile::Empty;
            }
        }
    }

    fn adjacent_filter(&self, x: u16, y: u16, tile: Tile) -> impl Iterator<Item = (u16, u16)> {
        self.adjacent_tiles(x, y)
            .filter_map(move |(x, y, t)| (t == tile).then(|| (x, y)))
    }

    pub(crate) fn reset_timer(&mut self) {
        self.start = SystemTime::now()
    }

    pub(crate) fn unix_timestamp(&self) -> u128 {
        self.start
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis()
    }

    fn adjacent_tiles(&self, x: u16, y: u16) -> impl Iterator<Item = (u16, u16, Tile)> {
        let (x, y) = (x as i16, y as i16);
        let data = [
            self.try_tile(x - 1, y),
            self.try_tile(x, y - 1),
            self.try_tile(x + 1, y),
            self.try_tile(x, y + 1),
        ];
        data.into_iter().flatten()
    }
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
