extern crate chrono;
extern crate clap;
use chrono::prelude::*;
use clap::{App, Arg, SubCommand};
use std::ffi;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

mod builder;

const POST_DATE_FORMAT: &str = "%Y-%m-%d-%H:%M";
const POSTS_DIR: &str = "/Users/jml/src/notebook/posts"; // Parameterize this.
const STATIC_DIR: &str = "/Users/jml/src/notebook/static"; // Parameterize this.
const OUTPUT_DIR: &str = "/Users/jml/src/notebook/output"; // Parameterize this.

fn main() {
    let app = App::new("blake")
        .version("0.1.0")
        .author("Jonathan M. Lange <jml@mumak.net>")
        .about("Situated blogging platform")
        .subcommand(SubCommand::with_name("new"))
        .subcommand(SubCommand::with_name("edit"))
        .subcommand(
            SubCommand::with_name("build")
                .arg(
                    Arg::with_name("--rebuild").help("Rebuild everything, even if it's up-to-date"),
                )
                .arg(
                    Arg::with_name("--posts-only")
                        .help("Only build posts, don't build the indexes."),
                ),
        );
    let matches = app.get_matches();
    match matches.subcommand_name() {
        Some("new") => new_post(),
        Some("edit") => edit_post(None),
        Some("build") => build(),
        Some(_) | None => {
            println!("Invalid subcommand given.");
            process::exit(2);
        }
    }
}

/// Create a new blog post.
fn new_post() {
    let now = Utc::now();
    let name = format!("{}", now.format(POST_DATE_FORMAT));
    edit_and_commit_post(Path::new(POSTS_DIR), &name);
}

fn edit_post(name: Option<ffi::OsString>) {
    let posts_dir = Path::new(POSTS_DIR);
    let name = name.or_else(|| get_latest_file(posts_dir));
    match name {
        None => {
            println!("Could not find post to edit.");
        }
        Some(n) => {
            let name = Path::new(&n).file_stem().unwrap();
            edit_and_commit_post(posts_dir, name.to_str().unwrap());
        }
    }
}

fn get_latest_file(posts_dir: &Path) -> Option<ffi::OsString> {
    let entries = fs::read_dir(posts_dir)
        .expect(&format!("Couldn't read directory: {}", posts_dir.display()));
    entries
        .filter_map(|entry| entry.ok().map(|e| e.file_name()))
        .max()
}

fn build() {
    let path = Path::new(OUTPUT_DIR);
    let output = builder::OutputPath {
        path: path.to_owned(),
    };
    builder::do_build(Path::new(STATIC_DIR), Path::new(POSTS_DIR), &output);
}

/// Edit the blog post with the given name inside the posts directory.
///
/// If it changes, ensure the change is committed.
fn edit_and_commit_post(posts_dir: &Path, name: &str) {
    let mut post_file = posts_dir.to_owned();
    post_file.push(name);
    post_file.set_extension("md");
    let changed = edit_file(&post_file);
    if changed {
        process::Command::new("git")
            .current_dir(posts_dir)
            .arg("add")
            .arg(post_file)
            .status()
            .expect("Could not add file");
        process::Command::new("git")
            .current_dir(posts_dir)
            .arg("commit")
            .arg("-m")
            .arg(format!("Add new post {}", name))
            .status()
            .expect("Could not commit file");
    }
}

fn edit_file(filename: &Path) -> bool {
    let prev = contents(filename);
    edit(filename);
    let current = contents(filename);
    prev != current
}

/// Get the contents of a file as a vector.
///
/// If the file doesn't exist, return None. Panic if we get any other kind of
/// error.
fn contents(path: &Path) -> Option<Vec<u8>> {
    match fs::read(&path) {
        Ok(bytes) => Some(bytes),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => None,
            _ => panic!("Could not read file: {}: {}", path.display(), err),
        },
    }
}

/// Edit a file in my preferred editor.
fn edit(file: &Path) {
    process::Command::new("emacsclient")
        .arg("-c")
        .arg(file)
        .status()
        .expect(&format!("Failed to edit file: {}", file.display()));
}
