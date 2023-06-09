use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub type SocketMap = Arc<Mutex<HashMap<SocketAddr, Arc<TcpStream>>>>;

pub trait SocketMapFunctions {
    fn build() -> SocketMap;
}

impl SocketMapFunctions for SocketMap {
    fn build() -> SocketMap {
        let hash_map = HashMap::new();
        let mutex = Mutex::new(hash_map);
        let arc = Arc::new(mutex);
        arc
    }
}
