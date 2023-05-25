use std::net::SocketAddr;
use std::collections::HashMap;

use anyhow::Error;
use anyhow::Ok;
use tokio::net::TcpStream;

pub struct Shared {
    streams: HashMap<SocketAddr, TcpStream>
}

impl Shared {
    pub fn new() -> Shared {
        Shared {
            streams: HashMap::new()
        }
    }

    pub fn set(&mut self, stream: TcpStream, socket_address: &SocketAddr) -> () {
        self.streams.insert(*socket_address, stream);
    }

    pub fn get(&self, socket_address: &SocketAddr) -> Result<&TcpStream, Error> {
        match self.streams.get(&socket_address) {
            Some(stream) => Ok(stream),
            None => Err(Error::msg("'TcpStream' not found"))
        }
    }

    pub fn streams(&self) -> &HashMap<SocketAddr, TcpStream> {
        &self.streams
    }
}
