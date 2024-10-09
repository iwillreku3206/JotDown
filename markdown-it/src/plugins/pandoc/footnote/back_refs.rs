//! Plugin to add anchor(s) to footnote definitions,
//! with links back to the reference(s).
//!
//! ```rust
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(parser);
//! markdown_it_footnote::references::add(parser);
//! markdown_it_footnote::definitions::add(parser);
//! markdown_it_footnote::back_refs::add(parser);
//! let root = parser.parse("[^label]\n\n[^label]: This is a footnote");
//! let mut names = vec![];
//! root.walk(|node,_| { names.push(node.name()); });
//! assert_eq!(names, vec![
//! "markdown_it::parser::core::root::Root",
//! "markdown_it::plugins::cmark::block::paragraph::Paragraph",
//! "markdown_it_footnote::references::FootnoteReference",
//! "markdown_it_footnote::definitions::FootnoteDefinition",
//! "markdown_it::plugins::cmark::block::paragraph::Paragraph",
//! "markdown_it::parser::inline::builtin::skip_text::Text",
//! "markdown_it_footnote::back_refs::FootnoteRefAnchor",
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
    md.add_rule::<FootnoteBackrefRule>();
}

#[derive(Debug)]
pub struct FootnoteRefAnchor {
    pub ref_ids: Vec<usize>,
}
impl NodeValue for FootnoteRefAnchor {
    fn render(&self, _: &Node, fmt: &mut dyn crate::Renderer, _options: &HashMap<String, String>, cache: &mut HashMap<String, String>) {
        for ref_id in self.ref_ids.iter() {
            fmt.text(" ");
            fmt.open(
                "a",
                &[
                    ("href", format!("#fnref{}", ref_id)),
                    ("class", String::from("footnote-backref")),
                ],
            );
            // # ↩ with escape code to prevent display as Apple Emoji on iOS
            fmt.text("\u{21a9}\u{FE0E}");
            fmt.close("a");
        }
    }
}

// This is an extension for the markdown parser.
struct FootnoteBackrefRule;

impl CoreRule for FootnoteBackrefRule {
    fn run(root: &mut Node, _: &MarkdownIt) {
        // TODO this seems very cumbersome
        // but it is also how the markdown_it::InlineParserRule works
        let data = root.cast_mut::<Root>().unwrap();
        let root_ext = std::mem::take(&mut data.ext);
        let map = match root_ext.get::<FootnoteMap>() {
            Some(map) => map,
            None => return,
        };

        // walk through the AST and add backref anchors to footnote definitions
        root.walk_mut(|node, _| {
            if let Some(def_node) = node.cast::<FootnoteDefinition>() {
                let ref_ids = {
                    match def_node.def_id {
                        Some(def_id) => map.referenced_by(def_id),
                        None => Vec::new(),
                    }
                };
                if !ref_ids.is_empty() {
                    // if the final child is a paragraph node,
                    // append the anchor to its children,
                    // otherwise simply append to the end of the node children
                    match node.children.last_mut() {
                        Some(last) => {
                            if last.is::<Paragraph>() {
                                last.children.push(Node::new(FootnoteRefAnchor { ref_ids }));
                            } else {
                                node.children.push(Node::new(FootnoteRefAnchor { ref_ids }));
                            }
                        }
                        _ => {
                            node.children.push(Node::new(FootnoteRefAnchor { ref_ids }));
                        }
                    }
                }
            }
        });

        let data = root.cast_mut::<Root>().unwrap();
        data.ext = root_ext;
    }
}
