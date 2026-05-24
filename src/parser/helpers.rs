use crate::{Block, BlockType, Heading};
use crate::markdown::list::{ListItem, OrderedListItem};
use crate::parser::{is_matching_fence_end, parse_fence_start, IndexedLine, Parse};

pub(crate) fn is_heading(line: &str) -> bool {
    Heading::parse(line).is_some()
}

pub(crate) fn is_code_block(line: &str) -> bool {
    line.starts_with("```") || line.starts_with("~~~")
}

pub(crate) fn is_unordered_list_item(line: &str) -> bool {
    ListItem::parse(line).is_some()
}

pub(crate) fn is_ordered_list_item(line: &str) -> bool {
    OrderedListItem::parse(line).is_some()
}

pub(crate) fn identify_block_type(line: &str) -> BlockType {
    let start = line.trim_start();

    if start.trim().is_empty() {
        return BlockType::Blank;
    }

    if is_heading(start) {
        return BlockType::Heading;
    }

    if is_code_block(start) {
        return BlockType::CodeBlock;
    }

    if is_unordered_list_item(start) {
        return BlockType::UnorderedList;
    }

    if is_ordered_list_item(start) {
        return BlockType::OrderedList;
    }

    BlockType::Paragraph
}


pub(crate) fn handle_empty(mut i: &mut usize) {
    *i += 1;
}

pub(crate) fn handle_paragraph<'a>(input: &'a str, lines: &Vec<IndexedLine>, mut blocks: &mut Vec<Block<'a>>, i: &mut usize) {
    let paragraph_start = lines[*i].start;
    let mut paragraph_end = lines[*i].end;
    *i += 1;

    while *i < lines.len() {
        let next_line = lines[*i].text;
        if !matches!(identify_block_type(next_line), BlockType::Paragraph) {
            break;
        }

        paragraph_end = lines[*i].end;
        *i += 1;
    }

    blocks.push(Block::Paragraph(input[paragraph_start..paragraph_end].trim()));
}
pub(crate) fn handle_unordered_list() {}
pub(crate) fn handle_ordered_list() {}
pub(crate) fn handle_code_block<'a>(input: &'a str, lines: &Vec<IndexedLine>, line: &'a str, mut blocks: &mut Vec<Block<'a>>, i: &mut usize) {

    let Some((fence, language)) = parse_fence_start(line) else {
        *i += 1;
        return;
    };

    let content_start = lines[*i].end;
    *i += 1;
    let mut closing_start = input.len();

    while *i < lines.len() {
        let code_line = lines[*i].text;
        if is_matching_fence_end(code_line, fence) {
            closing_start = lines[*i].start;
            *i += 1;
            break;
        }
        *i += 1;
    }

    let content = if content_start <= closing_start {
        &input[content_start..closing_start]
    } else {
        ""
    };

    blocks.push(Block::CodeBlock { language, content });
}

pub(crate) fn handle_heading<'a>(line: &'a str, mut blocks: &mut Vec<Block<'a>>, i: &mut usize) {
    if let Some(heading) = Heading::parse(line) {
        blocks.push(Block::Heading(heading));
    }
    *i += 1;
}