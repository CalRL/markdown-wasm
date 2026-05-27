use crate::html::{escape_html, ToHtml};
use crate::parser::{Parse};
use crate::parser::inline::parse_inlines;

#[derive(Debug)]
pub struct ListItem<'a> {
    pub indent: usize,
    pub marker: char,
    pub content: &'a str,
}

#[derive(Debug)]
pub struct OrderedListItem<'a> {
    pub indent: usize,
    pub number: usize,
    pub content: &'a str,
}

impl<'a> Parse<'a> for ListItem<'a> {
    fn parse(input: &'a str) -> Option<Self>
    where
        Self: Sized,
    {
        parse_unordered_list_item(input)
    }
}

impl<'a> Parse<'a> for OrderedListItem<'a> {
    fn parse(input: &'a str) -> Option<Self>
    where
        Self: Sized,
    {
        parse_ordered_list_item(input)
    }
}

impl<'a> ToHtml for ListItem<'a> {
    fn to_html(&self) -> String {
        format!("<li>{}</li>", parse_inlines(self.content))
    }
}

impl<'a> ToHtml for OrderedListItem<'a> {
    fn to_html(&self) -> String {
        format!("<li>{}</li>", parse_inlines(self.content))
    }
}

fn parse_unordered_list_item<'a>(line: &'a str) -> Option<ListItem<'a>> {
    let indent = line.chars().take_while(|c| *c == ' ').count();
    let trimmed = line.trim_start();

    let mut chars = trimmed.chars();

    match (chars.next(), chars.next()) {
        (Some(marker @ ('-' | '*' | '+')), Some(' ' | '\t')) => {
            Some(ListItem {
                indent,
                marker,
                content: chars.as_str().trim_start(),
            })
        }
        _ => None,
    }
}

fn parse_ordered_list_item<'a>(line: &'a str) -> Option<OrderedListItem<'a>> {
    let indent = line.chars().take_while(|c| *c == ' ').count();
    let trimmed = line.trim_start();

    let dot_index = trimmed.find('.')?;

    let number_part = &trimmed[..dot_index];

    if number_part.is_empty() || !number_part.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let after_dot = &trimmed[dot_index + 1..];

    if !after_dot.starts_with(' ') && !after_dot.starts_with('\t') {
        return None;
    }

    let number = number_part.parse::<usize>().ok()?;

    Some(OrderedListItem {
        indent,
        number,
        content: after_dot.trim_start(),
    })
}