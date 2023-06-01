use std::env;
use std::io;
use std::io::Write;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::process;

use once_cell::sync::Lazy;

use crate::logger::Mode;

struct Logger {  }

impl Logger {
    fn warning<M>(message: M) -> ()
    where M: ToString {
        let output = format!("[ WARNING ] {}\n", message.to_string());
        let mut stdout = io::stdout().lock();
        let _ = stdout.write_all(output.as_bytes());
        let _ = stdout.flush();
    }

    fn error<M>(message: M) -> !
    where M: ToString {
        let output = format!("[ ERROR ] {}\n", message.to_string());
        let mut stdout = io::stdout().lock();
        let _ = stdout.write_all(output.as_bytes());
        let _ = stdout.flush();
        process::exit(-1);
    }
}

pub struct Settings {
    logging_mode: Mode,
    host_address: SocketAddr,
}

fn update_logging_mode(settings: &mut Settings, logging_mode: &str) -> () {
    let logging_mode = logging_mode.parse();

    match logging_mode {
        Ok(logging_mode) => settings.logging_mode = logging_mode,
        Err(_) => Logger::error("'logger::Mode' does only support 'NONE', 'ALL' and 'WARNING'")
    };
}

fn update_host_ip(settings: &mut Settings, host_ip: &str) -> () {
    let host_ip = host_ip.parse();

    match host_ip {
        Ok(host_ip) => settings.host_address.set_ip(host_ip),
        Err(_) => Logger::error("'host_ip' must be a valid ip address")
    };
}

fn update_host_port(settings: &mut Settings, host_port: &str) -> () {
    let host_port = host_port.parse();

    match host_port {
        Ok(host_port) => settings.host_address.set_port(host_port),
        Err(_) => Logger::error("'host_port' must be a valid port")
    };
}

static SETTINGS: Lazy<Settings> = Lazy::new(|| {
    let mut settings = Settings {
        logging_mode: Mode::NONE,
        host_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
    };

    let mut arguments: Vec<String> = env::args().collect();

    arguments.remove(0);

    for argument in arguments {
        let splits = argument.split("=");
        let vector: Vec<&str> = splits.collect();

        match vector[0] {
            "logging_mode" => update_logging_mode(&mut settings, vector[1]),
            "host_ip" => update_host_ip(&mut settings, vector[1]),
            "host_port" => update_host_port(&mut settings, vector[1]),
            setting => Logger::warning(format!("Setting '{}' does not exist", setting))
        };
    }

    settings
});

impl Settings {
    pub fn logging_mode() -> Mode {
        SETTINGS.logging_mode
    }

    pub fn host_address() -> SocketAddr {
        SETTINGS.host_address
    }
}
