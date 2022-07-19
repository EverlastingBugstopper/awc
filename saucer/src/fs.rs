use anyhow::{anyhow, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use walkdir::WalkDir;

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
                        .with_context(|| format!("{}could not read {}", emoji, &path))?;
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
            .with_context(|| format!("{}could not write {}", emoji, &path))?;
        Ok(())
    }

    /// creates a directory
    pub fn create_dir<P>(path: P, emoji: impl Display)
    where
        P: AsRef<Utf8Path>,
    {
        let path = path.as_ref();
        Log::info(format!("{}creating {} directory", emoji, &path));
        let _ = fs::create_dir_all(path);
    }

    /// copies all contents from one directory to another
    pub fn copy_dir_all<I, O>(in_dir: I, out_dir: O, emoji: &str) -> Result<()>
    where
        I: AsRef<Utf8Path>,
        O: AsRef<Utf8Path>,
    {
        let in_dir = in_dir.as_ref();
        let out_dir = out_dir.as_ref();
        Log::info(format!(
            "{} copying contents of {} to {}",
            emoji, in_dir, out_dir
        ));
        for entry in WalkDir::new(in_dir) {
            if let Ok(entry) = entry {
                if let Ok(entry) = Utf8PathBuf::try_from(entry.path().to_path_buf()) {
                    if let Ok(metadata) = fs::metadata(&entry) {
                        if metadata.is_file() && !entry.to_string().contains("README.md") {
                            if let Some(entry_name) = entry.file_name() {
                                let out_file = out_dir.join(entry_name);
                                Log::info(format!("{} copying {} to {}", emoji, &entry, &out_file));
                                fs::copy(&entry, &out_file).with_context(|| {
                                    format!(
                                        "{} could not copy {} to {}",
                                        &emoji, &in_dir, &out_file
                                    )
                                })?;
                            }
                        } else if metadata.is_dir() {
                            if entry != in_dir {
                                if let Some(entry_name) = entry.file_name() {
                                    let out_dir = out_dir.join(entry_name);
                                    Log::info(format!(
                                        "{} copying {} to {}",
                                        emoji, &entry, &out_dir
                                    ));
                                    Fs::copy_dir_all(&entry, &out_dir, &emoji)?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
