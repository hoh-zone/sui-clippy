use declare_sui_clippy_lint::{declare_sui_clippy_lint, LintDef};
use regex::Regex;
use sui_clippy_utils::{is_line_comment_line, Diagnostic, Severity, SourceFile, Span};

use crate::pass::SourceLintPass;

declare_sui_clippy_lint! {
    pub LINT,
    "public_fun_transfer",
    Security,
    "transfer::* inside a `public fun` body widens who may invoke object moves; confirm this matches your capability model."
}

pub struct Pass;

pub static PASS: Pass = Pass;

impl SourceLintPass for Pass {
    fn lint_def(&self) -> &'static LintDef {
        &LINT
    }

    fn check_file(&self, file: &SourceFile) -> Vec<Diagnostic> {
        let transfer_re = Regex::new(r"\btransfer::(public_)?(transfer|share_object|freeze_object)\b")
            .expect("valid regex");
        let public_fun_re = Regex::new(r"\bpublic\s+fun\s+").expect("valid regex");

        let text = file.text.as_str();
        let mut diags = Vec::new();

        let mut search = 0usize;
        while let Some(m) = public_fun_re.find(&text[search..]) {
            let abs = search + m.start();
            let Some(open) = text[abs..].find('{') else {
                break;
            };
            let body_start = abs + open;
            let Some(body_end) = matching_brace_end(text, body_start) else {
                search = abs + m.len();
                continue;
            };
            let body = &text[body_start..=body_end];
            for cap in transfer_re.find_iter(body) {
                let abs_start = body_start + cap.start();
                let abs_end = body_start + cap.end();
                let (line_start, col_start) = byte_index_to_line_col(text, abs_start);
                let (line_end, col_end) = byte_index_to_line_col(text, abs_end.saturating_sub(1));
                let line_text = text
                    .lines()
                    .nth(line_start.saturating_sub(1) as usize)
                    .unwrap_or("");
                if is_line_comment_line(line_text) {
                    continue;
                }
                diags.push(Diagnostic::new(
                    LINT.id,
                    "transfer API appears inside `public fun` body",
                    Severity::Warning,
                    Span {
                        path: file.path().to_path_buf(),
                        line_start,
                        line_end,
                        col_start,
                        col_end,
                    },
                ));
            }
            search = body_end + 1;
        }
        diags
    }
}

fn matching_brace_end(text: &str, open_brace: usize) -> Option<usize> {
    if text.as_bytes().get(open_brace) != Some(&b'{') {
        return None;
    }
    let mut depth = 0i32;
    let mut in_str = false;
    let mut escape = false;
    let bytes = text.as_bytes();
    let mut i = open_brace;
    while i < bytes.len() {
        let b = bytes[i];
        if in_str {
            if escape {
                escape = false;
            } else if b == b'\\' {
                escape = true;
            } else if b == b'"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        if b == b'"' {
            in_str = true;
            i += 1;
            continue;
        }
        match b {
            b'{' => {
                depth += 1;
            }
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn byte_index_to_line_col(text: &str, byte_idx: usize) -> (u32, u32) {
    let mut line = 1u32;
    let mut col = 1u32;
    let mut i = 0usize;
    for ch in text.chars() {
        if i >= byte_idx {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
        i += ch.len_utf8();
    }
    (line, col)
}
