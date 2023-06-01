use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::io;
use std::io::Write;
use std::process;
use std::str::FromStr;

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

pub struct Logger {  }

impl Logger {
    pub fn info<M>(message: M) -> ()
    where M: ToString{
        match Settings::logging_mode() {
            Mode::NONE => return,
            Mode::ALL => (),
            Mode::WARNING => return,
        }

        let output = format!("[ INFO ] {}\n", message.to_string());
        let mut stdout = io::stdout().lock();
        let _ = stdout.write_all(output.as_bytes());
        let _ = stdout.flush();
    }

    pub fn warning<M>(message: M) -> ()
    where M: ToString {
        match Settings::logging_mode() {
            Mode::NONE => return,
            Mode::ALL => (),
            Mode::WARNING => (),
        }

        let output = format!("[ WARNING ] {}\n", message.to_string());
        let mut stdout = io::stdout().lock();
        let _ = stdout.write_all(output.as_bytes());
        let _ = stdout.flush();
    }

    pub fn error<M>(message: M) -> !
    where M: ToString {
        let output = format!("[ ERROR ] {}\n", message.to_string());
        let mut stdout = io::stdout().lock();
        let _ = stdout.write_all(output.as_bytes());
        let _ = stdout.flush();
        process::exit(-1);
    }
}
