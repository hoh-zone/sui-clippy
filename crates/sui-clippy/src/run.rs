use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Context;
use declare_sui_clippy_lint::{LintCategory, LintLevel};
use serde::Serialize;
use sui_clippy_config::SuiClippyConfig;
use sui_clippy_lints::{
    discover_workspace_package_roots, run_manifest_lints, run_source_lints, LintRunOptions,
};
use sui_clippy_utils::{diagnostics_to_sarif_json, Diagnostic, Severity, SourceFile};
use walkdir::WalkDir;

use crate::output_format::OutputFormat;

const DEFAULT_MOVE_EDITION: &str = "2024.beta";

/// Arguments for [`run`].
#[derive(Debug)]
pub struct RunParams {
    pub path: PathBuf,
    pub format: OutputFormat,
    pub with_sui_lint: bool,
    pub include_allowed: bool,
    pub skip_manifest: bool,
    pub fix_manifest: bool,
    /// When true with `--workspace`, also apply future source fixes (currently a no-op).
    pub fix_sources: bool,
    pub typed: bool,
    /// Lint every member package under a Move `[workspace].members` root (path must be a directory).
    pub workspace: bool,
    pub cli_overrides: HashMap<String, LintLevel>,
}

#[derive(Debug, Default)]
pub struct RunOutcome {
    pub exit_code: i32,
    /// Collected diagnostics from built-in source lints (severity already reflects effective level).
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Serialize)]
struct LintCatalogRow {
    id: &'static str,
    category: LintCategory,
    default_level: LintLevel,
    doc: &'static str,
}

/// Print all registered lints (TSV for humans, JSON array for `--format json|sarif`).
pub fn list_lint_catalog(format: OutputFormat) -> anyhow::Result<()> {
    match format {
        OutputFormat::Text => {
            for def in sui_clippy_lints::all_lint_defs() {
                println!(
                    "{}\t{:?}\t{:?}\t{}",
                    def.id,
                    def.category,
                    def.category.default_level(),
                    def.doc
                );
            }
        }
        OutputFormat::Json | OutputFormat::Sarif => {
            let rows: Vec<LintCatalogRow> = sui_clippy_lints::all_lint_defs()
                .map(|def| LintCatalogRow {
                    id: def.id,
                    category: def.category,
                    default_level: def.category.default_level(),
                    doc: def.doc,
                })
                .collect();
            println!("{}", serde_json::to_string(&rows)?);
        }
    }
    Ok(())
}

/// Merge CLI `-W`/`-A`/`-D` lists; later entries for the same id win in order allow, warn, deny.
#[must_use]
pub fn merge_cli_lint_overrides(
    allow: &[String],
    warn: &[String],
    deny: &[String],
) -> HashMap<String, LintLevel> {
    let mut m = HashMap::new();
    for a in allow {
        m.insert(a.clone(), LintLevel::Allow);
    }
    for w in warn {
        m.insert(w.clone(), LintLevel::Warn);
    }
    for d in deny {
        m.insert(d.clone(), LintLevel::Deny);
    }
    m
}

