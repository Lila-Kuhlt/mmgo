mod protocol;

use std::{
    collections::{hash_map::Entry, HashMap},
    io::{ErrorKind, Read},
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
};

use crate::protocol::Error;

#[derive(Default, Debug)]
struct UserAuth {
    users: HashMap<String, String>,
}

impl UserAuth {
    pub fn is_valid_or_insert(&mut self, username: String, password: String) -> Option<String> {
        match self.users.entry(username) {
            Entry::Vacant(v) => {
                let key = v.key().clone();
                v.insert(password);
                Some(key)
            }
            Entry::Occupied(o) => (o.get() == &password).then(|| o.key().clone()),
        }
    }
}
type Color = u32;
type Position = (u32, u32);

#[derive(Debug, Default, Clone)]
struct Board;

impl Board {
    fn place(&mut self, x: u32, y: u32, addr: SocketAddr) {
        todo!()
    }
}

#[derive(Debug)]
struct Connection {
    addr: SocketAddr,
    username: Option<String>,
    color: Color,
    stream: TcpStream,
    next_stone: Option<Position>,
}

#[derive(Default, Debug)]
struct GameState {
    users: Vec<Connection>,
    user_auth: UserAuth,
    board: Board,
}

enum Command {
    Login(String, String),
    Put(Position),
}
impl FromStr for Command {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut args = line.splitn(2, '\n');
        let args = std::array::from_fn(|_| args.next().unwrap_or_default());
        match args {
            ["LOGIN", user, password] => Ok(Command::Login(user.to_owned(), password.to_owned())),
            ["PUT", x, y] => {
                let x: u32 = x.parse().map_err(|_| Error::InvalidArgument)?;
                let y: u32 = y.parse().map_err(|_| Error::InvalidArgument)?;
                Ok(Command::Put((x, y)))
            }
            _ => Err(Error::UnknownCommand),
        }
    }
}

fn parse_line(stream: &mut TcpStream, parse: impl Fn(&str) -> Result<Command, Error>) -> Result<Command, Error> {
    let mut buf = [0; 1024];
    let bytes = stream.peek(&mut buf)?;
    let pos = buf[0..bytes]
        .iter()
        .position(|a| a == &b'\n')
        .ok_or(Error::WouldBlock)?;
    stream.read_exact(&mut buf[0..pos])?;
    let str = std::str::from_utf8(&buf[0..pos])?;
    parse(str)
}

impl GameState {
    fn process_user_input(&mut self) {
        for user in self.users.iter_mut() {
            loop {
                match parse_line(&mut user.stream, Command::from_str) {
                    Ok(Command::Login(username, password)) => {
                        if let Some(username) = self.user_auth.is_valid_or_insert(username, password) {
                            user.username = Some(username);
                        } else {
                            eprintln!("Invalid Credentials");
                        }
                    }
                    Ok(Command::Put((x, y))) => self.board.place(x, y, user.addr),
                    Err(Error::WouldBlock) => break,
                    Err(error) => eprintln!("error while reading user input: {error}"),
                }
            }
        }
    }
}

fn random_color() -> Color {
    todo!()
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1312")?;
    listener.set_nonblocking(true)?;
    let mut game = GameState::default();

    loop {
        if let Err(e) = accept_new_connections(&listener, &mut game) {
            eprintln!("Error while accepting a new connection: {e}");
        }
    }
}

fn accept_new_connections(listener: &TcpListener, game: &mut GameState) -> std::io::Result<()> {
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true)?;
                let con = Connection {
                    addr,
                    username: None,
                    color: random_color(),
                    stream,
                    next_stone: None,
                };
                game.users.push(con);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                break;
            }
            Err(e) => eprintln!("socket error: {e}"),
        }
    }
    Ok(())
}
