# Prettify

An extensible, language-agnostic code formatter based on [Prettier](https://prettier.io/) written in Rust.

The goal of this library is to provide a simple, fast, and extensible way to format code. Instead of formatting code for any specific language, it's language agnostic by accepting an intermediate representation instead of an AST.

That way, you can format code at the speed of Rust.

> Prettify is in beta. I believe it's feature complete and there are no bugs in this library. However, this library does not yet have 100% end-to-end test coverage, so it is possible there are bugs. Please file an issue if you find a bug.

---

This library is simply the core algorithm for Prettify. The actual parsing of each of different languages are in their own crates:

- [prettify-markdown](https://crates.io/crates/prettify_markdown)

This library is distributed as it's own crate in case you wish to use it in your own project or for your own language. That way you can simply describe how your language should be formatted and let Prettify take care of the rest!
