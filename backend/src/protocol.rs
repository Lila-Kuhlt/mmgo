use std::{fmt::Display, marker::PhantomData};

#[derive(Debug)]
pub enum Error {
    InvalidArgument,
    UnknownCommand,
    InvalidCredentials,
    WouldBlock,
    IO(std::io::Error),
    Utf8(std::str::Utf8Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}
impl From<std::str::Utf8Error> for Error {
    fn from(value: std::str::Utf8Error) -> Self {
        Error::Utf8(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
