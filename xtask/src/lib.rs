const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub(crate) fn relative_dir(input: &str) -> Utf8PathBuf {
    let manifest_dir = Utf8PathBuf::from_str(MANIFEST_DIR).unwrap();
    manifest_dir.parent().unwrap().join(input)
}

mod web;

use std::{env, str::FromStr};

pub use saucer::Result;
use saucer::{clap, Log, Parser, Timer, Utf8PathBuf};

use web::WebCommand;

#[derive(Debug, Parser)]
#[clap(
    name = "xtask",
    about = "Workflows used locally and in CI for developing Rover"
)]
pub struct Xtask {
    #[clap(subcommand)]
    pub crate_command: CrateCommand,
}

#[derive(Debug, Parser)]
pub enum CrateCommand {
    Web(WebCommand),
}

impl Xtask {
    pub fn run_from_args() -> Result<()> {
        Self::from_args().run()
    }

    pub fn run(&self) -> Result<()> {
        let timer = Timer::start();
        match &self.crate_command {
            CrateCommand::Web(command) => command.run(),
        }?;
        Log::info(format!("🎉 Success in {}!", timer.stop()));
        Ok(())
    }
}
