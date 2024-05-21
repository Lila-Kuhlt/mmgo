use std::{fmt::Display, marker::PhantomData};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::{io::BufStream, net::TcpStream};

pub trait Serialize {
    fn serialize(&self) -> Result<String, Error>;
}

pub trait Deserialize
where
    Self: Sized,
{
    fn deserialize(input: &str) -> Result<Self, Error>;
}

#[derive(Debug)]
pub enum Error {
    UnexpectedEof,
    InvalidArgument,
    UnknownCommand,
    UnexpectedCommand,
    Timeout,
    InvalidCredentials,
}

pub enum Protocol {
    Login(String, String),
    Put(u32, u32),
    Error(Error),
    Motd(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Protocol {
    fn serialize(&self) -> Result<String, Error> {
        Ok(match self {
            Protocol::Login(username, password) => format!("LOGIN {username} {password}"),
            Protocol::Put(x, y) => format!("PUT {x} {y}"),
            Protocol::Error(e) => format!("ERROR {e}"),
            Protocol::Motd(motd) => format!("MOTD {motd}"),
        })
    }

    fn deserialize(input: &str) -> Result<Self, Error> {
        let mut input = input.split(' ');

        macro_rules! parse {
            () => {{
                input
                    .next()
                    .ok_or(Error::UnexpectedEof)
                    .and_then(|val| val.parse().map_err(|_| Error::InvalidArgument))
            }};
        }

        Ok(match input.next().unwrap() {
            "LOGIN" => Self::Login(parse!()?, parse!()?),
            "PUT" => Self::Put(parse!()?, parse!()?),
            _ => Err(Error::UnknownCommand)?,
        })
    }
}

pub struct ProtocolStream {
    stream: BufStream<TcpStream>,
}

impl ProtocolStream {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: BufStream::new(stream),
        }
    }

    pub async fn write(&mut self, what: &Protocol) -> Result<(), Error> {
        self.stream
            .write_all(what.serialize()?.as_bytes())
            .await
            .expect("Could not write to stream, fuck you");

        let _ = self.stream.write_u8(b'\n').await;

        self.stream.flush().await.expect("Could not flush, fuck you");

        Ok(())
    }

    pub async fn read(&mut self) -> Result<Protocol, Error> {
        let mut buf = String::new();

        self.stream
            .read_line(&mut buf)
            .await
            .expect("Could not read line, also fix this");

        Protocol::deserialize(&buf)
    }
}

#[macro_export]
macro_rules! proto_expect {
    ($stream: expr, $e: pat) => {
        let $e = $stream.read().await? else {
            return Err(protocol::Error::UnexpectedCommand);
        };
    };
}
