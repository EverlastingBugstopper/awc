use std::{env, fmt::Display, fs, io, process::Command, str, time::Instant};

use camino::Utf8PathBuf;
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
            Printer::warn(e);
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
            Ok("production") => "./awc.prod.json",
            _ => "./awc.dev.json",
        }
    }

    fn emoji() -> &'static str {
        "📐 "
    }

    fn read() -> Result<Self, io::Error> {
        let path = Self::path();
        Printer::warn(format!("{} reading {}...", Self::emoji(), path));
        let contents = fs::read_to_string(path)?;
        serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
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
    fn run() -> Result<(), io::Error> {
        let (npm_install_result, handlebars_result) =
            rayon::join(|| Step::npm_install(), || Step::handlebars());

        let mut error = "".to_string();
        if let Err(e) = npm_install_result {
            error.push_str(&format!("npm install failure: {}\n", e));
        }
        if let Err(e) = handlebars_result {
            error.push_str(&format!("handlebars failure: {}\n", e));
        }

        if !error.is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, error));
        }

        let (css_result, js_result) = rayon::join(
            || Step::npm_run("build:css", "💅 "),
            || Step::npm_run("build:js", "🧳 "),
        );

        let mut error = "".to_string();
        if let Err(e) = css_result {
            error.push_str(&format!("CSS failure: {}\n", e));
        }
        if let Err(e) = js_result {
            error.push_str(&format!("JS failure: {}\n", e));
        }
        if error.is_empty() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, error))
        }
    }
}

struct Step {}

impl Step {
    /// `npm run ${args}` Runner
    fn npm_run(script: impl Display, emoji: impl Display) -> Result<(), io::Error> {
        Runner::new("npm", &["run", &script.to_string()]).run(emoji)
    }

    /// `npm install` Runner
    fn npm_install() -> Result<(), io::Error> {
        Runner::new("npm", &["install"]).run("⬇️ ")
    }

    /// Transform html w/handlebars sourced from an awc.json file
    fn handlebars() -> Result<(), io::Error> {
        let emoji = "🛵 ";
        Printer::warn(format!("{} handlebars::Handlebars", emoji));
        let template_path = "./src/ui/template.html";
        let out_path = "./public/index.html";
        let _ = fs::create_dir("./public");
        Printer::warn(format!("{} reading {}...", emoji, template_path));
        let template_contents = fs::read_to_string(template_path)?;
        Printer::warn(format!("{} reading handlebars config...", emoji));
        let config = Config::read()?;
        let base_url = config.base_url();
        let placeholder_schema = fs::read_to_string(config.placeholder_schema_path())?;
        Printer::warn(format!("{} templatizing {}...", emoji, template_path));
        let data = json!({
            "BASE_URL": base_url,
            "PLACEHOLDER_SCHEMA": placeholder_schema
        });
        Printer::warn(format!("{} {}", emoji, &data));
        let compiled_html = Handlebars::new()
            .render_template(&template_contents, &data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Printer::warn(format!("{} writing compiled html to {}", emoji, out_path));
        fs::write(out_path, compiled_html)?;
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
    fn run(&self, emoji: impl Display) -> Result<(), io::Error> {
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
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "{} {} failed with status {}",
                    emoji, &self.description, output.status
                ),
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
}
