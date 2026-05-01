use std::collections::HashMap;
use std::path::Path;

use declare_sui_clippy_lint::{LintDef, LintLevel};
use sui_clippy_config::SuiClippyConfig;
use sui_clippy_utils::{Diagnostic, SourceFile};

use crate::effective_level::finalize_diagnostic;
use crate::pass::{SourceLintPass, ALL_SOURCE_PASSES};

#[derive(Debug, Clone, Default)]
pub struct LintRunOptions {
    /// If true, include diagnostics whose effective level is `allow` (normally suppressed).
    pub include_allowed: bool,
    /// Per-lint overrides for this run (CLI `-W`/`-A`/`-D`), highest precedence over `sui-clippy.toml`.
    pub cli_overrides: HashMap<String, LintLevel>,
}

pub fn all_source_passes() -> &'static [&'static dyn SourceLintPass] {
    ALL_SOURCE_PASSES
}

/// Lint metadata for built-in source passes only (see crate `all_lint_defs` for the full list).
pub fn all_source_lint_defs() -> impl Iterator<Item = &'static LintDef> + Clone {
    ALL_SOURCE_PASSES.iter().map(|p| p.lint_def())
}

/// Run built-in source-level lints on one file.
pub fn run_source_lints(
    file: &SourceFile,
    package_root: &Path,
    config: &SuiClippyConfig,
    options: &LintRunOptions,
) -> Vec<Diagnostic> {
    let rel = file
        .path()
        .strip_prefix(package_root)
        .ok()
        .and_then(|p| p.to_str())
        .map(|s| s.replace('\\', "/"))
        .unwrap_or_default();

    if config.is_excluded(&rel) {
        return Vec::new();
    }

    let mut out = Vec::new();
    for pass in ALL_SOURCE_PASSES {
        for d in pass.check_file(file) {
            if let Some(x) = finalize_diagnostic(d, pass.lint_def(), config, options) {
                out.push(x);
            }
        }
    }
    out
}
