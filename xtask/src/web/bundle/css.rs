const EMOJI: &str = "💅 ";

use clap::Parser;

use saucer::{prelude::*, Process};

#[derive(Default, Clone, Copy, Debug, Parser)]
pub(crate) struct CssCommand {}

impl CssCommand {
    /// Creates a new CssCommand
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl Saucer for CssCommand {
    /// Runs tailwind to generate only the CSS we need
    fn run(&self) -> Result<()> {
        Process::new("npm", &["run", "build:css"]).run(EMOJI)
    }

    fn emoji(&self) -> String {
        EMOJI.to_string()
    }

    fn description(&self) -> String {
        "tailwindcss".to_string()
    }
}
