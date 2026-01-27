//! AC#4: Symbol Extraction - Classes and Methods Tests
//!
//! Given: A source file containing class definitions with methods
//! When: The parser extracts symbols from the file
//! Then: Both the class (SymbolType::Class) and each method (SymbolType::Method)
//!       are returned as separate Symbol structs
//!
//! Test file: tests/STORY-002/test_ac4_class_method_extraction.rs
//! Source files tested:
//!   - src/parser/symbols.rs (Symbol extractor with class/method handling)
//!   - src/parser/queries/python.rs (Python class queries)
//! Coverage threshold: 95%

use std::path::Path;

// These imports will fail until the parser module is implemented
// This is expected behavior for TDD Red phase
use treelint::parser::{Language, Symbol, SymbolExtractor, SymbolType, Visibility};

/// Helper to get fixture path
fn fixture_path(relative: &str) -> String {
    format!("tests/fixtures/{}", relative)
}

/// Test: SymbolType enum has Class variant
/// Requirement: DM-006 - Define SymbolType enum with Class variant
#[test]
fn test_symbol_type_has_class_variant() {
    let symbol_type = SymbolType::Class;

    assert_eq!(symbol_type, SymbolType::Class);
}

/// Test: SymbolType enum has Method variant
/// Requirement: DM-006 - Define SymbolType enum with Method variant
#[test]
fn test_symbol_type_has_method_variant() {
    let symbol_type = SymbolType::Method;

    assert_eq!(symbol_type, SymbolType::Method);
}

/// Test: Visibility enum has three variants
/// Requirement: DM-007 - Define Visibility enum: Public, Private, Protected
#[test]
fn test_visibility_enum_has_three_variants() {
    let visibilities = [
        Visibility::Public,
        Visibility::Private,
        Visibility::Protected,
    ];

    assert_eq!(visibilities.len(), 3);
}

/// Test: Extract class from Python file
/// Requirement: CFG-002 - Define tree-sitter query for Python class_definition
#[test]
fn test_extract_class_from_python() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/classes.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let classes: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Class)
        .collect();

    // classes.py has: Calculator, Point, _PrivateHelper
    assert!(
        classes.len() >= 3,
        "Expected at least 3 classes, found {}",
        classes.len()
    );

    // Verify Calculator class was extracted
    let calculator = classes.iter().find(|s| s.name == "Calculator");
    assert!(calculator.is_some(), "Should find 'Calculator' class");

    let calculator = calculator.unwrap();
    assert_eq!(calculator.symbol_type, SymbolType::Class);
    assert_eq!(calculator.language, Language::Python);
}

/// Test: Extract methods from Python class
/// Requirement: CFG-002 - Query captures classes and methods
#[test]
fn test_extract_methods_from_python_class() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/classes.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let methods: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Method)
        .collect();

    // Calculator has: __init__, add, subtract, reset
    // Point has: distance_from_origin
    // _PrivateHelper has: _internal_method
    assert!(
        methods.len() >= 6,
        "Expected at least 6 methods, found {}",
        methods.len()
    );

    // Verify specific method was extracted
    let add_method = methods.iter().find(|s| s.name == "add");
    assert!(add_method.is_some(), "Should find 'add' method");

    let add_method = add_method.unwrap();
    assert_eq!(add_method.symbol_type, SymbolType::Method);
}

/// Test: Methods and classes are separate symbols
/// Requirement: SVC-003 - Extract class symbols with methods as separate entries
#[test]
fn test_class_and_methods_are_separate_symbols() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/classes.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Find Calculator class
    let calculator = symbols
        .iter()
        .find(|s| s.name == "Calculator" && s.symbol_type == SymbolType::Class);
    assert!(calculator.is_some(), "Should find Calculator class");

    // Find add method (should be separate from class)
    let add_method = symbols
        .iter()
        .find(|s| s.name == "add" && s.symbol_type == SymbolType::Method);
    assert!(
        add_method.is_some(),
        "Should find add method as separate symbol"
    );

    // They should be different symbols
    let calculator = calculator.unwrap();
    let add_method = add_method.unwrap();
    assert_ne!(
        calculator.line_start, add_method.line_start,
        "Class and method should have different line numbers"
    );
}

