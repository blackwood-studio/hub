use anyhow::Error;
use cluster::config::Config;
use cluster::thread::Global;
use cluster::thread::Thread;
use config::Load;
use tokio::net::TcpListener;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: Config = Load::from("Config.toml")?;
    let global = Global::new();
    let listener = TcpListener::bind(&config.socket_address).await?;

    loop {
        let (stream, socket_address) = listener.accept().await?;

        {
            let mut guard = global.write().await;
            guard.streams.insert(socket_address, stream);
        }

        let global = global.clone();

        task::spawn(async move {
            {
                let guard = global.read().await;
                let thread = Thread::new(guard).await;
                let _ = thread.socket_process(socket_address).await;
            }

            let mut guard = global.try_write().unwrap();
            guard.streams.remove(&socket_address);
        });
    }
}
