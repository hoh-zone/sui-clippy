#![forbid(unsafe_code)]

use declare_sui_clippy_lint::LintDef;
use sui_clippy_utils::{Diagnostic, SourceFile};

/// Source-level lint (regex / heuristic today; typed AST later).
pub trait SourceLintPass: Send + Sync {
    fn lint_def(&self) -> &'static LintDef;
    fn check_file(&self, file: &SourceFile) -> Vec<Diagnostic>;
}

/// All built-in source passes, in stable order.
pub static ALL_SOURCE_PASSES: &[&dyn SourceLintPass] = &[
    &crate::lints::todo_comment::PASS,
    &crate::lints::tx_context_sender::PASS,
    &crate::lints::empty_public_entry::PASS,
    &crate::lints::public_fun_transfer::PASS,
    &crate::lints::bare_abort::PASS,
    &crate::lints::clock_timestamp::PASS,
    &crate::lints::dynamic_field_access::PASS,
    &crate::lints::test_only_in_sources::PASS,
];
