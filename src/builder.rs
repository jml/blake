use fs_extra::dir;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{ffi, fs, io};

use crate::posts;

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
    posts: &posts::Posts,
    output: &OutputPath,
) -> Result<(), Box<dyn Error>> {
    copy_static_resources(static_dir, &output.static_dir())?;

    let mut html_posts = build_posts(posts, &output.posts_dir())?;
    html_posts.sort_by_key(|post| *post.date());
    html_posts.reverse();

    html::write_index_html(&html_posts, &output.index())?;
    generate_feed(&output.feed());
    remove_deleted_posts(posts, &output.posts_dir())?;
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

fn build_posts(posts: &posts::Posts, output_dir: &Path) -> Result<Vec<html::Post>, Box<dyn Error>> {
    fs::create_dir_all(output_dir)?;
    let posts = posts.iter_posts()?;
    let mut html_posts = Vec::new();
    for post in posts {
        let post = post?;
        let html_post = html::Post::render(&post.path(), &post.date())?;
        let html_path = output_dir.join(post.name()).with_extension("html");
        html_post.write_html(&html_path)?;
        html_posts.push(html_post);
    }
    Ok(html_posts)
}

fn remove_deleted_posts(posts: &posts::Posts, output_dir: &Path) -> io::Result<()> {
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

fn generate_feed(feed_page: &Path) {
    // TODO: implement generate_feed. Ideally use third-party library.
    println!("generate_feed({})", feed_page.display());
}
