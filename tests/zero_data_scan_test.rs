use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn script_path() -> PathBuf {
    repo_root().join("tests/zero_data_scan.sh")
}

fn temp_project(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let root = std::env::temp_dir().join(format!(
        "glyph-zero-data-{}-{}-{}",
        name,
        std::process::id(),
        nonce
    ));

    fs::create_dir_all(root.join("src")).unwrap();
    root
}

fn write_file(path: &Path, contents: &str) {
    fs::write(path, contents).unwrap();
}

fn run_scan(root: &Path) -> std::process::Output {
    Command::new("bash")
        .arg(script_path())
        .arg(root)
        .output()
        .unwrap()
}

#[test]
fn clean_manifest_passes() {
    let root = temp_project("clean");
    write_file(
        &root.join("Cargo.toml"),
        r#"[package]
name = "glyph-sample"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
"#,
    );
    write_file(
        &root.join("src/lib.rs"),
        "pub fn ok() -> &'static str { \"ok\" }\n",
    );

    let output = run_scan(&root);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn forbidden_manifest_is_rejected() {
    let db_name = ["sq", "lx"].concat();
    let tracker_name = ["sen", "try"].concat();

    for (name, cargo_snippet, source_snippet) in [
        (
            db_name.as_str(),
            format!("{} = \"0.8\"\n", db_name),
            format!("use {}::query;\n", db_name),
        ),
        (
            tracker_name.as_str(),
            String::from("tracing = \"0.1\"\n"),
            format!("use {}::capture_message;\n", tracker_name),
        ),
    ] {
        let root = temp_project(name);
        write_file(
            &root.join("Cargo.toml"),
            &format!(
                r#"[package]
name = "glyph-sample-{name}"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
{cargo_snippet}"#,
                cargo_snippet = cargo_snippet,
            ),
        );
        write_file(&root.join("src/lib.rs"), &source_snippet);

        let output = run_scan(&root);
        assert!(!output.status.success());

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(name), "{}", stderr);

        let _ = fs::remove_dir_all(&root);
    }
}
