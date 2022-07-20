mod bundle;

use bundle::{AllCommands, AllOpts, BundleCommand};

use saucer::prelude::*;
use std::fmt::Debug;

/// Run the build step for `awc-web`
#[derive(Debug, Parser)]
pub struct WebCommand {
    /// Build `awc-web`
    #[clap(subcommand)]
    web_command: Option<WebCommands>,

    #[clap(flatten)]
    all_opts: AllOpts,
}

#[derive(Debug, Clone, Subcommand)]
enum WebCommands {
    /// Bundle source code for front-end
    Bundle(BundleCommand),
}

impl WebCommand {
    pub(crate) fn run(&self) -> Result<()> {
        match &self.web_command {
            Some(WebCommands::Bundle(command)) => command.run(),
            None => AllCommands {
                opts: self.all_opts.clone(),
            }
            .beam(),
        }
    }
}
