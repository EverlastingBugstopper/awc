use std::{
    fs,
    io::{self, Read},
    panic,
};

use clap::Parser;

use awc::{ApolloCompiler, AwcDiagnostic};

fn main() {
    panic::set_hook(Box::new(|_| {
        println!("{}", AwcDiagnostic::unrecoverable_failure().json())
    }));
    let app = AwcCli::from_args();
    app.run()
}

#[derive(Debug, Parser)]
#[clap(name = "awc", author, version)]
struct AwcCli {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

impl AwcCli {
    pub fn run(&self) {
        self.subcommand.run()
    }
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Lint a GraphQL schema
    Lint(LintRunner),
}

impl SubCommand {
    pub fn run(&self) {
        match self {
            Self::Lint(lint_runner) => {
                let result = lint_runner.run();
                result.print(lint_runner.json);
                if result.success() {
                    std::process::exit(0)
                } else {
                    std::process::exit(1)
                }
            }
        }
    }
}

#[derive(Debug, Parser)]
struct LintRunner {
    /// The GraphQL file to read from.
    ///
    /// If set to "-", it will be read from stdin.
    file: String,

    /// Provides machine readable output.
    #[clap(long)]
    json: bool,
}

impl LintRunner {
    fn run(&self) -> AwcDiagnostic {
        let contents: String = match &*self.file {
            "" => return AwcDiagnostic::error(format!("input was an empty string")),
            "-" => {
                let mut buffer = String::new();
                match io::stdin().read_to_string(&mut buffer) {
                    Ok(_) => {}
                    Err(e) => {
                        return AwcDiagnostic::error(format!(
                            "unable to read GraphQL from stdin: {}",
                            e
                        ))
                    }
                }
                buffer
            }
            path => {
                let contents = if fs::metadata(path).is_ok() {
                    match fs::read_to_string(path) {
                        Ok(contents) => contents,
                        Err(e) => {
                            return AwcDiagnostic::error(format!(
                                "unable to read {} from disk: {}",
                                path, e
                            ));
                        }
                    }
                } else {
                    return AwcDiagnostic::error(format!("{} does not exist", path));
                };
                contents
            }
        };
        let awc = ApolloCompiler::new(&contents);
        awc.into()
    }
}
