#![forbid(unsafe_code)]

use clap::Parser;
use sui_clippy::cli::CliArgs;
use sui_clippy::{list_lint_catalog, merge_cli_lint_overrides, run, RunParams};

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    if args.list_lints {
        let fmt = args.output_format();
        list_lint_catalog(fmt)?;
        return Ok(());
    }
    let overrides = merge_cli_lint_overrides(&args.allow_lint, &args.warn_lint, &args.deny_lint);
    let format = args.output_format();
    let outcome = run(RunParams {
        path: args.path,
        format,
        with_sui_lint: args.with_sui_lint,
        include_allowed: args.include_allowed,
        skip_manifest: args.skip_manifest,
        fix_manifest: args.fix_manifest || args.fix,
        fix_sources: args.fix_sources,
        typed: args.typed,
        workspace: args.workspace,
        cli_overrides: overrides,
    })?;
    std::process::exit(outcome.exit_code);
}
