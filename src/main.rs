use std::sync::Arc;

use anyhow::Error;
use cluster::config::Config;
use cluster::thread::Options;
use cluster::thread::Thread;
use config::Load;
use tokio::net::TcpListener;
use tokio::task;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: Config = Load::from("Config.toml")?;
    let options = Arc::new(Mutex::new(Options::new()));
    let listener = TcpListener::bind(&config.socket_address).await?;

    loop {
        let (stream, socket_address) = listener.accept().await?;
        
        {
            let mut options = options.lock().await;
            options.streams.insert(socket_address, stream);
        }

        {
            let options = Arc::clone(&options);

            task::spawn(async move {
                let options = options.lock().await;
                let thread = Thread::new(options).await;
                thread.socket_process(socket_address).await.unwrap();
            });
        }
    }
}
