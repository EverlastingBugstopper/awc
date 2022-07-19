mod stage;
mod step;

use std::fmt::Debug;

pub(crate) use stage::*;
pub(crate) use step::*;

use crate::framework::prelude::*;

/// Types that implement this trait can be run in stages
pub trait Command
where
    Self: Sync + Clone + Debug + 'static,
{
    /// Create the function that will be run for this step
    fn runner(&'static self) -> Box<(dyn Fn() -> Result<()> + Send + Sync + 'static)> {
        Box::new(|| self.run())
    }

    /// The function that a `Command` runs
    fn run(&self) -> Result<()>;

    /// the emoji for a `Command`
    fn emoji(&self) -> String {
        "".to_string()
    }

    /// the description for a `Command`
    fn description(&self) -> String;
}
