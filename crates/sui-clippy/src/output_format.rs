use clap::ValueEnum;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable diagnostics (default).
    #[default]
    Text,
    /// One JSON object per line (NDJSON), one diagnostic per line.
    Json,
    /// Single SARIF 2.1.0 JSON document on stdout.
    Sarif,
}
