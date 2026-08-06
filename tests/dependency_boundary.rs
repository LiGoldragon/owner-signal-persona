use std::process::Command;

#[test]
fn runtime_tree_excludes_bootstrap_and_retired_crates() {
    let output = Command::new("cargo")
        .args(["tree", "--edges", "normal", "--no-default-features"])
        .output()
        .expect("run cargo tree");
    assert!(output.status.success());
    let tree = String::from_utf8(output.stdout).expect("dependency tree");
    for forbidden in [
        "core-ethos",
        "name-table",
        "schema-rust",
        "rust-logos",
        "sema-translator",
        "signal-sema-translator",
        "structural-codec",
    ] {
        assert!(
            !tree.contains(forbidden),
            "runtime contains {forbidden}:\n{tree}"
        );
    }
}

#[test]
fn build_tree_has_one_corrected_schema_rust_and_published_producer() {
    let lockfile = include_str!("../Cargo.lock");
    assert_eq!(lockfile.matches("name = \"schema-rust\"").count(), 1);
    assert!(lockfile.contains(
        "schema-rust.git?rev=9e36587c85bd69357e9042729ba2df0052799756#9e36587c85bd69357e9042729ba2df0052799756"
    ));
    assert_eq!(lockfile.matches("name = \"signal-persona\"").count(), 1);
    assert!(lockfile.contains(
        "signal-persona.git?rev=7d2568d420869aa0cded49c3c04cc0ac180e66a2#7d2568d420869aa0cded49c3c04cc0ac180e66a2"
    ));

    let output = Command::new("cargo")
        .args(["tree", "--edges", "build", "--no-default-features"])
        .output()
        .expect("run cargo tree");
    assert!(output.status.success());
    let tree = String::from_utf8(output.stdout).expect("dependency tree");
    assert!(tree.contains("schema-rust.git?rev=9e36587c85bd69357e9042729ba2df0052799756#9e36587c"));
    assert!(
        tree.contains("signal-persona.git?rev=7d2568d420869aa0cded49c3c04cc0ac180e66a2#7d2568d4")
    );
}

#[test]
fn historical_schema_surface_is_absent() {
    for source in [
        include_str!("../Cargo.toml"),
        include_str!("../build.rs"),
        include_str!("../src/lib.rs"),
    ] {
        for forbidden in [".schema", "schema-dir", "CargoSchemaMetadata"] {
            assert!(
                !source.contains(forbidden),
                "historical token {forbidden} survived"
            );
        }
    }
}
