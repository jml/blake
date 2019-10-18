use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

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
}
