use hub::logger;
use hub::socket;
use hub::socket_map::SocketMap;
use hub::socket_map::SocketMapFunctions;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> () {
    let socket_map = SocketMap::build();
    let host = match TcpListener::bind("127.0.0.1:8080").await {
        Ok(host) => host,
        Err(_) => logger::error("Failed to setup tcp listener").await,
    };

    loop {
        match host.accept().await {
            Ok((socket, socket_address)) => socket::setup(socket_map.clone(), socket_address, socket).await,
            Err(_) => logger::warning("Failed to accept socket").await,
        };
    }
}
