use fs_extra::dir;
use std::{fs, io};
use std::path::{Path, PathBuf};

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

pub fn do_build(static_dir: &Path, posts_dir: &Path, output: &OutputPath) {
    copy_static_resources(static_dir, &output.static_dir());
    build_posts(posts_dir, &output.posts_dir());
    remove_deleted_posts(posts_dir, &output.posts_dir());
    generate_index(&output.index());
    generate_feed(&output.feed());
}

fn copy_static_resources(input_dir: &Path, output_dir: &Path) {
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
    // Ensure the directory exists.
    fs::create_dir(output_dir).expect(&format!("Could not create directory: {}", output_dir.display()));
    // Copy the one in the library.
    let options = dir::CopyOptions::new();
    dir::copy(input_dir, output_dir, &options).expect(&format!("Could not copy {} to {}", input_dir.display(), output_dir.display()));
}

fn build_posts(input_dir: &Path, output_dir: &Path) {
    println!(
        "build_posts({}, {})",
        input_dir.display(),
        output_dir.display()
    );
}

fn remove_deleted_posts(input_dir: &Path, output_dir: &Path) {
    println!(
        "remove_deleted_posts({}, {})",
        input_dir.display(),
        output_dir.display()
    );
}

fn generate_index(index_page: &Path) {
    println!("generate_index({})", index_page.display());
}

fn generate_feed(feed_page: &Path) {
    println!("generate_feed({})", feed_page.display());
}
