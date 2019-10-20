use chrono::prelude::*;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug)]
pub struct Posts {
    path: PathBuf,
}

impl Posts {
    pub fn new(path: PathBuf) -> Posts {
        Posts { path }
    }

    pub fn get_latest_file(&self) -> io::Result<Option<PathBuf>> {
        let entries = fs::read_dir(&self.path)?;
        let mut paths = vec![];
        for entry in entries {
            let entry = entry?;
            paths.push(entry.path())
        }
        Ok(paths.into_iter().max())
    }

    pub fn get_post_filename(&self, name: &str) -> PathBuf {
        let mut post_file = self.path.to_owned();
        post_file.push(name);
        post_file.set_extension("md");
        post_file
    }

    pub fn commit_post(&self, post_file: &Path, name: &str) -> io::Result<()> {
        process::Command::new("git")
            .current_dir(&self.path)
            .arg("add")
            .arg(post_file)
            .status()?;
        process::Command::new("git")
            .current_dir(&self.path)
            .arg("commit")
            .arg("-m")
            .arg(format!("Add new post {}", name))
            .status()?;
        Ok(())
    }

    /// Is there a post with the given name?
    pub fn is_post<T: AsRef<std::ffi::OsStr>>(&self, name: T) -> bool {
        let source_path = self.path.with_file_name(name).with_extension("md");
        source_path.is_file()
    }

    pub fn iter_posts(&self) -> io::Result<impl Iterator<Item=Result<Post, Error>>> {
        self.path.read_dir().map(
            |entries| {
                entries
                    .map(|entry| entry.map(|e| e.path()))
                    .filter_map(|entry| {
                        match entry {
                            Err(e) => Some(Err(e)),
                            Ok(path) => {
                                if has_extension(&path, "md") {
                                    Some(Ok(path))
                                } else {
                                    None
                                }
                            }
                        }
                    })
                    .map(|entry| {
                        let path = entry?;
                        Post::new(path)
                    })
            }
        )
    }
}

fn has_extension<T: AsRef<std::ffi::OsStr>>(path: &Path, extension: T) -> bool {
    match path.extension() {
        None => false,
        Some(ext) => ext == extension.as_ref(),
    }
}

/// Errors that can be caused by creating a Post.
#[derive(Debug)]
pub enum Error {
    NoFileName(PathBuf),
    BadFileName(PathBuf),
    IoError(io::Error),
    InvalidDateError(chrono::ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NoFileName(path) => write!(f, "No such filename: {}", path.display()),
            Error::BadFileName(path) => write!(f, "Cannot decode filename: {}", path.display()),
            Error::IoError(io_error) => write!(f, "Cannot read file: {}", io_error),
            Error::InvalidDateError(parse_error) => write!(f, "Filename is not a valid date: {}", parse_error),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::IoError(io_error) => Some(io_error),
            Error::InvalidDateError(parse_error) => Some(parse_error),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(error: chrono::ParseError) -> Self {
        Error::InvalidDateError(error)
    }
}

pub struct Post {
    path: PathBuf,
    name: String,
    date: DateTime<Utc>,
}

impl Post {
    pub fn new(path: PathBuf) -> Result<Post, Error> {
        let name = path.file_stem().ok_or_else(|| Error::NoFileName(path.clone()))?;
        let name = name.to_str().ok_or_else(|| Error::BadFileName(path.clone()))?;
        let name = String::from(name);
        let date = Utc.datetime_from_str(&name, crate::POST_DATE_FORMAT)?;
        Ok(Post { path, name, date })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
}
