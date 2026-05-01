use declare_sui_clippy_lint::{declare_sui_clippy_lint, LintDef};
use regex::Regex;
use sui_clippy_utils::{Diagnostic, Severity, SourceFile, Span};

use crate::pass::SourceLintPass;

declare_sui_clippy_lint! {
    /// TODO/FIXME in production Move sources.
    pub LINT,
    "todo_comment",
    Suspicious,
    "Track and remove TODO/FIXME markers before release."
}

pub struct Pass;

pub static PASS: Pass = Pass;

impl SourceLintPass for Pass {
    fn lint_def(&self) -> &'static LintDef {
        &LINT
    }

    fn check_file(&self, file: &SourceFile) -> Vec<Diagnostic> {
        let re = Regex::new(r"(?i)//\s*(TODO|FIXME)\b").expect("valid regex");
        let mut diags = Vec::new();
        for (line_no, line) in file.lines() {
            let Some(m) = re.find(line) else {
                continue;
            };
            let col_start = m.start() as u32 + 1;
            let col_end = m.end() as u32 + 1;
            diags.push(Diagnostic::new(
                LINT.id,
                format!("{} marker in comment", m.as_str().to_uppercase()),
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
