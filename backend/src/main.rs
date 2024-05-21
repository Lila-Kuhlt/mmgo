mod game;
mod network;

use std::io::{ErrorKind, Write};
use std::{net::TcpListener, str::FromStr};

use network::{Connection, UserAuth};

use crate::game::Board;
use crate::network::{Command, Error};

#[derive(Default, Debug)]
struct GameState {
    users: Vec<Connection>,
    user_auth: UserAuth,
    board: Board,
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
                    Err(Error::IO(e)) if e.kind() == ErrorKind::ConnectionAborted => {
                        // TODO: remove user
                        eprintln!("Lost connection to {}", user.addr);
                    }
                    Err(error) => eprintln!("error while reading user input: {error}"),
                }
            }
        }
    }

    fn broadcast_gamestate(&mut self) {
        let state = self.board.serialize();
        for user in self.users.iter_mut() {
            match user.stream.write_all(state.as_bytes()) {
                Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
                    // TODO: remove user
                    eprintln!("Lost connection to {}", user.addr);
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
    let mut game = GameState::default();

    loop {
        if let Err(e) = network::accept_new_connections(&listener, &mut game) {
            eprintln!("Error while accepting a new connection: {e}");
        }
        game.process_user_input();
        game.broadcast_gamestate();
    }
}
