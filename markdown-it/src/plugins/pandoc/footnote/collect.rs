//! Plugin to collect footnote definitions,
//! removing duplicate/unreferenced ones,
//! and move them to be the last child of the root node.
//!
//! ```rust
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(parser);
//! markdown_it_footnote::references::add(parser);
//! markdown_it_footnote::definitions::add(parser);
//! markdown_it_footnote::collect::add(parser);
//! let root = parser.parse("[^label]\n\n[^label]: This is a footnote\n\n> quote");
//! let mut names = vec![];
//! root.walk(|node,_| { names.push(node.name()); });
//! assert_eq!(names, vec![
//! "markdown_it::parser::core::root::Root",
//! "markdown_it::plugins::cmark::block::paragraph::Paragraph",
//! "markdown_it_footnote::references::FootnoteReference",
//! "markdown_it::plugins::cmark::block::blockquote::Blockquote",
//! "markdown_it::plugins::cmark::block::paragraph::Paragraph",
//! "markdown_it::parser::inline::builtin::skip_text::Text",
//! "markdown_it_footnote::collect::FootnotesContainerNode",
//! "markdown_it_footnote::definitions::FootnoteDefinition",
//! "markdown_it::plugins::cmark::block::paragraph::Paragraph",
//! "markdown_it::parser::inline::builtin::skip_text::Text",
//! ]);
//! ```
use std::collections::HashMap;

use crate::{
    parser::core::{CoreRule, Root},
    plugins::cmark::block::paragraph::Paragraph,
    MarkdownIt, Node, NodeValue,
};

use super::{definitions::FootnoteDefinition, FootnoteMap};

pub fn add(md: &mut MarkdownIt) {
    // insert this rule into parser
    md.add_rule::<FootnoteCollectRule>();
}

#[derive(Debug)]
struct PlaceholderNode;
impl NodeValue for PlaceholderNode {}

#[derive(Debug)]
pub struct FootnotesContainerNode;
impl NodeValue for FootnotesContainerNode {
    fn render(
        &self,
        node: &Node,
        fmt: &mut dyn crate::Renderer,
        options: &HashMap<String, String>, cache: &mut HashMap<String, String>,
    ) {
        let mut attrs = node.attrs.clone();
        attrs.push(("class", "footnotes".into()));
        fmt.cr();
        fmt.self_close("hr", &[("class", "footnotes-sep".into())]);
        fmt.cr();
        fmt.open("section", &attrs);
        fmt.cr();
        fmt.open("ol", &[("class", "footnotes-list".into())]);
        fmt.cr();
        fmt.contents(&node.children, options, cache);
        fmt.cr();
        fmt.close("ol");
        fmt.cr();
        fmt.close("section");
        fmt.cr();
    }
}

// This is an extension for the markdown parser.
struct FootnoteCollectRule;

impl CoreRule for FootnoteCollectRule {
    // This is a custom function that will be invoked once per document.
    //
    // It has `root` node of the AST as an argument and may modify its
    // contents as you like.
    //
    fn run(root: &mut Node, _: &MarkdownIt) {
        // TODO this seems very cumbersome
        // but it is also how the markdown_it::InlineParserRule works
        let data = root.cast_mut::<Root>().unwrap();
        let root_ext = std::mem::take(&mut data.ext);
        let map = match root_ext.get::<FootnoteMap>() {
            Some(map) => map,
            None => return,
        };

        // walk through the AST and extract all footnote definitions
        let mut defs = vec![];
        root.walk_mut(|node, _| {
            // TODO could use drain_filter if it becomes stable: https://github.com/rust-lang/rust/issues/43244
            // defs.extend(
            //     node.children
            //         .drain_filter(|child| !child.is::<FootnoteDefinition>())
            //         .collect(),
            // );

            for child in node.children.iter_mut() {
                if child.is::<FootnoteDefinition>() {
                    let mut extracted = std::mem::replace(child, Node::new(PlaceholderNode));
                    match extracted.cast::<FootnoteDefinition>() {
                        Some(def_node) => {
                            // skip footnotes that are not referenced
                            match def_node.def_id {
                                Some(def_id) => {
                                    if map.referenced_by(def_id).is_empty() {
                                        continue;
                                    }
                                }
                                None => continue,
                            }
                            if def_node.inline {
                                // for inline footnotes,
                                // we need to wrap the definition's children in a paragraph
                                let mut para = Node::new(Paragraph);
                                std::mem::swap(&mut para.children, &mut extracted.children);
                                extracted.children = vec![para];
                            }
                        }
                        None => continue,
                    }
                    defs.push(extracted);
                }
            }
            node.children.retain(|child| !child.is::<PlaceholderNode>());
        });
        if defs.is_empty() {
            return;
        }

        // wrap the definitions in a container and append them to the root
        let mut wrapper = Node::new(FootnotesContainerNode);
        wrapper.children = defs;
        root.children.push(wrapper);

        let data = root.cast_mut::<Root>().unwrap();
        data.ext = root_ext;
    }
}
