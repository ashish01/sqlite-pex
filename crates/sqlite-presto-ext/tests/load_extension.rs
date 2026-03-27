use std::collections::HashSet;
use std::path::PathBuf;
use std::process::{Command, Output};

// This file keeps lightweight extension-load + dynamic signature smoke coverage.
// Detailed value/edge-case regression assertions live in `sql_golden.rs`.

fn extension_path() -> PathBuf {
    let status = Command::new("cargo")
        .args(["build", "-p", "sqlite-presto-ext"])
        .status()
        .expect("failed to run cargo build for extension");
    assert!(status.success(), "cargo build failed for sqlite-presto-ext");

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .join("../../target/debug/libsqlite_presto_ext.dylib")
        .canonicalize()
        .expect("extension dylib should exist after build")
}

fn sqlite_query(extension: &PathBuf, sql: &str) -> Output {
    Command::new("sqlite3")
        .arg(":memory:")
        .arg(format!(".load {}", extension.display()))
        .arg(sql)
        .output()
        .unwrap_or_else(|err| panic!("failed to execute sqlite3 for query `{sql}`: {err}"))
}

fn discover_presto_signatures(extension: &PathBuf) -> Vec<(String, i32)> {
    let output = sqlite_query(
        extension,
        "select name || '|' || narg from pragma_function_list where name glob 'p_*' order by name, narg;",
    );

    assert!(
        output.status.success(),
        "failed to discover function signatures: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .map(|line| {
            let (name, narg) = line
                .split_once('|')
                .unwrap_or_else(|| panic!("invalid pragma output line: `{line}`"));
            let narg = narg
                .parse::<i32>()
                .unwrap_or_else(|err| panic!("invalid narg `{narg}` in line `{line}`: {err}"));
            (name.to_string(), narg)
        })
        .collect()
}

fn smoke_sql_for_signature(name: &str, narg: i32) -> String {
    let args = match narg {
        0 => String::new(),
        -1 => "NULL".to_string(),
        n if n > 0 => std::iter::repeat_n("NULL", n as usize)
            .collect::<Vec<_>>()
            .join(", "),
        _ => panic!("unexpected narg for `{name}`: {narg}"),
    };

    if args.is_empty() {
        format!("select typeof({name}());")
    } else {
        format!("select typeof({name}({args}));")
    }
}

#[test]
fn sqlite_can_load_extension() {
    let extension = extension_path();
    let output = sqlite_query(&extension, "select p_lower('HeLLo');");

    assert!(
        output.status.success(),
        "sqlite3 failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hello");
}

#[test]
fn smoke_test_all_implemented_sql_signatures() {
    let extension = extension_path();
    let signatures = discover_presto_signatures(&extension);

    let stubbed: HashSet<&str> = HashSet::from([
        "p_cosine_similarity",
        "p_dot_product",
        "p_l2_squared",
        "p_cume_dist",
        "p_dense_rank",
        "p_first_value",
        "p_lag",
        "p_last_value",
        "p_lead",
        "p_nth_value",
        "p_ntile",
        "p_percent_rank",
        "p_rank",
        "p_row_number",
    ]);

    let implemented = signatures
        .into_iter()
        .filter(|(name, _)| !stubbed.contains(name.as_str()))
        .collect::<Vec<_>>();

    assert!(
        !implemented.is_empty(),
        "no implemented p_* signatures discovered"
    );

    for (name, narg) in implemented {
        let sql = smoke_sql_for_signature(&name, narg);
        let output = sqlite_query(&extension, &sql);

        assert!(
            output.status.success(),
            "smoke query failed for {name}/{narg} with SQL `{sql}`: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let result_type = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if narg == 0 {
            assert!(
                matches!(
                    result_type.as_str(),
                    "null" | "integer" | "real" | "text" | "blob"
                ),
                "unexpected result type for {name}/{narg}: `{result_type}`"
            );
        } else {
            assert_eq!(
                result_type, "null",
                "expected null propagation for {name}/{narg}, got `{result_type}`"
            );
        }
    }
}
