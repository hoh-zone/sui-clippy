# Configuration

Project-local settings live in **`sui-clippy.toml`** at the Move package root (next to `Move.toml`).

The file is loaded by both the **`sui-clippy`** CLI and **`sui-clippy-lsp`**. Values are defined in the `sui_clippy_config` crate; see the default schema in the repository’s `crates/sui_clippy_config` sources.

## Overrides

CLI flags such as `--warn LINT` / `--allow LINT` / `--deny LINT` override config for a single run (see [Usage](usage.md)).

## Lint catalog

An authoritative list of lint ids and default levels is emitted by:

```bash
sui-clippy --list-lints
```

For a stable, linkable table and HTML index, regenerate [Lint index (generated)](lints_generated.md) with:

```bash
cargo run -p sui-clippy-docgen
```
