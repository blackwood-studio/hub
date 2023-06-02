use std::error::Error;

use cluster::logger::Logger;
use cluster::settings::Settings;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", Settings::logging_mode());
    println!("{}", Settings::host_address());

    let task_01 = task::spawn(async {
        Logger::info("A").await;
    });

    let task_02 = task::spawn(async {
        Logger::info("B").await;
    });

    let task_03 = task::spawn(async {
        Logger::info("C").await;
    });

    let task_04 = task::spawn(async {
        Logger::info("D").await;
    });

    Logger::info("This is a info").await;
    Logger::warning("This is a warning").await;

    task_01.await?;
    task_02.await?;
    task_03.await?;
    task_04.await?;

    Ok(())
}
