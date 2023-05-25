use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Error;
use anyhow::Ok;
use cluster::shared::Shared;
use config::Load;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;
use tokio::task;

const BUFFER_SIZE: usize = 4096;

fn default_socket_address() -> String { String::from("127.0.0.1:3000") }

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default = "default_socket_address")]
    socket_address: String
}

fn broadcast(shared: &MutexGuard<Shared>, buffer: [u8; BUFFER_SIZE]) -> Result<(), Error> {
    for (_, stream) in shared.streams() {
        stream.try_write(&buffer)?;
    }

    Ok(())
}

async fn socket_process(shared: Arc<Mutex<Shared>>, socket_address: SocketAddr) -> Result<(), Error> {
    let shared = shared.try_lock()?;
    let stream = shared.get(&socket_address)?;
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        stream.readable().await?;
        let size = stream.try_read(&mut buffer)?;
        if size > 0 { broadcast(&shared, buffer)? }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: Config = Load::from("Config.toml")?;
    let shared = Arc::new(Mutex::new(Shared::new()));
    let listener = TcpListener::bind(&config.socket_address).await?;

    loop {
        let (stream, socket_address) = listener.accept().await?;
        let shared = Arc::clone(&shared);
        shared.try_lock()?.set(stream, &socket_address);
        task::spawn(socket_process(shared, socket_address));
    }
}
