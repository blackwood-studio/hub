use std::error::Error;

use cluster::logger::Logger;
use cluster::settings::Settings;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", Settings::logging_mode());
    println!("{}", Settings::host_address());

    Logger::info("This is a info");
    Logger::warning("This is a warning");
    Ok(())
}
