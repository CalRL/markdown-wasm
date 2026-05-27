use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn escape_html_into(input: &str, out: &mut String) {
    let mut last = 0;

    for (i, ch) in input.char_indices() {
        let replacement = replace(ch);
        if let Some(rep) = replacement {
            out.push_str(&input[last..i]);
            out.push_str(rep);
            last = i + ch.len_utf8();
        }
    }

    out.push_str(&input[last..]);
} 

fn replace<'a>(input: char) -> Option<&'a str>{
    let replacement: &str = match input {
        '&' => "&amp;",
        '<' => "&lt;",
        '>' => "&gt;",
        '"' => "&quot;",
        '\'' => "&#39;",
        _ => return None
    };

    Some(replacement)
}

pub trait ToHtml {
    fn to_html(&self) -> String;
}