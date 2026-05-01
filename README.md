# sui-clippy

Sui Move linter with a **Clippy-shaped** CLI: named lints, categories, `sui-clippy.toml`, JSON/SARIF output, optional `--typed` compiler pass (`--features move_compiler`), and **`sui-clippy-lsp`**.

## Documentation

| | |
|--|--|
| **Book (mdBook)** | [`book/`](book/) — run `mdbook build` inside that directory after [installing mdBook](https://github.com/rust-lang/mdBook). Modeled on [rust-clippy’s book](https://github.com/rust-lang/rust-clippy/tree/master/book). |
| **Lint list (web)** | **GitHub Pages:** *Settings → Pages → GitHub Actions*, then push to `main` / `master` (`.github/workflows/pages.yml`). Site: `https://YOUR_USER.github.io/REPO_NAME/` — book at `/`, searchable lints at `/lints/`. Local: `cargo run -p sui-clippy-docgen && cd book && mdbook build`, then serve `book/build/`. Same role as [rust-clippy’s lint index](https://rust-lang.github.io/rust-clippy/stable/index.html). |
| **Code Scanning** | [book/src/github_code_scanning.md](book/src/github_code_scanning.md) (mdBook chapter) |

Regenerate the embedded lint table in the book and the gh-pages bundle:

```bash
cargo run -p sui-clippy-docgen
```
