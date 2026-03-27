# SQL Integration Fixtures

This directory contains SQL scripts used by Rust integration tests to validate the SQLite extension through real SQL execution.

## Current fixture

- `smoke-load.sql`
  - A single, comprehensive fixture (by project choice) covering:
    - all currently implemented `p_*` SQL signatures (null-smoke rows)
    - representative happy-path checks
    - explicit edge-case checks across numeric, text/regex, binary/hash, URL/IP/UUID families

## Conventions

To keep golden outputs stable and reviewable:

1. Emit deterministic, single-row outputs per assertion.
2. Prefix each assertion with a label (`SECTION|...`, `SMOKE|...`, `EDGE|...`).
3. Avoid nondeterministic raw outputs (e.g., random/uuid values); assert deterministic derivatives (type/length/pattern) instead.
4. Keep query ordering stable.
5. Prefer scalar values over multi-column output.

## How this fixture is executed

The integration test `crates/sqlite-presto-ext/tests/sql_golden.rs` runs `smoke-load.sql` via `sqlite3` with:

- `.bail on`
- `.headers off`
- `.mode list`

and compares stdout against `tests/golden/smoke-load.out`.

## Updating the fixture

When you add or change queries in `smoke-load.sql`:

1. Re-run the fixture output generation:
   - `sqlite3 :memory: ".load ./target/debug/libsqlite_presto_ext" ".headers off" ".mode list" ".read tests/sql/smoke-load.sql" > tests/golden/smoke-load.out`
2. Run tests:
   - `cargo test -p sqlite-presto-ext --tests`
3. Confirm golden diff is expected and committed with SQL changes.
