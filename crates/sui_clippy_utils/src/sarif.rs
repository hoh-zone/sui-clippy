//! Minimal SARIF 2.1.0 serialization for CI (GitHub Code Scanning–compatible shape).

use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Context;
use serde::Serialize;

use crate::{Diagnostic, Severity};

#[derive(Serialize)]
struct SarifRoot {
    version: &'static str,
    #[serde(rename = "$schema")]
    schema: &'static str,
    runs: [SarifRun; 1],
}

#[derive(Serialize)]
struct SarifRun {
    tool: Tool,
    results: Vec<SarifResult>,
}

#[derive(Serialize)]
struct Tool {
    driver: Driver,
    /// SARIF `toolComponent` entries (GitHub Code Scanning reads `driver` + optional `extensions`).
    extensions: Vec<ToolExtension>,
}

#[derive(Serialize)]
struct ToolExtension {
    name: String,
    #[serde(rename = "semanticVersion")]
    semantic_version: String,
    #[serde(rename = "informationUri")]
    information_uri: String,
}

#[derive(Serialize)]
struct Driver {
    name: String,
    version: String,
    information_uri: String,
    rules: Vec<SarifRule>,
}

#[derive(Serialize)]
struct SarifRule {
    id: String,
    #[serde(rename = "shortDescription")]
    short_description: Message,
}

#[derive(Serialize)]
struct SarifResult {
    rule_id: String,
    level: &'static str,
    message: Message,
    locations: Vec<Location>,
}

#[derive(Serialize)]
struct Message {
    text: String,
}

#[derive(Serialize)]
struct Location {
    physical_location: PhysicalLocation,
}

#[derive(Serialize)]
struct PhysicalLocation {
    artifact_location: ArtifactLocation,
    region: Region,
}

#[derive(Serialize)]
struct ArtifactLocation {
    uri: String,
}

#[derive(Serialize)]
struct Region {
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

fn severity_to_sarif_level(s: Severity) -> &'static str {
    match s {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
        Severity::Allow => "note",
    }
}

/// Build a single-run SARIF document with all diagnostics.
pub fn diagnostics_to_sarif_json(
    driver_name: &str,
    driver_version: &str,
    information_uri: &str,
    diags: &[Diagnostic],
) -> anyhow::Result<String> {
    let extensions = vec![ToolExtension {
        name: format!("{driver_name}-lints"),
        semantic_version: driver_version.to_string(),
        information_uri: information_uri.to_string(),
    }];
    let rule_ids: BTreeSet<&str> = diags.iter().map(|d| d.lint_id.as_str()).collect();
    let rules: Vec<SarifRule> = rule_ids
        .into_iter()
        .map(|id| {
            let text = diags
                .iter()
                .find(|d| d.lint_id == id)
                .map(|d| d.message.clone())
                .unwrap_or_else(|| id.to_string());
            SarifRule {
                id: id.to_string(),
                short_description: Message { text },
            }
        })
        .collect();

    let results: Vec<SarifResult> = diags
        .iter()
        .map(|d| {
            let uri = path_to_uri(&d.span.path).unwrap_or_else(|| d.span.path.display().to_string());
            SarifResult {
                rule_id: d.lint_id.clone(),
                level: severity_to_sarif_level(d.severity),
                message: Message {
                    text: d.message.clone(),
                },
                locations: vec![Location {
                    physical_location: PhysicalLocation {
                        artifact_location: ArtifactLocation { uri },
                        region: Region {
                            start_line: d.span.line_start,
                            start_column: d.span.col_start,
                            end_line: d.span.line_end,
                            end_column: d.span.col_end,
                        },
                    },
                }],
            }
        })
        .collect();

    let root = SarifRoot {
        version: "2.1.0",
        schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        runs: [SarifRun {
            tool: Tool {
                driver: Driver {
                    name: driver_name.to_string(),
                    version: driver_version.to_string(),
                    information_uri: information_uri.to_string(),
                    rules,
                },
                extensions,
            },
            results,
        }],
    };

    serde_json::to_string_pretty(&root).context("serialize SARIF")
}

fn path_to_uri(path: &Path) -> Option<String> {
    let abs = std::fs::canonicalize(path).ok()?;
    let s = abs.to_string_lossy();
    if s.starts_with('/') {
        Some(format!("file://{s}"))
    } else {
        Some(format!("file:///{}", s.replace('\\', "/")))
    }
}
