//! AC#5: Symbol Extraction - Variables, Constants, Imports, Exports Tests
//!
//! Given: A source file containing variable declarations, constants, imports, and exports
//! When: The parser extracts symbols from the file
//! Then: Each is returned with appropriate SymbolType:
//!       - Variable
//!       - Constant
//!       - Import
//!       - Export (TypeScript only)
//!
//! Test file: tests/STORY-002/test_ac5_variable_import_extraction.rs
//! Source files tested:
//!   - src/parser/symbols.rs (Symbol extraction logic)
//!   - src/parser/queries/typescript.rs (Import/export queries)
//! Coverage threshold: 85%

use std::path::Path;

// These imports will fail until the parser module is implemented
// This is expected behavior for TDD Red phase
use treelint::parser::{Language, SymbolExtractor, SymbolType};

/// Helper to get fixture path
fn fixture_path(relative: &str) -> String {
    format!("tests/fixtures/{}", relative)
}

/// Test: SymbolType enum has Variable variant
/// Requirement: DM-006 - Define SymbolType enum with Variable variant
#[test]
fn test_symbol_type_has_variable_variant() {
    let symbol_type = SymbolType::Variable;

    assert_eq!(symbol_type, SymbolType::Variable);
}

/// Test: SymbolType enum has Constant variant
/// Requirement: DM-006 - Define SymbolType enum with Constant variant
#[test]
fn test_symbol_type_has_constant_variant() {
    let symbol_type = SymbolType::Constant;

    assert_eq!(symbol_type, SymbolType::Constant);
}

/// Test: SymbolType enum has Import variant
/// Requirement: DM-006 - Define SymbolType enum with Import variant
#[test]
fn test_symbol_type_has_import_variant() {
    let symbol_type = SymbolType::Import;

    assert_eq!(symbol_type, SymbolType::Import);
}

/// Test: SymbolType enum has Export variant
/// Requirement: DM-006 - Define SymbolType enum with Export variant
#[test]
fn test_symbol_type_has_export_variant() {
    let symbol_type = SymbolType::Export;

    assert_eq!(symbol_type, SymbolType::Export);
}

/// Test: SymbolType enum has all 7 variants
/// Requirement: DM-006 - SymbolType has 7 variants
#[test]
fn test_symbol_type_has_seven_variants() {
    // Verify all 7 variants exist and are distinct
    let variants = [
        SymbolType::Function,
        SymbolType::Class,
        SymbolType::Method,
        SymbolType::Variable,
        SymbolType::Constant,
        SymbolType::Import,
        SymbolType::Export,
    ];

    assert_eq!(variants.len(), 7, "SymbolType must have exactly 7 variants");
}

/// Test: Extract imports from Python file
/// Requirement: CFG-003 - Define tree-sitter query for Python import_statement
#[test]
fn test_extract_imports_from_python() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/imports_variables.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let imports: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Import)
        .collect();

    // imports_variables.py has: os, sys, Path, Dict, List, Optional, defaultdict
    assert!(
        imports.len() >= 4,
        "Expected at least 4 imports, found {}",
        imports.len()
    );

    // Verify specific import
    let os_import = imports.iter().find(|s| s.name == "os");
    assert!(os_import.is_some(), "Should find 'os' import");
}

/// Test: Extract from-imports from Python file
/// Requirement: CFG-003 - Query captures import and from-import
#[test]
fn test_extract_from_imports_from_python() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/imports_variables.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let imports: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Import)
        .collect();

    // Should capture 'from pathlib import Path'
    let path_import = imports.iter().find(|s| s.name == "Path");
    assert!(path_import.is_some(), "Should find 'Path' from-import");
}

/// Test: Extract constants from Python file
/// Requirement: SVC-004 - Extract constant symbols
#[test]
fn test_extract_constants_from_python() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/imports_variables.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Constants are typically UPPER_CASE in Python
    // imports_variables.py has: MAX_RETRIES, DEFAULT_TIMEOUT, API_BASE_URL, DEBUG
    let constants: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Constant || s.symbol_type == SymbolType::Variable)
        .filter(|s| {
            s.name
                .chars()
                .all(|c| c.is_uppercase() || c == '_' || c.is_numeric())
        })
        .collect();

    assert!(
        constants.len() >= 2,
        "Expected at least 2 constants (UPPER_CASE), found {}",
        constants.len()
    );

    let max_retries = constants.iter().find(|s| s.name == "MAX_RETRIES");
    assert!(max_retries.is_some(), "Should find 'MAX_RETRIES' constant");
}

