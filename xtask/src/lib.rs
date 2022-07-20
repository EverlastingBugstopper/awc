const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub(crate) fn relative_dir(input: &str) -> Utf8PathBuf {
    let manifest_dir = Utf8PathBuf::from_str(MANIFEST_DIR).unwrap();
    let parent = manifest_dir.parent().unwrap();
    parent
        .join(input)
        .strip_prefix(parent)
        .unwrap()
        .to_path_buf()
}

mod web;

use std::{env, str::FromStr};

pub use saucer::Result;
use saucer::{prelude::*, Log, Timer, Utf8PathBuf};

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

#[derive(Subcommand, Debug)]
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
