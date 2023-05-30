use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Error;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;

use crate::constants::BUFFER_SIZE;
use crate::option::ErrCast;

pub struct Global {
    pub streams: HashMap<SocketAddr, TcpStream>
}

impl Global {
    pub fn new() -> Arc<RwLock<Global>> {
        let global = Global { streams: HashMap::new() };
        let rw_lock = RwLock::new(global);
        Arc::new(rw_lock)
    }
}

pub struct Thread<'a> {
    global: RwLockReadGuard<'a, Global>
}

impl<'a> Thread<'a> {
    pub async fn new(global: RwLockReadGuard<'a, Global>) -> Thread<'a> {
        Thread { global }
    }

    async fn broadcast(&self, buffer: [u8; BUFFER_SIZE]) -> Result<(), Error> {
        let streams = &self.global.streams;

        for (_, stream) in streams {
            match stream.peer_addr() {
                Ok(_) => stream.try_write(&buffer)?,
                Err(_) => 0
            };
        }
    
        Ok(())
    }
    
    pub async fn socket_process(&self, socket_address: SocketAddr) -> Result<(), Error> {
        let stream = self.global.streams.get(&socket_address).to_err()?;

        loop {
            let mut buffer = [0; BUFFER_SIZE];
            stream.readable().await?;
            
            match stream.try_read(&mut buffer) {
                Ok(0) => break Ok(()),
                Ok(_) => self.broadcast(buffer).await?,
                Err(_) => self.broadcast(buffer).await?,
            };
        }
    }
}
