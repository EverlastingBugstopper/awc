const EMOJI: &str = "⬇️ ";

use clap::Parser;

use crate::framework::{prelude::*, Process};

#[derive(Default, Clone, Copy, Debug, Parser)]
pub(crate) struct DepsCommand {}

impl DepsCommand {
    /// Creates a new DepCommand
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl Command for DepsCommand {
    /// Installs node dependencies
    fn run(&self) -> Result<()> {
        Process::new("npm", &["install"]).run(EMOJI)
    }

    fn emoji(&self) -> String {
        EMOJI.to_string()
    }

    fn description(&self) -> String {
        "installing npm dependencies".to_string()
    }
}
