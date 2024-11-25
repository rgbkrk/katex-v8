# katex-v8

KaTeX bindings that use the v8 engine for Rust.

This was made just to try out the `v8` crate. Suffice it to say, I'm impressed.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
katex-v8 = "0.0.1"
```

## Usage

Here's a simple example of how to use `katex-v8`:

```rust
use katex_v8::{render, Opts, Error};

fn main() -> Result<(), Error> {
    let opts = Opts::new().display_mode(true);
    let html = render("E = mc^2", &opts)?;
    println!("{}", html);
    Ok(())
}
```

This will render the LaTeX equation "E = mc^2" to HTML using KaTeX via the v8 engine. Note: you _will_ need the katex CSS in your HTML.

```html
<head>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css" integrity="sha384-nB0miv6/jRmo5UMMR1wu3Gz6NLsoTkbqJghGIsx//Rlm+ZU03BU6SQNC66uf4l5+" crossorigin="anonymous">
</head>
```

## Options

You can customize the rendering with the `Opts` struct:

```rust
let opts = Opts::new().display_mode(true);
```

Currently, only the `display_mode` option is supported.
