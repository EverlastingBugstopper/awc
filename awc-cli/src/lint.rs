const READ_EMOJI: &str = "📚 ";

use std::{
    io::{self, Read},
    sync::mpsc::channel,
    time::Duration,
};

use awc::{AwcCompiler, AwcResult};
use saucer::{anyhow, Fs, Logger, Parser, Result};

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

#[derive(Debug, Parser)]
pub struct LintCommand {
    /// The GraphQL file to read from.
    ///
    /// If set to "-", it will be read from stdin.
    #[clap(long)]
    schema: String,

    /// Provides machine readable output.
    #[clap(long)]
    json: bool,

    /// Enable file watching for your schema.
    ///
    /// This option is incompatible with `--schema -`.
    #[clap(long)]
    watch: bool,

    /// Configures whether to fail if there are validation warnings.
    #[clap(long)]
    strict: bool,
}

impl LintCommand {
    pub fn run(&self) -> Result<()> {
        if !self.watch {
            let (proposed_schema, _) = self.get_schema_and_maybe_path()?;
            self.print_lint(&proposed_schema);
            Ok(())
        } else {
            self.lint_and_watch()
        }
    }

    fn get_schema_and_maybe_path(&self) -> Result<(String, Option<String>)> {
        match &*self.schema {
            "" => Err(anyhow!("input was an empty string")),
            "-" => {
                let mut buffer = String::new();
                match io::stdin().read_to_string(&mut buffer) {
                    Ok(_) => Ok((buffer, None)),
                    Err(e) => Err(anyhow!("unable to read GraphQL from stdin: {}", e)),
                }
            }
            path => {
                let contents = Fs::read_file(&path, READ_EMOJI)?;
                Ok((contents, Some(path.to_string())))
            }
        }
    }

    fn lint(&self, proposed_schema: &str) -> AwcResult {
        AwcCompiler::new(&proposed_schema, self.strict).run()
    }

    fn print_lint(&self, proposed_schema: &str) {
        let diagnostics = self.lint(proposed_schema);
        if self.json {
            Logger::stdout(diagnostics.json())
        } else {
            Logger::info(diagnostics.pretty())
        }
    }

    fn lint_and_watch(&self) -> Result<()> {
        let (proposed_schema, maybe_path) = self.get_schema_and_maybe_path()?;

        if let Some(path) = maybe_path {
            self.print_lint(&proposed_schema);

            let (broadcaster, listener) = channel();
            let mut watcher = watcher(broadcaster, Duration::from_secs(1))?;
            watcher.watch(&path, RecursiveMode::NonRecursive)?;

            Logger::info(format!("👀 Watching {} for changes", path));
            loop {
                match listener.recv() {
                    Ok(event) => match &event {
                        DebouncedEvent::NoticeWrite(_) => {
                            Logger::info(format!("🔃 Change detected in {}", &path))
                        }
                        DebouncedEvent::Write(_) => {
                            match Fs::read_file(&path, READ_EMOJI) {
                                Ok(contents) => self.print_lint(&contents),
                                Err(e) => {
                                    Logger::error(
                                        format!("Could not read {} from disk", &path),
                                        Some(anyhow!("{}", e)),
                                    );
                                }
                            };
                        }
                        DebouncedEvent::Error(e, _) => {
                            Logger::error(
                                format!("unknown error while watching {}", &path),
                                Some(anyhow!("{}", e)),
                            );
                        }
                        _ => {}
                    },
                    Err(e) => Logger::error(
                        format!("unknown error while watching {}", &path),
                        Some(anyhow!(e)),
                    ),
                }
            }
        } else {
            Err(anyhow!(
                "You cannot combine the `--watch` flag with the `--schema -` argument."
            ))
        }
    }
}
