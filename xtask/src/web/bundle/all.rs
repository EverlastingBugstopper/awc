use crate::web::bundle::{
    BucketCommand, BucketOpts, CssCommand, DepsCommand, HtmlCommand, JsCommand,
};
use saucer::{prelude::*, SauceStage};

use super::HtmlCommandOpts;

#[derive(Clone, Debug, Parser)]
pub(crate) struct AllCommands {
    #[clap(long, env = "AWC_SKIP_NODE_DEPS")]
    skip_node_deps: bool,

    #[clap(flatten)]
    html_opts: HtmlCommandOpts,

    #[clap(flatten)]
    bucket_opts: BucketOpts,
}

impl Saucer for AllCommands {
    fn description(&self) -> String {
        "cargo xtask web bundle all".to_string()
    }

    /// Runs all bundle steps, parallelizing where possible
    fn run(&self) -> Result<()> {
        let total_stages = 3;
        let mut current_stage = 1;
        SauceStage::new(
            current_stage,
            total_stages,
            SauceStage::new(
                current_stage + 1,
                total_stages,
                HtmlCommand {
                    opts: self.html_opts.clone(),
                },
                BucketCommand {
                    opts: self.bucket_opts.clone(),
                },
            ),
            DepsCommand::new(),
        )
        .run()?;
        current_stage += 2;
        SauceStage::new(
            current_stage,
            total_stages,
            CssCommand::new(),
            JsCommand::new(),
        )
        .run()?;
        Ok(())
    }
}
