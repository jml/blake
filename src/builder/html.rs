extern crate comrak;

use std::path::Path;
use std::fs;

use chrono::prelude::*;
use comrak::ComrakOptions;
use lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TERA: Tera = compile_templates!("templates/*.html");
}

pub fn build_html(source: &Path, dest: &Path, date: &DateTime<Utc>) {
    println!(
        "build_html({}, {}, {})",
        source.display(),
        dest.display(),
        date
    );
    let contents =
        fs::read_to_string(source).expect(&format!("Could not read file: {}", source.display()));
    let (title, output) = render_markdown(&contents);
    let mut context = tera::Context::new();
    context.insert("post", &output);
    context.insert("title", &title);
    context.insert("date", &date.format("%Y-%m-%d").to_string());
    let rendered = TERA
        .render("post.html", &context)
        .expect(&format!("Could not render template: {:?}", TERA.templates));
    fs::write(dest, rendered).expect(&format!("Couldn't write to file: {}", dest.display()));
}

fn render_markdown(contents: &str) -> (&str, String) {
    let arena = comrak::Arena::new();
    let root = comrak::parse_document(&arena, contents, &ComrakOptions::default());
    let mut html = vec![];
    comrak::format_html(root, &ComrakOptions::default(), &mut html).expect("Couldn't format HTML");
    let html_str = String::from_utf8(html).expect("Invalid unicode");
    ("", html_str)
    // TODO: Fork
    // file:///Users/jml/src/blake/target/doc/src/comrak/html.rs.html#13-28
    // and change footnotes to sidenotes.
    // TODO: Make a new function to extra title from markdown AST
}
