pub struct Heading {
    level: HeadingLevel,
    text: String,
}

impl Heading {
    fn parse(line: &str) -> Option<Heading> {
        parse(line)
    }
}

fn parse(input: &str) -> Option<Heading> {
    let trimmed: &str = input.trim();
    let (hashes, text) = trimmed.split_once(' ')?;
    let level: HeadingLevel = match hashes {
        "#" => HeadingLevel::H1,
        "##" => HeadingLevel::H2,
        "###" => HeadingLevel::H3,
        _ => return None
    };
    Some(Heading {
        level,
        text: text.into(),
    })
}

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