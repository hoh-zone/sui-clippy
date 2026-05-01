# GitHub Code Scanning with sui-clippy SARIF

This repository emits [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) when you pass `--format sarif`. The document includes `runs[0].tool.driver` (name, version, `informationUri`, `rules`) and `runs[0].tool.extensions` with a secondary tool component for downstream consumers.

## Upload from GitHub Actions

1. Run sui-clippy on your Move package and write SARIF to a file, for example:

   ```bash
   sui-clippy --format sarif /path/to/move/package > results.sarif
   ```

2. Use the official [`github/codeql-action/upload-sarif`](https://github.com/github/codeql-action/tree/main/upload-sarif) step so Code Scanning ingests the file:

   ```yaml
   - name: Run sui-clippy (SARIF)
     run: sui-clippy --format sarif ./my-move-pkg > sui-clippy.sarif

   - name: Upload SARIF
     uses: github/codeql-action/upload-sarif@v3
     with:
       sarif_file: sui-clippy.sarif
   ```

3. Ensure the job has the `security-events: write` permission when required by your organization’s Code Scanning defaults.

## Notes

- Paths in SARIF use `file://` URIs when the diagnostic span can be canonicalized on disk.
- Compiler-backed diagnostics from `--typed` (when built with `--features move_compiler`) use the rule id `move_compiler` in SARIF `results[].ruleId`.
