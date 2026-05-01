use std::path::PathBuf;

use serde_json::Value;
use sui_clippy_utils::{diagnostics_to_sarif_json, Diagnostic, Severity, Span};

#[test]
fn sarif_has_version_and_results() {
    let d = Diagnostic::new(
        "todo_comment",
        "example",
        Severity::Warning,
        Span {
            path: PathBuf::from("/tmp/example.move"),
            line_start: 1,
            line_end: 1,
            col_start: 1,
            col_end: 5,
        },
    );
    let s = diagnostics_to_sarif_json("sui-clippy", "0.4.0", "https://example.invalid", &[d])
        .unwrap();
    let v: Value = serde_json::from_str(&s).unwrap();
    assert_eq!(v["version"], "2.1.0");
    assert!(!v["runs"][0]["results"].as_array().unwrap().is_empty());
    let rules = v["runs"][0]["tool"]["driver"]["rules"]
        .as_array()
        .expect("driver.rules");
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0]["id"], "todo_comment");
    assert_eq!(rules[0]["shortDescription"]["text"], "example");
    let ext = v["runs"][0]["tool"]["extensions"]
        .as_array()
        .expect("tool.extensions");
    assert!(!ext.is_empty());
    assert_eq!(ext[0]["name"], "sui-clippy-lints");
}
