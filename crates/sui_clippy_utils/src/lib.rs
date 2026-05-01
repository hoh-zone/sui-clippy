#![forbid(unsafe_code)]

mod diagnostic;
mod sarif;
mod source_file;

pub use diagnostic::{Diagnostic, Severity, Span};
pub use sarif::diagnostics_to_sarif_json;
pub use source_file::{is_line_comment_line, SourceFile};
