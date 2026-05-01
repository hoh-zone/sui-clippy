use declare_sui_clippy_lint::{declare_sui_clippy_lint, LintDef};
use regex::Regex;
use sui_clippy_utils::{is_line_comment_line, Diagnostic, Severity, SourceFile, Span};

use crate::pass::SourceLintPass;

declare_sui_clippy_lint! {
    pub LINT,
    "bare_abort",
    Suspicious,
    "`abort` with a magic numeric code is easy to misuse; prefer named constants or structured errors."
}

pub struct Pass;

pub static PASS: Pass = Pass;

impl SourceLintPass for Pass {
    fn lint_def(&self) -> &'static LintDef {
        &LINT
    }

    fn check_file(&self, file: &SourceFile) -> Vec<Diagnostic> {
        let re = Regex::new(r"\babort\s+\d+\b").expect("valid regex");
        let mut diags = Vec::new();
        for (line_no, line) in file.lines() {
            if is_line_comment_line(line) {
                continue;
            }
            let Some(m) = re.find(line) else {
                continue;
            };
            diags.push(Diagnostic::new(
                LINT.id,
                "abort with numeric literal; consider a named constant or clearer error path",
                Severity::Warning,
                Span {
                    path: file.path().to_path_buf(),
                    line_start: line_no,
                    line_end: line_no,
                    col_start: m.start() as u32 + 1,
                    col_end: m.end() as u32 + 1,
                },
            ));
        }
        diags
    }
}
