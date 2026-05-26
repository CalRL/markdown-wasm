# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
# Large Markdown Benchmark Document

This is a larger markdown sample designed to benchmark markdown parsers.

It contains repeated sections with headings, paragraphs, lists, blockquotes, and code blocks.

---

## Section 1

Markdown parsers often need to process many different syntactic forms. Some files are simple, while others contain nested structures, long paragraphs, and many inline tokens.

- Alpha
- Beta
- Gamma
- Delta
- Epsilon

```rust
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Heading {
        level: u8,
        content: &'a str,
    },
    Paragraph {
        content: &'a str,
    },
    CodeBlock {
        language: Option<&'a str>,
        content: &'a str,
    },
}
```

## Section 2

This paragraph contains **bold text**, *italic text*, `inline code`, and a [link](https://example.com). It is intended to test inline parsing performance as well as block parsing performance.

> A blockquote can contain important information.
> It may also continue over several lines.
> Some parsers treat this as one block.

1. First ordered item
2. Second ordered item
3. Third ordered item
4. Fourth ordered item

## Section 3

Another paragraph is included here to increase the size of the document. Real markdown files often contain a lot of plain text, so benchmark samples should not only include edge cases.

```ts
export function parseMarkdown(input: string): string {
    return input
        .split("\n")
        .map(line => `<p>${line}</p>`)
        .join("");
}
```

## Section 4

Tables are common in GitHub-flavoured markdown.

| Name | Language | Purpose |
|---|---|---|
| pulldown-cmark | Rust | Markdown parsing |
| markdown-it | JavaScript | Markdown parsing |
| comrak | Rust | CommonMark parsing |
| marked | JavaScript | Markdown rendering |

## Section 5

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer posuere, neque at commodo aliquet, turpis risus dignissim massa, sed vehicula neque nisi non justo.

Sed non ante sed justo fermentum consequat. Cras venenatis augue vitae orci efficitur, sed dictum elit efficitur.

Maecenas blandit mi non quam pharetra, sed sagittis risus aliquet.

## Section 6

- Nested-looking item one
    - Child item one
    - Child item two
- Nested-looking item two
    - Child item three
    - Child item four

## Section 7

```html
<article>
    <h1>Hello world</h1>
    <p>This is some generated HTML.</p>
</article>
```

## Section 8

Markdown is often used for project documentation. A benchmark should include content that looks like real documentation rather than only synthetic stress cases.

### Installation

```bash
cargo add markdown-wasm
cargo build --release
```

### Usage

```rust
let input = "# Hello";
let html = parse_markdown(input);
println!("{html}");
```

## Section 9

The parser should be tested with repeated content because repeated sections help smooth out random measurement noise.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

This is another normal paragraph. It does not contain much syntax, but it still matters because most documents are mostly plain text.

## Section 10

Final benchmark section.

- Parse markdown
- Render HTML
- Measure elapsed time
- Calculate operations per second

```rust
let ops_per_second = operations as f64 / elapsed.as_secs_f64();
println!("{ops_per_second}");
```

End of document.
