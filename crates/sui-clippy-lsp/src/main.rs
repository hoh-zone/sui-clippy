#![forbid(unsafe_code)]

//! Minimal stdio LSP: `textDocument/didOpen` and `didSave` publish diagnostics from
//! `sui_clippy_lints::run_source_lints` (same passes as the CLI).

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use anyhow::Context;
use lsp_server::{Connection, Message, Response};
use lsp_types::notification::Notification;
use lsp_types::notification::{DidOpenTextDocument, DidSaveTextDocument};
use lsp_types::{
    Diagnostic as LspDiagnostic, DiagnosticSeverity, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, NumberOrString, Position, PublishDiagnosticsParams, Range,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
};
use sui_clippy_config::SuiClippyConfig;
use sui_clippy_lints::{run_source_lints, LintRunOptions};
use sui_clippy_utils::{Diagnostic as ScDiagnostic, Severity, SourceFile};

fn uri_to_path(uri: &Uri) -> anyhow::Result<PathBuf> {
    url::Url::parse(uri.as_str())
        .ok()
        .and_then(|u| u.to_file_path().ok())
        .context("only file:// URIs are supported")
}

fn find_move_package_root(start: &Path) -> Option<PathBuf> {
    let mut cur = start.to_path_buf();
    loop {
        if cur.join("Move.toml").is_file() {
            return Some(cur);
        }
        if !cur.pop() {
            return None;
        }
    }
}

fn sc_to_lsp(d: ScDiagnostic) -> LspDiagnostic {
    let range = Range {
        start: Position {
            line: d.span.line_start.saturating_sub(1),
            character: d.span.col_start.saturating_sub(1),
        },
        end: Position {
            line: d.span.line_end.saturating_sub(1),
            character: d.span.col_end.saturating_sub(1),
        },
    };
    let severity = match d.severity {
        Severity::Error => Some(DiagnosticSeverity::ERROR),
        Severity::Warning => Some(DiagnosticSeverity::WARNING),
        Severity::Note => Some(DiagnosticSeverity::INFORMATION),
        Severity::Allow => Some(DiagnosticSeverity::HINT),
    };
    LspDiagnostic {
        range,
        severity,
        code: Some(NumberOrString::String(d.lint_id)),
        source: Some("sui-clippy".to_string()),
        message: d.message,
        ..Default::default()
    }
}

fn lint_open_buffer(uri: &Uri, text: String, version: i32) -> anyhow::Result<PublishDiagnosticsParams> {
    let path = uri_to_path(uri)?;
    if path.extension() != Some(OsStr::new("move")) {
        return Ok(PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics: vec![],
            version: Some(version),
        });
    }
    let pkg_root = find_move_package_root(path.parent().unwrap_or(Path::new(".")))
        .with_context(|| format!("no Move.toml above {}", path.display()))?;
    let rel = path
        .strip_prefix(&pkg_root)
        .ok()
        .and_then(|p| p.to_str().map(|s| s.replace('\\', "/")));
    let sf = match rel {
        Some(r) => SourceFile::with_rel(path.clone(), text, r),
        None => SourceFile::new(path.clone(), text),
    };
    let config = SuiClippyConfig::load(&pkg_root).unwrap_or_default();
    let opts = LintRunOptions {
        include_allowed: false,
        cli_overrides: Default::default(),
    };
    let diagnostics = run_source_lints(&sf, &pkg_root, &config, &opts)
        .into_iter()
        .map(sc_to_lsp)
        .collect();
    Ok(PublishDiagnosticsParams {
        uri: uri.clone(),
        diagnostics,
        version: Some(version),
    })
}

fn lint_saved_file(uri: &Uri) -> anyhow::Result<PublishDiagnosticsParams> {
    let path = uri_to_path(uri)?;
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("read {}", path.display()))?;
    lint_open_buffer(uri, text, 0)
}

fn main() -> anyhow::Result<()> {
    let (conn, io_threads) = Connection::stdio();
    let caps = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        ..Default::default()
    })?;
    let _init = conn.initialize(caps)?;

    for msg in conn.receiver.iter() {
        match msg {
            Message::Notification(n) => {
                if n.method == DidOpenTextDocument::METHOD {
                    let params: DidOpenTextDocumentParams =
                        serde_json::from_value(n.params.clone())?;
                    let doc = params.text_document;
                    if let Ok(publish) = lint_open_buffer(&doc.uri, doc.text, doc.version) {
                        conn.sender.send(Message::Notification(lsp_server::Notification {
                            method: "textDocument/publishDiagnostics".into(),
                            params: serde_json::to_value(&publish)?,
                        }))?;
                    }
                } else if n.method == DidSaveTextDocument::METHOD {
                    let params: DidSaveTextDocumentParams =
                        serde_json::from_value(n.params.clone())?;
                    if let Ok(mut publish) = lint_saved_file(&params.text_document.uri) {
                        publish.version = None;
                        conn.sender.send(Message::Notification(lsp_server::Notification {
                            method: "textDocument/publishDiagnostics".into(),
                            params: serde_json::to_value(&publish)?,
                        }))?;
                    }
                }
            }
            Message::Request(req) => {
                if conn.handle_shutdown(&req)? {
                    break;
                }
                let resp = Response::new_err(
                    req.id,
                    lsp_server::ErrorCode::MethodNotFound as i32,
                    "unsupported request".into(),
                );
                conn.sender.send(Message::Response(resp))?;
            }
            _ => {}
        }
    }
    io_threads.join()?;
    Ok(())
}
