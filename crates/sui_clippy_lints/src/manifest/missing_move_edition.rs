use declare_sui_clippy_lint::declare_sui_clippy_lint;
use std::path::Path;
use sui_clippy_utils::{Diagnostic, Severity, Span};

declare_sui_clippy_lint! {
    pub LINT,
    "missing_move_edition",
    Style,
    "Move.toml [package] should declare an `edition` (for example Move 2024) for consistent tooling and language behavior."
}

pub fn check(move_toml_path: &Path, raw: &str, value: &toml::Value) -> Vec<Diagnostic> {
    let Some(pkg) = value.get("package").and_then(|p| p.as_table()) else {
        return Vec::new();
    };
    if pkg.contains_key("edition") {
        return Vec::new();
    }
    let line = super::line_of_table_header(raw, "package").unwrap_or(1);
    vec![Diagnostic::new(
        LINT.id,
        "[package] is missing `edition`",
        Severity::Warning,
        Span {
            path: move_toml_path.to_path_buf(),
            line_start: line,
            line_end: line,
            col_start: 1,
            col_end: 1,
        },
    )]
}
