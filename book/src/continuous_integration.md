# Continuous integration

## SARIF and GitHub Code Scanning

`sui-clippy` can emit SARIF for GitHub’s **Code Scanning** workflow. See the full guide:

**[GitHub Code Scanning (SARIF)](github_code_scanning.md)**

## GitHub Actions (minimal)

```yaml
- uses: dtolnay/rust-toolchain@stable
- run: cargo install --path crates/sui-clippy
- name: sui-clippy
  run: sui-clippy --format sarif ./my-move-pkg > sui-clippy.sarif
- uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: sui-clippy.sarif
```

Adjust the package path and permissions (`security-events: write`) to match your org.

## Documentation site (GitHub Pages)

The book and the searchable lint list are built together by **`.github/workflows/pages.yml`** (see `book/README.md` in the repository). After you set **Settings → Pages → Source** to **GitHub Actions**, pushes to `main` or `master` publish:

- The mdBook at the site root.
- The lint list at **`/lints/`** (static files under `book/src/lints/`, produced by `cargo run -p sui-clippy-docgen`).
