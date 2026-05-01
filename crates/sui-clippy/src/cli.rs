use std::path::PathBuf;

use clap::Parser;

use crate::output_format::OutputFormat;

#[derive(Parser, Debug)]
#[command(version, about = "Sui Move source linter (Clippy-shaped)")]
pub struct CliArgs {
    /// Path to Move package (directory containing Move.toml) or a single .move file.
    #[arg(default_value = ".")]
    pub path: PathBuf,
    /// Output format (use `json` for one diagnostic JSON per line).
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
    /// Shorthand for `--format json` (deprecated in favor of `--format json`).
    #[arg(long, conflicts_with = "format")]
    pub json: bool,
    /// Run `sui move build --lint` in the package and print combined stdout/stderr (best-effort).
    #[arg(long)]
    pub with_sui_lint: bool,
    /// Include diagnostics suppressed to `allow` by config (for tooling).
    #[arg(long, hide = true)]
    pub include_allowed: bool,
    /// List built-in lint ids and exit.
    #[arg(long)]
    pub list_lints: bool,
    /// Skip Move.toml manifest checks.
    #[arg(long)]
    pub skip_manifest: bool,
    /// Insert a default `edition` under `[package]` when missing (mutates Move.toml).
    #[arg(long)]
    pub fix_manifest: bool,
    /// Apply supported automatic fixes (currently: same as `--fix-manifest`).
    #[arg(long)]
    pub fix: bool,
    /// Reserved for future source-level autofixes (no-op today).
    #[arg(long)]
    pub fix_sources: bool,
    /// Lint every package listed under `[workspace].members` (path must be the workspace directory).
    #[arg(long)]
    pub workspace: bool,
    /// Run the Move compiler on `sources/` (needs `sui-clippy` built with `--features move_compiler`).
    #[arg(long)]
    pub typed: bool,
    /// Set lint to warn (repeatable). Overrides `sui-clippy.toml` for this run.
    #[arg(long = "warn", value_name = "LINT")]
    pub warn_lint: Vec<String>,
    /// Set lint to allow (repeatable).
    #[arg(long = "allow", value_name = "LINT")]
    pub allow_lint: Vec<String>,
    /// Set lint to deny (repeatable).
    #[arg(long = "deny", value_name = "LINT")]
    pub deny_lint: Vec<String>,
}

impl CliArgs {
    #[must_use]
    pub fn output_format(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else {
            self.format
        }
    }
}
