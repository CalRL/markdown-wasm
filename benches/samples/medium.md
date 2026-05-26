
# Medium Markdown Sample

This file contains a more realistic mix of markdown features.

## Introduction

Markdown is commonly used for writing documentation, blog posts, READMEs, changelogs, and technical notes.

The goal of this sample is to include paragraphs, headings, lists, code blocks, links, and inline formatting.

## Features

- Headings
- Paragraphs
- Unordered lists
- Ordered lists
- Inline `code`
- Code blocks
- Links
- Bold and italic text

## Example List

1. Parse the markdown input.
2. Build an internal representation.
3. Render the output as HTML.
4. Return the final string.

## Code Example

```rust
pub fn parse_markdown(input: &str) -> String {
    let mut html = String::new();

    for line in input.lines() {
        if let Some(title) = line.strip_prefix("# ") {
            html.push_str("<h1>");
            html.push_str(title);
            html.push_str("</h1>");
        } else {
            html.push_str("<p>");
            html.push_str(line);
            html.push_str("</p>");
        }
    }

    html
}
```