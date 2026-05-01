#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use declare_sui_clippy_lint::LintLevel;
use globset::Glob;
use serde::{Deserialize, Serialize};

/// Resolved configuration for a package root.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuiClippyConfig {
    /// Per-lint level overrides (`lint_id` -> level).
    #[serde(default)]
    pub lint_levels: HashMap<String, LintLevel>,
    /// Glob patterns relative to package root; matched paths are skipped.
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawFile {
    #[serde(default)]
    exclude: Vec<String>,
    #[serde(default)]
    lint: HashMap<String, String>,
}

impl SuiClippyConfig {
    /// Load `sui-clippy.toml` from `package_root`, or return defaults if missing.
    pub fn load(package_root: &Path) -> anyhow::Result<Self> {
        let path = package_root.join("sui-clippy.toml");
        if !path.is_file() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("read {}", path.display()))?;
        let raw: RawFile = toml::from_str(&text)
            .with_context(|| format!("parse {}", path.display()))?;
        let mut lint_levels = HashMap::new();
        for (id, s) in raw.lint {
            let level = parse_level(&s).with_context(|| format!("invalid level for `{id}`"))?;
            lint_levels.insert(id, level);
        }
        Ok(Self {
            lint_levels,
            exclude: raw.exclude,
        })
    }

    #[must_use]
    pub fn level_for(&self, lint_id: &str, category_default: LintLevel) -> LintLevel {
        self.lint_levels
            .get(lint_id)
            .copied()
            .unwrap_or(category_default)
    }

    /// `relative_path` uses `/` separators relative to the package root.
    #[must_use]
    pub fn is_excluded(&self, relative_path: &str) -> bool {
        self.exclude.iter().any(|pattern| {
            Glob::new(pattern)
                .ok()
                .is_some_and(|g| g.compile_matcher().is_match(relative_path))
        })
    }
}

fn parse_level(s: &str) -> anyhow::Result<LintLevel> {
    match s.to_ascii_lowercase().as_str() {
        "allow" => Ok(LintLevel::Allow),
        "warn" | "warning" => Ok(LintLevel::Warn),
        "deny" | "forbid" => Ok(LintLevel::Deny),
        _ => anyhow::bail!("unknown level `{s}`"),
    }
}
