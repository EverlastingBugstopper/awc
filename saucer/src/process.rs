use std::{
    process::{Command, Stdio},
    str,
};

use anyhow::{anyhow, Context, Result};
use buildstructor::buildstructor;
use camino::Utf8PathBuf;

use super::Logger;

pub struct Process {
    bin: String,
    args: Vec<String>,
    description: String,
}

#[buildstructor]
impl Process {
    /// Create a `Process` to run later
    #[builder]
    pub fn new<I, S, B>(bin: B, args: Option<I>) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
        B: AsRef<str>,
    {
        let bin = bin.as_ref().to_string();
        which::which(&bin).with_context(|| format!("Could not find {}", &bin))?;
        let mut description = format!("$ {}", bin);
        let args = if let Some(args) = args {
            args.into_iter()
                .map(|arg| {
                    description.push(' ');
                    let arg = arg.as_ref().to_string();
                    description.push_str(&arg);
                    arg
                })
                .collect()
        } else {
            Vec::new()
        };

        Ok(Self {
            bin,
            args,
            description,
        })
    }

    /// Run a `Process` and print its output
    #[builder(entry = "runner", exit = "run")]
    pub fn run(
        &self,
        prefix: Option<String>,
        path: Option<Utf8PathBuf>,
        suppress_stderr: Option<bool>,
        suppress_stdout: Option<bool>,
    ) -> Result<()> {
        let log_stderr = !suppress_stderr.unwrap_or(false);
        let log_stdout = !suppress_stdout.unwrap_or(false);
        let prefix = prefix.unwrap_or("".to_string());
        let message = format!("{}{}", &prefix, &self.description);
        if !message.is_empty() {
            Logger::info(message);
        }
        let mut command = Command::new(&self.bin);

        if let Some(path) = path {
            command.current_dir(&path);
        }

        // Pipe to NULL is required for Windows to not hang
        // https://github.com/rust-lang/rust/issues/45572
        command
            .args(&self.args)
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        let handle = command.spawn()?;
        let output = handle.wait_with_output()?;
        if log_stderr {
            if let Ok(stderr) = str::from_utf8(&output.stderr) {
                for line in stderr.lines() {
                    eprintln!("{}{}", &prefix, &line)
                }
            }
        }
        if log_stdout {
            if let Ok(stdout) = str::from_utf8(&output.stdout) {
                for line in stdout.lines() {
                    eprintln!("{}{}", &prefix, &line)
                }
            }
        }
        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!(
                "{}{} failed with status {}",
                &prefix,
                &self.description,
                output.status
            ))
        }
    }
}

// pub struct BackgroundTask {
//     child: Child,
// }

// impl BackgroundTask {
//     pub fn new(mut command: Command) -> Result<Self> {
//         let child = command
//             .spawn()
//             .with_context(|| "Could not spawn child process")?;

//         Ok(Self { child })
//     }
// }

// impl Drop for BackgroundTask {
//     fn drop(&mut self) {
//         #[cfg(unix)]
//         {
//             // attempt to stop gracefully
//             let pid = self.child.id();
//             unsafe {
//                 libc::kill(libc::pid_t::from_ne_bytes(pid.to_ne_bytes()), libc::SIGTERM);
//             }

//             for _ in 0..10 {
//                 if self.child.try_wait().ok().flatten().is_some() {
//                     break;
//                 }
//                 std::thread::sleep(std::time::Duration::from_secs(1));
//             }
//         }

//         if self.child.try_wait().ok().flatten().is_none() {
//             // still alive? kill it with fire
//             let _ = self.child.kill();
//         }

//         let _ = self.child.wait();
//     }
// }
