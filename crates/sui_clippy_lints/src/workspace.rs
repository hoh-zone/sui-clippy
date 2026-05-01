//! Discover Move package roots for single-package and `[workspace].members` layouts.

use std::path::{Path, PathBuf};

use anyhow::Context;

/// If `Move.toml` defines `[workspace].members`, return each member directory that contains its
/// own `Move.toml` (sorted). Otherwise return a single root: `workspace_dir` (the directory
/// containing the manifest used for discovery).
pub fn discover_workspace_package_roots(workspace_dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let manifest_path = workspace_dir.join("Move.toml");
    if !manifest_path.is_file() {
        anyhow::bail!(
            "expected Move.toml at {}",
            manifest_path.display()
        );
    }
    let raw = std::fs::read_to_string(&manifest_path)
        .with_context(|| format!("read {}", manifest_path.display()))?;
    let v: toml::Value = toml::from_str(&raw).context("parse Move.toml")?;
    if let Some(ws) = v.get("workspace").and_then(|x| x.as_table())
        && let Some(members) = ws.get("members").and_then(|x| x.as_array())
    {
        let mut out = Vec::new();
        for m in members {
            if let Some(s) = m.as_str() {
                let member_root = workspace_dir.join(s);
                if member_root.join("Move.toml").is_file() {
                    out.push(member_root);
                }
            }
        }
        if !out.is_empty() {
            out.sort();
            return Ok(out);
        }
    }
    Ok(vec![workspace_dir.to_path_buf()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_members_expand() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/move_workspace");
        let roots = discover_workspace_package_roots(&root).unwrap();
        assert_eq!(roots.len(), 2);
        assert_eq!(
            roots[0].file_name().and_then(|s| s.to_str()),
            Some("pkg_a")
        );
        assert_eq!(
            roots[1].file_name().and_then(|s| s.to_str()),
            Some("pkg_b")
        );
    }

    #[test]
    fn single_package_fallback() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pkg");
        let roots = discover_workspace_package_roots(&root).unwrap();
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].file_name().and_then(|s| s.to_str()), Some("pkg"));
    }
}
