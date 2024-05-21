pub type GoResult<T> = Result<T, GoError>;

pub enum GoError {
    OutOfBounds,
    Suicide,
}

pub struct Board {
    players: Vec<u128>,
    current: usize,
    size: usize,
}

pub enum Pos {
    Index(u16),
    Cartesian(u8, u8),
}

impl Pos {
    pub fn index(index: impl Into<u16>) -> Self {
        Self::index(index.into())
    }

    pub fn xy(x: impl Into<u8>, y: impl Into<u8>) -> Self {
        Self::Cartesian(x.into(), y.into())
    }

    pub fn as_index(&self, size: usize) -> usize {
        match self {
            Pos::Index(i) => i % (size * size),
            Pos::Cartesian(x, y) => y * size as usize + x,
        }
    }

    pub fn as_xy(&self, size: usize) -> (usize, usize) {
        let dim = (size * size);
        match self {
            Pos::Index(i) => ((i % dim) / size, (i % dim) % size),
            Pos::Cartesian(x, y) => (x as usize, y as usize),
        }
    }
}

impl Board {
    fn is_suicide(&self, index: usize) -> bool {
        let mut explored = 0u128;
    }

    pub fn put(&mut self, player: usize, pos: Pos) -> GoResult<()> {
        let index = pos.as_index(self.size);

        Ok(())
    }
}
