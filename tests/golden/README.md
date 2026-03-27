# Golden Outputs

This directory stores expected stdout snapshots for SQL fixture integration tests.

## Current golden

- `smoke-load.out` — expected output for `tests/sql/smoke-load.sql`.

## Purpose

Goldens provide regression protection for SQL-visible behavior:

- function availability/smoke calls
- stable semantic outputs
- edge-case handling behavior encoded in fixture rows

## Update workflow

If `tests/sql/smoke-load.sql` changes intentionally:

1. Regenerate the golden file:
   - `sqlite3 :memory: ".load ./target/debug/libsqlite_presto_ext" ".headers off" ".mode list" ".read tests/sql/smoke-load.sql" > tests/golden/smoke-load.out`
2. Run:
   - `cargo test -p sqlite-presto-ext --tests`
3. Review the golden diff carefully before committing.

## Notes

- Keep output deterministic and line-oriented.
- Preserve label prefixes (`SECTION|`, `SMOKE|`, `EDGE|`) for readable diffs.
- Error-path assertions are validated by dedicated Rust integration checks (`sql_golden.rs`) rather than by golden stdout snapshot.