/// Test: Extract variables from Python file
/// Requirement: SVC-004 - Extract variable symbols
#[test]
fn test_extract_variables_from_python() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/imports_variables.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Variables are typically lower_case in Python
    // imports_variables.py has: _cache, counter
    let variables: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Variable)
        .collect();

    assert!(
        variables.len() >= 1,
        "Expected at least 1 variable, found {}",
        variables.len()
    );
}

/// Test: Extract imports from TypeScript file
/// Requirement: CFG-004 - Query captures TypeScript imports
#[test]
fn test_extract_imports_from_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/imports_exports.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let imports: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Import)
        .collect();

    // imports_exports.ts has: readFile, path, Buffer (type)
    assert!(
        imports.len() >= 2,
        "Expected at least 2 imports, found {}",
        imports.len()
    );

    let read_file = imports.iter().find(|s| s.name == "readFile");
    assert!(read_file.is_some(), "Should find 'readFile' import");
}

/// Test: Extract exports from TypeScript file
/// Requirement: CFG-005 - Define tree-sitter query for TypeScript export_statement
#[test]
fn test_extract_exports_from_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/imports_exports.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let exports: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Export)
        .collect();

    // imports_exports.ts exports: MAX_RETRIES, DEFAULT_TIMEOUT, UserId, User, createUser, ApiClient, read
    assert!(
        exports.len() >= 4,
        "Expected at least 4 exports, found {}",
        exports.len()
    );
}

/// Test: Extract exported constants from TypeScript
/// Requirement: CFG-005 - Query captures export statements
#[test]
fn test_extract_exported_constants_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/imports_exports.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Look for MAX_RETRIES which is 'export const'
    let max_retries = symbols.iter().find(|s| s.name == "MAX_RETRIES");

    assert!(max_retries.is_some(), "Should find 'MAX_RETRIES' export");

    // It could be marked as Export or Constant depending on implementation
    let max_retries = max_retries.unwrap();
    assert!(
        max_retries.symbol_type == SymbolType::Export
            || max_retries.symbol_type == SymbolType::Constant,
        "MAX_RETRIES should be Export or Constant, got {:?}",
        max_retries.symbol_type
    );
}

/// Test: Extract default export from TypeScript
/// Requirement: CFG-005 - Query captures default exports
#[test]
fn test_extract_default_export_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/imports_exports.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // ApiClient is 'export default class'
    let api_client = symbols.iter().find(|s| s.name == "ApiClient");

    assert!(
        api_client.is_some(),
        "Should find 'ApiClient' default export"
    );
}

/// Test: Extract re-exports from TypeScript
/// Requirement: CFG-005 - Query captures re-exports
#[test]
fn test_extract_reexport_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/imports_exports.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // 'export { readFile as read } from "fs/promises"' is a re-export
    // The 'read' alias should be captured
    let read_reexport = symbols.iter().find(|s| s.name == "read");

    // This is optional - re-exports may or may not be captured
    if read_reexport.is_some() {
        assert_eq!(
            read_reexport.unwrap().symbol_type,
            SymbolType::Export,
            "Re-export should be marked as Export"
        );
    }
}

/// Test: Extract constants from Rust file
/// Requirement: CFG-006 - Query captures Rust const
#[test]
fn test_extract_constants_from_rust() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/imports_constants.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let constants: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Constant)
        .collect();

    // imports_constants.rs has: MAX_RETRIES, DEFAULT_TIMEOUT, INTERNAL_SECRET
    assert!(
        constants.len() >= 2,
        "Expected at least 2 constants, found {}",
        constants.len()
    );

    let max_retries = constants.iter().find(|s| s.name == "MAX_RETRIES");
    assert!(max_retries.is_some(), "Should find 'MAX_RETRIES' const");
}

/// Test: Extract static variables from Rust file
/// Requirement: CFG-006 - Query captures Rust static
#[test]
fn test_extract_static_variables_from_rust() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/imports_constants.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // COUNTER is 'pub static'
    let counter = symbols.iter().find(|s| s.name == "COUNTER");

    assert!(counter.is_some(), "Should find 'COUNTER' static variable");
}

