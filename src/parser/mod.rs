pub mod helpers;
use crate::Block;
use crate::Heading;
use crate::html::{ToHtml, escape_html};

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
            _ => {"".into()}
        }
    }
}