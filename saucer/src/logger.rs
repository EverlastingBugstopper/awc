#[cfg(debug_assertions)]
const DEBUG_EMOJI: &str = "👀 ";

const ERROR_EMOJI: &str = "❌ ";

use std::fmt::Display;

#[cfg(debug_assertions)]
use std::fmt::Debug;

use anyhow::{anyhow, Error};

/// Log information to stderr
pub struct Logger {}

impl Logger {
    /// log info
    pub fn info(message: impl Display) {
        log::info!("{}", message)
    }

    /// log debug
    #[cfg(debug_assertions)]
    pub fn debug(message: impl Debug) {
        log::debug!("{:?}", message)
    }

    /// log an error with a message
    pub fn error(message: impl Display, original_error: Option<Error>) {
        let err = if let Some(original_error) = original_error {
            anyhow!("{}", message).context(original_error)
        } else {
            anyhow!("{}", message)
        };
        log::error!("{}{:?}", ERROR_EMOJI, err);
    }

    /// print info
    pub fn stdout(message: impl Display) {
        println!("{}", message);
    }

    /// eprint debug
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub fn quick_debug(message: impl Debug) {
        eprintln!("{}{:#?}", DEBUG_EMOJI, message);
    }
}
