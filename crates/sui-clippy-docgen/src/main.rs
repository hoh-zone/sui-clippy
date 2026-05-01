//! Regenerate:
//! - `book/src/lints_generated.md` — mdBook chapter
//! - `book/src/lints/index.html`, `style.css`, `script.js` — copied into mdBook output as `/lints/`

#![forbid(unsafe_code)]

use std::path::PathBuf;

use anyhow::Context;
use declare_sui_clippy_lint::{LintCategory, LintLevel};
use serde::Serialize;

#[derive(Serialize)]
struct LintRow {
    id: &'static str,
    category: LintCategory,
    default_level: LintLevel,
    doc: &'static str,
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("sui-clippy-docgen must live in crates/sui-clippy-docgen under workspace root")
        .to_path_buf()
}

fn md_cell(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

fn json_enum_label(v: impl Serialize) -> String {
    serde_json::to_value(v)
        .expect("enum serializes to string")
        .as_str()
        .expect("lint enums serialize as lowercase strings")
        .to_string()
}

fn write_lints_md(path: &std::path::Path, rows: &[LintRow]) -> anyhow::Result<()> {
    let mut buf = String::new();
    buf.push_str("# Lint index (generated)\n\n");
    buf.push_str("> This file is produced by `cargo run -p sui-clippy-docgen`. Do not edit by hand.\n\n");
    buf.push_str("See the [Lint categories](lints.md) chapter for what each group means.\n\n");
    buf.push_str("| Lint ID | Category | Default level | Documentation |\n");
    buf.push_str("|---------|----------|---------------|---------------|\n");
    for r in rows {
        let cat_s = json_enum_label(r.category);
        let def_s = json_enum_label(r.default_level);
        buf.push_str(&format!(
            "| [`{id}`](lints_generated.html#{id}) | `{cat_s}` | `{def_s}` | {doc} |\n",
            id = r.id,
            cat_s = cat_s,
            def_s = def_s,
            doc = md_cell(r.doc),
        ));
    }
    std::fs::write(path, buf).with_context(|| format!("write {}", path.display()))
}

fn write_lint_list_html(path: &std::path::Path, rows: &[LintRow]) -> anyhow::Result<()> {
    let json = serde_json::to_string(rows).context("serialize lints json")?;
    let count = rows.len();
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <title>sui-clippy lints</title>
  <link rel="stylesheet" href="style.css"/>
</head>
<body>
  <header class="page-header">
    <h1>sui-clippy lints <span class="badge">{count} rules</span></h1>
    <p class="subtitle">Sui Move linter — same idea as <a href="https://rust-lang.github.io/rust-clippy/stable/index.html">rust-clippy’s lint list</a>.</p>
  </header>
  <div class="toolbar">
    <label for="q">Search</label>
    <input id="q" type="search" placeholder="id, category, or doc text…" autocomplete="off"/>
    <label for="cat">Category</label>
    <select id="cat"><option value="">(all)</option></select>
  </div>
  <table id="lint-table">
    <thead><tr><th>Lint</th><th>Category</th><th>Default</th><th>Documentation</th></tr></thead>
    <tbody id="lint-body"></tbody>
  </table>
  <p class="footer">Generated from crate metadata · <a href="https://github.com/hoh/sui-clippy">repository</a></p>
  <script type="application/json" id="lint-json">{json}</script>
  <script src="script.js"></script>
</body>
</html>"#,
        count = count,
        json = json,
    );
    std::fs::write(path, html).with_context(|| format!("write {}", path.display()))
}

fn main() -> anyhow::Result<()> {
    let root = workspace_root();
    let mut rows: Vec<LintRow> = sui_clippy_lints::all_lint_defs()
        .map(|d| LintRow {
            id: d.id,
            category: d.category,
            default_level: d.category.default_level(),
            doc: d.doc,
        })
        .collect();
    rows.sort_by(|a, b| a.id.cmp(b.id));

    let md_path = root.join("book/src/lints_generated.md");
    let lint_dir = root.join("book/src/lints");
    std::fs::create_dir_all(md_path.parent().unwrap()).ok();
    std::fs::create_dir_all(&lint_dir).ok();

    write_lints_md(&md_path, &rows)?;
    write_lint_list_html(&lint_dir.join("index.html"), &rows)?;

    let style = include_str!("../assets/style.css");
    let script = include_str!("../assets/script.js");
    std::fs::write(lint_dir.join("style.css"), style).context("write style.css")?;
    std::fs::write(lint_dir.join("script.js"), script).context("write script.js")?;

    eprintln!("Wrote {}", md_path.display());
    eprintln!("Wrote {}", lint_dir.join("index.html").display());
    eprintln!("Wrote {}", lint_dir.join("style.css").display());
    eprintln!("Wrote {}", lint_dir.join("script.js").display());
    Ok(())
}
