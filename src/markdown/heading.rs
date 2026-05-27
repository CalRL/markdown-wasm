
use crate::parser::{Parse};
use crate::html::ToHtml;
use crate::parser::inline::parse_inlines;

#[derive(Debug)]
pub struct Heading<'a> {
    level: HeadingLevel,
    text: &'a str,
}

fn parse_heading(input: &str) -> Option<Heading> {
    let trimmed: &str = input.trim();
    let (hashes, text) = trimmed.split_once(' ')?;
    let level: HeadingLevel = match hashes {
        "#" => HeadingLevel::H1,
        "##" => HeadingLevel::H2,
        "###" => HeadingLevel::H3,
        "####" => HeadingLevel::H4,
        "#####" => HeadingLevel::H5,
        "######" => HeadingLevel::H6,
        _ => return None
    };
    Some(Heading {
        level,
        text: text.into(),
    })
}

#[derive(Debug)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6
}

impl TryFrom<&str> for HeadingLevel {
    type Error = ();

    fn try_from(hashes: &str) -> Result<HeadingLevel, Self::Error> {
        let level: HeadingLevel = match hashes {
            "#" => HeadingLevel::H1,
            "##" => HeadingLevel::H2,
            "###" => HeadingLevel::H3,
            "####" => HeadingLevel::H4,
            "#####" => HeadingLevel::H5,
            "######" => HeadingLevel::H6,
            _ => return Err(())
        };

        Ok(level)
    }
}

impl<'a> Parse<'a> for Heading<'a> {
    fn parse(input: &'a str) -> Option<Self>
    where
        Self: Sized
    {
        parse_heading(input)
    }
}

impl<'a> ToHtml for Heading<'a> {
    fn to_html(&self) -> String {
        let tag = match self.level {
            HeadingLevel::H1 => "h1",
            HeadingLevel::H2 => "h2",
            HeadingLevel::H3 => "h3",
            HeadingLevel::H4 => "h4",
            HeadingLevel::H5 => "h5",
            HeadingLevel::H6 => "h6",
        };

        format!(
            "<{tag}>{}</{tag}>",
            parse_inlines(self.text)
        )
    }
}
