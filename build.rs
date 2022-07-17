use std::{env, fmt::Display, io, process::Command, str};

fn main() -> Result<(), io::Error> {
    if env::var_os("CARGO_FEATURE_HTTP").is_some() {
        Npm::run_script("build")?;
    }
    Ok(())
}

struct Npm {}

impl Npm {
    fn run_script(script: &str) -> Result<(), io::Error> {
        Printer::warn(format!("$ npm run {}", script));
        Runner::run(Command::new("npm").args(&["run", script]))?;
        Ok(())
    }
}

struct Runner {}

impl Runner {
    fn run(command: &mut Command) -> Result<(), io::Error> {
        let output = command.output()?;
        if let Ok(stdout) = str::from_utf8(&output.stdout) {
            Printer::warn(stdout);
        }
        if let Ok(stderr) = str::from_utf8(&output.stderr) {
            Printer::warn(stderr);
        }
        Ok(())
    }
}

struct Printer {}

impl Printer {
    fn warn(message: impl Display) {
        for line in message.to_string().lines() {
            println!("cargo:warning={}", line);
        }
    }
}
