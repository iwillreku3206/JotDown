use crate::MarkdownIt;

pub mod katex;

pub fn add(md: &mut MarkdownIt) {
    katex::add(md);
}
