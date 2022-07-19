mod lint;
use lint::LintCommand;

pub use saucer::{Parser, Result};

#[derive(Debug, Parser)]
#[clap(name = "awc", author, version)]
pub struct AwcCli {
    #[clap(subcommand)]
    awc_command: AwcCommand,
}

impl AwcCli {
    pub fn run_from_args() -> Result<()> {
        Self::from_args().run()
    }

    pub fn run(&self) -> Result<()> {
        self.awc_command.run()
    }
}

#[derive(Parser, Debug)]
pub enum AwcCommand {
    /// Lint a GraphQL schema
    Lint(LintCommand),
}

impl AwcCommand {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Lint(command) => command.run(),
        }
    }
}
