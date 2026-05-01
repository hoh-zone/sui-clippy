mod git_dep_unpinned;
mod missing_move_edition;
mod wildcard_git_ref;

pub mod fix;

mod engine;

use declare_sui_clippy_lint::LintDef;

pub use engine::run_manifest_lints;

pub static MANIFEST_LINT_DEFS: &[&LintDef] = &[
    &missing_move_edition::LINT,
    &git_dep_unpinned::LINT,
    &wildcard_git_ref::LINT,
];

fn first_line_containing(raw: &str, needle: &str) -> Option<u32> {
    raw.lines().enumerate().find_map(|(i, line)| {
        line.contains(needle)
            .then_some(u32::try_from(i).ok()? + 1)
    })
}

fn line_of_table_header(raw: &str, table: &str) -> Option<u32> {
    let header = format!("[{table}]");
    first_line_containing(raw, &header)
}

/// Yields `(section_name, dep_name, dep_inline_table)` for entries that declare `git`.
pub(crate) fn each_git_dep_table(root: &toml::Value) -> Vec<(&'static str, String, &toml::value::Table)> {
    let mut out = Vec::new();
    for sec in [
        "dependencies",
        "dev-dependencies",
        "build-dependencies",
    ] {
        let Some(deps) = root.get(sec).and_then(|v| v.as_table()) else {
            continue;
        };
        for (name, dep_val) in deps {
            let Some(tbl) = dep_val.as_table() else {
                continue;
            };
            if tbl.contains_key("git") {
                out.push((sec, name.clone(), tbl));
            }
        }
    }
    out
}
