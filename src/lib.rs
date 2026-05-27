mod token;
pub mod markdown;
pub mod parser;
pub mod html;

use wasm_bindgen::prelude::*;
use markdown::heading::Heading;
use crate::markdown::list::{ListItem, OrderedListItem};
use crate::html::escape_html;
use crate::html::ToHtml;

#[wasm_bindgen]
pub fn parse_markdown(input: &str) -> String {
    parser::parse_blocks(input)
        .into_iter()
        .map(|block| block.to_html())
        .collect::<String>()
}

#[wasm_bindgen]
pub fn escape_html_text(input: &str) -> String {
    escape_html(input)
}

pub fn parse_blocks_only(input: &str) -> usize {
    parser::parse_blocks(input).len()
}

#[derive(Debug)]
enum Block<'a> {
    Heading(Heading<'a>),
    Paragraph(&'a str),
    UnorderedList(Vec<ListItem<'a>>),
    OrderedList(Vec<OrderedListItem<'a>>),
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}

pub(crate) enum BlockType {
    Blank,
    Heading,
    Paragraph,
    UnorderedList,
    OrderedList,
    CodeBlock,
}

#[cfg(test)]
mod tests {
    use super::parse_markdown;

    #[test]
    fn parses_multiline_paragraph_as_single_block() {
        let input = "first line\nsecond line\nthird line";
        let html = parse_markdown(input);

        assert_eq!(html, "<p>first line\nsecond line\nthird line</p>");
    }

    #[test]
    fn parses_fenced_code_block_with_language() {
        let input = "```rust\nfn main() {}\nprintln!(\"hi\");\n```";
        let html = parse_markdown(input);

        assert_eq!(
            html,
            "<pre><code class=\"language-rust\">\nfn main() {}\nprintln!(&quot;hi&quot;);\n</code></pre>"
        );
    }

    #[test]
    fn parses_unordered_list_item_with_continuation_lines() {
        let input = "- item one\n  continuation line\n\n- item two";
        let html = parse_markdown(input);

        assert_eq!(
            html,
            "<ul><li>item one\n  continuation line</li><li>item two</li></ul>"
        );
    }

    #[test]
    fn parses_ordered_list_item_with_continuation_lines() {
        let input = "1. first\n   detail\n2. second";
        let html = parse_markdown(input);

        assert_eq!(
            html,
            "<ol><li>first\n   detail</li><li>second</li></ol>"
        );
    }

    #[test]
    fn preserves_block_order_in_mixed_document() {
        let input = "# Title\n\nparagraph line\n\n- item";
        let html = parse_markdown(input);

        assert_eq!(html, "<h1>Title</h1><p>paragraph line</p><ul><li>item</li></ul>");
    }

    #[test]
    fn handles_unterminated_code_fence() {
        let input = "```\nlet x = 1;\nlet y = 2;";
        let html = parse_markdown(input);

        assert_eq!(html, "<pre><code>\nlet x = 1;\nlet y = 2;</code></pre>");
    }

    #[test]
    fn escapes_html_in_paragraph_and_heading() {
        let input = "# <Title>\n\n5 < 7 & 9 > 3";
        let html = parse_markdown(input);

        assert_eq!(html, "<h1>&lt;Title&gt;</h1><p>5 < 7 & 9 > 3</p>");
    }

    #[test]
    fn escapes_html_in_code_block_and_language() {
        let input = "```ru\"st\nif a < b {\n  println!(\"x & y\");\n}\n```";
        let html = parse_markdown(input);

        assert_eq!(
            html,
            "<pre><code class=\"language-ru&quot;st\">\nif a &lt; b {\n  println!(&quot;x &amp; y&quot;);\n}\n</code></pre>"
        );
    }

    #[test]
    fn parses_tilde_fence_code_block() {
        let input = "~~~js\nconst x = 1;\n~~~";
        let html = parse_markdown(input);

        assert_eq!(
            html,
            "<pre><code class=\"language-js\">\nconst x = 1;\n</code></pre>"
        );
    }

    #[test]
    fn supports_nested_indented_list_continuation() {
        let input = "- parent\n  child line\n    deeper child";
        let html = parse_markdown(input);

        assert_eq!(
            html,
            "<ul><li>parent\n  child line\n    deeper child</li></ul>"
        );
    }

    #[test]
    fn parses_multiple_paragraph_blocks_separated_by_blank_lines() {
        let input = "para one line 1\npara one line 2\n\n\npara two";
        let html = parse_markdown(input);

        assert_eq!(html, "<p>para one line 1\npara one line 2</p><p>para two</p>");
    }

    #[test]
    fn parses_inline_code_span_in_paragraph() {
        let input = "Use `cargo test` locally.";
        let html = parse_markdown(input);

        assert_eq!(html, "<p>Use <code>cargo test</code> locally.</p>");
    }

    #[test]
    fn parses_inline_link_in_paragraph() {
        let input = "Visit [Rust](https://www.rust-lang.org).";
        let html = parse_markdown(input);

        assert_eq!(
            html,
            "<p>Visit <a href=\"https://www.rust-lang.org\">Rust</a>.</p>"
        );
    }
}