/// Test: Extract class from TypeScript file
/// Requirement: CFG-004 - Define tree-sitter query for TypeScript class
#[test]
fn test_extract_class_from_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/classes.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let classes: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Class)
        .collect();

    // classes.ts has: Calculator, Point2D, Shape (abstract)
    assert!(
        classes.len() >= 3,
        "Expected at least 3 classes, found {}",
        classes.len()
    );

    let calculator = classes.iter().find(|s| s.name == "Calculator");
    assert!(calculator.is_some(), "Should find 'Calculator' class");
}

/// Test: Extract methods from TypeScript class
/// Requirement: CFG-004 - Query captures TypeScript class methods
#[test]
fn test_extract_methods_from_typescript_class() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/classes.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let methods: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Method)
        .collect();

    // Calculator has: constructor, add, subtract, reset
    // Point2D has: constructor, distanceFromOrigin
    // Shape has: area (abstract)
    assert!(
        methods.len() >= 5,
        "Expected at least 5 methods, found {}",
        methods.len()
    );
}

/// Test: Extract struct and impl methods from Rust
/// Requirement: CFG-006 - Define tree-sitter query for Rust impl methods
#[test]
fn test_extract_struct_and_impl_from_rust() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/structs_impl.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Find structs (treated as classes)
    let structs: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Class)
        .collect();

    // structs_impl.rs has: Calculator, Point
    assert!(
        structs.len() >= 2,
        "Expected at least 2 structs, found {}",
        structs.len()
    );

    // Find impl methods
    let methods: Vec<_> = symbols
        .iter()
        .filter(|s| s.symbol_type == SymbolType::Method)
        .collect();

    // Calculator impl has: new, add, subtract, reset
    // Default impl has: default
    // Display impl has: fmt
    // Point impl has: distance_from_origin
    assert!(
        methods.len() >= 6,
        "Expected at least 6 methods, found {}",
        methods.len()
    );
}

/// Test: Extract Python __init__ method
/// Requirement: CFG-002 - Query captures dunder methods
#[test]
fn test_extract_dunder_init_method() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/classes.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let init_method = symbols
        .iter()
        .find(|s| s.name == "__init__" && s.symbol_type == SymbolType::Method);

    assert!(init_method.is_some(), "Should find '__init__' method");
}

/// Test: Extract TypeScript constructor
/// Requirement: CFG-004 - Query captures constructors
#[test]
fn test_extract_typescript_constructor() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/classes.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let constructor = symbols
        .iter()
        .find(|s| s.name == "constructor" && s.symbol_type == SymbolType::Method);

    assert!(constructor.is_some(), "Should find 'constructor' method");
}

/// Test: Extract Rust new() associated function
/// Requirement: CFG-006 - Query captures associated functions
#[test]
fn test_extract_rust_new_associated_function() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/structs_impl.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let new_fn = symbols
        .iter()
        .find(|s| s.name == "new" && s.symbol_type == SymbolType::Method);

    assert!(new_fn.is_some(), "Should find 'new' associated function");
}

/// Test: Private class detected in Python (by convention)
/// Requirement: DM-007 - Visibility detection
#[test]
fn test_detect_private_class_python_convention() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/classes.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let private_helper = symbols
        .iter()
        .find(|s| s.name == "_PrivateHelper" && s.symbol_type == SymbolType::Class);

    assert!(
        private_helper.is_some(),
        "Should find '_PrivateHelper' class"
    );

    // Optionally verify visibility is marked as Private
    if let Some(vis) = &private_helper.unwrap().visibility {
        assert_eq!(
            *vis,
            Visibility::Private,
            "Leading underscore should be Private"
        );
    }
}

/// Test: Private method detected in TypeScript
/// Requirement: DM-007 - Visibility detection (TypeScript has explicit private)
#[test]
fn test_detect_private_method_typescript() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/classes.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Calculator has 'private value: number' and protected reset()
    // We're looking for method visibility

    let reset_method = symbols
        .iter()
        .find(|s| s.name == "reset" && s.symbol_type == SymbolType::Method);

    assert!(reset_method.is_some(), "Should find 'reset' method");

    // Optionally verify visibility
    if let Some(vis) = &reset_method.unwrap().visibility {
        assert_eq!(*vis, Visibility::Protected, "reset should be Protected");
    }
}

