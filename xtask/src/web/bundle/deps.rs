const EMOJI: &str = "⬇️  ";

use saucer::{prelude::*, Process};

#[derive(Default, Clone, Copy, Debug, Parser)]
pub(crate) struct DepsCommand {}

impl DepsCommand {
    /// Creates a new DepCommand
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl Saucer for DepsCommand {
    /// Installs node dependencies
    fn beam(&self) -> Result<()> {
        Process::new("npm", &["install"]).run(EMOJI)
    }

    fn prefix(&self) -> String {
        EMOJI.to_string()
    }

    fn description(&self) -> String {
        "installing npm dependencies".to_string()
    }
}
