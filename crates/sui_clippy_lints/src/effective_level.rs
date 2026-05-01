use declare_sui_clippy_lint::{LintDef, LintLevel};
use sui_clippy_config::SuiClippyConfig;
use sui_clippy_utils::{Diagnostic, Severity};

use crate::LintRunOptions;

/// Apply `sui-clippy.toml`, CLI overrides, and `include_allowed` to a raw diagnostic.
pub(crate) fn finalize_diagnostic(
    mut d: Diagnostic,
    def: &'static LintDef,
    config: &SuiClippyConfig,
    options: &LintRunOptions,
) -> Option<Diagnostic> {
    let default = def.category.default_level();
    let from_file = config.level_for(def.id, default);
    let effective = options
        .cli_overrides
        .get(def.id)
        .copied()
        .unwrap_or(from_file);
    if effective == LintLevel::Allow && !options.include_allowed {
        return None;
    }
    d.severity = match effective {
        LintLevel::Allow => Severity::Allow,
        LintLevel::Warn => Severity::Warning,
        LintLevel::Deny => Severity::Error,
    };
    if d.severity != Severity::Allow || options.include_allowed {
        Some(d)
    } else {
        None
    }
}
