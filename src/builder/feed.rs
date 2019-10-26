use std::fs::OpenOptions;
use std::io;
use std::path::Path;

use atom_syndication::{Content, Entry, Feed, Link, Person};

use super::html::Post;

pub fn generate_feed(posts: &[Post], feed_page: &Path) -> io::Result<()> {
    // TODO: implement generate_feed. Ideally use third-party library.
    let feed = make_feed(posts);
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(feed_page)?;
    // TODO: I get compiler errors saying this doesn't implement From<Error>,
    // but the docs generated from the code says it does.
    feed.write_to(file).unwrap();
    Ok(())
}

fn make_feed(posts: &[Post]) -> Feed {
    let mut feed = base_feed();
    let entries = make_entries(posts);
    feed.set_entries(entries);
    // TODO: Figure out why I can't iterate over entries.
    let updated = posts.iter().map(|entry| entry.updated()).max();
    if let Some(updated) = updated {
        feed.set_updated(updated.to_rfc3339());
    }
    feed
}

fn make_entries(posts: &[Post]) -> Vec<Entry> {
    posts.iter().map(make_entry).collect()
}

fn make_entry(post: &Post) -> Entry {
    let mut entry = Entry::default();
    entry.set_id(post.url());
    entry.set_links(vec![make_entry_link(post)]);
    entry.set_content(make_post_content(post));
    entry.set_updated(post.updated().to_rfc3339());
    entry.set_published(post.published().to_rfc3339());
    entry
}

fn make_entry_link(post: &Post) -> Link {
    let mut link = Link::default();
    link.set_href(post.url());
    link
}

fn make_post_content(post: &Post) -> Content {
    // TODO: Use a different HTML body (one that doesn't have sidenotes) for the feed.
    let mut content = Content::default();
    content.set_value(post.body().to_string());
    content.set_src(post.url().to_string());
    content.set_content_type("html".to_string());
    content
}

fn base_feed() -> Feed {
    let mut feed = Feed::default();
    feed.set_id(format!("{}/", crate::SITE_URL));
    feed.set_title("jml's notebook");
    feed.set_authors(vec![me()]);
    feed.set_links(vec![site_link(), self_link()]);
    feed
}

fn me() -> Person {
    let mut me = Person::default();
    me.set_name("Jonathan M. Lange");
    me.set_email("jml@mumak.net".to_string());
    me
}

fn site_link() -> Link {
    let mut link = Link::default();
    link.set_href(crate::SITE_URL);
    link.set_rel("alternate");
    link.set_hreflang("en".to_string());
    link
}

fn self_link() -> Link {
    let mut link = Link::default();
    link.set_href(format!("{}/feed.xml", crate::SITE_URL));
    link.set_rel("self");
    link.set_hreflang("en".to_string());
    link
}
