use std::{fmt::Display, process::Command, str};

use anyhow::{anyhow, Result};

use super::Log;

pub struct Process {
    bin: String,
    args: Vec<String>,
    description: String,
}

impl Process {
    /// Create a `Process` to run later
    pub fn new<I, S>(bin: impl Display, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let bin = bin.to_string();
        let mut description = format!("$ {}", bin);
        let args = args
            .into_iter()
            .map(|arg| {
                let arg_str = arg.as_ref();
                description.push(' ');
                description.push_str(arg_str);
                arg_str.to_string()
            })
            .collect();

        Self {
            bin,
            args,
            description,
        }
    }

    /// Run a `Process` and print its output
    pub fn run(&self, emoji: impl Display) -> Result<()> {
        Log::info(format!("{} {}", emoji, &self.description));
        let output = Command::new(&self.bin).args(&self.args).output()?;
        if let Ok(stdout) = str::from_utf8(&output.stdout) {
            for line in stdout.lines() {
                Log::info(format!("{} {}", emoji, line));
            }
        }
        if let Ok(stderr) = str::from_utf8(&output.stderr) {
            for line in stderr.lines() {
                Log::info(format!("{} {}", emoji, line));
            }
        }
        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!(
                "{} {} failed with status {}",
                emoji,
                &self.description,
                output.status
            ))
        }
    }
}
