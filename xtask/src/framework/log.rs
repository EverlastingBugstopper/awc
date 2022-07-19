const EMOJI: &str = "⌛ ";

use std::fmt::Display;

#[cfg(debug_assertions)]
use std::fmt::Debug;

pub(crate) struct Log {}

impl Log {
    /// Print info
    pub(crate) fn info(message: impl Display) {
        eprintln!("{}{}", EMOJI, message);
    }

    /// Print debug
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub(crate) fn debug(message: impl Debug) {
        eprintln!("👀 {:#?}", message);
    }
}
