//! Typed / compiler-backed analysis (optional `move_compiler` feature).

use std::path::Path;

use sui_clippy_utils::Diagnostic;

#[cfg(feature = "move_compiler")]
pub fn run_compiler_probe(package_root: &Path) -> anyhow::Result<Vec<Diagnostic>> {
    run_compiler_probe_impl(package_root)
}

#[cfg(not(feature = "move_compiler"))]
pub fn run_compiler_probe(_package_root: &Path) -> anyhow::Result<Vec<Diagnostic>> {
    Ok(Vec::new())
}

#[cfg(feature = "move_compiler")]
fn run_compiler_probe_impl(package_root: &Path) -> anyhow::Result<Vec<Diagnostic>> {
    use std::collections::BTreeMap;
    use std::str::FromStr;

    use anyhow::Context;
    use move_compiler::diagnostics::Diagnostic as McDiagnostic;
    use move_compiler::editions::{Edition, Flavor};
    use move_compiler::shared::{NumericalAddress, PackageConfig};
    use move_compiler::{Compiler, Flags};

    let move_toml = package_root.join("Move.toml");
    let raw = std::fs::read_to_string(&move_toml)
        .with_context(|| format!("read {}", move_toml.display()))?;
    let v: toml::Value = toml::from_str(&raw).context("parse Move.toml")?;

    let edition_str = v
        .get("package")
        .and_then(|p| p.get("edition"))
        .and_then(|e| e.as_str())
        .unwrap_or("2024.beta");
    let edition = Edition::from_str(edition_str).context("parse edition")?;

    let flavor_str = v
        .get("package")
        .and_then(|p| p.get("flavor"))
        .and_then(|f| f.as_str())
        .unwrap_or("sui");
    let flavor = Flavor::from_str(flavor_str).context("parse flavor")?;

    let mut named_address_map = BTreeMap::new();
    if let Some(table) = v.get("addresses").and_then(|a| a.as_table()) {
        for (k, val) in table {
            if let Some(s) = val.as_str() {
                if s == "_" {
                    continue;
                }
                if let Ok(a) = NumericalAddress::parse_str(s) {
                    named_address_map.insert(k.clone(), a);
                }
            }
        }
    }

    let sources_dir = package_root.join("sources");
    if !sources_dir.is_dir() {
        return Ok(Vec::new());
    }

    let sources_str = sources_dir
        .canonicalize()
        .unwrap_or(sources_dir)
        .to_string_lossy()
        .into_owned();

    let mut pkg_cfg = PackageConfig::default();
    pkg_cfg.edition = edition;
    pkg_cfg.flavor = flavor;

    let compiler = Compiler::from_files(None, vec![sources_str], vec![], named_address_map)
        .set_default_config(pkg_cfg)
        .set_flags(Flags::testing());

    let (files, units_res) = compiler.build().context("move compiler build")?;

    let mc_diags: Vec<McDiagnostic> = match units_res {
        Ok((_units, warn_diags)) => warn_diags.into_vec(),
        Err(err_diags) => err_diags.into_vec(),
    };

    Ok(mc_diags
        .into_iter()
        .filter_map(|d| mc_diagnostic_to_ours(&files, d))
        .collect())
}

#[cfg(feature = "move_compiler")]
fn mc_diagnostic_to_ours(
    files: &move_compiler::shared::files::MappedFiles,
    d: move_compiler::diagnostics::Diagnostic,
) -> Option<Diagnostic> {
    use move_compiler::diagnostics::codes::Severity as McSeverity;
    use sui_clippy_utils::{Severity, Span};

    let loc = d.primary_loc();
    let span = files.position_opt(&loc)?;
    let path = files.file_path(&span.file_hash).clone();
    let start = span.start;
    let end = span.end;
    let line_start = start.user_line() as u32;
    let line_end = end.user_line() as u32;
    let col_start = start.user_column() as u32;
    let col_end = (end.user_column() as u32).max(col_start);

    let sev = match d.info().severity() {
        McSeverity::BlockingError | McSeverity::NonblockingError | McSeverity::Bug => {
            Severity::Error
        }
        McSeverity::Warning => Severity::Warning,
        McSeverity::Note => Severity::Note,
    };

    let msg = format!("{} — {}", d.info().message(), d.primary_msg());

    Some(Diagnostic::new(
        "move_compiler",
        msg,
        sev,
        Span {
            path,
            line_start,
            line_end,
            col_start,
            col_end,
        },
    ))
}

#[must_use]
pub const fn move_compiler_feature_enabled() -> bool {
    cfg!(feature = "move_compiler")
}
