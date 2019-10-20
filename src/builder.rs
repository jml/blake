use fs_extra::dir;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{ffi, fs, io};

use crate::posts::{Post, Posts};

mod html;
mod sidenotes;

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

pub fn build(
    static_dir: &Path,
    posts: &Posts,
    output: &OutputPath,
) -> Result<(), Box<dyn Error>> {
    copy_static_resources(static_dir, &output.static_dir())?;
    build_posts(posts, &output.posts_dir())?;
    remove_deleted_posts(posts, &output.posts_dir())?;
    generate_index(&output.index());
    generate_feed(&output.feed());
    Ok(())
}

// TODO: Add logging.

fn copy_static_resources(
    input_dir: &Path,
    output_dir: &Path,
) -> Result<u64, fs_extra::error::Error> {
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

fn build_posts(posts: &Posts, output_dir: &Path) -> Result<(), Box<dyn Error>> {
    let posts = posts.iter_posts()?;
    for post in posts {
        let post = post?;
        build_post(&post, output_dir)?;
    }
    Ok(())
}

fn build_post(post: &Post, output_dir: &Path) -> Result<(), Box<dyn Error>> {
    let html_path = output_dir.with_file_name(post.name()).with_extension("html");
    html::build_html(&post.path(), &html_path, &post.date())
}

fn remove_deleted_posts(posts: &Posts, output_dir: &Path) -> io::Result<()> {
    let html_posts = fs::read_dir(output_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(ffi::OsStr::new("html")));
    for html_path in html_posts {
        let name = html_path
            .file_stem()
            .expect("Files from a directory listing should have file names");
        if !posts.is_post(name) {
            fs::remove_file(html_path)?;
        }
    }
    Ok(())
}

fn generate_index(index_page: &Path) {
    // TODO: implement generate_index
    println!("generate_index({})", index_page.display());
}

fn generate_feed(feed_page: &Path) {
    // TODO: implement generate_feed. Ideally use third-party library.
    println!("generate_feed({})", feed_page.display());
}
