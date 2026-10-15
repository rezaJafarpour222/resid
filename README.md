# Resid

**Resid** is a command-line tool for generating PDF files from HTML and CSS. It is designed for Persian/RTL documents such as invoices, receipts, reports, and administrative templates. It supports right-to-left text, spacing, Flexbox, tables, and SVG.

## Features

- PDF output with configurable page sizes
- HTML and inline `<style>` CSS
- Persian and right-to-left text
- Margins, padding, borders, backgrounds
- Basic Flexbox layouts
- CSS tables
- Inline and external SVG
- Font and page-size selection from CLI
- Example documents in `examples/`

## Requirements

- Rust toolchain
- Required fonts installed or available to the app

Supported fonts:

```text
Estedad
Gandom
Parastoo
Samim
Shabnam
Vazirmatn
```

## Build

```bash
cargo build --release
```

Binary output:

```text
target/release/resid
```

## Quick start

```bash
resid --create output.pdf --from input.html
```

Or short form:

```bash
resid -c output.pdf -f input.html
```

## CLI options

```text
--from, -f     HTML file path or HTML string
--create, -c   Output PDF path
--font         Font name (default: Vazirmatn)
--page         Page size (default: a4)
```

Supported page sizes:

```text
a3
a4
a4-landscape
a5
a6
```

### Example

```bash
resid \
  --from examples/invoice-professional.html \
  --create output.pdf \
  --font Vazirmatn \
  --page a4
```

## Examples

The `examples/` directory contains sample HTML files:

```text
invoice1.html ... invoice6.html
invoice-professional.html
receipt.html
logo-left.html
logo-right.html
svg.html
```

Run one:

```bash
resid --from examples/receipt.html --create receipt.pdf
```

## HTML support

Supported structure:

```text
html, body, header, footer, main
section, article, aside, nav
div, p, h1-h6
table, thead, tbody, tfoot, tr, td, th
br, img, svg
```

Supported HTML attributes:

```text
id
class
style
dir
src
width
height
```

## CSS support

The parser supports the properties below. Layout behavior varies by property.

```text
display
direction
position
width, height, min/max width/height
margin, padding
border, border-radius
font-family, font-size, font-weight, line-height
text-align, color, background/background-color
opacity
flex-direction, flex-wrap, justify-content, align-items
flex-grow, flex-shrink, flex-basis, gap
list-style-type, list-style-position
page-break-before, page-break-after
```

## SVG

Inline SVG:

```html
<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <circle cx="50" cy="50" r="40" fill="red" />
</svg>
```

External SVG:

```html
<img class="logo" src="assets/logo.svg" alt="Logo">
```

Relative paths are resolved against the HTML file location when loading through `--from`.

## Tests

Run all tests:

```bash
cargo test
```

Run integration tests:

```bash
cargo test --test integration
```

## Development

Project structure:

```text
src/
  composition/   HTML + CSS composition
  css/           parser and computed styles
  document/      layout and page dimensions
  font/          font loading and shaping
  html/           HTML parser
  pdf/            PDF generation, fonts, and SVG
  cli.rs         command-line interface
  main.rs        entry point
```

Format code:

```bash
cargo fmt
```

Check warnings:

```bash
cargo clippy
```

## Current limitations

- CSS support is still partial. Grid, float, transform, shadows, and advanced effects are not fully implemented.
- Advanced pagination, repeated table headers, headers, and footers are not complete.
- Fonts may require environment setup.
- SVG rendering depends on `usvg` behavior and supported CSS features.

---

Resid is optimized for Persian and right-to-left documents. Compare your HTML with examples in `examples/` for best results.
```