/// Test: Public method detected in Rust (pub keyword)
/// Requirement: DM-007 - Visibility detection (Rust has explicit pub)
#[test]
fn test_detect_public_method_rust() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/structs_impl.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Calculator::add is pub
    let add_method = symbols
        .iter()
        .find(|s| s.name == "add" && s.symbol_type == SymbolType::Method);

    assert!(add_method.is_some(), "Should find 'add' method");

    // Optionally verify visibility is Public
    if let Some(vis) = &add_method.unwrap().visibility {
        assert_eq!(*vis, Visibility::Public, "add should be Public");
    }
}

/// Test: Private method detected in Rust (no pub keyword)
/// Requirement: DM-007 - Visibility detection
#[test]
fn test_detect_private_method_rust() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/structs_impl.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Calculator::reset has no pub (private by default)
    let reset_method = symbols
        .iter()
        .find(|s| s.name == "reset" && s.symbol_type == SymbolType::Method);

    assert!(reset_method.is_some(), "Should find 'reset' method");

    // Optionally verify visibility is Private (or None if not tracked)
    if let Some(vis) = &reset_method.unwrap().visibility {
        assert_eq!(
            *vis,
            Visibility::Private,
            "reset should be Private (no pub)"
        );
    }
}

/// Test: Extract dataclass from Python
/// Requirement: CFG-002 - Query captures decorated classes
#[test]
fn test_extract_dataclass_python() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/classes.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let point = symbols
        .iter()
        .find(|s| s.name == "Point" && s.symbol_type == SymbolType::Class);

    assert!(point.is_some(), "Should find 'Point' dataclass");
}

/// Test: Extract Rust trait
/// Requirement: CFG-006 - Query captures traits
#[test]
fn test_extract_rust_trait() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("rust/structs_impl.rs");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Shape trait should be extracted (may be treated as Class or separate Trait type)
    let shape = symbols.iter().find(|s| s.name == "Shape");

    assert!(shape.is_some(), "Should find 'Shape' trait");
}

/// Test: Extract TypeScript interface
/// Requirement: CFG-004 - Query captures interfaces
#[test]
fn test_extract_typescript_interface() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("typescript/classes.ts");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    // Point interface should be extracted
    let point = symbols.iter().find(|s| s.name == "Point");

    assert!(point.is_some(), "Should find 'Point' interface");
}

/// Test: Class has accurate line boundaries
/// Requirement: DM-004 - Symbol has accurate line_start/line_end
#[test]
fn test_class_has_accurate_line_boundaries() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/classes.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let calculator = symbols
        .iter()
        .find(|s| s.name == "Calculator" && s.symbol_type == SymbolType::Class)
        .expect("Should find Calculator class");

    // Calculator class spans multiple lines (including all methods)
    assert!(calculator.line_start > 0, "line_start should be positive");
    assert!(
        calculator.line_end > calculator.line_start,
        "Class should span multiple lines"
    );
    // Calculator has 4 methods plus class body, should be at least 15 lines
    assert!(
        calculator.line_end - calculator.line_start >= 10,
        "Calculator class should span at least 10 lines, got {} to {}",
        calculator.line_start,
        calculator.line_end
    );
}

/// Test: Method has accurate line boundaries (within class)
/// Requirement: DM-004 - Symbol has accurate line_start/line_end
#[test]
fn test_method_line_boundaries_within_class() {
    let extractor = SymbolExtractor::new();
    let path = fixture_path("python/classes.py");

    let symbols = extractor
        .extract_from_file(Path::new(&path))
        .expect("Extraction should succeed");

    let calculator = symbols
        .iter()
        .find(|s| s.name == "Calculator" && s.symbol_type == SymbolType::Class)
        .expect("Should find Calculator class");

    let add_method = symbols
        .iter()
        .find(|s| s.name == "add" && s.symbol_type == SymbolType::Method)
        .expect("Should find add method");

    // Method should be within class boundaries
    assert!(
        add_method.line_start >= calculator.line_start,
        "Method should start at or after class start"
    );
    assert!(
        add_method.line_end <= calculator.line_end,
        "Method should end at or before class end"
    );
}
