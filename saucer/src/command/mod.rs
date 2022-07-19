mod parallel;
mod stage;

use std::fmt::Debug;

pub use parallel::*;
pub use stage::*;

use crate::prelude::*;

/// Types that implement this trait can be run in stages
pub trait Saucer
where
    Self: Sync + Clone + Debug + 'static,
{
    /// Create the function that will be run for this step
    fn runner(&'static self) -> Box<(dyn Fn() -> Result<()> + Send + Sync + 'static)> {
        Box::new(|| self.run())
    }

    /// The function that a `Saucer` runs
    fn run(&self) -> Result<()>;

    /// the emoji for a `Saucer`
    fn emoji(&self) -> String {
        "".to_string()
    }

    /// the description for a `Saucer`
    fn description(&self) -> String;
}
