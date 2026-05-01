//! Safe, opt-in manifest edits (explicit CLI flag only).

use std::fs;
use std::path::Path;

use anyhow::Context;

/// If `[package]` exists and has no `edition`, insert `edition = "<edition>"` immediately after the `[package]` line.
///
/// Returns `true` if the file was modified.
pub fn insert_default_edition_if_missing(
    move_toml_path: &Path,
    edition: &str,
) -> anyhow::Result<bool> {
    let raw = fs::read_to_string(move_toml_path)
        .with_context(|| format!("read {}", move_toml_path.display()))?;
    let value: toml::Value = toml::from_str(&raw).context("parse Move.toml")?;
    let Some(pkg) = value.get("package").and_then(|p| p.as_table()) else {
        anyhow::bail!("Move.toml has no [package] table");
    };
    if pkg.contains_key("edition") {
        return Ok(false);
    }

    let lines: Vec<&str> = raw.lines().collect();
    let mut out = String::new();
    let mut inserted = false;
    for (i, line) in lines.iter().enumerate() {
        out.push_str(line);
        out.push('\n');
        if line.trim() == "[package]" && !inserted {
            let next = lines.get(i + 1).copied().unwrap_or("");
            if next.trim().starts_with("edition") {
                continue;
            }
            out.push_str(&format!("edition = \"{edition}\"\n"));
            inserted = true;
        }
    }

    if !inserted {
        anyhow::bail!("could not find a `[package]` line to insert `edition` after");
    }

    fs::write(move_toml_path, out).with_context(|| format!("write {}", move_toml_path.display()))?;
    Ok(true)
}
