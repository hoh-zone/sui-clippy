# Lint index (generated)

> This file is produced by `cargo run -p sui-clippy-docgen`. Do not edit by hand.

See the [Lint categories](lints.md) chapter for what each group means.

| Lint ID | Category | Default level | Documentation |
|---------|----------|---------------|---------------|
| [`bare_abort`](lints_generated.html#bare_abort) | `suspicious` | `warn` | `abort` with a magic numeric code is easy to misuse; prefer named constants or structured errors. |
| [`clock_timestamp`](lints_generated.html#clock_timestamp) | `suspicious` | `warn` | Clock reads can be manipulated by validators in tests; on-chain code should treat timestamps as hints, not strong security boundaries. |
| [`dynamic_field_access`](lints_generated.html#dynamic_field_access) | `security` | `allow` | Dynamic field APIs are easy to get wrong; verify key types, ownership, and access control. |
| [`empty_public_entry`](lints_generated.html#empty_public_entry) | `correctness` | `deny` | A public entry function with an empty body is almost certainly a mistake or unsafe stub. |
| [`git_dep_unpinned`](lints_generated.html#git_dep_unpinned) | `suspicious` | `warn` | Git dependencies without `rev`, `branch`, or `tag` are not reproducible builds. |
| [`missing_move_edition`](lints_generated.html#missing_move_edition) | `style` | `warn` | Move.toml [package] should declare an `edition` (for example Move 2024) for consistent tooling and language behavior. |
| [`public_fun_transfer`](lints_generated.html#public_fun_transfer) | `security` | `allow` | transfer::* inside a `public fun` body widens who may invoke object moves; confirm this matches your capability model. |
| [`test_only_in_sources`](lints_generated.html#test_only_in_sources) | `nursery` | `allow` | `#[test_only]` in non-test paths is easy to ship accidentally; prefer `tests/` modules or `#[test_only]` only in test-only files. |
| [`todo_comment`](lints_generated.html#todo_comment) | `suspicious` | `warn` | Track and remove TODO/FIXME markers before release. |
| [`tx_context_sender`](lints_generated.html#tx_context_sender) | `suspicious` | `warn` | Using tx_context::sender() can encourage self-transfers; prefer returning objects to callers for PTB composability where appropriate. |
| [`wildcard_git_ref`](lints_generated.html#wildcard_git_ref) | `correctness` | `deny` | Wildcard `rev` or `branch` on a git dependency is almost never intended for production. |
