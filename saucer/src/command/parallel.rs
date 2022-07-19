use anyhow::Result;

use crate::{prelude::*, Timer};

use std::{fmt::Display, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ParallelSaucers<F, S>
where
    F: Saucer,
    S: Saucer,
{
    phantom_first: PhantomData<F>,
    phantom_second: PhantomData<S>,
}

impl<F, S> ParallelSaucers<F, S>
where
    F: Saucer,
    S: Saucer,
{
    /// run your two `ParallelCommand`s in parallel and aggregate any errors
    pub fn run(
        emoji: impl Display,
        description: impl Display,
        first: &F,
        second: &S,
    ) -> Result<()> {
        let timer = Timer::start();
        let results = rayon::join(|| first.run(), || second.run());
        let elapsed = timer.stop();
        match results {
            (Ok(()), Ok(())) => Ok(()),
            (Err(e), Ok(())) => Err(e).with_context(|| {
                format!(
                    "{} ❌ {} ({}) failed with 1 error in {}",
                    first.emoji(),
                    first.description(),
                    description,
                    elapsed
                )
            }),
            (Ok(()), Err(e)) => Err(e).with_context(|| {
                format!(
                    "{} ❌ {} ({}) failed with 1 error in {}",
                    second.emoji(),
                    second.description(),
                    description,
                    elapsed
                )
            }),
            (Err(first_err), Err(second_err)) => {
                Err(first_err).context(second_err).context(format!(
                    "{} ❌ '{}' failed with 2 errors in {}",
                    emoji, description, elapsed
                ))
            }
        }
    }
}
