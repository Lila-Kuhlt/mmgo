use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    hash::Hasher,
    io::{ErrorKind, Read},
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
    time::{Duration, SystemTime},
};

use crate::{
    game::{Board, Position},
    GameState,
};

#[derive(Debug)]
pub enum Error {
    InvalidArgument,
    UnknownCommand,
    ConnectionLost,
    WouldBlock,
    GameFull,
    IO(std::io::Error),
    Utf8(std::str::Utf8Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::WouldBlock => Error::WouldBlock,
            ErrorKind::BrokenPipe | ErrorKind::ConnectionAborted => Error::ConnectionLost,
            _ => Error::IO(value),
        }
    }
}
impl From<std::str::Utf8Error> for Error {
    fn from(value: std::str::Utf8Error) -> Self {
        Error::Utf8(value)
    }
}
pub(crate) type Color = u32;

#[derive(Debug)]
pub(crate) struct Connection {
    pub(crate) addr: SocketAddr,
    pub(crate) username: Option<String>,
    color: Color,
    pub(crate) char: u8,
    pub(crate) stream: TcpStream,
    pub(crate) next_stone: Option<Position>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Command {
    Login(String, String),
    Put(Position),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut args = line.splitn(3, ' ');
        let args = std::array::from_fn(|_| args.next().unwrap_or_default());
        dbg!(&args);
        match args {
            ["LOGIN", user, password] => Ok(Command::Login(user.to_owned(), password.to_owned())),
            ["PUT", x, y] => {
                let x: u16 = x.parse().map_err(|_| Error::InvalidArgument)?;
                let y: u16 = y.parse().map_err(|_| Error::InvalidArgument)?;
                Ok(Command::Put((x, y)))
            }
            _ => Err(Error::UnknownCommand),
        }
    }
}

pub(crate) fn parse_line(
    stream: &mut TcpStream,
    parse: impl Fn(&str) -> Result<Command, Error>,
) -> Result<Command, Error> {
    let mut buf = [0; 1024];
    let bytes = stream.peek(&mut buf)?;
    if bytes == 0 {
        return Err(Error::ConnectionLost);
    }
    let pos = buf[0..bytes]
        .iter()
        .position(|a| a == &b'\n')
        .ok_or(Error::WouldBlock)?;
    stream.read_exact(&mut buf[0..=pos])?;
    let str = std::str::from_utf8(&buf[0..pos])?;
    if pos > 0 {
        parse(str)
    } else {
        panic!("{:?}", &buf[..bytes]);
    }
}
pub(crate) fn accept_new_connections(listener: &TcpListener, game: &mut GameState) -> Result<(), Error> {
    fn random_color() -> Color {
        std::collections::hash_map::DefaultHasher::new().finish() as Color
    }
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                stream.set_nonblocking(true)?;
                let Some(char) = game.alloc_char(addr) else {
                    return Err(Error::GameFull);
                };
                let con = Connection {
                    addr,
                    username: None,
                    color: random_color(),
                    char,
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
pub(crate) fn accept_new_ws(listener: &TcpListener, game: &mut GameState) -> Result<(), Error> {
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("got new connection from {addr}");
                let websocket = tungstenite::accept(stream).unwrap();
                game.frontend = Some(websocket);
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                break;
            }
            Err(e) => eprintln!("socket error: {e}"),
        }
    }
    Ok(())
}
#[derive(Default, Debug)]
pub(crate) struct UserAuth {
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

pub(crate) enum FrontendMessage<'a> {
    Board(&'a Board),
    End,
}

impl<'a> Display for FrontendMessage<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontendMessage::Board(board) => {
                write!(
                    f,
                    "BOARD {:?} {} {} {}",
                    board
                        .unix_timestamp(),
                    board.width,
                    board.height,
                    board.serialize()
                )
            }
            FrontendMessage::End => write!(f, "END"),
        }
    }
}
