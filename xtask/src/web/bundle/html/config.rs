use std::env;

use saucer::{Context, Fs, Log, Result, Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Config {
    /// set the base URL
    base_url: String,

    /// set the placeholder schema
    placeholder_schema_path: Utf8PathBuf,
}

impl Config {
    /// Read an awc.json from disk
    pub(crate) fn read<P>(path: Option<P>, prefix: &str) -> Result<Self>
    where
        P: AsRef<Utf8Path>,
    {
        let path: Utf8PathBuf = if let Some(path) = path {
            path.as_ref().into()
        } else {
            Self::default_path().into()
        };
        let contents = Fs::read_file(&path, &prefix).context("Could not read awc.json")?;
        let config: Self = serde_json::from_str(&contents)
            .with_context(|| format!("{} invalid config at {}", prefix, &path))?;
        Ok(config)
    }

    /// Get JSON of config
    pub(crate) fn json(&self, prefix: &str) -> Result<Value> {
        let json = json!({
          "BASE_URL": &self.base_url,
          "PLACEHOLDER_SCHEMA": &self.placeholder_schema(&prefix)?
        });
        Log::info(format!("{} {}", prefix, &json));
        Ok(json)
    }

    /// Find placeholder schema
    pub(crate) fn placeholder_schema(&self, prefix: &str) -> Result<String> {
        let contents = Fs::read_file(&self.placeholder_schema_path, &prefix).context("Could not read contents of schema file designated in awc.json['placeholder_schema_path']")?;
        Ok(contents)
    }

    /// Get the default awc.json path if one is not provided
    fn default_path() -> &'static Utf8Path {
        match env::var("AWC_ENV").as_deref() {
            Ok("production") => "./awc-web/awc.prod.json",
            _ => "./awc-web/awc.dev.json",
        }
        .into()
    }
}
