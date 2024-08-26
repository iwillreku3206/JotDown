use std::collections::HashMap;

fn main() {
    let mut parser = markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(&mut parser);
    markdown_it::plugins::extra::add(&mut parser);
    markdown_it::plugins::html::add(&mut parser);
    markdown_it::plugins::sourcepos::add(&mut parser);
    markdown_it::plugins::gfm::add(&mut parser);
    markdown_it::plugins::pandoc::add(&mut parser);
    println!("{}", parser.parse("# Test 1").render(&HashMap::new()));
}
