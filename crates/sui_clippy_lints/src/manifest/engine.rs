use std::path::Path;

use anyhow::Context;
use sui_clippy_config::SuiClippyConfig;
use sui_clippy_utils::Diagnostic;

use super::{git_dep_unpinned, missing_move_edition, wildcard_git_ref};
use crate::effective_level::finalize_diagnostic;
use crate::LintRunOptions;

/// Parse `Move.toml` and run manifest-only lints.
pub fn run_manifest_lints(
    move_toml_path: &Path,
    raw: &str,
    config: &SuiClippyConfig,
    options: &LintRunOptions,
) -> anyhow::Result<Vec<Diagnostic>> {
    let value: toml::Value = toml::from_str(raw).context("parse Move.toml as TOML")?;
    let mut out = Vec::new();

    for d in missing_move_edition::check(move_toml_path, raw, &value) {
        if let Some(x) = finalize_diagnostic(d, &missing_move_edition::LINT, config, options) {
            out.push(x);
        }
    }
    for d in git_dep_unpinned::check(move_toml_path, raw, &value) {
        if let Some(x) = finalize_diagnostic(d, &git_dep_unpinned::LINT, config, options) {
            out.push(x);
        }
    }
    for d in wildcard_git_ref::check(move_toml_path, raw, &value) {
        if let Some(x) = finalize_diagnostic(d, &wildcard_git_ref::LINT, config, options) {
            out.push(x);
        }
    }

    Ok(out)
}
