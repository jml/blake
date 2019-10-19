use std::collections::HashMap;
use std::error::Error;
use std::string::FromUtf8Error;

use comrak;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{Arena, ComrakOptions};

pub fn render<'a>(
    arena: &'a Arena<AstNode<'a>>,
    root: &'a AstNode<'a>,
    options: &ComrakOptions,
) -> Result<(), Box<dyn Error>> {
    // Algorithm
    // - Find all the NodeValue::FootnoteDefinitions
    // - Render them
    // - Store them in a hashmap
    // - Remove them from the doc
    // - Find all the NodeValue::FootnoteReferences
    // - Replace them with render_sidenote
    //
    // Note:
    // - Cannot have sidenote references in sidenote definitions
    //
    // Questions:
    // - What to do when multiple references to the same sidenote?
    //   Probably link to the original one?

    let footnotes = find_footnote_definitions(root);
    let mut rendered_footnotes = HashMap::with_capacity(footnotes.len());
    // TODO: This eagerly renders all the footnote definitions, even
    // if they aren't referenced.
    let mut options = options.clone();
    options.ext_footnotes = false;
    for (tag, node) in footnotes.iter() {
        node.detach();
        let sidenote = render_footnode_definition_as_sidenote(arena, tag, node, &options)?;
        rendered_footnotes.insert(tag, sidenote);
    }

    replace_footnote_references(root, &rendered_footnotes);
    Ok(())
}

/// Find all the footnote definitions in parsed Markdown.
fn find_footnote_definitions<'a>(root: &'a AstNode<'a>) -> HashMap<Vec<u8>, &'a AstNode<'a>> {
    let mut footnotes = HashMap::new();
    for node in root.descendants() {
        let ast = node.data.borrow();
        if let NodeValue::FootnoteDefinition(tag) = &ast.value {
            footnotes.insert(tag.clone(), node);
        }
    }
    footnotes
}

/// Render a footnote definition as a sidenote.
fn render_footnode_definition_as_sidenote<'a>(
    arena: &'a Arena<AstNode<'a>>,
    tag: &Vec<u8>,
    node: &'a AstNode<'a>,
    options: &ComrakOptions,
) -> Result<NodeValue, Box<dyn Error>> {
    let mut html = vec![];
    let document = arena.alloc(AstNode::from(NodeValue::Document));
    for (i, child) in node.children().enumerate() {
        assert!(i < 1, "Footnote definitions have one child, a paragraph.");
        for grandchild in child.children() {
            grandchild.detach();
            document.append(grandchild);
        }
    }
    comrak::format_html(document, &options, &mut html)?;
    let sidenote = render_sidenote_html(tag.clone(), html)?;
    Ok(NodeValue::HtmlInline(sidenote))
}

/// Create HTML for a sidenote.
///
/// From https://edwardtufte.github.io/tufte-css/#sidenotes:
///
/// > Sidenotes consist of two elements: a superscript reference number that
/// > goes inline with the text, and a sidenote with content. To add the former,
/// > just put a label and dummy checkbox into the text where you want the
/// > reference to go, like so:
/// >
/// > ```html
/// >     <label for="sn-demo"
/// >            class="margin-toggle sidenote-number">
/// >     </label>
/// >     <input type="checkbox"
/// >            id="sn-demo"
/// >            class="margin-toggle"/>
/// > ```
/// >
/// > You must manually assign a reference id to each side or margin note,
/// > replacing “sn-demo” in the `for` and the `id` attribute values with an
/// > appropriate descriptor. It is useful to use prefixes like `sn-` for
/// > sidenotes and `mn-` for margin notes.
/// >
/// > Immediately adjacent to that sidenote reference in the main text goes the
/// > sidenote content itself, in a span with class sidenote. This tag is also
/// > inserted directly in the middle of the body text, but is either pushed
/// > into the margin or hidden by default. Make sure to position your sidenotes
/// > correctly by keeping the sidenote-number label close to the sidenote
/// > itself.
fn render_sidenote_html(name: Vec<u8>, html: Vec<u8>) -> Result<Vec<u8>, FromUtf8Error> {
    let name = String::from_utf8(name)?;
    let html = String::from_utf8(html)?;
    let output = format!(
        "<span><label class=\"margin-toggle sidenote-number\" for=\"sn-{}\"></label>\
         <input class=\"margin-toggle\" id=\"sn-{}\" type=\"checkbox\"/>\
         <span class=\"sidenote\">{}</span></span>",
        name, name, html
    );
    Ok(output.into_bytes())
}

fn replace_footnote_references<'a>(
    root: &'a AstNode<'a>,
    footnotes: &HashMap<&Vec<u8>, NodeValue>,
) {
    for node in root.descendants() {
        let mut ast = node.data.borrow_mut();
        if let NodeValue::FootnoteReference(tag) = &ast.value {
            if let Some(sidenote) = footnotes.get(tag) {
                ast.value = sidenote.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_footnote_definitions() {
        let arena = comrak::Arena::new();
        let options = ComrakOptions {
            ext_footnotes: true,
            ..ComrakOptions::default()
        };
        let contents = "I mentioned[^1] a thing.

[^1]: The thing I mentioned
";
        let root = comrak::parse_document(&arena, contents, &options);
        let footnotes = find_footnote_definitions(root);
        let keys = footnotes.keys().collect::<Vec<&Vec<u8>>>();
        assert_eq!(keys, vec![b"1"]);
    }

    #[test]
    fn test_render_footnote_definition() {
        let arena = comrak::Arena::new();
        let options = ComrakOptions {
            ext_footnotes: true,
            ..ComrakOptions::default()
        };
        let contents = "I mentioned[^1] a thing.

[^1]: Word
";
        let root = comrak::parse_document(&arena, contents, &options);
        let footnotes = find_footnote_definitions(root);
        let definitions: Vec<&&AstNode> = footnotes.values().collect();
        let definition = *definitions[0];
        let value =
            render_footnode_definition_as_sidenote(&arena, &Vec::from("1"), definition, &options)
                .unwrap();
        match value {
            NodeValue::HtmlInline(html) => assert_eq!(String::from_utf8(html).unwrap(), "<span><label class=\"margin-toggle sidenote-number\" for=\"sn-1\"></label><input class=\"margin-toggle\" id=\"sn-1\" type=\"checkbox\"/><span class=\"sidenote\">Word</span></span>"),
            _ => panic!("Unexpected value"),
        }
    }
}
