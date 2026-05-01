use declare_sui_clippy_lint::{declare_sui_clippy_lint, LintDef};
use regex::Regex;
use sui_clippy_utils::{Diagnostic, Severity, SourceFile, Span};

use crate::pass::SourceLintPass;

declare_sui_clippy_lint! {
    pub LINT,
    "empty_public_entry",
    Correctness,
    "A public entry function with an empty body is almost certainly a mistake or unsafe stub."
}

pub struct Pass;

pub static PASS: Pass = Pass;

impl SourceLintPass for Pass {
    fn lint_def(&self) -> &'static LintDef {
        &LINT
    }

    fn check_file(&self, file: &SourceFile) -> Vec<Diagnostic> {
        let re = Regex::new(
            r"(?s)public\s+entry\s+fun\s+[A-Za-z0-9_]+\s*\([^)]*\)\s*:\s*[^;{]+\{\s*\}",
        )
        .expect("valid regex");
        let text = &file.text;
        let mut diags = Vec::new();
        for m in re.find_iter(text) {
            let (line_start, col_start) = byte_index_to_line_col(text, m.start());
            let (line_end, col_end) = byte_index_to_line_col(text, m.end().saturating_sub(1));
            diags.push(Diagnostic::new(
                LINT.id,
                "public entry function has an empty body",
                Severity::Warning,
                Span {
                    path: file.path().to_path_buf(),
                    line_start,
                    line_end,
                    col_start,
                    col_end,
                },
            ));
        }
        diags
    }
}

fn byte_index_to_line_col(text: &str, byte_idx: usize) -> (u32, u32) {
    let mut line = 1u32;
    let mut col = 1u32;
    let mut i = 0usize;
    for ch in text.chars() {
        if i >= byte_idx {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
        i += ch.len_utf8();
    }
    (line, col)
}
