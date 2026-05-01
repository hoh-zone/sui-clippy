# Language server (`sui-clippy-lsp`)

The **`sui-clippy-lsp`** binary provides a minimal [LSP](https://microsoft.github.io/language-server-protocol/) server over **stdio**.

## Capabilities

- After **`textDocument/didOpen`** and **`textDocument/didSave`** for `*.move` files, it runs the same **`run_source_lints`** pipeline as the CLI and publishes **`textDocument/publishDiagnostics`**.
- Other requests receive **`MethodNotFound`** (the server is intentionally small).

## Editor setup

1. Install the binary (`cargo install --path crates/sui-clippy-lsp`).
2. Point your editor’s Move / custom LSP entry at **`sui-clippy-lsp`** with stdio transport.
3. Open a file under a normal Move package so `Move.toml` can be discovered upward from the file path.

Diagnostics respect `sui-clippy.toml` when present at the package root.
