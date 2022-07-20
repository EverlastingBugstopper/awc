mod parallel;

use std::fmt::Debug;

pub use parallel::*;

use crate::prelude::*;

/// Types that implement this trait can be run in parallel
/// if used in combination with `ParallelSaucer`
pub trait Saucer
where
    Self: Sync + Clone + Debug + 'static,
{
    /// Create the function that will be run for this step
    fn runner(&'static self) -> Box<(dyn Fn() -> Result<()> + Send + Sync + 'static)> {
        Box::new(|| self.beam())
    }

    /// The function that a `Saucer` runs
    fn beam(&self) -> Result<()>;

    /// the prefix for logs printed by a `Saucer`
    fn prefix(&self) -> String {
        "".to_string()
    }

    /// the description for a `Saucer`
    fn description(&self) -> String;
}

/// A no-op saucer. This is helpful when creating `ParallelSaucer`s.
#[derive(Debug, Clone, Default)]
pub struct EmptySaucer {}

impl EmptySaucer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Saucer for EmptySaucer {
    fn description(&self) -> String {
        "".to_string()
    }

    fn beam(&self) -> Result<()> {
        Ok(())
    }
}