pub fn run(params: RunParams) -> anyhow::Result<RunOutcome> {
    let RunParams {
        path,
        format,
        with_sui_lint,
        include_allowed,
        skip_manifest,
        fix_manifest,
        fix_sources,
        typed,
        workspace,
        cli_overrides,
    } = params;

    if fix_sources && format == OutputFormat::Text {
        eprintln!("sui-clippy: note: `--fix-sources` is reserved; no source edits are applied yet.");
    }

    let path = fs::canonicalize(&path).with_context(|| format!("canonicalize {}", path.display()))?;

    if workspace && path.is_file() {
        anyhow::bail!("--workspace requires a directory path (the Move workspace root containing Move.toml)");
    }

    let package_roots: Vec<PathBuf> = if workspace {
        discover_workspace_package_roots(&path).with_context(|| "workspace package discovery")?
    } else if path.is_file() {
        vec![find_package_root(path.parent().unwrap_or(Path::new(".")))
            .with_context(|| "could not find Move.toml for this file")?]
    } else {
        vec![find_package_root(&path).with_context(|| "could not find Move.toml under this path")?]
    };

    let options = LintRunOptions {
        include_allowed,
        cli_overrides,
    };

    let mut diags = Vec::new();
    for package_root in &package_roots {
        let config = SuiClippyConfig::load(package_root)?;

        let move_toml = package_root.join("Move.toml");
        if fix_manifest && !skip_manifest {
            let applied = sui_clippy_lints::insert_default_edition_if_missing(
                &move_toml,
                DEFAULT_MOVE_EDITION,
            )?;
            if applied && format == OutputFormat::Text {
                eprintln!(
                    "sui-clippy: inserted `edition = \"{DEFAULT_MOVE_EDITION}\"` into {}",
                    move_toml.display()
                );
            }
        }

        if !skip_manifest {
            let raw = fs::read_to_string(&move_toml)
                .with_context(|| format!("read {}", move_toml.display()))?;
            diags.extend(run_manifest_lints(
                &move_toml,
                &raw,
                &config,
                &options,
            )?);
        }

        let scan_target = if workspace {
            package_root.clone()
        } else {
            path.clone()
        };
        for file in collect_move_files(package_root, &scan_target)? {
            let text =
                fs::read_to_string(&file).with_context(|| format!("read {}", file.display()))?;
            let rel = file
                .strip_prefix(package_root)
                .ok()
                .and_then(|p| p.to_str().map(|s| s.replace('\\', "/")));
            let sf = if let Some(rel) = rel {
                SourceFile::with_rel(file, text, rel)
            } else {
                SourceFile::new(file, text)
            };
            diags.extend(run_source_lints(&sf, package_root, &config, &options));
        }

        if typed {
            if !sui_clippy_lints::typed::move_compiler_feature_enabled() {
                anyhow::bail!(
                    "`--typed` requires building sui-clippy with `--features move_compiler` (large Sui git dependency)"
                );
            }
            diags.extend(sui_clippy_lints::typed::run_compiler_probe(package_root)?);
        }

        if with_sui_lint {
            match invoke_sui_move_lint(package_root) {
                Ok(status) => {
                    if format == OutputFormat::Text {
                        if !status.stdout.is_empty() {
                            print!("{}", String::from_utf8_lossy(&status.stdout));
                        }
                        if !status.stderr.is_empty() {
                            eprint!("{}", String::from_utf8_lossy(&status.stderr));
                        }
                    } else {
                        if !status.stdout.is_empty() {
                            eprint!("{}", String::from_utf8_lossy(&status.stdout));
                        }
                        if !status.stderr.is_empty() {
                            eprint!("{}", String::from_utf8_lossy(&status.stderr));
                        }
                    }
                    if !status.success {
                        diags.push(Diagnostic::new(
                            "sui_move_lint",
                            format!(
                                "`sui move build --lint` failed for {} (exit {:?})",
                                package_root.display(),
                                status.code
                            ),
                            Severity::Error,
                            sui_clippy_utils::Span {
                                path: move_toml.clone(),
                                line_start: 1,
                                line_end: 1,
                                col_start: 1,
                                col_end: 1,
                            },
                        ));
                    }
                }
                Err(e) => {
                    eprintln!("sui-clippy: {e:#}");
                    diags.push(Diagnostic::new(
                        "sui_move_lint",
                        format!("failed to run `sui move build --lint` for {}: {e:#}", package_root.display()),
                        Severity::Error,
                        sui_clippy_utils::Span {
                            path: move_toml,
                            line_start: 1,
                            line_end: 1,
                            col_start: 1,
                            col_end: 1,
                        },
                    ));
                }
            }
        }
    }

    diags.sort_by(|a, b| {
        a.span
            .path
            .cmp(&b.span.path)
            .then_with(|| a.span.line_start.cmp(&b.span.line_start))
    });

    let exit_err = diags.iter().any(|d| d.severity == Severity::Error);
    let sui_lint_reported = diags.iter().any(|d| d.lint_id == "sui_move_lint");

    match format {
        OutputFormat::Json => {
            for d in &diags {
                println!("{}", serde_json::to_string(d)?);
            }
        }
        OutputFormat::Sarif => {
            let doc = diagnostics_to_sarif_json(
                "sui-clippy",
                env!("CARGO_PKG_VERSION"),
                "https://github.com/hoh/sui-clippy",
                &diags,
            )?;
            println!("{doc}");
        }
        OutputFormat::Text => {
            for d in &diags {
                print_diagnostic(d);
            }
            if diags.is_empty() && !with_sui_lint {
                println!("sui-clippy: no issues from built-in lints.");
            }
            if sui_lint_reported {
                eprintln!(
                    "sui-clippy: `sui move build --lint` reported issues for one or more packages (see diagnostics above)."
                );
            }
        }
    }

    let exit_code = if exit_err { 1 } else { 0 };
    Ok(RunOutcome {
        exit_code,
        diagnostics: diags,
    })
}

struct SuiInvokeResult {
    success: bool,
    code: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

fn invoke_sui_move_lint(package_root: &Path) -> anyhow::Result<SuiInvokeResult> {
    let out = Command::new("sui")
        .args(["move", "build", "--lint"])
        .current_dir(package_root)
        .output();
    match out {
        Ok(o) => Ok(SuiInvokeResult {
            success: o.status.success(),
            code: o.status.code(),
            stdout: o.stdout,
            stderr: o.stderr,
        }),
        Err(e) => anyhow::bail!(
            "failed to spawn `sui move build --lint`: {e}\nInstall the Sui CLI or omit --with-sui-lint."
        ),
    }
}

fn print_diagnostic(d: &Diagnostic) {
    let sev = match d.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
        Severity::Allow => "allow",
    };
    println!(
        "{}: {} [{}]: {}",
        d.span.path.display(),
        sev,
        d.lint_id,
        d.message
    );
    println!(
        "  --> {}:{}:{}",
        d.span.path.display(),
        d.span.line_start,
        d.span.col_start
    );
    if let Some(n) = &d.note {
        println!("   = note: {n}");
    }
}

fn find_package_root(start: &Path) -> Option<PathBuf> {
    let mut cur = start.to_path_buf();
    loop {
        if cur.join("Move.toml").is_file() {
            return Some(cur);
        }
        if !cur.pop() {
            return None;
        }
    }
}

fn collect_move_files(package_root: &Path, user_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if user_path.is_file() {
        return Ok(vec![user_path.to_path_buf()]);
    }
    let sources = package_root.join("sources");
    if !sources.is_dir() {
        anyhow::bail!(
            "expected `sources/` under {}; run from a Move package root",
            package_root.display()
        );
    }
    let mut files = Vec::new();
    for e in WalkDir::new(&sources).into_iter().filter_map(Result::ok) {
        if !e.file_type().is_file() {
            continue;
        }
        if e.path().extension().is_some_and(|x| x == "move") {
            files.push(e.path().to_path_buf());
        }
    }
    files.sort();
    Ok(files)
}
