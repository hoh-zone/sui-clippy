use std::path::PathBuf;

use declare_sui_clippy_lint::LintLevel;
use sui_clippy_config::SuiClippyConfig;
use sui_clippy_lints::{run_source_lints, LintRunOptions};
use sui_clippy_utils::SourceFile;

fn fixture(name: &str) -> (PathBuf, PathBuf) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pkg");
    let file = root.join("sources").join(name);
    (root, file)
}

#[test]
fn todo_comment_fires() {
    let (root, path) = fixture("demo.move");
    let text = std::fs::read_to_string(&path).unwrap();
    let sf = SourceFile::new(path, text);
    let diags = run_source_lints(
        &sf,
        &root,
        &SuiClippyConfig::default(),
        &LintRunOptions::default(),
    );
    assert!(
        diags.iter().any(|d| d.lint_id == "todo_comment"),
        "{diags:?}"
    );
}

#[test]
fn tx_context_sender_fires() {
    let (root, path) = fixture("demo.move");
    let text = std::fs::read_to_string(&path).unwrap();
    let sf = SourceFile::new(path, text);
    let diags = run_source_lints(
        &sf,
        &root,
        &SuiClippyConfig::default(),
        &LintRunOptions::default(),
    );
    assert!(
        diags.iter().any(|d| d.lint_id == "tx_context_sender"),
        "{diags:?}"
    );
}

#[test]
fn public_fun_transfer_respects_allow_by_default() {
    let (root, path) = fixture("demo.move");
    let text = std::fs::read_to_string(&path).unwrap();
    let sf = SourceFile::new(path, text);
    let diags = run_source_lints(
        &sf,
        &root,
        &SuiClippyConfig::default(),
        &LintRunOptions::default(),
    );
    assert!(
        !diags.iter().any(|d| d.lint_id == "public_fun_transfer"),
        "security lint should be allow by default: {diags:?}"
    );
}

#[test]
fn public_fun_transfer_when_included() {
    let (root, path) = fixture("demo.move");
    let text = std::fs::read_to_string(&path).unwrap();
    let sf = SourceFile::new(path, text);
    let diags = run_source_lints(
        &sf,
        &root,
        &SuiClippyConfig::default(),
        &LintRunOptions {
            include_allowed: true,
            ..Default::default()
        },
    );
    assert!(
        diags.iter().any(|d| d.lint_id == "public_fun_transfer"),
        "{diags:?}"
    );
}

#[test]
fn bare_abort_fires() {
    let (root, path) = fixture("demo.move");
    let text = std::fs::read_to_string(&path).unwrap();
    let sf = SourceFile::new(path, text);
    let diags = run_source_lints(
        &sf,
        &root,
        &SuiClippyConfig::default(),
        &LintRunOptions::default(),
    );
    assert!(
        diags.iter().any(|d| d.lint_id == "bare_abort"),
        "{diags:?}"
    );
}

#[test]
fn dynamic_field_respects_allow_by_default() {
    let (root, path) = fixture("demo.move");
    let text = std::fs::read_to_string(&path).unwrap();
    let sf = SourceFile::new(path, text);
    let diags = run_source_lints(
        &sf,
        &root,
        &SuiClippyConfig::default(),
        &LintRunOptions::default(),
    );
    assert!(
        !diags.iter().any(|d| d.lint_id == "dynamic_field_access"),
        "{diags:?}"
    );
}

#[test]
fn dynamic_field_cli_warn() {
    let (root, path) = fixture("demo.move");
    let text = std::fs::read_to_string(&path).unwrap();
    let sf = SourceFile::new(path, text);
    let mut opts = LintRunOptions::default();
    opts
        .cli_overrides
        .insert("dynamic_field_access".into(), LintLevel::Warn);
    let diags = run_source_lints(&sf, &root, &SuiClippyConfig::default(), &opts);
    assert!(
        diags.iter().any(|d| d.lint_id == "dynamic_field_access"),
        "{diags:?}"
    );
}

#[test]
fn clock_timestamp_fires() {
    let (root, path) = fixture("demo.move");
    let text = std::fs::read_to_string(&path).unwrap();
    let sf = SourceFile::new(path, text);
    let diags = run_source_lints(
        &sf,
        &root,
        &SuiClippyConfig::default(),
        &LintRunOptions::default(),
    );
    assert!(
        diags.iter().any(|d| d.lint_id == "clock_timestamp"),
        "{diags:?}"
    );
}

#[test]
fn test_only_in_sources_when_enabled() {
    let (root, path) = fixture("demo.move");
    let text = std::fs::read_to_string(&path).unwrap();
    let sf = SourceFile::new(path, text);
    let mut opts = LintRunOptions::default();
    opts
        .cli_overrides
        .insert("test_only_in_sources".into(), LintLevel::Warn);
    let diags = run_source_lints(&sf, &root, &SuiClippyConfig::default(), &opts);
    assert!(
        diags.iter().any(|d| d.lint_id == "test_only_in_sources"),
        "{diags:?}"
    );
}
