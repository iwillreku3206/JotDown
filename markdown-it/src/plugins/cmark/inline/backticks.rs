//! Code spans
//!
//! `` `looks like this` ``
//!
//! <https://spec.commonmark.org/0.30/#code-span>
use std::collections::HashMap;

use crate::generics::inline::code_pair;
use crate::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct CodeInline {
    pub marker: char,
    pub marker_len: usize,
}

impl NodeValue for CodeInline {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer, options: &HashMap<String, String>) {
        fmt.open("code", &node.attrs);
        fmt.contents(&node.children, options);
        fmt.close("code");
    }
}

pub fn add(md: &mut MarkdownIt) {
    code_pair::add_with::<'`'>(md, |len| {
        Node::new(CodeInline {
            marker: '`',
            marker_len: len,
        })
    });
}
