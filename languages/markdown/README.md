# Prettify Markdown

Format Markdown at the speed of Rust.

> Note: A CLI is in the works if you just want to format your own code. The Markdown formatting is shared as it's own lib in case you wish to use it in your own project.

Prettify extensible, language-agnostic code formatter based on [Prettier](https://prettier.io/) written in Rust. The core of Prettify is in it's own crate, [prettify](https://crates.io/crates/prettify).

This library is designed to be used with the core [prettify](https://crates.io/crates/prettify) crate. This library parses the markdown to create the Prettify doc, and Prettify core will then format that doc.

```rs
use prettify::print;
use prettify_markdown::format_markdown;

fn prettify_markdown(file_contents: &str) -> String {
    print(format_markdown(file_contents))
}
```

> Note: this crate is in very early alpha. Most features of Markdown are not supported at this time. This library will eventually support all features of [GitHub flavored Markdown](https://github.github.com/gfm).
