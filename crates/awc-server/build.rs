use std::{env, fmt::Debug, fmt::Display, fs, process::Command, str, time::Instant};

use anyhow::{anyhow, Context, Error};
use camino::{Utf8Path, Utf8PathBuf};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;

fn main() {
    let start = Instant::now();
    match Toolchain::run() {
        Ok(()) => {
            let elapsed = start.elapsed();
            let millis = elapsed.as_millis() as u64;
            let elapsed_msg = if millis > 1000 {
                let secs = elapsed.as_secs();
                format!("{} seconds, {} ms", secs, millis - secs * 1000)
            } else {
                format!("{} ms", millis)
            };
            Printer::warn(format!("🎉 Success in {}!", elapsed_msg));
            std::process::exit(0);
        }
        Err(e) => {
            Printer::warn_debug(e);
            std::process::exit(1);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    /// set the base URL
    base_url: String,

    /// set the placeholder schema
    placeholder_schema_path: Utf8PathBuf,
}

impl Config {
    fn path() -> &'static str {
        match env::var("NODE_ENV").as_deref() {
            Ok("production") => "awc.prod.json",
            _ => "awc.dev.json",
        }
    }

    fn emoji() -> &'static str {
        "📐 "
    }

    fn read() -> Result<Self, anyhow::Error> {
        let path = Self::path();
        let contents = File::read(path, Self::emoji())?;
        let config: Self = serde_json::from_str(&contents)
            .with_context(|| format!("{} invalid config at {}", Self::emoji(), path))?;
        Ok(config)
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn placeholder_schema_path(&self) -> &Utf8PathBuf {
        &self.placeholder_schema_path
    }
}

struct Toolchain {}

impl Toolchain {
    fn run() -> Result<(), Error> {
        let (npm_install_result, handlebars_result) =
            rayon::join(|| Step::npm_install(), || Step::handlebars());

        let mut stage_one_errors = 0;
        let stage_one_error = anyhow!("unrecoverable error in stage 1");
        let stage_one_error = if let Err(npm_install_err) = npm_install_result {
            stage_one_errors += 1;
            stage_one_error.context(npm_install_err)
        } else {
            stage_one_error
        };
        let stage_one_error = if let Err(handlebars_err) = handlebars_result {
            stage_one_errors += 1;
            stage_one_error.context(handlebars_err)
        } else {
            stage_one_error
        };

        if stage_one_errors > 0 {
            return Err(stage_one_error);
        }

        let (css_result, js_result) = rayon::join(
            || Step::npm_run("build:css", "💅 "),
            || Step::npm_run("build:js", "🧳 "),
        );

        let mut stage_two_errors = 0;
        let stage_two_error = anyhow!("unrecoverable error in stage 1");
        let stage_two_error = if let Err(css_error) = css_result {
            stage_two_errors += 1;
            stage_two_error.context(css_error)
        } else {
            stage_two_error
        };
        let stage_two_error = if let Err(js_error) = js_result {
            stage_two_errors += 1;
            stage_two_error.context(js_error)
        } else {
            stage_two_error
        };
        if stage_two_errors > 0 {
            return Err(stage_two_error);
        };

        let stage_three_error = anyhow!("required files not found");
        let mut stage_three_errors = 0;
        let stage_three_error = if let Err(e) = fs::metadata("./public/index.html") {
            stage_three_errors += 1;
            stage_three_error
                .context(e)
                .context("could not find ./public/index.html")
        } else {
            stage_three_error
        };
        let stage_three_error = if let Err(e) = fs::metadata("./public/index.css") {
            stage_three_errors += 1;
            stage_three_error
                .context(e)
                .context("could not find ./public/index.css")
        } else {
            stage_three_error
        };
        let stage_three_error = if let Err(e) = fs::metadata("./public/index.js") {
            stage_three_errors += 1;
            stage_three_error
                .context(e)
                .context("could not find ./public/index.js")
        } else {
            stage_three_error
        };
        let stage_three_error = if let Err(e) = fs::metadata("./public/favicon.ico") {
            stage_three_errors += 1;
            stage_three_error
                .context(e)
                .context("could not find ./public/favicon.ico")
        } else {
            stage_three_error
        };
        if stage_three_errors == 0 {
            Ok(())
        } else {
            Err(stage_three_error)
        }
    }
}

struct Step {}

impl Step {
    /// `npm run ${args}` Runner
    fn npm_run(script: impl Display, emoji: impl Display) -> Result<(), Error> {
        Runner::new("npm", &["run", &script.to_string()]).run(emoji)
    }

    /// `npm install` Runner
    fn npm_install() -> Result<(), Error> {
        Runner::new("npm", &["install"]).run("⬇️ ")
    }

    /// Transform html w/handlebars sourced from an awc.json file
    fn handlebars() -> Result<(), Error> {
        let emoji = "🛵 ";
        Printer::warn(format!("{} transforming html", emoji));
        let template_path = "./ui/template.html";
        let out_path = "./public/index.html";
        let _ = fs::create_dir("public");
        let template_contents = File::read(template_path, emoji)?;
        let config = Config::read()?;
        let base_url = config.base_url();
        let placeholder_schema = File::read(config.placeholder_schema_path(), emoji)?;
        Printer::warn(format!("{} templatizing {}...", emoji, template_path));
        let data = json!({
            "BASE_URL": base_url,
            "PLACEHOLDER_SCHEMA": placeholder_schema
        });
        Printer::warn(format!("{} {}", emoji, &data));
        let compiled_html = Handlebars::new().render_template(&template_contents, &data)?;
        File::write(out_path, compiled_html, emoji)?;
        let old_fav = "./ui/favicon.ico";
        let new_fav = "./public/favicon.ico";
        fs::copy(old_fav, new_fav)
            .with_context(|| format!("{} could not copy {} to {}", emoji, old_fav, new_fav))?;
        Ok(())
    }
}

struct File {}

impl File {
    /// reads a file from disk
    fn read<P>(path: P, emoji: impl Display) -> Result<String, Error>
    where
        P: AsRef<Utf8Path>,
    {
        let path = path.as_ref();
        Printer::warn(format!("{} reading {}...", emoji, &path));
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("{} could not read {}", emoji, &path))?;
        Ok(contents)
    }

    /// writes a file to disk
    fn write<P, C>(path: P, contents: C, emoji: impl Display) -> Result<(), Error>
    where
        P: AsRef<Utf8Path>,
        C: AsRef<[u8]>,
    {
        let path = path.as_ref();
        Printer::warn(format!("{} writing {}...", emoji, &path));
        fs::write(&path, contents)
            .with_context(|| format!("{} could not write {}", emoji, &path))?;
        Ok(())
    }
}

struct Runner {
    bin: String,
    args: Vec<String>,
    description: String,
}

impl Runner {
    fn new<I, S>(bin: impl Display, args: I) -> Self
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

    /// Run a command and print its output
    fn run(&self, emoji: impl Display) -> Result<(), Error> {
        Printer::warn(format!("{} {}", emoji, &self.description));
        let output = Command::new(&self.bin).args(&self.args).output()?;
        if let Ok(stdout) = str::from_utf8(&output.stdout) {
            for line in stdout.lines() {
                Printer::warn(format!("{} {}", emoji, line));
            }
        }
        if let Ok(stderr) = str::from_utf8(&output.stderr) {
            for line in stderr.lines() {
                Printer::warn(format!("{} {}", emoji, line));
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

struct Printer {}

impl Printer {
    /// Print a cargo warning
    fn warn(message: impl Display) {
        for line in message.to_string().lines() {
            println!("cargo:warning={}", line);
        }
    }

    fn warn_debug(message: impl Debug) {
        println!("cargo:warning={:?}", message);
    }
}
