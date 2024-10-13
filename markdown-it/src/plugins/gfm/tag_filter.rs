// COPYRIGHT DISCLAIMER: This file is taken from the
// markdown-it-rust/markdown-it-rs repository
//
// This code is licensed under the Apache License 2.0.
// You can read the full license text here:
//
// https://www.apache.org/licenses/LICENSE-2.0

use regex::Regex;


use crate::{
    parser::{core::CoreRule, inline::builtin::InlineParserRule},
    plugins::html::{html_block::HtmlBlock, html_inline::HtmlInline},
    MarkdownIt, Node,
};

/// Implement the Disallowed Raw HTML (tagfilter) rule
struct TagFilter;
impl CoreRule for TagFilter {
    fn run(root: &mut Node, _md: &MarkdownIt) {
        let regex =
            Regex::new(r#"<(?i)(iframe|noembed|noframes|plaintext|script|title|textarea|xmp)"#)
                .unwrap();
        root.walk_mut(|node, _| {
            if let Some(value) = node.cast_mut::<HtmlBlock>() {
                value.content = regex.replace_all(&value.content, "&lt;$1").to_string();
            }
            if let Some(value) = node.cast_mut::<HtmlInline>() {
                value.content = regex.replace_all(&value.content, "&lt;$1").to_string();
            }
        });
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<TagFilter>().after::<InlineParserRule>();
}
