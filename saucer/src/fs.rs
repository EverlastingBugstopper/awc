use anyhow::{anyhow, Context, Result};
use camino::Utf8Path;

use crate::Log;

use std::fs;

#[derive(Default, Copy, Clone)]
/// Interact with a file system
pub struct Fs {}

impl Fs {
    /// reads a file from disk
    pub fn read_file<P>(path: P, prefix: &str) -> Result<String>
    where
        P: AsRef<Utf8Path>,
    {
        let path = path.as_ref();
        match fs::metadata(path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    Log::info(format!("{}reading {} from disk", prefix, &path));
                    let contents = fs::read_to_string(&path)
                        .with_context(|| format!("{}could not read {}", prefix, &path))?;
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
    pub fn write_file<P, C>(path: P, contents: C, prefix: &str) -> Result<()>
    where
        P: AsRef<Utf8Path>,
        C: AsRef<[u8]>,
    {
        let path = path.as_ref();
        Log::info(format!("{} writing {} to disk", prefix, &path));
        fs::write(&path, contents)
            .with_context(|| format!("{}could not write {}", prefix, &path))?;
        Ok(())
    }

    /// creates a directory
    pub fn create_dir<P>(path: P, prefix: &str)
    where
        P: AsRef<Utf8Path>,
    {
        let path = path.as_ref();
        Log::info(format!("{}creating {} directory", prefix, &path));
        let _ = fs::create_dir_all(path);
    }

    /// copies all contents from one directory to another
    pub fn copy_dir_all<I, O>(in_dir: I, out_dir: O, prefix: &str) -> Result<()>
    where
        I: AsRef<Utf8Path>,
        O: AsRef<Utf8Path>,
    {
        let in_dir = in_dir.as_ref();
        let out_dir = out_dir.as_ref();
        Log::info(format!(
            "{} copying contents of {} to {}",
            prefix, in_dir, out_dir
        ));
        for entry in in_dir
            .read_dir_utf8()
            .context("cannot read from your file system")?
        {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if let Ok(metadata) = fs::metadata(&entry_path) {
                    if metadata.is_file() && !entry_path.to_string().contains("README.md") {
                        if let Some(entry_name) = entry_path.file_name() {
                            let out_file = out_dir.join(entry_name);
                            Log::info(format!(
                                "{} copying {} to {}",
                                prefix, &entry_path, &out_file
                            ));
                            fs::copy(&entry_path, &out_file).with_context(|| {
                                format!("{} could not copy {} to {}", &prefix, &in_dir, &out_file)
                            })?;
                        }
                    } else if metadata.is_dir() {
                        if entry_path != in_dir {
                            if let Some(entry_name) = entry_path.file_name() {
                                let out_dir = out_dir.join(entry_name);
                                Log::info(format!(
                                    "{} copying {} to {}",
                                    prefix, &entry_path, &out_dir
                                ));
                                Fs::copy_dir_all(&entry_path, &out_dir, &prefix)?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
