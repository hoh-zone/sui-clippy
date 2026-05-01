# Usage

## Basic check

Run inside a Move package (directory containing `Move.toml`):

```bash
sui-clippy
```

Or point at a path:

```bash
sui-clippy path/to/package
sui-clippy path/to/module.move
```

## Output formats

- **Human text** (default): one diagnostic per block.
- **JSON**: `--format json` — one JSON object per line (one diagnostic per line).
- **SARIF**: `--format sarif` — single SARIF 2.1 document for CI (see [GitHub Code Scanning](github_code_scanning.md)).

## Lint levels from the CLI

Same spirit as `rustc -W` / `-A` / `-D`:

```bash
sui-clippy --warn todo_comment --deny bare_abort .
```

## Workspace of Move packages

If the root `Move.toml` defines `[workspace].members`, lint each member:

```bash
sui-clippy --workspace /path/to/workspace/root
```

The path must be a **directory** (the workspace root). Use `--skip-manifest` if you only want source rules.

## Sui CLI integration

Forward output from the Sui toolchain:

```bash
sui-clippy --with-sui-lint .
```

## Compiler-backed pass (`--typed`)

When `sui-clippy` is built with `move_compiler`, `--typed` runs the Move compiler on `sources/` and attaches diagnostics (rule id `move_compiler`). Without the feature, `--typed` exits with an error telling you to rebuild with `--features move_compiler`.

## Listing lints

```bash
sui-clippy --list-lints
sui-clippy --list-lints --format json
```

## Manifest fixes

```bash
sui-clippy --fix-manifest          # insert default edition when missing
sui-clippy --fix                   # alias for --fix-manifest today
```

`--fix-sources` is reserved for future autofixes and is currently a no-op (with a note on stderr in text mode).
