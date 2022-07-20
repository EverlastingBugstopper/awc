use crate::web::bundle::{
    BucketCommand, BucketOpts, CssCommand, DepsCommand, HtmlCommand, JsCommand,
};
use saucer::{prelude::*, SauceStage};

use super::HtmlCommandOpts;

#[derive(Clone, Debug, Parser)]
pub(crate) struct AllCommands {
    #[clap(flatten)]
    pub(crate) opts: AllOpts,
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct AllOpts {
    /// skip installing node dependencies
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
        if self.opts.skip_node_deps {
            self.html_and_bucket(current_stage, total_stages).run()?;
        } else {
            self.deps_and_html_and_bucket(current_stage, total_stages)
                .run()?;
        }
        current_stage += 2;
        self.css_and_js(current_stage, total_stages).run()?;
        Ok(())
    }
}

impl AllCommands {
    fn deps_and_html_and_bucket(
        &self,
        current_stage: usize,
        total_stages: usize,
    ) -> SauceStage<SauceStage<HtmlCommand, BucketCommand>, DepsCommand> {
        SauceStage::new(
            current_stage,
            total_stages,
            self.html_and_bucket(current_stage + 1, total_stages),
            DepsCommand::new(),
        )
    }

    fn html_and_bucket(
        &self,
        current_stage: usize,
        total_stages: usize,
    ) -> SauceStage<HtmlCommand, BucketCommand> {
        SauceStage::new(
            current_stage,
            total_stages,
            HtmlCommand {
                opts: self.opts.html_opts.clone(),
            },
            BucketCommand {
                opts: self.opts.bucket_opts.clone(),
            },
        )
    }

    fn css_and_js(
        &self,
        current_stage: usize,
        total_stages: usize,
    ) -> SauceStage<CssCommand, JsCommand> {
        SauceStage::new(
            current_stage,
            total_stages,
            CssCommand::new(),
            JsCommand::new(),
        )
    }
}
