//! Performance benchmarks for output formatters.
//!
//! NFR Requirements (STORY-005):
//! - NFR-001: JSON serialization under 5ms for 100 results (p95)
//! - NFR-002: Text formatting under 10ms for 100 results (p95)

use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use treelint::output::json::{SearchOutput, SearchQuery, SearchResult, SearchStats};
use treelint::output::text::TextFormatter;

/// Generate sample search results for benchmarking.
///
/// Creates `count` realistic search results with varying content
/// to simulate real-world usage.
fn generate_results(count: usize) -> Vec<SearchResult> {
    (0..count)
        .map(|i| SearchResult {
            symbol_type: if i % 3 == 0 {
                "Function".to_string()
            } else if i % 3 == 1 {
                "Method".to_string()
            } else {
                "Class".to_string()
            },
            name: format!("symbol_name_{}", i),
            file: format!("src/module_{}/file_{}.rs", i / 10, i % 10),
            lines: (i * 10 + 1, i * 10 + 20),
            signature: format!(
                "pub fn symbol_name_{}(arg1: String, arg2: i32) -> Result<(), Error>",
                i
            ),
            body: Some(format!(
                r#"    let result = process_data(arg1)?;
    if arg2 > 0 {{
        return Ok(result);
    }}
    // Comment line {}
    validate_input(&result)?;
    transform_output(result)"#,
                i
            )),
            language: Some("rust".to_string()),
        })
        .collect()
}

/// Generate a complete SearchOutput with the given results.
fn generate_search_output(results: Vec<SearchResult>) -> SearchOutput {
    let mut skipped_by_type = HashMap::new();
    skipped_by_type.insert("binary".to_string(), 5);
    skipped_by_type.insert("unsupported".to_string(), 3);

    SearchOutput::new(
        SearchQuery {
            symbol: "test_symbol".to_string(),
            symbol_type: Some("Function".to_string()),
            case_insensitive: Some(false),
            regex: Some(false),
            context_mode: "full".to_string(),
        },
        results,
        SearchStats {
            files_searched: 150,
            elapsed_ms: 42,
            files_skipped: 8,
            skipped_by_type,
            languages_searched: vec!["rust".to_string(), "python".to_string()],
        },
    )
}

/// Benchmark JSON serialization with 100 results.
///
/// NFR-001: Must complete in under 5ms (p95).
fn bench_json_serialization_100(c: &mut Criterion) {
    let results = generate_results(100);
    let output = generate_search_output(results);

    c.bench_function("json_serialize_100_results", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&output)).unwrap();
            black_box(json)
        })
    });
}

/// Benchmark JSON serialization with 1000 results (stress test).
fn bench_json_serialization_1000(c: &mut Criterion) {
    let results = generate_results(1000);
    let output = generate_search_output(results);

    c.bench_function("json_serialize_1000_results", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&output)).unwrap();
            black_box(json)
        })
    });
}

/// Benchmark text formatting with 100 results (no colors).
///
/// NFR-002: Must complete in under 10ms (p95).
fn bench_text_formatting_100(c: &mut Criterion) {
    let results = generate_results(100);
    let formatter = TextFormatter::new(false, false);

    c.bench_function("text_format_100_results", |b| {
        b.iter(|| {
            let text = formatter.format(
                black_box("test_symbol"),
                black_box(&results),
                black_box(42),
                black_box(150),
            );
            black_box(text)
        })
    });
}

/// Benchmark text formatting with 100 results (with colors).
fn bench_text_formatting_100_colored(c: &mut Criterion) {
    let results = generate_results(100);
    let formatter = TextFormatter::new(false, true);

    c.bench_function("text_format_100_results_colored", |b| {
        b.iter(|| {
            let text = formatter.format(
                black_box("test_symbol"),
                black_box(&results),
                black_box(42),
                black_box(150),
            );
            black_box(text)
        })
    });
}

/// Benchmark text formatting with 100 results (signatures only).
fn bench_text_formatting_100_signatures(c: &mut Criterion) {
    let results = generate_results(100);
    let formatter = TextFormatter::new(true, false);

    c.bench_function("text_format_100_results_signatures", |b| {
        b.iter(|| {
            let text = formatter.format(
                black_box("test_symbol"),
                black_box(&results),
                black_box(42),
                black_box(150),
            );
            black_box(text)
        })
    });
}

/// Benchmark text formatting with 1000 results (stress test).
fn bench_text_formatting_1000(c: &mut Criterion) {
    let results = generate_results(1000);
    let formatter = TextFormatter::new(false, false);

    c.bench_function("text_format_1000_results", |b| {
        b.iter(|| {
            let text = formatter.format(
                black_box("test_symbol"),
                black_box(&results),
                black_box(42),
                black_box(150),
            );
            black_box(text)
        })
    });
}

criterion_group!(
    benches,
    bench_json_serialization_100,
    bench_json_serialization_1000,
    bench_text_formatting_100,
    bench_text_formatting_100_colored,
    bench_text_formatting_100_signatures,
    bench_text_formatting_1000,
);
criterion_main!(benches);
