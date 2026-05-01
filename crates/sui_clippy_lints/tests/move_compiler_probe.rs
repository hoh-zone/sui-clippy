#![cfg(feature = "move_compiler")]

use std::path::PathBuf;

#[test]
fn typed_probe_reports_diagnostics_for_fixture_without_deps() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pkg");
    let diags = sui_clippy_lints::typed::run_compiler_probe(&root).expect("probe");
    assert!(
        !diags.is_empty(),
        "expected Move compiler diagnostics (fixture references `sui::` without deps)"
    );
    assert!(diags.iter().all(|d| d.lint_id == "move_compiler"));
}
