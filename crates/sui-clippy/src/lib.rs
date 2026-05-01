#![forbid(unsafe_code)]

pub mod cli;
pub mod output_format;
mod run;

pub use output_format::OutputFormat;
pub use run::{list_lint_catalog, merge_cli_lint_overrides, run, RunOutcome, RunParams};
