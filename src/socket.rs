use std::net::SocketAddr;
use std::sync::Arc;

use crate::logger;
use crate::socket_map::SocketMap;
use tokio::net::TcpStream;
use tokio::task;

const BUFFER_SIZE: usize = 1024;

async fn remove_socket(socket_map: SocketMap, socket_address: SocketAddr) -> () {
    let mut socket_map = socket_map.lock().await;

    logger::info(format!("Socket '{}' left", socket_address)).await;

    socket_map.remove(&socket_address);
}

async fn broadcast(socket_map: SocketMap, socket_address: SocketAddr, buffer: [u8; BUFFER_SIZE]) -> () {
    let socket_map = socket_map.lock().await;

    logger::info(format!("Got message from '{}'", socket_address)).await;

    for (socket_address, socket) in socket_map.iter() {
        match socket.try_write(&buffer) {
            Ok(_) => logger::info(format!("Sent message to '{}'", socket_address)).await,
            Err(_) => logger::warning(format!("Sending message to '{}' failed", socket_address)).await,
        };
    }
}

async fn lifecycle(socket_map: SocketMap, socket_address: SocketAddr, socket: Arc<TcpStream>) -> () {
    loop {
        let mut buffer = [0; BUFFER_SIZE];

        match socket.readable().await {
            Err(_) => logger::warning("Socket failed to be readable ").await,
            Ok(_) => (),
        };

        match socket.try_read(&mut buffer) {
            Ok(0) => return remove_socket(socket_map.clone(), socket_address).await,
            Ok(_) => broadcast(socket_map.clone(), socket_address, buffer).await,
            Err(_) => (),
        };
    }
}

pub async fn setup(socket_map: SocketMap, socket_address: SocketAddr, socket: TcpStream) -> () {
    let socket = Arc::new(socket);

    {
        let mut socket_map = socket_map.lock().await;
        socket_map.insert(socket_address, socket.clone());
    }

    logger::info(format!("Got new socket '{}'", socket_address)).await;

    task::spawn(lifecycle(socket_map.clone(), socket_address, socket));
}
