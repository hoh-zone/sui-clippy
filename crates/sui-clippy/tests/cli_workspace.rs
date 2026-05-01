use std::path::PathBuf;
use std::process::Command;

#[test]
fn workspace_scans_each_member_package() {
    let ws = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../sui_clippy_lints/tests/fixtures/move_workspace");
    let out = Command::new(env!("CARGO_BIN_EXE_sui-clippy"))
        .args(["--workspace", "--skip-manifest"])
        .arg(&ws)
        .output()
        .expect("spawn sui-clippy");
    assert!(
        out.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
}
