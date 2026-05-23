mod token;
pub mod markdown;
pub mod parser;
pub mod html;

use wasm_bindgen::prelude::*;
use markdown::heading::Heading;
use crate::markdown::list::{ListItem, OrderedListItem};
use crate::html::ToHtml;

#[wasm_bindgen]
pub fn parse_markdown(input: String) -> String {
    input
        .lines()
        .filter_map(parser::parse_line)
        .map(|block| block.to_html())
        .collect::<String>()
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

enum BlockType {
    Blank,
    Heading,
    Paragraph,
    UnorderedList,
    OrderedList,
    CodeBlock
}

#[cfg(test)]
mod tests {}
