use chrono::prelude::*;
use fs_extra::dir;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, io};

mod html;

pub struct OutputPath {
    pub path: PathBuf,
}

impl OutputPath {
    pub fn index(&self) -> PathBuf {
        self.path.join("index.html")
    }

    pub fn feed(&self) -> PathBuf {
        self.path.join("feed.xml")
    }

    pub fn posts_dir(&self) -> PathBuf {
        self.path.join("posts")
    }

    pub fn static_dir(&self) -> PathBuf {
        self.path.join("static")
    }
}

pub fn build(static_dir: &Path, posts_dir: &Path, output: &OutputPath) -> Result<(), Box<dyn Error>> {
    copy_static_resources(static_dir, &output.static_dir())?;
    build_posts(posts_dir, &output.posts_dir())?;
    remove_deleted_posts(posts_dir, &output.posts_dir());
    generate_index(&output.index());
    generate_feed(&output.feed());
    Ok(())
}

fn copy_static_resources(input_dir: &Path, output_dir: &Path) -> Result<u64, fs_extra::error::Error> {
    // Delete the existing static directory.
    match fs::remove_dir_all(output_dir) {
        Ok(_) => {}
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => {}
            _ => panic!(
                "Could not remove directory: {}: {}",
                output_dir.display(),
                err
            ),
        },
    }
    // Copy the one in the library.
    let mut options = dir::CopyOptions::new();
    options.copy_inside = true;
    dir::copy(input_dir, output_dir, &options)
}

fn build_posts(input_dir: &Path, output_dir: &Path) -> Result<(), Box<dyn Error>> {
    println!(
        "build_posts({}, {})",
        input_dir.display(),
        output_dir.display()
    );
    let entries = fs::read_dir(input_dir)?;
    let posts = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("md"));
    for post_path in posts {
        let name = post_path
            .file_stem()
            .ok_or_else(|| format!("Post has no file name: {}", post_path.display()))?;
        let html_path = output_dir.with_file_name(name).with_extension("html");
        let date_str = name
            .to_str()
            .ok_or_else(|| format!("Couldn't convert date to string: {:?}", name))?;
        let date = Utc.datetime_from_str(date_str, "%Y-%m-%d-%H:%M")?;
        html::build_html(&post_path, &html_path, &date)?;
    }
    Ok(())
}

fn remove_deleted_posts(input_dir: &Path, output_dir: &Path) {
    // TODO: implement remove_deleted_posts
    println!(
        "remove_deleted_posts({}, {})",
        input_dir.display(),
        output_dir.display()
    );
}

fn generate_index(index_page: &Path) {
    // TODO: implement generate_index
    println!("generate_index({})", index_page.display());
}

fn generate_feed(feed_page: &Path) {
    // TODO: implement generate_feed
    println!("generate_feed({})", feed_page.display());
}
