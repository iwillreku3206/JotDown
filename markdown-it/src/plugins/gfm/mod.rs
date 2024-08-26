use crate::MarkdownIt;

use self::heading_anchors::HeadingAnchorOptions;

pub mod autolinks;
pub mod heading_anchors;
pub mod tag_filter;
pub mod tasklist;

pub fn add(md: &mut MarkdownIt) {
    autolinks::add(md);
    tag_filter::add(md);
    tasklist::add(md);
    heading_anchors::add_with_options(
        md,
        HeadingAnchorOptions {
            inner_html: String::new(),
            ..Default::default()
        },
    );
}