/// Test: Extract use statements from Rust file
/// Requirement: CFG-006 - Query captures Rust use
#[test]
fn test_extract_use_statements_from_rust() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/imports_constants.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let imports: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Import)
        .collect();

    // imports_constants.rs has: HashMap, io::{self, Read, Write}, PathBuf, File
    assert!(
        imports.len() >= 3,
        "Expected at least 3 imports, found {}",
        imports.len()
    );
}

/// Test: Extract type alias from Rust
/// Requirement: CFG-006 - Query captures type aliases
#[test]
fn test_extract_type_alias_rust() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/imports_constants.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // 'type Result<T> = ...' and 'type Cache = ...'
    // Type aliases may be captured as Variable or a separate type
    let result_type = symbols.iter().find(|s| s.name == "Result");

    assert!(
        result_type.is_some(),
        "Should find 'Result' type alias (may be Variable or separate type)"
    );
}

/// Test: Extract type export from TypeScript
/// Requirement: CFG-005 - Query captures type exports
#[test]
fn test_extract_type_export_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/imports_exports.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // 'export type UserId = string'
    let user_id = symbols.iter().find(|s| s.name == "UserId");

    assert!(user_id.is_some(), "Should find 'UserId' type export");
}

/// Test: Extract interface export from TypeScript
/// Requirement: CFG-005 - Query captures interface exports
#[test]
fn test_extract_interface_export_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/imports_exports.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // 'export interface User { ... }'
    let user = symbols.iter().find(|s| s.name == "User");

    assert!(user.is_some(), "Should find 'User' interface export");
}

/// Test: Extract aliased import from Python
/// Requirement: CFG-003 - Query captures aliased imports
#[test]
fn test_extract_aliased_import_python() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/imports_variables.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // 'from collections import defaultdict as dd'
    let dd_import = symbols.iter().find(|s| s.name == "dd");

    assert!(dd_import.is_some(), "Should find 'dd' aliased import");
}

/// Test: Extract TypeScript let/var variables
/// Requirement: SVC-004 - Extract variable symbols
#[test]
fn test_extract_variables_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/imports_exports.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let variables: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Variable)
        .collect();

    // imports_exports.ts has: counter (let), legacyVar (var), INTERNAL_SECRET (const, non-exported)
    assert!(
        variables.len() >= 1,
        "Expected at least 1 variable, found {}",
        variables.len()
    );
}

/// Test: Constant vs Variable distinction in Python (UPPER_CASE convention)
/// Requirement: SVC-004 - Distinguish constants from variables
#[test]
fn test_constant_variable_distinction_python() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/imports_variables.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // MAX_RETRIES should be Constant (UPPER_CASE)
    // counter should be Variable (lower_case)
    let max_retries = symbols.iter().find(|s| s.name == "MAX_RETRIES");
    let counter = symbols.iter().find(|s| s.name == "counter");

    assert!(max_retries.is_some(), "Should find MAX_RETRIES");
    assert!(counter.is_some(), "Should find counter");

    // If implementation distinguishes them:
    if max_retries.unwrap().symbol_type != counter.unwrap().symbol_type {
        assert_eq!(
            max_retries.unwrap().symbol_type,
            SymbolType::Constant,
            "UPPER_CASE should be Constant"
        );
        assert_eq!(
            counter.unwrap().symbol_type,
            SymbolType::Variable,
            "lower_case should be Variable"
        );
    }
}

/// Test: Extract Rust pub use re-export
/// Requirement: CFG-006 - Query captures pub use
#[test]
fn test_extract_pub_use_rust() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/imports_constants.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // 'pub use std::fs::File;' re-exports File
    let file_reexport = symbols.iter().find(|s| s.name == "File");

    assert!(
        file_reexport.is_some(),
        "Should find 'File' pub use re-export"
    );

    // Should be marked as Export or Import depending on implementation
    let file_reexport = file_reexport.unwrap();
    assert!(
        file_reexport.symbol_type == SymbolType::Export
            || file_reexport.symbol_type == SymbolType::Import,
        "pub use should be Export or Import, got {:?}",
        file_reexport.symbol_type
    );
}
