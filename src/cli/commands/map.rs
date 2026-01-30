//! Map command implementation.
//!
//! This module provides the `treelint map` command for generating a
//! repository map showing all symbols organized by file with optional
//! relevance scoring.
//!
//! # Progress Indicator
//!
//! For operations that take longer than 2 seconds, a progress spinner
//! is displayed to provide user feedback.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use indicatif::{ProgressBar, ProgressStyle};

use crate::cli::args::{MapArgs, OutputFormat, SymbolType as CliSymbolType};
use crate::index::{calculate_relevance_scores, IndexStorage};
use crate::output::json::MapOutput;
use crate::output::text::format_map_text;
use crate::parser::SymbolType as ParserSymbolType;
use crate::TreelintError;

/// Threshold for showing progress indicator (2 seconds).
const PROGRESS_THRESHOLD_SECS: u64 = 2;

/// Execute the map command.
///
/// Generates a repository map of all symbols, optionally filtered by type
/// and sorted by relevance score.
///
/// # Arguments
///
/// * `args` - The map command arguments
///
/// # Returns
///
/// `Ok(())` if successful, `Err(TreelintError)` otherwise.
pub fn execute(args: MapArgs) -> Result<(), TreelintError> {
    let start_time = Instant::now();

    // Determine output format
    let format = args.format.unwrap_or_else(|| {
        if atty::is(atty::Stream::Stdout) {
            OutputFormat::Text
        } else {
            OutputFormat::Json
        }
    });

    // Get project root (current directory)
    let project_root = std::env::current_dir().map_err(|e| {
        TreelintError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Cannot determine current directory: {}", e),
        ))
    })?;

    // Check if index exists
    let db_path = project_root.join(".treelint").join("index.db");
    if !db_path.exists() {
        return Err(TreelintError::Cli(
            "Index not found. Run 'treelint index' first.".to_string(),
        ));
    }

    // Set up progress indicator (only show if operation takes > 2 seconds)
    let mut progress = ProgressIndicator::new();

    // Open storage and get symbols
    let storage = IndexStorage::new(&project_root)
        .map_err(|e| TreelintError::Cli(format!("Failed to open index: {}", e)))?;

    // Get all symbols
    let mut symbols = storage
        .get_all_symbols()
        .map_err(|e| TreelintError::Cli(format!("Failed to query symbols: {}", e)))?;

    // Check if we should show progress (> 2 seconds elapsed)
    if start_time.elapsed() > Duration::from_secs(PROGRESS_THRESHOLD_SECS) {
        progress.start("Generating repository map...");
    }

    // Apply type filter if specified
    if let Some(ref cli_type) = args.symbol_type {
        let parser_type = cli_symbol_type_to_parser_type(cli_type);
        symbols.retain(|s| s.symbol_type == parser_type);
    }

    // Calculate relevance if ranked (using tree-sitter based extraction)
    let relevance_scores = if args.ranked {
        // Update progress message
        if start_time.elapsed() > Duration::from_secs(PROGRESS_THRESHOLD_SECS) {
            progress.start("Calculating relevance scores...");
        }
        calculate_relevance_scores(&symbols)
    } else {
        HashMap::new()
    };

    // Sort by relevance if ranked
    if args.ranked {
        symbols.sort_by(|a, b| {
            let score_a = relevance_scores
                .get(&format!("{}:{}", a.file_path, a.name))
                .unwrap_or(&0.0);
            let score_b = relevance_scores
                .get(&format!("{}:{}", b.file_path, b.name))
                .unwrap_or(&0.0);
            score_b
                .partial_cmp(score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    // Build output
    if start_time.elapsed() > Duration::from_secs(PROGRESS_THRESHOLD_SECS) {
        progress.start("Building output...");
    }
    let map_output = build_map_output(&symbols, args.ranked, &relevance_scores);

    // Stop progress indicator before output
    progress.finish();

    // Format and print
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&map_output)
                .map_err(|e| TreelintError::Parse(format!("Failed to serialize JSON: {}", e)))?;
            println!("{}", json);
        }
        OutputFormat::Text => {
            let text = format_map_text(&map_output, args.ranked);
            print!("{}", text);
        }
    }

    Ok(())
}

