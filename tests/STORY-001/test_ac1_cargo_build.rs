//! AC#1: Cargo Project Compiles Successfully
//!
//! Tests that:
//! - Cargo.toml exists with correct metadata (name='treelint', version='0.1.0', edition='2021')
//! - All dependencies exist in dependencies.md
//! - rust-version = '1.70.0' is set
//!
//! TDD Phase: RED - These tests should FAIL until implementation is complete.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Test: Cargo.toml exists in project root
#[test]
fn test_cargo_toml_exists() {
    let cargo_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    assert!(
        cargo_path.exists(),
        "Cargo.toml must exist at project root: {}",
        cargo_path.display()
    );
}

/// Test: Cargo.toml contains correct package name
#[test]
fn test_cargo_toml_package_name_is_treelint() {
    let cargo_content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("Failed to read Cargo.toml");

    assert!(
        cargo_content.contains("name = \"treelint\""),
        "Cargo.toml must have name = \"treelint\"\n\nActual content:\n{}",
        cargo_content
    );
}

/// Test: Cargo.toml contains correct version
#[test]
fn test_cargo_toml_version_is_0_1_0() {
    let cargo_content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("Failed to read Cargo.toml");

    assert!(
        cargo_content.contains("version = \"0.1.0\""),
        "Cargo.toml must have version = \"0.1.0\"\n\nActual content:\n{}",
        cargo_content
    );
}

/// Test: Cargo.toml uses Rust edition 2021
#[test]
fn test_cargo_toml_edition_is_2021() {
    let cargo_content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("Failed to read Cargo.toml");

    assert!(
        cargo_content.contains("edition = \"2021\""),
        "Cargo.toml must have edition = \"2021\"\n\nActual content:\n{}",
        cargo_content
    );
}

/// Test: Cargo.toml sets MSRV to 1.70.0 (CFG-003)
#[test]
fn test_cargo_toml_rust_version_is_1_70_0() {
    let cargo_content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("Failed to read Cargo.toml");

    assert!(
        cargo_content.contains("rust-version = \"1.70.0\"")
            || cargo_content.contains("rust-version = \"1.70\""),
        "Cargo.toml must have rust-version = \"1.70.0\" (MSRV)\n\nActual content:\n{}",
        cargo_content
    );
}

/// Test: clap dependency is present (from dependencies.md)
#[test]
fn test_cargo_toml_has_clap_dependency() {
    let cargo_content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("Failed to read Cargo.toml");

    assert!(
        cargo_content.contains("clap"),
        "Cargo.toml must include clap dependency (per dependencies.md)\n\nActual content:\n{}",
        cargo_content
    );
}

/// Test: serde dependency is present (from dependencies.md)
#[test]
fn test_cargo_toml_has_serde_dependency() {
    let cargo_content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("Failed to read Cargo.toml");

    assert!(
        cargo_content.contains("serde"),
        "Cargo.toml must include serde dependency (per dependencies.md)\n\nActual content:\n{}",
        cargo_content
    );
}

/// Test: serde_json dependency is present (from dependencies.md)
#[test]
fn test_cargo_toml_has_serde_json_dependency() {
    let cargo_content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("Failed to read Cargo.toml");

    assert!(
        cargo_content.contains("serde_json"),
        "Cargo.toml must include serde_json dependency (per dependencies.md)\n\nActual content:\n{}",
        cargo_content
    );
}

/// Test: thiserror dependency is present (from dependencies.md)
#[test]
fn test_cargo_toml_has_thiserror_dependency() {
    let cargo_content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("Failed to read Cargo.toml");

    assert!(
        cargo_content.contains("thiserror"),
        "Cargo.toml must include thiserror dependency (per dependencies.md)\n\nActual content:\n{}",
        cargo_content
    );
}

/// Test: anyhow dependency is present (from dependencies.md)
#[test]
fn test_cargo_toml_has_anyhow_dependency() {
    let cargo_content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("Failed to read Cargo.toml");

    assert!(
        cargo_content.contains("anyhow"),
        "Cargo.toml must include anyhow dependency (per dependencies.md)\n\nActual content:\n{}",
        cargo_content
    );
}

/// Test: Release profile has LTO enabled (CFG-004)
#[test]
fn test_cargo_toml_release_profile_has_lto() {
    let cargo_content =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .expect("Failed to read Cargo.toml");

    assert!(
        cargo_content.contains("[profile.release]") && cargo_content.contains("lto"),
        "Cargo.toml must have [profile.release] with lto setting\n\nActual content:\n{}",
        cargo_content
    );
}

/// Test: cargo build succeeds (produces treelint binary)
#[test]
fn test_cargo_build_succeeds() {
    let output = Command::new("cargo")
        .args(["build"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo build");

    assert!(
        output.status.success(),
        "cargo build must succeed\n\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Test: treelint binary exists after build
#[test]
fn test_treelint_binary_exists_after_build() {
    // First ensure build completes
    let build_output = Command::new("cargo")
        .args(["build"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute cargo build");

    assert!(
        build_output.status.success(),
        "cargo build must succeed before checking binary"
    );

    // Check binary exists in target/debug/
    let binary_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
        .join(if cfg!(windows) {
            "treelint.exe"
        } else {
            "treelint"
        });

    assert!(
        binary_path.exists(),
        "treelint binary must exist at: {}",
        binary_path.display()
    );
}
