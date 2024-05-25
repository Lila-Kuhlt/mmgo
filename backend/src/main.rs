mod game;
mod network;

use std::io::{ErrorKind, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::{net::TcpListener, str::FromStr};

use network::{Connection, FrontendMessage, UserAuth};
use tungstenite::WebSocket;

use crate::game::Board;
use crate::network::{Command, Error};

#[derive(Debug)]
struct GameState {
    users: Vec<Connection>,
    user_auth: UserAuth,
    board: Board,
    chars: Vec<Option<SocketAddr>>,
    disconnected: Vec<SocketAddr>,
    frontend: Option<WebSocket<TcpStream>>,
}

impl GameState {
    fn new(size: u16) -> Self {
        GameState {
            board: Board::new(size, size),
            chars: vec![None; 'z' as usize - 'A' as usize],
            users: Vec::new(),
            user_auth: UserAuth::default(),
            disconnected: Vec::new(),
            frontend: None,
        }
    }

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

    fn place_pieces(&mut self) {
        for user in &self.users {
            if let Some((x, y)) = user.next_stone {
                self.board.try_place(x as u16, y as u16, user.char)
            }
        }
        for user in &mut self.users {
            if let Some((x, y)) = user.next_stone.take() {
                self.board.resolve_conflict(x, y);
            }
        }
        for user in &mut self.users {
            if let Some((x, y)) = user.next_stone.take() {
                self.board.kill_neighbors(x, y);
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

    fn send_frontend(&mut self, msg: String) {
        let Some(ref mut frontend) = self.frontend else { return };
        let message = tungstenite::Message::text(msg);
        match frontend.send(message) {
            Err(tungstenite::Error::Io(e))
                if matches!(e.kind(), ErrorKind::ConnectionAborted | ErrorKind::BrokenPipe) => {}
            Err(e) => {
                self.frontend = None;
                eprintln!("Error while sending {e}");
            }
            _ => (),
        }
    }

    fn update_frontend(&mut self) {
        self.send_frontend(FrontendMessage::Board(&self.board).to_string());
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
    let listener = TcpListener::bind("0.0.0.0:1312")?;
    let ws_listener = TcpListener::bind("0.0.0.0:1213")?;
    listener.set_nonblocking(true)?;
    ws_listener.set_nonblocking(true)?;
    let mut game = GameState::new(50);
    loop {
        if let Err(e) = network::accept_new_connections(&listener, &mut game) {
            eprintln!("Error while accepting a new connection: {e}");
        }
        if let Err(e) = network::accept_new_ws(&ws_listener, &mut game) {
            eprintln!("Error while accepting a new connection: {e}");
        }
        game.process_user_input();
        game.remove_disconnected_users();
        game.place_pieces();
        game.update_frontend();
        game.broadcast_gamestate();

        std::thread::sleep(Duration::from_millis(300));
    }
}
