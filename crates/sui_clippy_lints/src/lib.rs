#![forbid(unsafe_code)]

mod effective_level;
mod lint_engine;
mod lints;
mod manifest;
mod pass;
pub mod typed;
mod workspace;

pub use workspace::discover_workspace_package_roots;

use declare_sui_clippy_lint::LintDef;

pub use lint_engine::{
    all_source_lint_defs, all_source_passes, run_source_lints, LintRunOptions,
};
pub use manifest::fix::insert_default_edition_if_missing;
pub use manifest::run_manifest_lints;
pub use pass::SourceLintPass;

/// All lint definitions (source passes + manifest checks).
pub fn all_lint_defs() -> impl Iterator<Item = &'static LintDef> + Clone {
    lint_engine::all_source_lint_defs()
        .chain(manifest::MANIFEST_LINT_DEFS.iter().copied())
}
