//! AC#4: Symbol Type Filtering Tests
//!
//! Given: User wants to see only specific symbol types
//! When: User runs `treelint map --type function`
//! Then:
//!   - Output includes only symbols of the specified type
//!   - Total counts reflect filtered results
//!
//! Source files tested:
//!   - src/cli/commands/map.rs (Type filtering)
//! Coverage threshold: 95%
//!
//! Requirements:
//!   - MAP-005: Filter symbols by type

use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Test: `treelint map --type function` filters to only functions
/// Requirement: MAP-005 - Filter symbols by type
#[test]
fn test_map_type_function_filters_functions_only() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def my_function():
    pass

class MyClass:
    def method(self):
        pass

CONSTANT = 42
"#,
    )
    .unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--type", "function"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    // Should not contain class
    assert!(
        !stdout.contains("MyClass"),
        "Filtered output should not contain class.\n\nActual output:\n{}",
        stdout
    );

    // Should contain function
    assert!(
        stdout.contains("my_function"),
        "Filtered output should contain function.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: `treelint map --type class` filters to only classes
/// Requirement: MAP-005 - Filter symbols by type
#[test]
fn test_map_type_class_filters_classes_only() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def standalone_function():
    pass

class FirstClass:
    pass

class SecondClass:
    pass
"#,
    )
    .unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--type", "class"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Should contain classes
    assert!(
        stdout.contains("FirstClass") && stdout.contains("SecondClass"),
        "Filtered output should contain classes.\n\nActual output:\n{}",
        stdout
    );

    // Should not contain standalone function
    assert!(
        !stdout.contains("standalone_function"),
        "Filtered output should not contain function.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: `treelint map --type method` filters to only methods
/// Requirement: MAP-005 - Filter symbols by type
#[test]
fn test_map_type_method_filters_methods_only() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def standalone():
    pass

class MyClass:
    def method_one(self):
        pass

    def method_two(self):
        pass
"#,
    )
    .unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--type", "method"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Should contain methods
    assert!(
        stdout.contains("method_one") || stdout.contains("method_two"),
        "Filtered output should contain methods.\n\nActual output:\n{}",
        stdout
    );

    // Should not contain standalone function
    assert!(
        !stdout.contains("standalone"),
        "Filtered output should not contain standalone function.\n\nActual output:\n{}",
        stdout
    );
}

/// Test: Filtered total_symbols count reflects filter
/// Requirement: MAP-005 - Total counts reflect filtered results
#[test]
fn test_map_type_filter_updates_total_count() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("example.py"),
        r#"
def func1(): pass
def func2(): pass
class Class1: pass
class Class2: pass
class Class3: pass
"#,
    )
    .unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act: Get unfiltered count
    let unfiltered = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json"])
        .assert()
        .success();

    let unfiltered_stdout = String::from_utf8_lossy(&unfiltered.get_output().stdout);
    let unfiltered_json: Value = serde_json::from_str(&unfiltered_stdout).expect("Invalid JSON");

    // Act: Get filtered count (functions only)
    let filtered = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--type", "function"])
        .assert()
        .success();

    let filtered_stdout = String::from_utf8_lossy(&filtered.get_output().stdout);
    let filtered_json: Value = serde_json::from_str(&filtered_stdout).expect("Invalid JSON");

    // Assert: Filtered count should be less than unfiltered
    let unfiltered_count = unfiltered_json["total_symbols"].as_i64().unwrap_or(0);
    let filtered_count = filtered_json["total_symbols"].as_i64().unwrap_or(0);

    assert!(
        filtered_count < unfiltered_count,
        "Filtered count ({}) should be less than unfiltered count ({}).",
        filtered_count,
        unfiltered_count
    );

    // Filtered count should be 2 (two functions)
    assert_eq!(filtered_count, 2, "Should have exactly 2 functions.");
}

/// Test: --type flag accepts all valid symbol types
/// Requirement: MAP-005 - function, class, method, variable, constant, import, export
#[test]
fn test_map_type_flag_accepts_valid_types() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("example.py"), "def foo(): pass").unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    let valid_types = [
        "function", "class", "method", "variable", "constant", "import",
    ];

    for symbol_type in valid_types {
        // Act & Assert: Each type should be accepted (not error)
        Command::cargo_bin("treelint")
            .unwrap()
            .current_dir(temp_dir.path())
            .args(["map", "--type", symbol_type])
            .assert()
            .success();
    }
}

/// Test: --type flag is recognized
/// Requirement: MAP-005 - Type filter flag
#[test]
fn test_map_type_flag_recognized() {
    // Act & Assert
    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");
    cmd.args(["map", "--help"])
        .assert()
        .stdout(predicates::str::contains("type"));
}

/// Test: Invalid --type value produces error
/// Requirement: MAP-005 - Validation of type parameter
#[test]
fn test_map_type_invalid_produces_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Act
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--type", "invalid_type_xyz"])
        .assert();

    // Assert: Should fail with non-zero exit code
    output.failure();
}

/// Test: Multiple files filtered correctly
/// Requirement: MAP-005 - Filter works across files
#[test]
fn test_map_type_filter_across_multiple_files() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(
        temp_dir.path().join("funcs.py"),
        "def f1(): pass\ndef f2(): pass",
    )
    .unwrap();
    fs::write(
        temp_dir.path().join("classes.py"),
        "class C1: pass\nclass C2: pass",
    )
    .unwrap();

    // Build index
    Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["index"])
        .assert()
        .success();

    // Act: Filter for functions
    let output = Command::cargo_bin("treelint")
        .unwrap()
        .current_dir(temp_dir.path())
        .args(["map", "--format", "json", "--type", "function"])
        .assert()
        .success();

    // Assert
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Invalid JSON");

    // total_symbols should be 2 (functions only)
    assert_eq!(
        json["total_symbols"].as_i64(),
        Some(2),
        "Should have 2 functions total.\n\nJSON:\n{}",
        serde_json::to_string_pretty(&json).unwrap()
    );
}