/// Progress indicator that shows a spinner for long-running operations.
///
/// The indicator is only shown if the operation takes longer than the
/// configured threshold (2 seconds by default).
struct ProgressIndicator {
    bar: Option<ProgressBar>,
    active: Arc<AtomicBool>,
}

impl ProgressIndicator {
    /// Create a new progress indicator.
    fn new() -> Self {
        Self {
            bar: None,
            active: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start showing the progress indicator with a message.
    fn start(&mut self, message: &str) {
        // Only create spinner if outputting to TTY
        if !atty::is(atty::Stream::Stderr) {
            return;
        }

        if !self.active.load(Ordering::SeqCst) {
            let bar = ProgressBar::new_spinner();
            bar.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template("{spinner:.cyan} {msg}")
                    .unwrap_or_else(|_| ProgressStyle::default_spinner()),
            );
            bar.set_message(message.to_string());
            bar.enable_steady_tick(Duration::from_millis(100));
            self.bar = Some(bar);
            self.active.store(true, Ordering::SeqCst);
        } else if let Some(ref bar) = self.bar {
            bar.set_message(message.to_string());
        }
    }

    /// Stop the progress indicator.
    fn finish(&self) {
        if let Some(ref bar) = self.bar {
            bar.finish_and_clear();
        }
    }
}

impl Drop for ProgressIndicator {
    fn drop(&mut self) {
        self.finish();
    }
}

/// Convert CLI symbol type to parser symbol type.
fn cli_symbol_type_to_parser_type(cli_type: &CliSymbolType) -> ParserSymbolType {
    match cli_type {
        CliSymbolType::Function => ParserSymbolType::Function,
        CliSymbolType::Class => ParserSymbolType::Class,
        CliSymbolType::Method => ParserSymbolType::Method,
        CliSymbolType::Variable => ParserSymbolType::Variable,
        CliSymbolType::Constant => ParserSymbolType::Constant,
        CliSymbolType::Import => ParserSymbolType::Import,
        CliSymbolType::Export => ParserSymbolType::Export,
    }
}

/// Build the map output structure.
fn build_map_output(
    symbols: &[crate::parser::Symbol],
    include_relevance: bool,
    relevance_scores: &HashMap<String, f64>,
) -> MapOutput {
    use crate::output::json::{FileSymbols, MapSymbol};
    use std::collections::BTreeMap;

    let total_symbols = symbols.len();

    // Group symbols by file
    let mut by_file: BTreeMap<String, FileSymbols> = BTreeMap::new();
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    let mut file_set = std::collections::HashSet::new();

    for symbol in symbols {
        file_set.insert(symbol.file_path.clone());

        // Count by type
        let type_str = format!("{:?}", symbol.symbol_type).to_lowercase();
        *type_counts.entry(type_str).or_insert(0) += 1;

        // Build MapSymbol
        let key = format!("{}:{}", symbol.file_path, symbol.name);
        let relevance = if include_relevance {
            relevance_scores.get(&key).copied()
        } else {
            None
        };

        let map_symbol = MapSymbol {
            name: symbol.name.clone(),
            symbol_type: format!("{:?}", symbol.symbol_type).to_lowercase(),
            lines: vec![symbol.line_start, symbol.line_end],
            relevance,
        };

        // Add to file group
        let file_entry = by_file
            .entry(symbol.file_path.clone())
            .or_insert_with(|| FileSymbols {
                language: format!("{:?}", symbol.language),
                symbols: Vec::new(),
            });
        file_entry.symbols.push(map_symbol);
    }

    // Sort symbols within each file by relevance if ranked, otherwise by line number
    for file_symbols in by_file.values_mut() {
        if include_relevance {
            file_symbols.symbols.sort_by(|a, b| {
                let score_a = a.relevance.unwrap_or(0.0);
                let score_b = b.relevance.unwrap_or(0.0);
                score_b
                    .partial_cmp(&score_a)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            file_symbols
                .symbols
                .sort_by_key(|s| s.lines.first().copied().unwrap_or(0));
        }
    }

    MapOutput {
        total_symbols,
        total_files: file_set.len(),
        by_file,
        by_type: type_counts,
    }
}
