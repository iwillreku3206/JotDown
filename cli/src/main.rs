use std::collections::HashMap;

fn main() {
    let mut parser = markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(&mut parser);
    markdown_it::plugins::extra::add(&mut parser);
    markdown_it::plugins::html::add(&mut parser);
    markdown_it::plugins::sourcepos::add(&mut parser);
    markdown_it::plugins::gfm::add(&mut parser);
    markdown_it::plugins::pandoc::add(&mut parser);
    println!(
        "{}",
        parser
            .parse(
                r#"
# Test

```mermaid
<script>alert(window.location)</script>
flowchart TD
    a-->b
````
        "#
            )
            .render(&HashMap::new(), &mut HashMap::new())
    );
}
