# Prettify

An extensible, language-agnostic code formatter based on [Prettier](https://prettier.io/) written in Rust.

The goal of this library is to provide a simple, fast, and extensible way to format code. Instead of formatting code for any specific language, it's language agnostic by accepting an intermediate representation instead of an AST.

That way, you can format code at the speed of Rust.

## Contribute

Contributions are welcome. This repository is a monorepo and uses cargo. If you do not have cargo installed, follow [the instructions are the Rust website](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Testing

This repository has two types of tests: unit tests and snapshot tests. All new features and bug fixes should add both a new snapshot test and applicable unit tests.

To run all tests, use the following command:

```bash
cargo test
```

You can run the unit tests for any specific library by running the following command:

```bash
cargo test -p [library-name]

# For example:
cargo test -p prettify
```

This repository uses unit tests as you would in most other Rust repositories.

Prettify uses snapshot testing for it's end-to-end tests. That way, we can ensure formatting works properly for every language in every edge case. The repository uses [`insta`](https://insta.rs/) for snapshot testing.

You can run all snapshot tests with the following command:

```bash
cargo insta test
```

To add a new snapshot test, simply add the desired file to the appropriate folder inside of `./cli/tests/files`. Then, run the following command to review the snapshot:

```bash
cargo insta test --review
```

If the generated snapshot looks correct, you can accept it.
