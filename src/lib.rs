mod parser;
mod token;
pub mod markdown;

use wasm_bindgen::prelude::*;
use markdown::heading::Heading;

#[wasm_bindgen]
pub fn parse_markdown(input: String) -> String {
    format!("<p>{}</>", input)
}



pub enum MarkdownToken {
    Heading(Heading),
    Paragraph(String),
    UnorderedList(Vec<String>),
    OrderedList(Vec<String>),
    CodeBlock { language: Option<String>, code: String },
    BlockQuote(String),
}

#[cfg(test)]
mod tests {
    use super::*;
}
