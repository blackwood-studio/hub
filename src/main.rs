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
            let mut global = global.lock().await;
            global.streams.insert(socket_address, stream);
        }

        let global = global.clone();

        task::spawn(async move {
            let global = global.lock().await;
            let thread = Thread::new(global).await;
            let _ = thread.socket_process(socket_address).await;
        });
    }
}
