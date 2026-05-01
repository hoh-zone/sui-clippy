use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: PathBuf,
    pub text: String,
    /// Path relative to the package root (forward slashes), when known (e.g. from the CLI runner).
    pub rel_to_package: Option<String>,
}

/// Whole-line `//` comments (leading whitespace allowed). Does not handle block comments.
#[must_use]
pub fn is_line_comment_line(line: &str) -> bool {
    line.trim_start().starts_with("//")
}

impl SourceFile {
    #[must_use]
    pub fn new(path: PathBuf, text: String) -> Self {
        Self {
            path,
            text,
            rel_to_package: None,
        }
    }

    #[must_use]
    pub fn with_rel(path: PathBuf, text: String, rel_to_package: impl Into<String>) -> Self {
        Self {
            path,
            text,
            rel_to_package: Some(rel_to_package.into().replace('\\', "/")),
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn lines(&self) -> impl Iterator<Item = (u32, &str)> + '_ {
        self.text
            .lines()
            .enumerate()
            .map(|(i, line)| (i as u32 + 1, line))
    }
}
