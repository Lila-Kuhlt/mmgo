use std::net::SocketAddr;

pub(crate) type Position = (u32, u32);

#[derive(Debug, Default, Clone)]
pub(crate) struct Board;

impl Board {
    pub(crate) fn place(&mut self, x: u32, y: u32, addr: SocketAddr) {
        todo!()
    }

    pub(crate) fn serialize(&self) -> String {
        format!("BOARD")
    }
}

pub enum GoError {
    OutOfBounds,
    Suicide,
}
