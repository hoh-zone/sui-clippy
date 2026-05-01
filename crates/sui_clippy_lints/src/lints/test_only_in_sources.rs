use declare_sui_clippy_lint::{declare_sui_clippy_lint, LintDef};
use sui_clippy_utils::{is_line_comment_line, Diagnostic, Severity, SourceFile, Span};

use crate::pass::SourceLintPass;

declare_sui_clippy_lint! {
    pub LINT,
    "test_only_in_sources",
    Nursery,
    "`#[test_only]` in non-test paths is easy to ship accidentally; prefer `tests/` modules or `#[test_only]` only in test-only files."
}

/// True when this file lives under the package's Move `tests/` tree (not e.g. Rust `tests/fixtures/`).
fn path_looks_like_test_tree(file: &SourceFile) -> bool {
    let Some(rel) = file.rel_to_package.as_deref() else {
        return false;
    };
    rel.starts_with("tests/") || rel.contains("/tests/")
}

pub struct Pass;

pub static PASS: Pass = Pass;

impl SourceLintPass for Pass {
    fn lint_def(&self) -> &'static LintDef {
        &LINT
    }

    fn check_file(&self, file: &SourceFile) -> Vec<Diagnostic> {
        if path_looks_like_test_tree(file) {
            return Vec::new();
        }
        let mut diags = Vec::new();
        for (line_no, line) in file.lines() {
            if is_line_comment_line(line) {
                continue;
            }
            let t = line.trim_start();
            if !t.starts_with("#[test_only") {
                continue;
            }
            let col_start = line
                .find("#[test_only")
                .map(|i| i as u32 + 1)
                .unwrap_or(1);
            let col_end = col_start + "#[test_only".len() as u32;
            diags.push(Diagnostic::new(
                LINT.id,
                "`#[test_only]` in a non-`tests/` source path",
                Severity::Warning,
                Span {
                    path: file.path().to_path_buf(),
                    line_start: line_no,
                    line_end: line_no,
                    col_start,
                    col_end,
                },
            ));
        }
        diags
    }
}
