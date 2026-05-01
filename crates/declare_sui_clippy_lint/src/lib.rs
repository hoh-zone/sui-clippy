//! Declarative lint metadata (Clippy-style) for sui-clippy.

#![forbid(unsafe_code)]

/// Lint categories aligned with rust-clippy defaults, plus Sui-specific groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LintCategory {
    Correctness,
    Suspicious,
    Style,
    Complexity,
    Perf,
    Pedantic,
    Nursery,
    Restriction,
    Cargo,
    /// Sui object model, PTB composability, framework usage.
    Sui,
    /// Security-sensitive heuristics (expect false positives; prefer `nursery` until stable).
    Security,
}

impl LintCategory {
    /// Default diagnostic level when not overridden by config.
    #[must_use]
    pub const fn default_level(self) -> LintLevel {
        match self {
            Self::Correctness => LintLevel::Deny,
            Self::Suspicious | Self::Style | Self::Complexity | Self::Perf => LintLevel::Warn,
            Self::Pedantic
            | Self::Nursery
            | Self::Restriction
            | Self::Cargo
            | Self::Sui
            | Self::Security => LintLevel::Allow,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LintLevel {
    Allow,
    Warn,
    Deny,
}

/// Static description of one lint rule.
#[derive(Debug)]
pub struct LintDef {
    pub id: &'static str,
    pub category: LintCategory,
    pub doc: &'static str,
}

/// Declare a lint static (metadata only). Implementations live in `sui_clippy_lints`.
#[macro_export]
macro_rules! declare_sui_clippy_lint {
    (
        $(#[$meta:meta])*
        $vis:vis $NAME:ident,
        $id:literal,
        $category:ident,
        $doc:literal
    ) => {
        $(#[$meta])*
        $vis static $NAME: $crate::LintDef = $crate::LintDef {
            id: $id,
            category: $crate::LintCategory::$category,
            doc: $doc,
        };
    };
}
