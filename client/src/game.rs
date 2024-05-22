use crate::Command;

pub struct GameState {
    id: u8,
    map: Map,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            id: 0,
            map: Map::new(10, 10),
        }
    }
    pub fn process_response(&mut self, response: &str) {
        println!("  -> {}", response);
        match response.split_once(' ').unwrap_or((response, "")) {
            ("BOARD", data) => {
                let (char, data) = data.split_once(' ').unwrap();
                let (x, data) = data.split_once(' ').unwrap();
                let (y, data) = data.split_once(' ').unwrap();
                self.map = Map::parse(x.parse().unwrap(), y.parse().unwrap(), data).expect("Failed to parse map");
                self.id = char.chars().nth(0).unwrap() as u8 - b'0';
                self.generate_response();
            }
            _ => panic!("Unknown response: {}", response),
        }
    }

    fn generate_response(&self) -> Command {
        Command::Put(0, 0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Map {
    width: u16,
    height: u16,
    data: Vec<Tile>,
}

impl Map {
    fn new(width: u16, height: u16) -> Map {
        Map {
            width,
            height,
            data: vec![Tile::Empty; (width * height) as usize],
        }
    }

    fn get(&self, x: u16, y: u16) -> Tile {
        self.data[(y * self.width + x) as usize]
    }

    fn parse(width: u16, height: u16, data: &str) -> Result<Map, ()> {
        let data: Vec<_> = data.chars().map(Tile::from_char).collect();
        if data.len() != (width * height) as usize {
            return Err(());
        }
        Ok(Map { width, height, data })
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum Tile {
    #[default]
    Empty,
    Wall,
    Player(u8),
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::Wall,
            'A'..='z' => Tile::Player(c as u8 - b'A'),
            _ => panic!("Unknown tile type: {}", c),
        }
    }
}
