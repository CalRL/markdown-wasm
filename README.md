# markdown-wasm

Small Rust + wasm markdown parser.

## GFM Feature Checklist

### Core Markdown

- [x] Headings (`#` to `###` currently parsed)
- [x] Paragraph blocks (including multi-line paragraphs)
- [x] Unordered lists
- [x] Ordered lists
- [x] Fenced code blocks (``` and ~~~)
- [x] HTML escaping in rendered output

### GitHub Flavored Markdown (GFM)

- [ ] Tables
- [ ] Task lists (`- [ ]` / `- [x]`)
- [ ] Strikethrough (`~~text~~`)
- [ ] Autolink literals (`https://example.com`)
- [ ] Footnotes
- [ ] Pipe table alignment (`:---`, `:---:`, `---:`)

### Other Common Markdown Features

- [ ] Blockquotes
- [ ] Inline code
- [ ] Emphasis (`*italic*`, `**bold**`)
- [ ] Links (`[text](url)`)
- [ ] Images (`![alt](src)`)
- [ ] Horizontal rules (`---`)
