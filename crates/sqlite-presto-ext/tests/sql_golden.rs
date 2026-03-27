use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::OnceLock;

fn extension_path() -> &'static PathBuf {
    static EXT_PATH: OnceLock<PathBuf> = OnceLock::new();
    EXT_PATH.get_or_init(|| {
        let status = Command::new("cargo")
            .args(["build", "-p", "sqlite-presto-ext"])
            .status()
            .expect("failed to run cargo build for extension");
        assert!(status.success(), "cargo build failed for sqlite-presto-ext");

        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/debug/libsqlite_presto_ext.dylib")
            .canonicalize()
            .expect("extension dylib should exist after build")
    })
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root should be resolvable")
}

fn normalize_text(value: &str) -> String {
    value.replace("\r\n", "\n")
}

fn run_sql_script(extension: &Path, script_path: &Path) -> Output {
    let mut child = Command::new("sqlite3")
        .arg(":memory:")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn sqlite3");

    let control = format!(
        ".load {}\n.bail on\n.headers off\n.mode list\n.read {}\n",
        extension.display(),
        script_path.display()
    );

    {
        use std::io::Write;
        let stdin = child
            .stdin
            .as_mut()
            .expect("sqlite3 child stdin should be available");
        stdin
            .write_all(control.as_bytes())
            .expect("failed to write sqlite3 control script");
    }

    child
        .wait_with_output()
        .expect("failed to wait for sqlite3 process")
}

fn run_single_query(extension: &Path, query: &str) -> Output {
    let mut child = Command::new("sqlite3")
        .arg(":memory:")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn sqlite3");

    let control = format!(
        ".load {}\n.bail on\n.headers off\n.mode list\n{}\n",
        extension.display(),
        query
    );

    {
        use std::io::Write;
        let stdin = child
            .stdin
            .as_mut()
            .expect("sqlite3 child stdin should be available");
        stdin
            .write_all(control.as_bytes())
            .expect("failed to write sqlite3 query");
    }

    child
        .wait_with_output()
        .expect("failed to wait for sqlite3 process")
}

fn first_diff(expected: &str, actual: &str) -> String {
    let expected_lines = expected.lines().collect::<Vec<_>>();
    let actual_lines = actual.lines().collect::<Vec<_>>();
    let max = expected_lines.len().max(actual_lines.len());

    for idx in 0..max {
        let e = expected_lines.get(idx).copied().unwrap_or("<missing>");
        let a = actual_lines.get(idx).copied().unwrap_or("<missing>");
        if e != a {
            return format!(
                "line {} mismatch\nexpected: {}\nactual:   {}",
                idx + 1,
                e,
                a
            );
        }
    }

    "no line-level diff found".to_string()
}

#[test]
fn smoke_sql_fixture_matches_golden_output() {
    let repo = repo_root();
    let sql_path = repo.join("tests/sql/smoke-load.sql");
    let golden_path = repo.join("tests/golden/smoke-load.out");

    let output = run_sql_script(extension_path(), &sql_path);
    assert!(
        output.status.success(),
        "sqlite3 failed running fixture {}: {}",
        sql_path.display(),
        String::from_utf8_lossy(&output.stderr)
    );

    let actual = normalize_text(&String::from_utf8_lossy(&output.stdout));
    let expected = normalize_text(
        &std::fs::read_to_string(&golden_path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", golden_path.display())),
    );

    assert!(
        actual == expected,
        "golden mismatch for {}\n{}",
        golden_path.display(),
        first_diff(&expected, &actual)
    );
}

#[test]
fn representative_error_paths_are_stable() {
    let cases = [
        (
            "mod_by_zero",
            "select p_mod(1, 0);",
            "domain error: mod by zero",
        ),
        (
            "factorial_negative",
            "select p_factorial(-1);",
            "domain error: factorial input must be >= 0",
        ),
        (
            "from_base_invalid",
            "select p_from_base('zz', 2);",
            "invalid argument: invalid value for radix",
        ),
        (
            "chr_invalid",
            "select p_chr(-1);",
            "invalid argument: invalid codepoint",
        ),
        (
            "url_invalid",
            "select p_url_extract_host('not-a-url');",
            "invalid argument: invalid URL",
        ),
        (
            "ip_invalid",
            "select p_ip_prefix('300.1.1.1', 24);",
            "invalid argument: invalid IP address",
        ),
        (
            "from_hex_invalid",
            "select p_from_hex('xyz');",
            "invalid argument: invalid hex input",
        ),
        (
            "from_utf8_invalid",
            "select p_from_utf8(x'ff');",
            "invalid argument: invalid UTF-8 input",
        ),
        (
            "hamming_mismatch",
            "select p_hamming_distance('ab', 'a');",
            "invalid argument: hamming_distance requires equal-length strings",
        ),
        (
            "regex_invalid",
            "select p_regexp_like('abc', '(');",
            "invalid argument: invalid regex pattern",
        ),
    ];

    for (label, query, expected_error) in cases {
        let output = run_single_query(extension_path(), query);
        assert!(
            !output.status.success(),
            "expected query to fail for case `{label}`, but it succeeded"
        );

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(expected_error),
            "unexpected stderr for case `{label}`\nquery: {query}\nexpected to contain: {expected_error}\nactual stderr: {stderr}"
        );
    }
}
