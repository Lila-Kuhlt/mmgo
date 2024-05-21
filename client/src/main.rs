use std::{io::BufRead, io::Write as IOWrite};

mod game;
use game::*;

const USER: &str = "nixi";
const PASSWORD: &str = "test";

fn main() -> std::io::Result<()> {
    println!("Starting go client");

    let mut stream = std::net::TcpStream::connect("127.0.0.1:1312")?;
    let mut reader = std::io::BufReader::new(stream.try_clone()?);

    let mut gamestate = GameState::new();

    stream.send_command(Command::Login(USER.to_owned(), PASSWORD.to_owned()))?;

    loop {
        let mut response = String::new();
        reader.read_line(&mut response)?;
        if response.contains('\n') {
            let response = response.trim();
            for line in response.lines() {
                gamestate.process_response(line);
            }
        }
    }
}

enum Command {
    Put(u32, u32),
    Login(String, String),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Put(x, y) => write!(f, "FIRE {} {}", x, y),
            Command::Login(user, pass) => write!(f, "LOGIN {} {}", user, pass),
        }
    }
}

trait CommandSink {
    fn send_command(&mut self, command: Command) -> std::io::Result<()>;
}

impl<T: IOWrite> CommandSink for T {
    fn send_command(&mut self, command: Command) -> std::io::Result<()> {
        println!("  <- {}", command.to_string());
        writeln!(self, "{command}")?;
        self.flush()
    }
}
