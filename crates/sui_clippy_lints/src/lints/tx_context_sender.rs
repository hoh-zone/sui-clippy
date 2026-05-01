use declare_sui_clippy_lint::{declare_sui_clippy_lint, LintDef};
use regex::Regex;
use sui_clippy_utils::{is_line_comment_line, Diagnostic, Severity, SourceFile, Span};

use crate::pass::SourceLintPass;

declare_sui_clippy_lint! {
    /// `tx_context::sender()` patterns (composability / self-transfer smell).
    pub LINT,
    "tx_context_sender",
    Suspicious,
    "Using tx_context::sender() can encourage self-transfers; prefer returning objects to callers for PTB composability where appropriate."
}

pub struct Pass;

pub static PASS: Pass = Pass;

const SUI_NOTE: &str = "Sui built-in linters may also flag related patterns when using `sui move build --lint`.";

impl SourceLintPass for Pass {
    fn lint_def(&self) -> &'static LintDef {
        &LINT
    }

    fn check_file(&self, file: &SourceFile) -> Vec<Diagnostic> {
        let re = Regex::new(
            r"\b(tx_context::sender|sui::tx_context::sender|::tx_context::sender)\s*\(",
        )
        .expect("valid regex");
        let mut diags = Vec::new();
        for (line_no, line) in file.lines() {
            if is_line_comment_line(line) {
                continue;
            }
            let Some(m) = re.find(line) else {
                continue;
            };
            diags.push(
                Diagnostic::new(
                    LINT.id,
                    "call to tx_context::sender(); verify intent (self-transfer vs composable API)",
                    Severity::Warning,
                    Span {
                        path: file.path().to_path_buf(),
                        line_start: line_no,
                        line_end: line_no,
                        col_start: m.start() as u32 + 1,
                        col_end: m.end() as u32 + 1,
                    },
                )
                .with_note(SUI_NOTE),
            );
        }
        diags
    }
}
