# Lint categories

sui-clippy groups rules into **categories** (same names as [rust-clippy](https://doc.rust-lang.org/clippy/)). Each category has a **default level** (`allow`, `warn`, or `deny`) used when neither `sui-clippy.toml` nor the CLI overrides a lint.

| Category       | Role | Default |
|----------------|------|---------|
| `correctness`  | Likely bugs or useless code | **deny** |
| `suspicious`   | Very likely unintended | **warn** |
| `style`        | Idiomatic / readable Move | **warn** |
| `complexity`   | Simpler equivalent forms | **warn** |
| `perf`         | Cheaper patterns | **warn** |
| `pedantic`     | Stricter / noisier | **allow** |
| `nursery`      | New or experimental | **allow** |
| `restriction`  | Opinionated bans | **allow** |
| `cargo`        | `Move.toml` / packaging | **allow** |
| `sui`          | Sui object / PTB patterns | **allow** |
| `security`     | Security heuristics (expect noise) | **allow** |

## Full index

- **In this book:** [Lint index (generated)](lints_generated.md) — Markdown table (anchors `#lint_id`).
- **Searchable page:** [hosted lint list](./lints/index.html) on GitHub Pages (built by `.github/workflows/pages.yml`). Files live under `book/src/lints/` and are produced by `cargo run -p sui-clippy-docgen` before `mdbook build`. UX is modeled on [Clippy’s hosted lint list](https://rust-lang.github.io/rust-clippy/stable/index.html).

## Rules not in the generated table

Some diagnostics are produced at run time and are not declared as static `LintDef` entries:

- **`move_compiler`** — only when `--typed` is used and the binary is built with `--features move_compiler`.
- **`sui_move_lint`** — when `--with-sui-lint` is set and the spawned `sui move build --lint` fails.

## Regenerating docs

Whenever you add or change a lint’s metadata:

```bash
cargo run -p sui-clippy-docgen
```

Then rebuild the book (`mdbook build` inside `book/`) if you ship HTML.
