# markdown-wasm

Small Rust + wasm markdown parser.

## Install

Requirements:

- Rust and Cargo
- Node.js (or Bun)
- wasm-pack

Install wasm-pack:

```bash
cargo install wasm-pack
```

## Build and run locally

Build the wasm package:

```bash
npm run build
```

This generates the TS/WASM bindings in `pkg/`.

## Usage

The module comes with one function: parse_markdown(input: string)

```js
import { parse_markdown } from "markdown_wasm";

await init();

const input = `# Hello`
const html = parse_markdown(input);

console.log(html);
```

## Feature Checklist

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
