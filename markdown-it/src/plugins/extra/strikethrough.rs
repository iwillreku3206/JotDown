//! Strikethrough syntax (like `~~this~~`)
use std::collections::HashMap;

use crate::generics::inline::emph_pair;
use crate::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct Strikethrough {
    pub marker: char,
}

impl NodeValue for Strikethrough {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer, options: &HashMap<String, String>) {
        fmt.open("s", &node.attrs);
        fmt.contents(&node.children, options);
        fmt.close("s");
    }
}

pub fn add(md: &mut MarkdownIt) {
    emph_pair::add_with::<'~', 2, true>(md, || Node::new(Strikethrough { marker: '~' }));
}
