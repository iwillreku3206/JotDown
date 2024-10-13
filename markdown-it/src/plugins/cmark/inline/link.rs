//! Links
//!
//! `![link](<to> "stuff")`
//!
//! <https://spec.commonmark.org/0.30/#links>
use std::collections::HashMap;

use crate::generics::inline::full_link;
use crate::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct Link {
    pub url: String,
    pub title: Option<String>,
}

impl NodeValue for Link {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer, options: &HashMap<String, String>, cache: &mut Cache) {
        let mut attrs = node.attrs.clone();
        attrs.push(("href", self.url.clone()));

        if let Some(title) = &self.title {
            attrs.push(("title", title.clone()));
        }

        fmt.open("a", &attrs);
        fmt.contents(&node.children, options, cache);
        fmt.close("a");
    }
}

pub fn add(md: &mut MarkdownIt) {
    full_link::add::<false>(md, |href, title| {
        Node::new(Link {
            url: href.unwrap_or_default(),
            title,
        })
    });
}
