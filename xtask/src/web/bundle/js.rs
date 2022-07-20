const EMOJI: &str = "🧳 ";

use saucer::{prelude::*, Process};

#[derive(Default, Clone, Copy, Debug, Parser)]
pub(crate) struct JsCommand {}

impl JsCommand {
    /// Creates a new JsCommand
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl Saucer for JsCommand {
    /// Transpiles TypeScript source to minified JavaScript
    fn beam(&self) -> Result<()> {
        Process::new("npm", &["run", "build:js"]).run(EMOJI)
    }

    fn prefix(&self) -> String {
        EMOJI.to_string()
    }

    fn description(&self) -> String {
        "webpack/swc".to_string()
    }
}
