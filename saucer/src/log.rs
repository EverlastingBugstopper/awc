#[cfg(debug_assertions)]
const DEBUG_EMOJI: &str = "👀 ";

const ERROR_EMOJI: &str = "❌ ";

use std::fmt::Display;

#[cfg(debug_assertions)]
use std::fmt::Debug;

use anyhow::{anyhow, Error};

/// Log information to stderr
pub struct Log {}

impl Log {
    /// eprint info
    pub fn info(message: impl Display) {
        let lines: Vec<String> = message.to_string().lines().map(|l| l.to_string()).collect();
        let num_lines = lines.len();
        if num_lines > 1 {
            eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
        }
        for line in lines {
            eprintln!("{}", line);
        }
        if num_lines > 1 {
            eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        }
    }

    /// eprint error
    pub fn error(message: impl Display, original_error: Option<Error>) {
        let err = if let Some(original_error) = original_error {
            anyhow!("{}", message).context(original_error)
        } else {
            anyhow!("{}", message)
        };
        eprintln!("{}{:?}", ERROR_EMOJI, err);
    }

    /// print info
    pub fn stdout(message: impl Display) {
        println!("{}", message);
    }

    /// eprint debug
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub fn debug(message: impl Debug) {
        eprintln!("{}{:#?}", DEBUG_EMOJI, message);
    }
}
