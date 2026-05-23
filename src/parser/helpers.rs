use crate::{BlockType, Heading};
use crate::markdown::list::{ListItem, OrderedListItem};
use crate::parser::Parse;

fn is_heading(line: &str) -> bool {
    Heading::parse(line).is_some()
}

fn is_code_block(line: &str) -> bool {
    line.starts_with("```") || line.starts_with("~~~")
}

fn identify_block_type(line: &str) -> BlockType {
    let start = line.trim_start();

    if start.trim().is_empty() {
        return BlockType::Blank;
    }

    if Heading::parse(start).is_some() {
        return BlockType::Heading;
    }

    if is_code_block(start) {
        return BlockType::CodeBlock;
    }

    if ListItem::parse(start).is_some() {
        return BlockType::UnorderedList;
    }

    if OrderedListItem::parse(start).is_some() {
        return BlockType::OrderedList;
    }

    BlockType::Paragraph
}