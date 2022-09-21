use anyhow::{anyhow, Context, Result};
use camino::{ReadDirUtf8, Utf8Path};

use crate::Logger;

use std::{
    fs::{self, File},
    str,
};

/// Interact with a file system
#[derive(Default, Copy, Clone)]
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
                    Logger::info(format!("{}reading {} from disk", prefix, &path));
                    let contents = fs::read_to_string(&path)
                        .with_context(|| format!("{}could not read {}", prefix, &path))?;
                    if contents.is_empty() {
                        Err(anyhow!("'{}' was empty", contents))
                    } else {
                        Ok(contents)
                    }
                } else {
                    Err(anyhow!("{}'{}' is not a file", prefix, path))
                }
            }
            Err(e) => Err(anyhow!("{}could not find '{}'", prefix, path).context(e)),
        }
    }

    /// writes a file to disk
    pub fn write_file<P, C>(path: P, contents: C, prefix: &str) -> Result<()>
    where
        P: AsRef<Utf8Path>,
        C: AsRef<[u8]>,
    {
        let path = path.as_ref();
        let contents = str::from_utf8(contents.as_ref()).with_context(|| {
            format!(
                "tried to write contents to {} that was invalid UTF-8",
                &path
            )
        })?;
        if !path.exists() {
            File::create(&path)
                .with_context(|| format!("{} does not exist and it could not be created", &path))?;
        }
        if !path.exists() {
            File::create(&path)
                .with_context(|| format!("{} does not exist and it could not be created", &path))?;
        }
        Logger::info(format!("{}writing {} to disk", prefix, &path));
        fs::write(&path, contents)
            .with_context(|| format!("{}could not write {}", prefix, &path))?;
        Ok(())
    }

    /// creates a directory
    pub fn create_dir_all<P>(path: P, prefix: &str) -> Result<()>
    where
        P: AsRef<Utf8Path>,
    {
        let path = path.as_ref();
        Logger::info(format!("{}creating {} directory", prefix, &path));
        fs::create_dir_all(&path)
            .with_context(|| format!("could not create {} directory", &path))?;
        Ok(())
    }

    /// get contents of a directory
    pub fn get_dir_entries<D>(dir: D, prefix: &str) -> Result<ReadDirUtf8>
    where
        D: AsRef<Utf8Path>,
    {
        let dir = dir.as_ref();
        let entries = dir
            .read_dir_utf8()
            .with_context(|| format!("{}could not read entries of {}", prefix, dir))?;
        Ok(entries)
    }

    /// assert that a file exists
    pub fn assert_path_exists<F>(file: F, prefix: &str) -> Result<()>
    where
        F: AsRef<Utf8Path>,
    {
        let file = file.as_ref();
        Self::metadata(file, prefix)?;
        Ok(())
    }

    /// get metadata about a file path
    pub fn metadata<F>(file: F, prefix: &str) -> Result<fs::Metadata>
    where
        F: AsRef<Utf8Path>,
    {
        let file = file.as_ref();
        fs::metadata(file).with_context(|| format!("{}could not find {}", prefix, file))
    }

    /// copies one file to another
    pub fn copy<I, O>(in_path: I, out_path: O, prefix: &str) -> Result<()>
    where
        I: AsRef<Utf8Path>,
        O: AsRef<Utf8Path>,
    {
        let in_path = in_path.as_ref();
        let out_path = out_path.as_ref();
        Logger::info(format!("{}copying {} to {}", prefix, in_path, out_path));
        // attempt to remove the old file
        // but do not error if it doesn't exist.
        let _ = fs::remove_file(&out_path);
        fs::copy(in_path, &out_path)
            .with_context(|| format!("{}could not copy {} to {}", &prefix, &in_path, &out_path))?;
        Ok(())
    }

    /// recursively removes directories
    pub fn remove_dir_all<D>(dir: D, prefix: &str) -> Result<()>
    where
        D: AsRef<Utf8Path>,
    {
        let dir = dir.as_ref();
        if Self::path_is_dir(dir, prefix)? {
            fs::remove_dir_all(dir)
                .with_context(|| format!("{}could not remove {}", prefix, dir))?;
            Ok(())
        } else {
            Err(anyhow!(
                "{}could not remove {} because it is not a directory",
                prefix,
                dir
            ))
        }
    }

    /// checks if a path is a directory, errors if the path does not exist
    pub fn path_is_dir<D>(dir: D, prefix: &str) -> Result<bool>
    where
        D: AsRef<Utf8Path>,
    {
        let dir = dir.as_ref();
        Ok(Self::metadata(dir, prefix).map(|m| m.is_dir())?)
    }

    /// copies all contents from one directory to another
    pub fn copy_dir_all<I, O>(in_dir: I, out_dir: O, prefix: &str) -> Result<()>
    where
        I: AsRef<Utf8Path>,
        O: AsRef<Utf8Path>,
    {
        let in_dir = in_dir.as_ref();
        let out_dir = out_dir.as_ref();
        Self::create_dir_all(out_dir, prefix)?;
        Logger::info(format!(
            "{}copying contents of {} to {}",
            prefix, in_dir, out_dir
        ));
        for entry in Self::get_dir_entries(in_dir, prefix)? {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                if let Ok(metadata) = fs::metadata(&entry_path) {
                    if metadata.is_file() {
                        if let Some(entry_name) = entry_path.file_name() {
                            let out_file = out_dir.join(entry_name);
                            Logger::info(format!(
                                "{}copying {} to {}",
                                prefix, &entry_path, &out_file
                            ));
                            fs::copy(&entry_path, &out_file).with_context(|| {
                                format!(
                                    "{}could not copy {} to {}",
                                    &prefix, &entry_path, &out_file
                                )
                            })?;
                        }
                    } else if metadata.is_dir() {
                        if entry_path != in_dir {
                            if let Some(entry_name) = entry_path.file_name() {
                                let out_dir = out_dir.join(entry_name);
                                Logger::info(format!(
                                    "{}copying {} to {}",
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
