use std::path::PathBuf;
use std::process::Command;

#[test]
fn typed_without_move_compiler_feature_fails() {
    let exe = env!("CARGO_BIN_EXE_sui-clippy");
    let pkg = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../sui_clippy_lints/tests/fixtures/pkg");
    let out = Command::new(exe)
        .args(["--typed", "--skip-manifest"])
        .current_dir(&pkg)
        .output()
        .expect("run sui-clippy");
    assert!(
        !out.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("move_compiler") || String::from_utf8_lossy(&out.stdout).contains("move_compiler"),
        "{stderr}"
    );
}
