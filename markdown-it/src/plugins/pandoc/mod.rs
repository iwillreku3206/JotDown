use crate::MarkdownIt;

pub mod deflist;
pub mod footnote;

pub fn add(md: &mut MarkdownIt) {
    deflist::add(md);
    footnote::add(md);
}
