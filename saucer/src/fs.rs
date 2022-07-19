use anyhow::{anyhow, Context, Result};
use camino::Utf8Path;

use super::Log;

use std::{fmt::Display, fs};

#[derive(Default, Copy, Clone)]
/// Interact with a file system
pub struct Fs {}

impl Fs {
    /// reads a file from disk
    pub fn read_file<P>(path: P, emoji: impl Display) -> Result<String>
    where
        P: AsRef<Utf8Path>,
    {
        let path = path.as_ref();
        match fs::metadata(path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    Log::info(format!("{}reading {} from disk", emoji, &path));
                    let contents = fs::read_to_string(&path)
                        .with_context(|| format!("{} could not read {}", emoji, &path))?;
                    if contents.is_empty() {
                        Err(anyhow!("'{}' was empty", contents))
                    } else {
                        Ok(contents)
                    }
                } else {
                    Err(anyhow!("'{}' is not a file", path))
                }
            }
            Err(e) => Err(anyhow!("Could not find '{}'", path).context(e)),
        }
    }

    /// writes a file to disk
    pub fn write_file<P, C>(path: P, contents: C, emoji: impl Display) -> Result<()>
    where
        P: AsRef<Utf8Path>,
        C: AsRef<[u8]>,
    {
        let path = path.as_ref();
        Log::info(format!("{} writing {} to disk", emoji, &path));
        fs::write(&path, contents)
            .with_context(|| format!("{} could not write {}", emoji, &path))?;
        Ok(())
    }

    /// creates a directory
    pub fn create_dir<P>(path: P, emoji: impl Display)
    where
        P: AsRef<Utf8Path>,
    {
        let path = path.as_ref();
        Log::info(format!("{} creating {} directory", emoji, &path));
        let _ = fs::create_dir_all(path);
    }
}
