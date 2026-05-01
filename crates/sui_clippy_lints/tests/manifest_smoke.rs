use std::path::PathBuf;

use declare_sui_clippy_lint::LintLevel;
use sui_clippy_config::SuiClippyConfig;
use sui_clippy_lints::{run_manifest_lints, LintRunOptions};
use sui_clippy_utils::Severity;

#[test]
fn manifest_flags_missing_edition_and_deps() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/bad_manifest");
    let move_toml = root.join("Move.toml");
    let raw = std::fs::read_to_string(&move_toml).unwrap();
    let diags = run_manifest_lints(
        &move_toml,
        &raw,
        &SuiClippyConfig::default(),
        &LintRunOptions::default(),
    )
    .unwrap();
    assert!(
        diags.iter().any(|d| d.lint_id == "missing_move_edition"),
        "{diags:?}"
    );
    assert!(
        diags.iter().any(|d| d.lint_id == "git_dep_unpinned"),
        "{diags:?}"
    );
    let wild = diags
        .iter()
        .find(|d| d.lint_id == "wildcard_git_ref")
        .expect("wildcard");
    assert_eq!(wild.severity, Severity::Error);
}

#[test]
fn manifest_allow_missing_edition() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/bad_manifest");
    let move_toml = root.join("Move.toml");
    let raw = std::fs::read_to_string(&move_toml).unwrap();
    let mut cfg = SuiClippyConfig::default();
    cfg.lint_levels.insert(
        "missing_move_edition".into(),
        LintLevel::Allow,
    );
    let diags = run_manifest_lints(
        &move_toml,
        &raw,
        &cfg,
        &LintRunOptions::default(),
    )
    .unwrap();
    assert!(
        !diags
            .iter()
            .any(|d| d.lint_id == "missing_move_edition"),
        "{diags:?}"
    );
}
