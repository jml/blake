use std::fs;
use std::path::Path;

use chrono::prelude::*;
use comrak;
use comrak::nodes::{NodeHeading, NodeValue};
use comrak::ComrakOptions;
use lazy_static::lazy_static;
use tera::{compile_templates, Tera};

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

fn render_markdown(contents: &str) -> (Option<String>, String) {
    let arena = comrak::Arena::new();
    let root = comrak::parse_document(&arena, contents, &ComrakOptions::default());
    let title = find_title(root).map(|s| s.to_owned());
    let mut html = vec![];
    comrak::format_html(root, &ComrakOptions::default(), &mut html).expect("Couldn't format HTML");
    let html_str = String::from_utf8(html).expect("Invalid unicode");
    (title, html_str)
    // TODO: Fork
    // file:///Users/jml/src/blake/target/doc/src/comrak/html.rs.html#13-28
    // and change footnotes to sidenotes.
}

/// Find the title in the post.
///
/// Assumes that the first Heading 1 is the title.
fn find_title<'a>(root: &'a comrak::nodes::AstNode<'a>) -> Option<String> {
    for node in root.descendants() {
        if let NodeValue::Heading(NodeHeading { level: 1, .. }) = node.data.borrow().value {
            // TODO: Is there a way to avoid the clone?
            let text = node
                .children()
                .filter_map(|child| child.data.borrow().value.text().cloned())
                .flatten()
                .collect::<Vec<u8>>();
            return String::from_utf8(text).ok();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_title() {
        let arena = comrak::Arena::new();
        let contents = "no heading here";
        let root = comrak::parse_document(&arena, contents, &ComrakOptions::default());
        assert_eq!(find_title(&root), None)
    }

    #[test]
    fn test_title() {
        let arena = comrak::Arena::new();
        let contents = "# title

paragraph text";
        let root = comrak::parse_document(&arena, contents, &ComrakOptions::default());
        assert_eq!(find_title(&root), Some(String::from("title")))
    }

    #[test]
    fn test_two_titles() {
        let arena = comrak::Arena::new();
        let contents = "# title

paragraph text

# second title

more text
";
        let root = comrak::parse_document(&arena, contents, &ComrakOptions::default());
        assert_eq!(find_title(&root), Some(String::from("title")))
    }
}
