# Tasks: SQL Golden Integration Coverage for Implemented `p_*` Functions

> Generated from PLAN.md by structured-dev skill on 2026-03-25

## Progress
- Total: 8 tasks
- Completed: 8
- Remaining: 0

## Tasks

### Task 1: Add SQL fixture runner integration test
- **Status:** ☑ Done
- **Files:** `crates/sqlite-presto-ext/tests/sql_golden.rs` (new)
- **What:** Create a dedicated integration test that loads the extension, executes `tests/sql/smoke-load.sql` through `sqlite3`, captures stdout/stderr, and compares stdout to `tests/golden/smoke-load.out`.
- **Why:** Makes SQL fixture coverage enforceable via `cargo test`.
- **Acceptance criteria:**
  - [x] Test builds extension and runs SQL fixture from repository paths.
  - [x] Test fails with clear diagnostics when SQL execution fails.
  - [x] Test compares actual output against golden file and reports mismatch context.

### Task 2: Stabilize SQL runner output conventions
- **Status:** ☑ Done
- **Files:** `crates/sqlite-presto-ext/tests/sql_golden.rs` (new), `tests/sql/smoke-load.sql`
- **What:** Ensure deterministic output conventions (`.headers off`, `.mode list`, deterministic ordering/labels) and reusable helper code in the runner.
- **Why:** Prevent flaky goldens and improve maintainability.
- **Acceptance criteria:**
  - [x] Runner enforces deterministic sqlite3 output mode.
  - [x] SQL fixture output format is deterministic and diff-friendly.
  - [x] No nondeterministic ordering in fixture assertions.
- **Depends on:** Task 1

### Task 3: Expand `smoke-load.sql` happy-path coverage across implemented signatures
- **Status:** ☑ Done
- **Files:** `tests/sql/smoke-load.sql`
- **What:** Add broad query coverage in one large file for implemented function families (numeric/bitwise, text/regex, binary/hash, URL/IP/UUID), including overloads where available.
- **Why:** Verifies actual SQL-call behavior beyond null-only dynamic smoke calls.
- **Acceptance criteria:**
  - [x] All implemented function families are represented in the fixture.
  - [x] Overloads receive at least one explicit happy-path query where applicable.
  - [x] Fixture remains organized with clear section headers and labels.
- **Depends on:** Task 2

### Task 4: Add explicit edge-case assertions in SQL fixture
- **Status:** ☑ Done
- **Files:** `tests/sql/smoke-load.sql`, `crates/sqlite-presto-ext/tests/sql_golden.rs`
- **What:** Add edge cases for null propagation, boundary values, malformed inputs, and deterministic error-path checks where practical.
- **Why:** Catches semantic regressions and error-handling drift.
- **Acceptance criteria:**
  - [x] Null-propagation checks exist for representative functions in each family.
  - [x] Boundary/invalid-input checks exist for key numeric/text/net/binary methods.
  - [x] Error-path assertions are stable and deterministic (validated in `representative_error_paths_are_stable`).
- **Depends on:** Task 3

### Task 5: Regenerate and validate golden output
- **Status:** ☑ Done
- **Files:** `tests/golden/smoke-load.out`
- **What:** Generate expected output from the expanded SQL fixture and store as canonical golden output.
- **Why:** Establishes regression baseline for CI/integration runs.
- **Acceptance criteria:**
  - [x] Golden output fully matches the expanded SQL fixture output.
  - [x] Golden file is deterministic across reruns on same environment.
  - [x] No ad-hoc/manual output formatting outside agreed conventions.
- **Depends on:** Task 4

### Task 6: Align existing `load_extension.rs` dynamic smoke with fixture approach
- **Status:** ☑ Done
- **Files:** `crates/sqlite-presto-ext/tests/load_extension.rs`
- **What:** Keep and refine the dynamic signature smoke test (all implemented signatures) so it complements SQL golden tests without overlap confusion.
- **Why:** Maintains full-surface method execution while fixture test focuses on semantic assertions.
- **Acceptance criteria:**
  - [x] Dynamic test still covers all implemented signatures and skips known stubs.
  - [x] Test naming/comments clearly distinguish dynamic smoke vs SQL golden.
  - [x] No brittle assumptions introduced by fixture expansion.
- **Depends on:** Task 1

### Task 7: Update SQL fixture documentation
- **Status:** ☑ Done
- **Files:** `tests/sql/README.md`, `tests/golden/README.md`
- **What:** Document how to run/update SQL golden tests, expected file conventions, and how new functions/edge cases should be added.
- **Why:** Keeps future contributions consistent and lowers maintenance burden.
- **Acceptance criteria:**
  - [x] README files explain runner behavior and update workflow.
  - [x] Conventions for labels/order/determinism are documented.
  - [x] Guidance covers adding both happy-path and edge-case rows.
- **Depends on:** Tasks 1, 5

### Task 8: Final verification pass
- **Status:** ☑ Done
- **Files:** `TASKS.md` (verification section)
- **What:** Run full verification (`cargo test -p sqlite-presto-ext --tests`, `cargo test`, `cargo check`, formatting check) and record results.
- **Why:** Confirms the new integration layer is stable and repository-wide checks still pass.
- **Acceptance criteria:**
  - [x] Targeted integration tests pass.
  - [x] Full workspace tests pass (or unrelated pre-existing failures are clearly reported).
  - [x] Verification outcomes are recorded in `TASKS.md`.
- **Depends on:** Tasks 5, 6, 7

## Verification
- **Tests:** pass
  - `cargo test -p sqlite-presto-ext --tests`
  - `cargo test`
- **Lint/format:** fail (`cargo fmt --all -- --check`) due **pre-existing** formatting issues in `crates/presto-text/src/lib.rs` unrelated to this change set.
- **Type check:** pass (`cargo check`)
- **Smoke test:** pass
  - Dynamic signature smoke: `smoke_test_all_implemented_sql_signatures`
  - SQL golden fixture smoke: `smoke_sql_fixture_matches_golden_output`
- **Summary:** Added a first-class SQL fixture/golden integration test harness, expanded `smoke-load.sql` to cover all currently implemented SQL signatures plus explicit family-level edge checks, regenerated `smoke-load.out`, added stable error-path integration checks, and documented fixture/golden maintenance workflow.