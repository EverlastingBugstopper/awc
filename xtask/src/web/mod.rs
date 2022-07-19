mod bundle;

use bundle::BundleCommand;

use crate::framework::prelude::*;
use camino::Utf8PathBuf;
use std::fmt::Debug;

#[derive(Debug, Parser)]
pub struct WebCommand {
    /// Build `awc-web`
    #[clap(subcommand)]
    web_command: WebCommands,

    /// Path to an `awc.json` handlebars file.
    ///
    /// https://docs.rs/handlebars/latest/handlebars/
    #[clap(long, env = "AWC_CONFIG")]
    awc_config: Option<Utf8PathBuf>,
}

#[derive(Debug, Clone, Parser)]
enum WebCommands {
    /// Bundle source code for front-end
    Bundle(BundleCommand),
}

impl WebCommand {
    pub(crate) fn run(&self) -> Result<()> {
        match &self.web_command {
            WebCommands::Bundle(command) => command.run()?,
        }
        Ok(())
    }
}
