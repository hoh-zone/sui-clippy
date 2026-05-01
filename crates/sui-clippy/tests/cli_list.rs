use std::process::Command;

use serde_json::Value;

#[test]
fn list_lints_prints_todo_comment() {
    let exe = env!("CARGO_BIN_EXE_sui-clippy");
    let out = Command::new(exe)
        .args(["--list-lints"])
        .output()
        .expect("run sui-clippy");
    assert!(out.status.success(), "stderr={}", String::from_utf8_lossy(&out.stderr));
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("todo_comment"), "{s}");
    assert!(s.contains("dynamic_field_access"), "{s}");
    assert!(s.contains("missing_move_edition"), "{s}");
    assert!(s.contains("git_dep_unpinned"), "{s}");
    assert!(s.contains("clock_timestamp"), "{s}");
    assert!(s.contains("test_only_in_sources"), "{s}");
}

#[test]
fn list_lints_json_is_array() {
    let exe = env!("CARGO_BIN_EXE_sui-clippy");
    let out = Command::new(exe)
        .args(["--list-lints", "--format", "json"])
        .output()
        .expect("run sui-clippy");
    assert!(out.status.success(), "stderr={}", String::from_utf8_lossy(&out.stderr));
    let v: Value = serde_json::from_slice(&out.stdout).expect("json");
    let arr = v.as_array().expect("array");
    assert!(arr.iter().any(|e| e["id"] == "clock_timestamp"));
}
