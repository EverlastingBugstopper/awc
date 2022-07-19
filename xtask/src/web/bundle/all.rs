use crate::{
    framework::{prelude::*, Log, Stage, Timer},
    web::bundle::{CssCommand, DepsCommand, HtmlCommand, JsCommand},
};

use super::HtmlCommandOpts;

#[derive(Clone, Debug, Parser)]
pub(crate) struct AllCommands {
    #[clap(long, env = "AWC_SKIP_NODE_DEPS")]
    skip_node_deps: bool,

    #[clap(flatten)]
    html_opts: HtmlCommandOpts,
}

impl Command for AllCommands {
    fn description(&self) -> String {
        "cargo xtask web bundle all".to_string()
    }

    /// Runs all bundle steps, parallelizing where possible
    fn run(&self) -> Result<()> {
        let total_stages = 2;
        let mut current_stage = 1;
        let timer = Timer::start();
        Stage::new(
            current_stage,
            total_stages,
            DepsCommand::new(),
            HtmlCommand {
                opts: self.html_opts.clone(),
            },
        )
        .run()?;
        let elapsed = timer.stop();
        Log::info(format!(
            "stage [{}/{}] completed in {}",
            current_stage, total_stages, elapsed
        ));
        current_stage += 1;
        let timer = Timer::start();
        Stage::new(
            current_stage,
            total_stages,
            CssCommand::new(),
            JsCommand::new(),
        )
        .run()?;
        let elapsed = timer.stop();
        Log::info(format!(
            "stage [{}/{}] completed in {}",
            current_stage, total_stages, elapsed
        ));
        Ok(())
    }
}
