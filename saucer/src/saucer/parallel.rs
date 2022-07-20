use crate::{Context, Logger, Result, Saucer, Timer};

#[derive(Debug, Clone)]
pub struct ParallelSaucer<F, S>
where
    F: Saucer,
    S: Saucer,
{
    stage_num: usize,
    total_stages: usize,
    prefix: String,
    first: Box<F>,
    second: Box<S>,
}

impl<F, S> ParallelSaucer<F, S>
where
    F: Saucer,
    S: Saucer,
{
    pub fn new(first: F, second: S, prefix: &str, stage_num: usize, total_stages: usize) -> Self {
        Self {
            stage_num,
            total_stages,
            prefix: prefix.to_string(),
            first: Box::new(first),
            second: Box::new(second),
        }
    }
}

impl<F, S> Saucer for ParallelSaucer<F, S>
where
    F: Saucer,
    S: Saucer,
{
    fn beam(&self) -> Result<()> {
        let timer = Timer::start();
        self.join()?;
        let elapsed = timer.stop();
        Logger::info(format!(
            "{}{} completed in {}",
            self.prefix(),
            self.description(),
            &elapsed
        ));
        Ok(())
    }

    fn description(&self) -> String {
        let mut first = format!("{}{}", self.first.prefix(), self.first.description());
        let mut second = format!("{}{}", self.second.prefix(), self.second.description());

        let is_first_par_saucer = first.contains(&self.prefix);
        let is_second_par_saucer = second.contains(&self.prefix);

        if is_first_par_saucer {
            first = "".to_string();
        }

        if is_second_par_saucer {
            second = "".to_string();
        }

        match (first.as_str(), second.as_str()) {
            ("", "") => self.prefix(),
            ("", second_desc) => format!("{}", second_desc),
            (first_desc, "") => format!("{}", first_desc),
            (first_desc, second_desc) => {
                format!("{} & {}", first_desc, second_desc)
            }
        }
    }

    fn prefix(&self) -> String {
        format!(
            "{}[{}/{}] ",
            &self.prefix, &self.stage_num, &self.total_stages
        )
    }
}

impl<F, S> ParallelSaucer<F, S>
where
    F: Saucer,
    S: Saucer,
{
    /// run your two `ParallelSaucer`s in parallel and aggregate any errors
    pub fn join(&self) -> Result<()> {
        let timer = Timer::start();
        let results = rayon::join(|| self.first.beam(), || self.second.beam());
        let elapsed = timer.stop();
        match results {
            (Ok(()), Ok(())) => Ok(()),
            (Err(e), Ok(())) => Err(e).with_context(|| {
                format!(
                    "{}{}❌ {} ({}) failed with 1 error in {}",
                    self.prefix(),
                    self.first.prefix(),
                    self.first.description(),
                    self.description(),
                    elapsed
                )
            }),
            (Ok(()), Err(e)) => Err(e).with_context(|| {
                format!(
                    "{}{}❌ {} ({}) failed with 1 error in {}",
                    self.prefix(),
                    self.second.prefix(),
                    self.second.description(),
                    self.description(),
                    elapsed
                )
            }),
            (Err(first_err), Err(second_err)) => {
                Err(first_err).context(second_err).context(format!(
                    "{}{}{}❌ '{}' failed with 2 errors in {}",
                    self.prefix(),
                    self.first.prefix(),
                    self.second.prefix(),
                    self.description(),
                    elapsed
                ))
            }
        }
    }
}
