use declare_sui_clippy_lint::declare_sui_clippy_lint;
use std::path::Path;
use sui_clippy_utils::{Diagnostic, Severity, Span};

declare_sui_clippy_lint! {
    pub LINT,
    "git_dep_unpinned",
    Suspicious,
    "Git dependencies without `rev`, `branch`, or `tag` are not reproducible builds."
}

pub fn check(move_toml_path: &Path, raw: &str, value: &toml::Value) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for (section, dep_name, tbl) in super::each_git_dep_table(value) {
        if tbl.contains_key("local") {
            continue;
        }
        if tbl.contains_key("rev") || tbl.contains_key("branch") || tbl.contains_key("tag") {
            continue;
        }
        let needle = format!("{dep_name} =");
        let line = super::first_line_containing(raw, &needle).unwrap_or(1);
        out.push(Diagnostic::new(
            LINT.id,
            format!("unpinned git dependency `{dep_name}` in [{section}]"),
            Severity::Warning,
            Span {
                path: move_toml_path.to_path_buf(),
                line_start: line,
                line_end: line,
                col_start: 1,
                col_end: 1,
            },
        ));
    }
    out
}
