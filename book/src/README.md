# sui-clippy

**sui-clippy** is a [Sui](https://sui.io/) Move linter with a **Clippy-shaped** architecture: named lints, categories, config file overrides, and machine-readable output (JSON / SARIF).

It complements `sui move build --lint` and focuses on **fast text-level checks** plus optional **Move compiler** integration when built with `--features move_compiler`.

## Where to go next

- [Installation](installation.md) — install the CLI (and optional LSP binary).
- [Usage](usage.md) — common flags, workspace mode, and `sui move build --lint`.
- [Lint index (generated)](lints_generated.md) — table of every built-in rule (regenerate with `cargo run -p sui-clippy-docgen`).
- **Web-style lint list** (search + category filter): on [GitHub Pages](./lints/index.html) after enabling the workflow (same role as [rust-clippy’s lint index](https://rust-lang.github.io/rust-clippy/stable/index.html)). Locally: `cargo run -p sui-clippy-docgen`, then `cd book && mdbook build` and open `book/build/lints/index.html` (or serve `book/build/` over HTTP).

The layout of this book follows the [Clippy book](https://doc.rust-lang.org/clippy/) structure in a shorter form: installation, usage, configuration, lints, CI, and tooling.
