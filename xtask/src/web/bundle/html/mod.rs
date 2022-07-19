const EMOJI: &str = "🛵 ";

mod config;
use config::Config;

use crate::framework::{prelude::*, Fs, Log};
use camino::Utf8PathBuf;
use handlebars::Handlebars;
use std::str;

#[derive(Clone, Debug, Parser)]
pub(crate) struct HtmlCommand {
    #[clap(flatten)]
    pub(crate) opts: HtmlCommandOpts,
}

#[derive(Debug, Clone, Parser)]
pub(crate) struct HtmlCommandOpts {
    #[clap(long)]
    awc_config: Option<Utf8PathBuf>,

    #[clap(long, default_value_t = HtmlCommandOpts::relative_dir("src/browser/template.html"))]
    template_file: Utf8PathBuf,

    #[clap(long, default_value_t = HtmlCommandOpts::relative_dir("public/index.html"))]
    public_file: Utf8PathBuf,
}

impl Command for HtmlCommand {
    /// Reads JSON from an awc.json and inserts it
    fn run(&self) -> Result<()> {
        let config = self.opts.get_config()?;
        let template = self.opts.read_template()?;
        let output = self.opts.templatize(&template, &config)?;
        self.opts.write_output(output)?;
        Ok(())
    }

    fn emoji(&self) -> String {
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
        Log::info(format!("template file: {}", &self.template_file));
        let contents =
            Fs::read_file(&self.template_file, EMOJI).context("Could not read template HTML")?;
        Ok(contents)
    }

    /// Templatizes HTML from awc.json
    fn templatize<C>(&self, contents: C, config: &Config) -> Result<String>
    where
        C: AsRef<[u8]>,
    {
        Log::info(format!("{} templatizing...", EMOJI));
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
        Fs::create_dir(Self::relative_dir("public"), EMOJI);
        Fs::write_file(&self.public_file, contents, EMOJI)
            .context("Could not write templatized HTML")?;
        Ok(())
    }

    fn relative_dir(path: &str) -> Utf8PathBuf {
        crate::relative_dir("awc-web").join(path)
    }
}
