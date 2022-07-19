use crate::{ParallelSaucers, Saucer};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SauceStage<F, S>
where
    F: Saucer,
    S: Saucer,
{
    stage_num: usize,
    total_stages: usize,
    first: Box<F>,
    second: Box<S>,
}

impl<F, S> SauceStage<F, S>
where
    F: Saucer,
    S: Saucer,
{
    pub fn new(stage_num: usize, total_stages: usize, first: F, second: S) -> Self {
        Self {
            stage_num,
            total_stages,
            first: Box::new(first),
            second: Box::new(second),
        }
    }
}

impl<F, S> Saucer for SauceStage<F, S>
where
    F: Saucer,
    S: Saucer,
{
    fn run(&self) -> Result<()> {
        ParallelSaucers::run(
            self.emoji(),
            self.description(),
            &*self.first,
            &*self.second,
        )
    }

    fn description(&self) -> String {
        format!("stage [{}/{}]", &self.stage_num, &self.total_stages)
    }

    fn emoji(&self) -> String {
        "🪩 ".to_string()
    }
}
