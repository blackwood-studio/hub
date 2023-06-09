use std::process;

use once_cell::sync::Lazy;
use tokio::io::AsyncWriteExt;
use tokio::io::Stdout;
use tokio::io::stdout;
use tokio::sync::Mutex;

static STDOUT: Lazy<Mutex<Stdout>> = Lazy::new(|| Mutex::new(stdout()));

async fn log(bytes: &[u8]) -> () {
    let mut stdout = STDOUT.lock().await;
    let _ = stdout.write_all(bytes).await;
    let _ = stdout.flush().await;
}

pub async fn info<M>(message: M) -> ()
where M: ToString {
    let output = format!("[ INFO ] {}\n", message.to_string());
    log(output.as_bytes()).await;
}

pub async fn warning<M>(message: M) -> ()
where M: ToString {
    let output = format!("[ WARNING ] {}\n", message.to_string());
    log(output.as_bytes()).await;
}

pub async fn error<M>(message: M) -> !
where M: ToString {
    let output = format!("[ ERROR ] {}\n", message.to_string());
    log(output.as_bytes()).await;
    process::exit(-1);
}
