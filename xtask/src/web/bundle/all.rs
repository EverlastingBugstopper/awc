const STAGE_PREFIX: &str = "🛸 stage ";

use crate::web::bundle::{
    BucketCommand, BucketOpts, CssCommand, DepsCommand, HtmlCommand, JsCommand,
};
use saucer::{prelude::*, ParallelSaucer};

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
    fn beam(&self) -> Result<()> {
        let total_stages = 3;
        if self.opts.skip_node_deps {
            self.html_and_bucket(1, total_stages).beam()?;
        } else {
            self.deps_and_html_and_bucket(1, total_stages).beam()?;
        }
        self.css_and_js(3, total_stages).beam()?;
        Ok(())
    }

    fn prefix(&self) -> String {
        STAGE_PREFIX.to_string()
    }
}

impl AllCommands {
    fn deps_and_html_and_bucket(
        &self,
        current_stage: usize,
        total_stages: usize,
    ) -> ParallelSaucer<ParallelSaucer<HtmlCommand, BucketCommand>, DepsCommand> {
        ParallelSaucer::new(
            self.html_and_bucket(current_stage, total_stages),
            DepsCommand::new(),
            &self.prefix(),
            current_stage + 1,
            total_stages,
        )
    }

    fn html_and_bucket(
        &self,
        current_stage: usize,
        total_stages: usize,
    ) -> ParallelSaucer<HtmlCommand, BucketCommand> {
        ParallelSaucer::new(
            HtmlCommand {
                opts: self.opts.html_opts.clone(),
            },
            BucketCommand {
                opts: self.opts.bucket_opts.clone(),
            },
            &self.prefix(),
            current_stage,
            total_stages,
        )
    }

    fn css_and_js(
        &self,
        current_stage: usize,
        total_stages: usize,
    ) -> ParallelSaucer<CssCommand, JsCommand> {
        ParallelSaucer::new(
            CssCommand::new(),
            JsCommand::new(),
            &self.prefix(),
            current_stage,
            total_stages,
        )
    }
}
