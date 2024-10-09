//! Emphasis and strong emphasis
//!
//! looks like `*this*` or `__that__`
//!
//! <https://spec.commonmark.org/0.30/#emphasis-and-strong-emphasis>
use std::collections::HashMap;

use crate::generics::inline::emph_pair;
use crate::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct Em {
    pub marker: char,
}

impl NodeValue for Em {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer, options: &HashMap<String, String>, cache: &mut HashMap<String, String>) {
        fmt.open("em", &node.attrs);
        fmt.contents(&node.children, options);
        fmt.close("em");
    }
}

#[derive(Debug)]
pub struct Strong {
    pub marker: char,
}

impl NodeValue for Strong {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer, options: &HashMap<String, String>, cache: &mut HashMap<String, String>) {
        fmt.open("strong", &node.attrs);
        fmt.contents(&node.children, options);
        fmt.close("strong");
    }
}

pub fn add(md: &mut MarkdownIt) {
    emph_pair::add_with::<'*', 1, true>(md, || Node::new(Em { marker: '*' }));
    emph_pair::add_with::<'_', 1, false>(md, || Node::new(Em { marker: '_' }));
    emph_pair::add_with::<'*', 2, true>(md, || Node::new(Strong { marker: '*' }));
    emph_pair::add_with::<'_', 2, false>(md, || Node::new(Strong { marker: '_' }));
}
