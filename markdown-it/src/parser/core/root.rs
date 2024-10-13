use std::collections::HashMap;

use crate::parser::extset::RootExtSet;
use crate::{Node, NodeValue, Renderer};
use crate::parser::cache::Cache;

#[derive(Debug)]
/// Root node of the AST.
pub struct Root {
    pub content: String,
    pub ext: RootExtSet,
}

impl Root {
    pub fn new(content: String) -> Self {
        Self {
            content,
            ext: RootExtSet::new(),
        }
    }
}

impl NodeValue for Root {
    fn render(
        &self,
        node: &Node,
        fmt: &mut dyn Renderer,
        options: &HashMap<String, String>,
        cache: &mut Cache,
    ) {
        fmt.contents(&node.children, options, cache);
    }
}
