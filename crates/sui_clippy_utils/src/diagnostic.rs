use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Allow,
    Note,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct Span {
    pub path: PathBuf,
    pub line_start: u32,
    pub line_end: u32,
    pub col_start: u32,
    pub col_end: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub lint_id: String,
    pub message: String,
    pub severity: Severity,
    pub span: Span,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Diagnostic {
    #[must_use]
    pub fn new(
        lint_id: impl Into<String>,
        message: impl Into<String>,
        severity: Severity,
        span: Span,
    ) -> Self {
        Self {
            lint_id: lint_id.into(),
            message: message.into(),
            severity,
            span,
            note: None,
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}
