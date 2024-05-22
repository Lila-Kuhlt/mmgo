mod game;
mod network;

use std::io::{ErrorKind, Write};
use std::net::SocketAddr;
use std::time::Duration;
use std::{net::TcpListener, str::FromStr};

use network::{Connection, UserAuth};

use crate::game::Board;
use crate::network::{Command, Error};

#[derive(Default, Debug)]
struct GameState {
    users: Vec<Connection>,
    user_auth: UserAuth,
    board: Board,
    chars: Vec<Option<SocketAddr>>,
    disconnected: Vec<SocketAddr>,
}

impl GameState {
    fn process_user_input(&mut self) {
        for user in self.users.iter_mut() {
            loop {
                match network::parse_line(&mut user.stream, Command::from_str) {
                    Ok(Command::Login(username, password)) => {
                        if let Some(username) = self.user_auth.is_valid_or_insert(username, password) {
                            user.username = Some(username);
                        } else {
                            eprintln!("Invalid Credentials");
                        }
                    }
                    Ok(Command::Put(pos)) => user.next_stone = Some(pos),
                    Err(Error::WouldBlock) => break,
                    Err(Error::ConnectionLost) => {
                        self.disconnected.push(user.addr);
                        eprintln!("Lost connection to {}", user.addr);
                        break;
                    }
                    Err(error) => eprintln!("error while reading user input: {error}"),
                }
            }
        }
    }

    pub(crate) fn alloc_char(&mut self, addr: SocketAddr) -> Option<u8> {
        let pos = self.chars.iter().position(Option::is_none)?;
        self.chars[pos] = Some(addr);
        Some(pos as u8 + b'A')
    }

    fn remove_user(&mut self, addr: SocketAddr) {
        eprintln!("Removing user {}", addr);
        if let Some(pos) = self.users.iter().position(|u| u.addr == addr) {
            self.users.swap_remove(pos);
        }
        if let Some(value) = self.chars.iter_mut().find(|x| **x == Some(addr)) {
            std::mem::take(value);
        }
    }

    fn remove_disconnected_users(&mut self) {
        for addr in std::mem::take(&mut self.disconnected) {
            self.remove_user(addr);
        }
    }

    fn broadcast_gamestate(&mut self) {
        let state = self.board.serialize();
        for user in self.users.iter_mut() {
            match writeln!(
                user.stream,
                "BOARD {} {} {} {}",
                user.char, self.board.width, self.board.height, state
            ) {
                Err(e) if e.kind() == ErrorKind::ConnectionAborted => (),
                Err(e) if e.kind() == ErrorKind::BrokenPipe => (),
                Err(e) if matches!(e.kind(), ErrorKind::ConnectionAborted | ErrorKind::BrokenPipe) => {
                    dbg!("foo");
                    self.disconnected.push(user.addr);
                }
                Err(e) => {
                    eprintln!("Error while sending {e}");
                }
                _ => (),
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1312")?;
    listener.set_nonblocking(true)?;
    let mut game = GameState {
        board: Board::new(10, 10),
        chars: vec![None; 'z' as usize - 'A' as usize],
        ..Default::default()
    };

    loop {
        if let Err(e) = network::accept_new_connections(&listener, &mut game) {
            eprintln!("Error while accepting a new connection: {e}");
        }
        game.process_user_input();
        game.remove_disconnected_users();
        game.broadcast_gamestate();
        std::thread::sleep(Duration::from_millis(500));
    }
}
