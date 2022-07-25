const EMOJI: &str = "🛵 ";

mod config;
use config::Config;

use handlebars::Handlebars;
use saucer::{prelude::*, Fs, Logger, Utf8PathBuf};
use std::str;

#[derive(Clone, Debug, Parser)]
pub(crate) struct HtmlCommand {
    #[clap(flatten)]
    pub(crate) opts: HtmlCommandOpts,
}

#[derive(Debug, Clone, Parser)]
pub(crate) struct HtmlCommandOpts {
    /// Path to an `awc.json` handlebars file.
    ///
    /// https://docs.rs/handlebars/latest/handlebars/
    #[clap(long, env = "AWC_CONFIG")]
    awc_config: Option<Utf8PathBuf>,

    /// Path to a templated HTML file
    #[clap(long, default_value_t = HtmlCommandOpts::relative_dir("browser/template.html"))]
    template_file: Utf8PathBuf,

    /// Destination path for templatized HTML
    #[clap(long, default_value_t = HtmlCommandOpts::relative_dir("server/public/index.html"))]
    public_file: Utf8PathBuf,
}

impl Saucer for HtmlCommand {
    /// Reads JSON from an awc.json and inserts it
    fn beam(&self) -> Result<()> {
        let config = self.opts.get_config()?;
        let template = self.opts.read_template()?;
        let output = self.opts.templatize(&template, &config)?;
        self.opts.write_output(output)?;
        Ok(())
    }

    fn prefix(&self) -> String {
        EMOJI.to_string()
    }

    fn description(&self) -> String {
        "rust::handlebars".to_string()
    }
}

impl HtmlCommandOpts {
    /// Read an `awc.json` file
    fn get_config(&self) -> Result<Config> {
        Config::read(self.awc_config.as_ref(), EMOJI)
    }

    /// Reads template HTML from disk
    fn read_template(&self) -> Result<String> {
        let contents =
            Fs::read_file(&self.template_file, EMOJI).context("Could not read template HTML")?;
        Ok(contents)
    }

    /// Templatizes HTML from awc.json
    fn templatize<C>(&self, contents: C, config: &Config) -> Result<String>
    where
        C: AsRef<[u8]>,
    {
        Logger::info(format!("{}templatizing from an awc.json file", EMOJI));
        let data = config.json(EMOJI)?;
        let compiled_html = Handlebars::new().render_template(
            str::from_utf8(contents.as_ref()).context("template was not valid UTF-8")?,
            &data,
        )?;
        Ok(compiled_html)
    }

    /// Writes templatized HTML to public directory
    fn write_output<C>(&self, contents: C) -> Result<()>
    where
        C: AsRef<[u8]>,
    {
        Fs::create_dir_all(Self::relative_dir("server/public"), EMOJI)?;
        Fs::write_file(&self.public_file, contents, EMOJI)
            .context("Could not write templatized HTML")?;
        Ok(())
    }

    fn relative_dir(path: &str) -> Utf8PathBuf {
        crate::relative_dir("awc-web/src").join(path)
    }
}
