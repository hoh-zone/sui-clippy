use std::fs;
use std::path::PathBuf;

use sui_clippy_lints::insert_default_edition_if_missing;

#[test]
fn insert_edition_writes_move_toml() {
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("Move.toml");
    fs::write(
        &path,
        r#"[package]
name = "tmp"
version = "0.0.1"

[dependencies]
"#,
    )
    .unwrap();
    assert!(insert_default_edition_if_missing(&path, "2024.beta").unwrap());
    let s = fs::read_to_string(&path).unwrap();
    assert!(s.contains("edition = \"2024.beta\""));
    assert!(!insert_default_edition_if_missing(&path, "2024.beta").unwrap());
}

#[test]
fn insert_edition_from_fixture_template() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/bad_manifest/Move.toml");
    let tmp = tempfile::tempdir().unwrap();
    let path = tmp.path().join("Move.toml");
    fs::copy(&root, &path).unwrap();
    assert!(insert_default_edition_if_missing(&path, "2024.beta").unwrap());
    let v: toml::Value = toml::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(
        v["package"]["edition"].as_str(),
        Some("2024.beta"),
        "{v:?}"
    );
}
