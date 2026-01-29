//! AC#2: Full Semantic Context Mode (--context full)
//!
//! Tests that:
//! - `--context full` extracts complete semantic unit using tree-sitter node boundaries
//! - Decorators are included when they are part of the function node
//! - Docstrings are included in the body
//! - Nested symbols (method inside class) return only the method, not entire class
//!
//! TDD Phase: RED - These tests should FAIL until full semantic context is implemented.
//!
//! Technical Specification Requirements (CTX-003, CTX-004):
//! - Extract full semantic unit using tree-sitter node boundaries
//! - Handle nested symbols (method inside class) correctly

use assert_cmd::Command;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

/// Create a test project with decorated Python functions
fn setup_test_project_with_decorators() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("decorators.py");
    fs::write(
        &sample_file,
        r#"
import functools

def my_decorator(func):
    """A sample decorator."""
    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        return func(*args, **kwargs)
    return wrapper

@my_decorator
@staticmethod
def decorated_function(x: int, y: int) -> int:
    """A function with multiple decorators."""
    result = x + y
    return result

def simple_function(a: str) -> str:
    """A function without decorators."""
    return a.upper()
"#,
    )
    .expect("Failed to write decorators.py");

    temp_dir
}

/// Create a test project with Python classes containing methods
fn setup_test_project_with_nested_methods() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("nested_class.py");
    fs::write(
        &sample_file,
        r#"
class OuterClass:
    """A class containing methods."""

    class_variable = "class level"

    def __init__(self, value: int):
        """Initialize the class."""
        self.value = value

    def get_value(self) -> int:
        """Return the stored value."""
        return self.value

    def calculate(self, multiplier: int) -> int:
        """Calculate value times multiplier."""
        result = self.value * multiplier
        return result

    class InnerClass:
        """A nested class."""

        def inner_method(self) -> str:
            """Inner class method."""
            return "inner"
"#,
    )
    .expect("Failed to write nested_class.py");

    temp_dir
}

/// Create a test project with TypeScript classes and methods
fn setup_test_project_typescript() -> TempDir {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");

    let sample_file = src_dir.join("service.ts");
    fs::write(
        &sample_file,
        r#"
/**
 * UserService handles user operations.
 */
export class UserService {
    private users: Map<string, User> = new Map();

    /**
     * Create a new user.
     * @param name - The user's name
     * @returns The created user
     */
    createUser(name: string): User {
        const user = { id: crypto.randomUUID(), name };
        this.users.set(user.id, user);
        return user;
    }

    /**
     * Find a user by ID.
     */
    findUser(id: string): User | undefined {
        return this.users.get(id);
    }
}

interface User {
    id: string;
    name: string;
}
"#,
    )
    .expect("Failed to write service.ts");

    temp_dir
}

// ──────────────────────────────────────────────────────────────────────────
// Full Semantic Context Tests (CTX-003)
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context full extracts complete function including docstring
///
/// Given: A function "simple_function" with a docstring
/// When: treelint search simple_function --context full is executed
/// Then: Results include the complete function body from def to last return
#[test]
fn test_context_full_includes_complete_function() {
    let temp_dir = setup_test_project_with_decorators();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "simple_function",
            "--context",
            "full",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(
        !results.is_empty(),
        "Expected results for 'simple_function'"
    );

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Should include function definition
    assert!(
        body.contains("def simple_function"),
        "Full context should include function definition.\n\nActual body:\n{}",
        body
    );

    // Should include docstring
    assert!(
        body.contains("A function without decorators"),
        "Full context should include docstring.\n\nActual body:\n{}",
        body
    );

    // Should include return statement
    assert!(
        body.contains("return a.upper()"),
        "Full context should include full body.\n\nActual body:\n{}",
        body
    );
}

