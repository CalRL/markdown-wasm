pub mod helpers;
use crate::Block;
use crate::BlockType;
use crate::markdown::heading::Heading;
use crate::markdown::list::{ListItem, OrderedListItem};
use crate::html::{ToHtml, escape_html};
use crate::parser::helpers::{identify_block_type, is_ordered_list_item, is_unordered_list_item};

pub trait Parse<'a> {
    fn parse(input: &'a str) -> Option<Self>
    where Self: Sized;
}

impl<'a> ToHtml for Block<'a> {
    fn to_html(&self) -> String {
        match self {
            Block::Heading(heading) => heading.to_html(),
            Block::Paragraph(text) => format!("<p>{}</p>", text),
            Block::UnorderedList(items) => {
                let items_html = items
                    .iter()
                    .map(|item| item.to_html())
                    .collect::<String>();

                format!("<ul>{}</ul>", items_html)
            }

            Block::OrderedList(items) => {
                let items_html = items
                    .iter()
                    .map(|item| item.to_html())
                    .collect::<String>();

                format!("<ol>{}</ol>", items_html)
            }
            Block::CodeBlock { language, content } => {
                match language {
                    Some(lang) => format!(
                        r#"<pre><code class="language-{}">{}</code></pre>"#,
                        escape_html(lang),
                        escape_html(content)
                    ),
                    None => format!(
                        "<pre><code>{}</code></pre>",
                        escape_html(content)
                    ),
                }
            }
        }
    }
}

pub struct BlockParser<'a> {
    input: &'a str,
    lines: Vec<IndexedLine<'a>>,
    blocks: Vec<Block<'a>>,
    line_index: usize,
    lines_len: usize
}

impl<'a> BlockParser<'a> {
    
}

pub(crate) fn parse_blocks<'a>(input: &'a str) -> Vec<Block<'a>> {
    let lines = indexed_lines(input);
    let lines_len = lines.len();
    let n = lines.len();
    let mut state: BlockParser = BlockParser {
        input,
        lines,
        blocks: Vec::new(),
        line_index: 0,
        lines_len,
    };
    while state.line_index < n {
        let line_info = &state.lines[state.line_index];
        let line = line_info.text;
        match identify_block_type(line) {
            BlockType::Blank => helpers::handle_empty(&mut state.line_index),
            BlockType::CodeBlock => helpers::handle_code_block(&mut state),
            BlockType::Heading => helpers::handle_heading(line, &mut state.blocks, &mut state.line_index),
            BlockType::UnorderedList => helpers::handle_unordered_list(&mut state),
            BlockType::OrderedList => helpers::handle_ordered_list(&mut state),
            BlockType::Paragraph => helpers::handle_paragraph(&mut state)
        }
    }

    state.blocks
}

fn parse_fence_start(line: &str) -> Option<(char, Option<&str>)> {
    let trimmed = line.trim_start();
    if !(trimmed.starts_with("```") || trimmed.starts_with("~~~")) {
        return None;
    }

    let fence = trimmed.chars().next()?;
    let language = trimmed[3..].trim();
    let language = if language.is_empty() {
        None
    } else {
        Some(language)
    };

    Some((fence, language))
}

fn is_matching_fence_end(line: &str, fence: char) -> bool {
    let trimmed = line.trim_start();
    if fence == '`' {
        trimmed.starts_with("```")
    } else {
        trimmed.starts_with("~~~")
    }
}

struct UnorderedItemStart {
    indent: usize,
    marker: char,
    content_offset: usize,
}

struct OrderedItemStart {
    indent: usize,
    number: usize,
    content_offset: usize,
}

fn parse_unordered_start(line: &str) -> Option<UnorderedItemStart> {
    let bytes = line.as_bytes();
    let mut idx = 0;

    while idx < bytes.len() && bytes[idx] == b' ' {
        idx += 1;
    }

    if idx >= bytes.len() {
        return None;
    }

    let marker_byte = bytes[idx];
    if marker_byte != b'-' && marker_byte != b'*' && marker_byte != b'+' {
        return None;
    }
    let marker = marker_byte as char;

    idx += 1;
    if idx >= bytes.len() {
        return None;
    }

    let ws = bytes[idx];
    if ws != b' ' && ws != b'\t' {
        return None;
    }

    idx += 1;
    while idx < bytes.len() && (bytes[idx] == b' ' || bytes[idx] == b'\t') {
        idx += 1;
    }

    let indent = count_leading_spaces(line);
    let content_offset = idx;

    Some(UnorderedItemStart {
        indent,
        marker,
        content_offset,
    })
}

fn parse_ordered_start(line: &str) -> Option<OrderedItemStart> {
    let bytes = line.as_bytes();
    let mut idx = 0;

    while idx < bytes.len() && bytes[idx] == b' ' {
        idx += 1;
    }

    let number_start = idx;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        idx += 1;
    }

    if idx == number_start {
        return None;
    }

    if idx >= bytes.len() || bytes[idx] != b'.' {
        return None;
    }

    let number = line[number_start..idx].parse::<usize>().ok()?;

    idx += 1;
    if idx >= bytes.len() || (bytes[idx] != b' ' && bytes[idx] != b'\t') {
        return None;
    }

    idx += 1;
    while idx < bytes.len() && (bytes[idx] == b' ' || bytes[idx] == b'\t') {
        idx += 1;
    }

    let indent = count_leading_spaces(line);

    Some(OrderedItemStart {
        indent,
        number,
        content_offset: idx,
    })
}

fn count_leading_spaces(line: &str) -> usize {
    line.chars().take_while(|c| *c == ' ').count()
}

pub(crate) struct IndexedLine<'a> {
    start: usize,
    end: usize,
    text: &'a str,
}

fn indexed_lines(input: &str) -> Vec<IndexedLine<'_>> {
    let mut lines = Vec::new();
    let mut start = 0;

    for segment in input.split_inclusive('\n') {
        let line_end = start + segment.len();
        let without_newline = segment.strip_suffix('\n').unwrap_or(segment);
        let text = without_newline.strip_suffix('\r').unwrap_or(without_newline);
        let end = start + text.len();

        lines.push(IndexedLine { start, end, text });
        start = line_end;
    }

    if input.is_empty() {
        return lines;
    }

    if !input.ends_with('\n') {
        if let Some(last) = lines.last_mut() {
            last.end = input.len();
            last.text = &input[last.start..last.end];
        }
    }

    lines
}
