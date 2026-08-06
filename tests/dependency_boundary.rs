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
        "schema-rust.git?rev=664335240a40728826cfaa09e3100cd867031912#664335240a40728826cfaa09e3100cd867031912"
    ));
    assert_eq!(lockfile.matches("name = \"signal-persona\"").count(), 1);
    assert!(lockfile.contains(
        "signal-persona.git?rev=2802259fb1344495b1ad3b701fe81e0b7f9df9c3#2802259fb1344495b1ad3b701fe81e0b7f9df9c3"
    ));
    assert_eq!(lockfile.matches("name = \"signal-frame\"").count(), 1);
    assert!(lockfile.contains(
        "signal-frame.git?rev=8aa0bcaeb29fe9e461a11706a469638d2fd109ac#8aa0bcaeb29fe9e461a11706a469638d2fd109ac"
    ));
    assert_eq!(lockfile.matches("name = \"dotos\"").count(), 1);
    assert!(lockfile.contains(
        "dotos.git?rev=80c7b17f7ad3cf547d2624c6a243e5de5f85c9f3#80c7b17f7ad3cf547d2624c6a243e5de5f85c9f3"
    ));

    let output = Command::new("cargo")
        .args(["tree", "--edges", "build", "--no-default-features"])
        .output()
        .expect("run cargo tree");
    assert!(output.status.success());
    let tree = String::from_utf8(output.stdout).expect("dependency tree");
    assert!(tree.contains("schema-rust.git?rev=664335240a40728826cfaa09e3100cd867031912#66433524"));
    assert!(
        tree.contains("signal-persona.git?rev=2802259fb1344495b1ad3b701fe81e0b7f9df9c3#2802259f")
    );
}

#[test]
fn dependency_graph_has_no_branch_family_pin() {
    for source in [include_str!("../Cargo.toml"), include_str!("../Cargo.lock")] {
        for forbidden in ["branch =", "?branch="] {
            assert!(
                !source.contains(forbidden),
                "moving dependency pin {forbidden} survived"
            );
        }
    }
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
