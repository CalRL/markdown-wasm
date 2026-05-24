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

pub(crate) fn parse_blocks<'a>(input: &'a str) -> Vec<Block<'a>> {
    let mut blocks = Vec::new();
    let lines = indexed_lines(input);
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].text;
        match identify_block_type(line) {
            BlockType::Blank => {
                helpers::handle_empty(&mut i);
            }
            BlockType::CodeBlock => {
                helpers::handle_code_block(input, &lines, line, &mut blocks, &mut i);
            }
            BlockType::Heading => {
                helpers::handle_heading(line, &mut blocks, &mut i)
            }
            BlockType::UnorderedList => {
                let mut items = Vec::new();

                while i < lines.len() {
                    let Some(start) = parse_unordered_start(lines[i].text) else {
                        break;
                    };

                    let content_start = lines[i].start + start.content_offset;
                    let mut content_end = lines[i].end;
                    i += 1;

                    while i < lines.len() {
                        let next_line = lines[i].text;
                        let next_indent = count_leading_spaces(next_line);

                        if is_unordered_list_item(next_line) || is_ordered_list_item(next_line) {
                            break;
                        }

                        if next_line.trim().is_empty() {
                            content_end = lines[i].end;
                            i += 1;
                            continue;
                        }

                        if next_indent > start.indent {
                            content_end = lines[i].end;
                            i += 1;
                            continue;
                        }

                        break;
                    }

                    items.push(ListItem {
                        indent: start.indent,
                        marker: start.marker,
                        content: input[content_start..content_end].trim_end(),
                    });
                }

                blocks.push(Block::UnorderedList(items));
            }
            BlockType::OrderedList => {
                let mut items = Vec::new();

                while i < lines.len() {
                    let Some(start) = parse_ordered_start(lines[i].text) else {
                        break;
                    };

                    let content_start = lines[i].start + start.content_offset;
                    let mut content_end = lines[i].end;
                    i += 1;

                    while i < lines.len() {
                        let next_line = lines[i].text;
                        let next_indent = count_leading_spaces(next_line);

                        if is_ordered_list_item(next_line) || is_unordered_list_item(next_line) {
                            break;
                        }

                        if next_line.trim().is_empty() {
                            content_end = lines[i].end;
                            i += 1;
                            continue;
                        }

                        if next_indent > start.indent {
                            content_end = lines[i].end;
                            i += 1;
                            continue;
                        }

                        break;
                    }

                    items.push(OrderedListItem {
                        indent: start.indent,
                        number: start.number,
                        content: input[content_start..content_end].trim_end(),
                    });
                }

                blocks.push(Block::OrderedList(items));
            }
            BlockType::Paragraph => {
                helpers::handle_paragraph(input, &lines, &mut blocks, &mut i)
            }
        }
    }

    blocks
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
    let indent = count_leading_spaces(line);
    let trimmed = line.trim_start_matches(' ');
    let mut chars = trimmed.chars();

    let marker = chars.next()?;
    if marker != '-' && marker != '*' && marker != '+' {
        return None;
    }

    let ws = chars.next()?;
    if ws != ' ' && ws != '\t' {
        return None;
    }

    let marker_offset = indent + marker.len_utf8() + ws.len_utf8();
    let content_offset = marker_offset + trimmed[marker_offset - indent..].chars().take_while(|c| *c == ' ' || *c == '\t').map(char::len_utf8).sum::<usize>();

    Some(UnorderedItemStart {
        indent,
        marker,
        content_offset,
    })
}

fn parse_ordered_start(line: &str) -> Option<OrderedItemStart> {
    let indent = count_leading_spaces(line);
    let trimmed = line.trim_start_matches(' ');
    let dot_index = trimmed.find('.')?;
    let number_part = &trimmed[..dot_index];

    if number_part.is_empty() || !number_part.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let rest = &trimmed[dot_index + 1..];
    if !rest.starts_with(' ') && !rest.starts_with('\t') {
        return None;
    }

    let number = number_part.parse::<usize>().ok()?;
    let base = indent + dot_index + 1;
    let whitespace_len = rest.chars().take_while(|c| *c == ' ' || *c == '\t').map(char::len_utf8).sum::<usize>();

    Some(OrderedItemStart {
        indent,
        number,
        content_offset: base + whitespace_len,
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
