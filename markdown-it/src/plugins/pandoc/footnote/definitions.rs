//! Plugin to parse footnote definitions
//!
//! ```rust
//! let parser = &mut markdown_it::MarkdownIt::new();
//! markdown_it::plugins::cmark::add(parser);
//! markdown_it_footnote::definitions::add(parser);
//! let root = parser.parse("[^label]: This is a footnote");
//! let mut names = vec![];
//! root.walk(|node,_| { names.push(node.name()); });
//! assert_eq!(names, vec![
//! "markdown_it::parser::core::root::Root",
//! "markdown_it_footnote::definitions::FootnoteDefinition",
//! "markdown_it::plugins::cmark::block::paragraph::Paragraph",
//! "markdown_it::parser::inline::builtin::skip_text::Text",
//! ]);
//! ```

use std::collections::HashMap;

use crate::parser::block::{BlockRule, BlockState};
use crate::plugins::cmark::block::reference::ReferenceScanner;
use crate::{MarkdownIt, Node, NodeValue, Renderer};

use super::FootnoteMap;

/// Add the footnote definition plugin to the parser
pub fn add(md: &mut MarkdownIt) {
    // insert this rule into block subparser
    md.block
        .add_rule::<FootnoteDefinitionScanner>()
        .before::<ReferenceScanner>();
}

#[derive(Debug)]
/// AST node for footnote definition
pub struct FootnoteDefinition {
    pub label: Option<String>,
    pub def_id: Option<usize>,
    pub inline: bool,
}

impl NodeValue for FootnoteDefinition {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer, options: &HashMap<String, String>) {
        let mut attrs = node.attrs.clone();
        if let Some(def_id) = self.def_id {
            attrs.push(("id", format!("fn{}", def_id)));
        }
        attrs.push(("class", "footnote-item".into()));

        fmt.cr();
        fmt.open("li", &attrs);
        fmt.contents(&node.children, options);
        fmt.close("li");
        fmt.cr();
    }
}

/// An extension for the block subparser.
struct FootnoteDefinitionScanner;

impl FootnoteDefinitionScanner {
    fn is_def(state: &mut BlockState) -> Option<(String, usize)> {
        if state.line_indent(state.line) >= state.md.max_indent {
            return None;
        }

        let mut chars = state.get_line(state.line).chars();

        // check line starts with the correct syntax
        let Some('[') = chars.next() else {
            return None;
        };
        let Some('^') = chars.next() else {
            return None;
        };

        // gather the label
        let mut label = String::new();
        // The labels in footnote references may not contain spaces, tabs, or newlines.
        // Backslash escapes form part of the label and do not escape anything
        loop {
            match chars.next() {
                None => return None,
                Some(']') => {
                    if let Some(':') = chars.next() {
                        break;
                    } else {
                        return None;
                    }
                }
                Some(' ') => return None,
                Some(c) => label.push(c),
            }
        }
        if label.is_empty() {
            return None;
        }
        // get number of spaces to next non-space character
        let mut spaces = 0;
        loop {
            match chars.next() {
                None => break,
                Some(' ') => spaces += 1,
                Some('\t') => spaces += 1, // spaces += 4 - spaces % 4,
                Some(_) => break,
            }
        }
        Some((label, spaces))
    }
}

impl BlockRule for FootnoteDefinitionScanner {
    fn check(state: &mut BlockState) -> Option<()> {
        // can interrupt a block elements,
        // but only if its a child of another footnote definition
        // TODO I think strictly only paragraphs should be interrupted, but this is not yet possible in markdown-it.rs
        if state.node.is::<FootnoteDefinition>() && Self::is_def(state).is_some() {
            return Some(());
        }
        None
    }

    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        let (label, spaces) = Self::is_def(state)?;

        // record the footnote label, so we can match references to it later
        let foot_map = state.root_ext.get_or_insert_default::<FootnoteMap>();
        let def_id = foot_map.add_def(&label);

        // temporarily set the current node to the footnote definition
        // so child nodes are added to it
        let new_node = Node::new(FootnoteDefinition {
            label: Some(label.clone()),
            def_id,
            inline: false,
        });
        let old_node = std::mem::replace(&mut state.node, new_node);

        // store the current line and its offsets, so we can restore later
        let first_line = state.line;
        let first_line_offsets = state.line_offsets[first_line].clone();

        // temporarily change the first line offsets to account for the footnote label
        // TODO this is not quite the same as pandoc where spaces >= 8 is code block (here >= 4)
        state.line_offsets[first_line].first_nonspace += "[^]:".len() + label.len() + spaces;
        state.line_offsets[first_line].indent_nonspace += "[^]:".len() as i32 + spaces as i32;
        // tokenize with a +4 space indent
        state.blk_indent += 4;
        state.md.block.tokenize(state);
        state.blk_indent -= 4;

        // get the number of lines the footnote definition occupies
        let num_lines = state.line - first_line;

        // restore the first line and its offsets
        state.line_offsets[first_line] = first_line_offsets;
        state.line = first_line;

        // restore the original node and return the footnote and number of lines it occupies
        Some((std::mem::replace(&mut state.node, old_node), num_lines))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let parser = &mut markdown_it::MarkdownIt::new();
        markdown_it::plugins::cmark::add(parser);
        markdown_it::plugins::sourcepos::add(parser);
        add(parser);
        let node = parser.parse("[^note]: a\n\nhallo\nthere\n");
        // println!("{:#?}", node);
        assert!(node.children.first().unwrap().is::<FootnoteDefinition>());

        // let text = node.render();
        // assert_eq!(text, "hallo\n")
    }
}
