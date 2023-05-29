use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Error;
use tokio::net::TcpStream;
use tokio::sync::{MutexGuard, Mutex};

use crate::constants::BUFFER_SIZE;
use crate::option::ErrCast;

pub struct Options {
    pub streams: HashMap<SocketAddr, TcpStream>
}

impl Options {
    pub fn new() -> Arc<Mutex<Options>> {
        let options = Options { streams: HashMap::new() };
        let mutex = Mutex::new(options);
        Arc::new(mutex)
    }
}

pub struct Thread<'a> {
    options: MutexGuard<'a, Options>
}

impl<'a> Thread<'a> {
    pub async fn new(options: MutexGuard<'a, Options>) -> Thread<'a> {
        Thread { options }
    }

    async fn broadcast(&self, buffer: [u8; BUFFER_SIZE]) -> Result<(), Error> {
        let streams = &self.options.streams;

        for (_, stream) in streams {
            stream.try_write(&buffer)?;
        }
    
        Ok(())
    }
    
    pub async fn socket_process(&self, socket_address: SocketAddr) -> Result<(), Error> {
        let stream = self.options.streams.get(&socket_address).to_err()?;
        let mut buffer = [0; BUFFER_SIZE];
    
        loop {
            stream.readable().await?;
            let size = stream.try_read(&mut buffer)?;
            if size > 0 { self.broadcast(buffer).await? }
        }
    }
}
