use std::error::Error;

use hub::logger;
use hub::socket;
use hub::socket_map::SocketMap;
use hub::socket_map::SocketMapFunctions;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket_map = SocketMap::build();
    let host = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        match host.accept().await {
            Ok((socket, socket_address)) => socket::setup(socket_map.clone(), socket_address, socket).await,
            Err(_) => logger::warning("Failed to accept socket").await,
        };
    }
}
