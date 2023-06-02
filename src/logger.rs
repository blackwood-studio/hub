use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::process;
use std::str::FromStr;

use once_cell::sync::Lazy;
use tokio::io::AsyncWriteExt;
use tokio::io::Stdout;
use tokio::io::stdout;
use tokio::sync::Mutex;

use crate::settings::Settings;

pub enum Mode {
    NONE,
    ALL,
    WARNING
}

impl Copy for Mode {  }

impl Clone for Mode {
    fn clone(&self) -> Self {
        match self {
            Self::NONE => Self::NONE,
            Self::ALL => Self::ALL,
            Self::WARNING => Self::WARNING,
        }
    }
}

impl Debug for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => write!(f, "NONE"),
            Self::ALL => write!(f, "ALL"),
            Self::WARNING => write!(f, "WARNING"),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => write!(f, "NONE"),
            Self::ALL => write!(f, "ALL"),
            Self::WARNING => write!(f, "WARNING"),
        }
    }
}

pub enum ModeError {
    InvalidString
}

impl Debug for ModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModeError::InvalidString => write!(f, "'logger::Mode' does only support 'NONE', 'ALL' and 'WARNING'")
        }
    }
}

impl Display for ModeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModeError::InvalidString => write!(f, "'logger::Mode' does only support 'NONE', 'ALL' and 'WARNING'")
        }
    }
}

impl Error for ModeError {  }

impl FromStr for Mode {
    type Err = ModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NONE" => Ok(Mode::NONE),
            "ALL" => Ok(Mode::ALL),
            "WARNING" => Ok(Mode::WARNING),
            _ => Err(ModeError::InvalidString)
        }
    }
}

static STDOUT: Lazy<Mutex<Stdout>> = Lazy::new(|| Mutex::new(stdout()));

pub struct Logger {  }

impl Logger {
    pub async fn info<M>(message: M) -> ()
    where M: ToString{
        match Settings::logging_mode() {
            Mode::NONE => return,
            Mode::ALL => (),
            Mode::WARNING => return,
        }

        let output = format!("[ INFO ] {}\n", message.to_string());
        let mut stdout = STDOUT.lock().await;
        let _ = stdout.write_all(output.as_bytes()).await;
        let _ = stdout.flush().await;
    }

    pub async fn warning<M>(message: M) -> ()
    where M: ToString {
        match Settings::logging_mode() {
            Mode::NONE => return,
            Mode::ALL => (),
            Mode::WARNING => (),
        }

        let output = format!("[ WARNING ] {}\n", message.to_string());
        let mut stdout = STDOUT.lock().await;
        let _ = stdout.write_all(output.as_bytes()).await;
        let _ = stdout.flush().await;
    }

    pub async fn error<M>(message: M) -> !
    where M: ToString {
        let output = format!("[ ERROR ] {}\n", message.to_string());
        let mut stdout = STDOUT.lock().await;
        let _ = stdout.write_all(output.as_bytes()).await;
        let _ = stdout.flush().await;
        process::exit(-1);
    }
}