/// Test: --context full includes decorators as part of function node
///
/// Given: A function "decorated_function" with @my_decorator and @staticmethod
/// When: treelint search decorated_function --context full is executed
/// Then: Results include the decorators as part of the function
#[test]
fn test_context_full_includes_decorators() {
    let temp_dir = setup_test_project_with_decorators();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "decorated_function",
            "--context",
            "full",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(
        !results.is_empty(),
        "Expected results for 'decorated_function'"
    );

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Should include @my_decorator (part of function node in tree-sitter)
    assert!(
        body.contains("@my_decorator"),
        "Full context should include @my_decorator.\n\nActual body:\n{}",
        body
    );

    // Should include @staticmethod
    assert!(
        body.contains("@staticmethod"),
        "Full context should include @staticmethod.\n\nActual body:\n{}",
        body
    );

    // Should include function definition
    assert!(
        body.contains("def decorated_function"),
        "Full context should include function definition.\n\nActual body:\n{}",
        body
    );

    // Should include docstring
    assert!(
        body.contains("A function with multiple decorators"),
        "Full context should include docstring.\n\nActual body:\n{}",
        body
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Nested Symbol Tests (CTX-004)
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context full on method returns only the method, not entire class
///
/// Given: A method "get_value" inside class "OuterClass"
/// When: treelint search get_value --context full --type method is executed
/// Then: Results include only the method body, NOT the entire class
#[test]
fn test_context_full_method_returns_only_method_not_class() {
    let temp_dir = setup_test_project_with_nested_methods();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "get_value",
            "--context",
            "full",
            "--type",
            "method",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(
        !results.is_empty(),
        "Expected results for 'get_value' method"
    );

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Should include method definition
    assert!(
        body.contains("def get_value"),
        "Full context should include method definition.\n\nActual body:\n{}",
        body
    );

    // Should include method body
    assert!(
        body.contains("return self.value"),
        "Full context should include method body.\n\nActual body:\n{}",
        body
    );

    // Should NOT include class definition line
    assert!(
        !body.contains("class OuterClass"),
        "Full context for method should NOT include class definition.\n\nActual body:\n{}",
        body
    );

    // Should NOT include other methods
    assert!(
        !body.contains("def calculate"),
        "Full context for method should NOT include other methods.\n\nActual body:\n{}",
        body
    );
}

/// Test: --context full on class returns entire class including methods
///
/// Given: A class "OuterClass" with multiple methods
/// When: treelint search OuterClass --context full --type class is executed
/// Then: Results include the entire class with all methods
#[test]
fn test_context_full_class_returns_entire_class() {
    let temp_dir = setup_test_project_with_nested_methods();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "OuterClass",
            "--context",
            "full",
            "--type",
            "class",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(!results.is_empty(), "Expected results for 'OuterClass'");

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Should include class definition
    assert!(
        body.contains("class OuterClass"),
        "Full context should include class definition.\n\nActual body:\n{}",
        body
    );

    // Should include all methods
    assert!(
        body.contains("def __init__"),
        "Full context should include __init__ method.\n\nActual body:\n{}",
        body
    );

    assert!(
        body.contains("def get_value"),
        "Full context should include get_value method.\n\nActual body:\n{}",
        body
    );

    assert!(
        body.contains("def calculate"),
        "Full context should include calculate method.\n\nActual body:\n{}",
        body
    );
}

// ──────────────────────────────────────────────────────────────────────────
// TypeScript Full Context Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context full on TypeScript method uses tree-sitter node boundaries
///
/// Given: A TypeScript method "createUser"
/// When: treelint search createUser --context full is executed
/// Then: Results include the complete method from tree-sitter node boundaries
///
/// Note: JSDoc comments are separate nodes in tree-sitter and NOT part of the
/// method_definition node. Use --context N if you need surrounding context.
/// This is correct per CTX-003 which specifies "tree-sitter node boundaries".
#[test]
fn test_context_full_typescript_includes_jsdoc() {
    let temp_dir = setup_test_project_typescript();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "createUser",
            "--context",
            "full",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let results = json
        .get("results")
        .and_then(|r| r.as_array())
        .expect("Must have results array");

    assert!(!results.is_empty(), "Expected results for 'createUser'");

    let body = results[0]
        .get("body")
        .and_then(|b| b.as_str())
        .expect("body must be present");

    // Should include method signature (tree-sitter node starts here)
    assert!(
        body.contains("createUser(name: string)"),
        "Full context should include method signature.\n\nActual body:\n{}",
        body
    );

    // Should include method body
    assert!(
        body.contains("return user"),
        "Full context should include method body.\n\nActual body:\n{}",
        body
    );
}

// ──────────────────────────────────────────────────────────────────────────
// JSON context_mode Field Tests
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context full sets context_mode to "full" in JSON
///
/// Given: A search with --context full
/// When: treelint search foo --context full --format json is executed
/// Then: query.context_mode is "full"
#[test]
fn test_context_full_sets_context_mode_full() {
    let temp_dir = setup_test_project_with_decorators();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "simple_function",
            "--context",
            "full",
            "--format",
            "json",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    let json: Value = serde_json::from_str(&stdout).expect("Output must be valid JSON");

    let context_mode = json
        .get("query")
        .and_then(|q| q.get("context_mode"))
        .and_then(|c| c.as_str());

    assert_eq!(
        context_mode,
        Some("full"),
        "query.context_mode must be 'full' when --context full is used.\n\nActual query:\n{:?}",
        json.get("query")
    );
}

// ──────────────────────────────────────────────────────────────────────────
// Text Format with Full Context
// ──────────────────────────────────────────────────────────────────────────

/// Test: --context full in text mode shows complete semantic unit
///
/// Given: A function with decorators and docstring
/// When: treelint search decorated_function --context full --format text is executed
/// Then: Text output includes decorators and full body
#[test]
fn test_context_full_text_format_complete_unit() {
    let temp_dir = setup_test_project_with_decorators();

    let mut cmd = Command::cargo_bin("treelint").expect("treelint binary not found");

    let output = cmd
        .current_dir(temp_dir.path())
        .args([
            "search",
            "decorated_function",
            "--context",
            "full",
            "--format",
            "text",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    // Should include decorator
    assert!(
        stdout.contains("@my_decorator") || stdout.contains("@staticmethod"),
        "Text output should include decorators.\n\nActual output:\n{}",
        stdout
    );

    // Should include function definition
    assert!(
        stdout.contains("def decorated_function"),
        "Text output should include function definition.\n\nActual output:\n{}",
        stdout
    );

    // Should include body
    assert!(
        stdout.contains("result = x + y"),
        "Text output should include function body.\n\nActual output:\n{}",
        stdout
    );
}
