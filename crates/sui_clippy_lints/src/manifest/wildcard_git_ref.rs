use declare_sui_clippy_lint::declare_sui_clippy_lint;
use std::path::Path;
use sui_clippy_utils::{Diagnostic, Severity, Span};

declare_sui_clippy_lint! {
    pub LINT,
    "wildcard_git_ref",
    Correctness,
    "Wildcard `rev` or `branch` on a git dependency is almost never intended for production."
}

pub fn check(move_toml_path: &Path, raw: &str, value: &toml::Value) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for (section, dep_name, tbl) in super::each_git_dep_table(value) {
        let bad_rev = tbl
            .get("rev")
            .and_then(|v| v.as_str())
            .is_some_and(|s| s.trim() == "*");
        let bad_branch = tbl
            .get("branch")
            .and_then(|v| v.as_str())
            .is_some_and(|s| s.trim() == "*");
        if !bad_rev && !bad_branch {
            continue;
        }
        let needle = format!("{dep_name} =");
        let line = super::first_line_containing(raw, &needle).unwrap_or(1);
        out.push(Diagnostic::new(
            LINT.id,
            format!("git dependency `{dep_name}` in [{section}] uses a wildcard ref"),
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
