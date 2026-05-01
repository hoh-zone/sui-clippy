# Installation

## `sui-clippy` CLI

From a clone of [the repository](https://github.com/hoh/sui-clippy):

```bash
cargo install --path crates/sui-clippy
```

You get the `sui-clippy` binary (and `cargo-sui-clippy` if your `Cargo.toml` installs both bins from that crate).

## Optional: Move compiler integration

For `--typed` (compiler-backed diagnostics), build with the large Sui git dependency:

```bash
cargo install --path crates/sui-clippy --features move_compiler
```

## Optional: LSP

```bash
cargo install --path crates/sui-clippy-lsp
```

Then configure your editor to run `sui-clippy-lsp` with stdio (see [Language server](lsp.md)).

## mdBook (to read or author this book)

Install [mdBook](https://github.com/rust-lang/mdBook), then from the `book/` directory:

```bash
mdbook build
```

HTML output is written to `book/build/` (see `book.toml`).
