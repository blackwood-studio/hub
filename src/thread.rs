use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Error;
use tokio::net::TcpStream;
use tokio::sync::{MutexGuard, Mutex};

use crate::constants::BUFFER_SIZE;
use crate::option::ErrCast;

pub struct Global {
    pub streams: HashMap<SocketAddr, TcpStream>
}

impl Global {
    pub fn new() -> Arc<Mutex<Global>> {
        let global = Global { streams: HashMap::new() };
        let mutex = Mutex::new(global);
        Arc::new(mutex)
    }
}

pub struct Thread<'a> {
    global: MutexGuard<'a, Global>
}

impl<'a> Thread<'a> {
    pub async fn new(global: MutexGuard<'a, Global>) -> Thread<'a> {
        Thread { global }
    }

    async fn broadcast(&self, buffer: [u8; BUFFER_SIZE]) -> Result<(), Error> {
        let streams = &self.global.streams;

        for (_, stream) in streams {
            stream.try_write(&buffer)?;
        }
    
        Ok(())
    }
    
    pub async fn socket_process(&self, socket_address: SocketAddr) -> Result<(), Error> {
        let stream = self.global.streams.get(&socket_address).to_err()?;
        let mut buffer = [0; BUFFER_SIZE];
    
        loop {
            stream.readable().await?;
            let size = stream.try_read(&mut buffer)?;
            if size > 0 { self.broadcast(buffer).await? }
        }
    }
}
