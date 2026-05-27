use crate::{Block, BlockType, Heading};
use crate::markdown::list::{ListItem, OrderedListItem};
use crate::parser::{count_leading_spaces, is_matching_fence_end, parse_fence_start, parse_ordered_start, parse_unordered_start, BlockParser, IndexedLine, Parse};

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


pub(crate) fn handle_empty(state: &mut usize) {
    *state += 1;
}

pub(crate) fn handle_paragraph<'a>(state: &mut BlockParser) {
    let paragraph_start = state.lines[state.line_index].start;
    let mut paragraph_end = state.lines[state.line_index].end;
    state.line_index += 1;

    while state.line_index < state.lines_len {
        let next_line = state.lines[state.line_index].text;
        if !matches!(identify_block_type(next_line), BlockType::Paragraph) {
            break;
        }

        paragraph_end = state.lines[state.line_index].end;
        state.line_index += 1;
    }

    state.blocks.push(Block::Paragraph(state.input[paragraph_start..paragraph_end].trim()));
}

pub(crate) fn handle_code_block<'a>(state: &mut BlockParser) {

    let Some((fence, language)) = parse_fence_start(&state.lines[state.line_index].text) else {
        state.line_index += 1;
        return;
    };

    let content_start = state.lines[state.line_index + 1].start;
    state.line_index += 1;
    let mut closing_start = state.input.len();

    while state.line_index < state.lines_len {
        let code_line = state.lines[state.line_index].text;
        if is_matching_fence_end(code_line, fence) {
            closing_start = state.lines[state.line_index].start;
            state.line_index += 1;
            break;
        }
        state.line_index += 1;
    }

    let content: &str = if content_start <= closing_start {
        &state.input[content_start..closing_start]
    } else {
        ""
    };

    state.blocks.push(Block::CodeBlock { language, content });
}

pub(crate) fn handle_heading<'a>(line: &'a str, mut blocks: &mut Vec<Block<'a>>, i: &mut usize) {
    if let Some(heading) = Heading::parse(line) {
        blocks.push(Block::Heading(heading));
    }
    *i += 1;
}

pub(crate) fn handle_ordered_list(state: &mut BlockParser) {
    let mut items = Vec::new();

    while state.line_index < state.lines_len {
        let Some(start) = parse_ordered_start(state.lines[state.line_index].text) else {
            break;
        };

        let content_start = state.lines[state.line_index].start + start.content_offset;
        let mut content_end = state.lines[state.line_index].end;
        state.line_index += 1;

        while state.line_index < state.lines_len {
            let next_line_info = &state.lines[state.line_index];
            let next_line = next_line_info.text;
            let next_indent = count_leading_spaces(next_line);

            if is_ordered_list_item(next_line) || is_unordered_list_item(next_line) {
                break;
            }

            if next_line.trim().is_empty() {
                content_end = next_line_info.end;
                state.line_index += 1;
                continue;
            }

            if next_indent > start.indent {
                content_end = next_line_info.end;
                state.line_index += 1;
                continue;
            }

            break;
        }

        items.push(OrderedListItem {
            indent: start.indent,
            number: start.number,
            content: state.input[content_start..content_end].trim_end(),
        });
    }

    state.blocks.push(Block::OrderedList(items));
}

pub(crate) fn handle_unordered_list(state: &mut BlockParser) {
    let mut items = Vec::new();

    while state.line_index < state.lines_len {
        let Some(start) = parse_unordered_start(state.lines[state.line_index].text) else {
            break;
        };

        let content_start = state.lines[state.line_index].start + start.content_offset;
        let mut content_end = state.lines[state.line_index].end;
        state.line_index += 1;

        while state.line_index < state.lines_len {
            let next_line_info = &state.lines[state.line_index];
            let next_line = next_line_info.text;
            let next_indent = count_leading_spaces(next_line);

            if is_unordered_list_item(next_line) || is_ordered_list_item(next_line) {
                break;
            }

            if next_line.trim().is_empty() {
                content_end = next_line_info.end;
                state.line_index += 1;
                continue;
            }

            if next_indent > start.indent {
                content_end = next_line_info.end;
                state.line_index += 1;
                continue;
            }

            break;
        }

        items.push(ListItem {
            indent: start.indent,
            marker: start.marker,
            content: state.input[content_start..content_end].trim_end(),
        });
    }

    state.blocks.push(Block::UnorderedList(items));
}