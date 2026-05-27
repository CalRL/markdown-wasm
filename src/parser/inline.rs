pub(crate) fn parse_inlines(text: &str) -> String {
    enum InlineMatch<'a> {
        CodeSpan {
            code_start: usize,
            code_end: usize,
            next_index: usize,
        },
        Link {
            label_start: usize,
            label_end: usize,
            url_start: usize,
            url_end: usize,
            next_index: usize,
        },
        Text(&'a str),
    }

    fn match_inline(text: &str, i: usize) -> InlineMatch<'_> {
        let rest = &text[i..];

        if rest.starts_with('`') {
            let mut ticks = 0;
            for ch in rest.chars() {
                if ch == '`' {
                    ticks += ch.len_utf8();
                } else {
                    break;
                }
            }

            let open = &text[i..i + ticks];
            let after_open = i + ticks;

            if let Some(close_rel) = text[after_open..].find(open) {
                let code_start = after_open;
                let code_end = after_open + close_rel;
                return InlineMatch::CodeSpan {
                    code_start,
                    code_end,
                    next_index: code_end + ticks,
                };
            }
        }

        if rest.starts_with('[') {
            if let Some(text_close_rel) = rest.find(']') {
                let text_close = i + text_close_rel;
                let after_text = text_close + 1;

                if after_text < text.len() && text[after_text..].starts_with('(') {
                    let url_start = after_text + 1;
                    if let Some(url_close_rel) = text[url_start..].find(')') {
                        let url_close = url_start + url_close_rel;

                        return InlineMatch::Link {
                            label_start: i + 1,
                            label_end: text_close,
                            url_start,
                            url_end: url_close,
                            next_index: url_close + 1,
                        };
                    }
                }
            }
        }

        let ch = rest.chars().next().unwrap();
        InlineMatch::Text(&text[i..i + ch.len_utf8()])
    }

    let mut out = String::with_capacity(text.len());
    let mut i = 0;

    while i < text.len() {
        match match_inline(text, i) {
            InlineMatch::CodeSpan {
                code_start,
                code_end,
                next_index,
            } => {
                out.push_str("<code>");
                out.push_str(&text[code_start..code_end]);
                out.push_str("</code>");
                i = next_index;
            }
            InlineMatch::Link {
                label_start,
                label_end,
                url_start,
                url_end,
                next_index,
            } => {
                out.push_str("<a href=\"");
                out.push_str(&text[url_start..url_end]);
                out.push_str("\">");
                out.push_str(&text[label_start..label_end]);
                out.push_str("</a>");
                i = next_index;
            }
            InlineMatch::Text(segment) => {
                out.push_str(segment);
                i += segment.len();
            }
        }
    }

    out
}
