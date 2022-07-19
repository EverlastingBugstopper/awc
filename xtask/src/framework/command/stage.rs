use crate::framework::{Command, ParallelCommands};

use anyhow::Result;

#[derive(Debug, Clone)]
pub(crate) struct Stage<F, S>
where
    F: Command,
    S: Command,
{
    stage_num: usize,
    total_stages: usize,
    first: Box<F>,
    second: Box<S>,
}

impl<F, S> Stage<F, S>
where
    F: Command,
    S: Command,
{
    pub(crate) fn new(stage_num: usize, total_stages: usize, first: F, second: S) -> Self {
        Self {
            stage_num,
            total_stages,
            first: Box::new(first),
            second: Box::new(second),
        }
    }
}

impl<F, S> Command for Stage<F, S>
where
    F: Command,
    S: Command,
{
    fn run(&self) -> Result<()> {
        ParallelCommands::run(
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
