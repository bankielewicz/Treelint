//! Search command implementation
//!
//! This module implements the `treelint search` command which searches
//! for symbols in the codebase and outputs results in JSON or text format.

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use walkdir::WalkDir;

use crate::cli::args::{OutputFormat, SearchArgs, SymbolType};
use crate::daemon::server::DaemonClient;
use crate::index::storage::IndexStorage;
use crate::output::json::{SearchOutput, SearchQuery, SearchResult, SearchStats};
use crate::output::text::TextFormatter;
use crate::output::OutputRouter;
use crate::parser::{extract_lines_context, ContextMode, Language, Symbol, SymbolExtractor};

/// The index directory name
const INDEX_DIR: &str = ".treelint";
/// The index database filename
const INDEX_FILE: &str = "index.db";
/// The daemon socket filename (Unix)
#[cfg(unix)]
const DAEMON_SOCKET: &str = "daemon.sock";
/// Progress bar template for indexing
const PROGRESS_BAR_TEMPLATE: &str =
    "{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} files ({eta})";

/// Validate and compile regex pattern if regex mode is enabled.
///
/// # Arguments
///
/// * `pattern` - The pattern string to compile
/// * `regex_enabled` - Whether regex mode is enabled
/// * `format` - Output format for error messages
///
/// # Returns
///
/// `Some(Regex)` if regex mode is enabled and pattern is valid,
/// `None` if regex mode is disabled.
/// Exits with code 1 if pattern is invalid.
fn validate_regex_pattern(
    pattern: &str,
    regex_enabled: bool,
    format: &OutputFormat,
) -> Option<Regex> {
    if !regex_enabled {
        return None;
    }

    match Regex::new(pattern) {
        Ok(re) => Some(re),
        Err(e) => {
            let error_msg = format!("Invalid regex pattern '{}': {}", pattern, e);
            let hint = "Check your regex syntax. Common issues: unescaped special characters, unbalanced parentheses.";

            if *format == OutputFormat::Json {
                eprintln!(
                    "{}",
                    serde_json::json!({
                        "error": error_msg,
                        "hint": hint
                    })
                );
            } else {
                eprintln!("Error: {}", error_msg);
                eprintln!("Hint: {}", hint);
            }
            std::process::exit(1);
        }
    }
}

/// Check if a symbol name matches the search pattern.
///
/// Handles regex matching, case-insensitive matching, and exact matching.
fn symbol_matches_pattern(
    symbol_name: &str,
    search_pattern: &str,
    regex_pattern: Option<&Regex>,
    ignore_case: bool,
) -> bool {
    if let Some(re) = regex_pattern {
        if ignore_case {
            // Build case-insensitive regex
            match Regex::new(&format!("(?i){}", search_pattern)) {
                Ok(ci_re) => ci_re.is_match(symbol_name),
                Err(_) => re.is_match(symbol_name),
            }
        } else {
            re.is_match(symbol_name)
        }
    } else if ignore_case {
        symbol_name.to_lowercase() == search_pattern.to_lowercase()
    } else {
        symbol_name == search_pattern
    }
}

/// Filter symbols based on search criteria.
///
/// # Arguments
///
/// * `symbols` - All symbols from the index
/// * `search_pattern` - The pattern to search for
/// * `regex_pattern` - Compiled regex if regex mode is enabled
/// * `ignore_case` - Whether to ignore case
/// * `type_filter` - Optional symbol type filter
///
/// # Returns
///
/// Filtered vector of matching symbols.
fn filter_symbols(
    symbols: Vec<Symbol>,
    search_pattern: &str,
    regex_pattern: Option<&Regex>,
    ignore_case: bool,
    type_filter: Option<&SymbolType>,
) -> Vec<Symbol> {
    let mut results: Vec<Symbol> = symbols
        .into_iter()
        .filter(|s| symbol_matches_pattern(&s.name, search_pattern, regex_pattern, ignore_case))
        .collect();

    // Apply type filter if specified
    if let Some(type_filter) = type_filter {
        let filter_type = convert_symbol_type(type_filter);
        results.retain(|s| s.symbol_type == filter_type);
    }

    results
}

/// Convert symbols to search results for output.
///
/// # Arguments
///
/// * `symbols` - The symbols to convert
/// * `context_mode` - The context extraction mode
/// * `source_cache` - Optional cache of file contents for line-based context extraction
fn symbols_to_search_results(
    symbols: &[Symbol],
    context_mode: &ContextMode,
    source_cache: &HashMap<String, String>,
) -> Vec<SearchResult> {
    symbols
        .iter()
        .map(|s| {
            let body = match context_mode {
                ContextMode::Signatures => None,
                ContextMode::Full => Some(s.body.clone().unwrap_or_default()),
                ContextMode::Lines(n) => {
                    // For lines mode, extract context from source file
                    if let Some(source) = source_cache.get(&s.file_path) {
                        extract_lines_context(source, s.line_start, s.line_end, *n)
                    } else {
                        // Fallback to stored body if source not available
                        Some(s.body.clone().unwrap_or_default())
                    }
                }
            };

            SearchResult {
                symbol_type: symbol_type_to_string(&s.symbol_type),
                name: s.name.clone(),
                file: s.file_path.clone(),
                lines: (s.line_start, s.line_end),
                signature: s.signature.clone().unwrap_or_default(),
                body,
                language: Some(language_to_string(&s.language)),
            }
        })
        .collect()
}

/// Collect unique languages from symbols for statistics.
///
/// # Arguments
///
/// * `symbols` - The symbols to extract languages from
///
/// # Returns
///
/// A sorted, deduplicated vector of language name strings.
fn collect_languages(symbols: &[Symbol]) -> Vec<String> {
    let mut languages: Vec<String> = symbols
        .iter()
        .map(|s| language_to_string(&s.language))
        .collect();
    languages.sort();
    languages.dedup();
    languages
}

/// Build a SearchOutput from the search arguments and results.
///
/// # Arguments
///
/// * `args` - The search arguments
/// * `context_mode` - The resolved context mode
/// * `results` - The search results
/// * `files_searched` - Number of files searched
/// * `elapsed_ms` - Search duration in milliseconds
/// * `languages` - Languages that were searched
fn build_search_output(
    args: &SearchArgs,
    context_mode: &ContextMode,
    results: Vec<SearchResult>,
    files_searched: u64,
    elapsed_ms: u64,
    languages: Vec<String>,
) -> SearchOutput {
    let query = SearchQuery {
        symbol: args.symbol.clone(),
        symbol_type: args.symbol_type.as_ref().map(|t| t.as_str().to_string()),
        case_insensitive: if args.ignore_case { Some(true) } else { None },
        regex: if args.regex { Some(true) } else { None },
        context_mode: context_mode.to_json_string(),
    };

    let stats = SearchStats {
        files_searched,
        elapsed_ms,
        files_skipped: 0,
        skipped_by_type: HashMap::new(),
        languages_searched: languages,
    };

    SearchOutput::new(query, results, stats)
}

/// Output search results in JSON format.
///
/// # Arguments
///
/// * `output` - The search output to serialize
fn output_json(output: &SearchOutput) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(output)?;
    println!("{}", json);
    Ok(())
}

/// Output search results in text format using TextFormatter.
///
/// # Arguments
///
/// * `symbol` - The search term
/// * `results` - The search results
/// * `signatures_only` - Whether to omit body content
/// * `elapsed_ms` - Search duration in milliseconds
/// * `files_searched` - Number of files searched
/// * `is_tty` - Whether stdout is a TTY (enables colors)
fn output_text(
    symbol: &str,
    results: &[SearchResult],
    signatures_only: bool,
    elapsed_ms: u64,
    files_searched: u64,
    is_tty: bool,
) {
    let formatter = TextFormatter::new(signatures_only, is_tty);
    let output = formatter.format(symbol, results, elapsed_ms, files_searched);
    print!("{}", output);
}

/// Resolve the context mode from search arguments.
///
/// Priority:
/// 1. If --signatures is set, return Signatures mode
/// 2. If --context is set, parse it (N or "full")
/// 3. Otherwise, default to Full mode
fn resolve_context_mode(args: &SearchArgs) -> ContextMode {
    if args.signatures {
        return ContextMode::Signatures;
    }

    if let Some(ref context_value) = args.context {
        // Already validated by clap, so this should always succeed
        ContextMode::from_cli_value(context_value).unwrap_or_default()
    } else {
        // Default to full semantic context
        ContextMode::Full
    }
}

/// Build a cache of source file contents for context extraction.
///
/// Only loads files that are needed for the matching symbols.
fn build_source_cache(symbols: &[Symbol]) -> HashMap<String, String> {
    let mut cache = HashMap::new();

    for symbol in symbols {
        if !cache.contains_key(&symbol.file_path) {
            if let Ok(content) = fs::read_to_string(&symbol.file_path) {
                cache.insert(symbol.file_path.clone(), content);
            }
        }
    }

    cache
}

/// Get the socket path for the current directory
fn get_socket_path(project_root: &Path) -> String {
    #[cfg(unix)]
    {
        project_root
            .join(INDEX_DIR)
            .join(DAEMON_SOCKET)
            .to_string_lossy()
            .to_string()
    }

    #[cfg(windows)]
    {
        format!(
            r"\\.\pipe\treelint-daemon-{}",
            project_root
                .to_string_lossy()
                .replace(['\\', '/', ':'], "-")
        )
    }
}

/// Try to search via daemon if available
///
/// Returns Some(symbols) if daemon search succeeded, None if daemon is not available
fn try_daemon_search(
    socket_path: &str,
    symbol: &str,
    symbol_type: Option<&SymbolType>,
    ignore_case: bool,
    use_regex: bool,
) -> Option<Vec<Symbol>> {
    let client = DaemonClient::connect(socket_path).ok()?;

    let type_str = symbol_type.map(|t| t.as_str());

    let request = serde_json::json!({
        "id": "search-1",
        "method": "search",
        "params": {
            "symbol": symbol,
            "type": type_str,
            "case_insensitive": ignore_case,
            "regex": use_regex
        }
    });

    let response = client.send_request(&request).ok()?;

    // Check if response is successful
    let result = response.get("result")?;

    // Parse results array into symbols
    let results_array = result.as_array()?;

    let symbols: Vec<Symbol> = results_array
        .iter()
        .filter_map(|item| {
            let name = item.get("name")?.as_str()?;
            let symbol_type_str = item.get("type")?.as_str()?;
            let file = item.get("file")?.as_str()?;
            let line_start = item.get("line_start")?.as_u64()? as usize;
            let line_end = item.get("line_end")?.as_u64()? as usize;
            let signature = item.get("signature").and_then(|s| s.as_str());
            let body = item.get("body").and_then(|b| b.as_str());

            Some(Symbol {
                name: name.to_string(),
                symbol_type: string_to_symbol_type(symbol_type_str),
                visibility: None,
                file_path: file.to_string(),
                line_start,
                line_end,
                signature: signature.map(String::from),
                body: body.map(String::from),
                language: Language::from_path(Path::new(file)).unwrap_or(Language::Python),
            })
        })
        .collect();

    Some(symbols)
}

/// Convert string to parser SymbolType
fn string_to_symbol_type(s: &str) -> crate::parser::SymbolType {
    match s.to_lowercase().as_str() {
        "function" => crate::parser::SymbolType::Function,
        "class" => crate::parser::SymbolType::Class,
        "method" => crate::parser::SymbolType::Method,
        "variable" => crate::parser::SymbolType::Variable,
        "constant" => crate::parser::SymbolType::Constant,
        "import" => crate::parser::SymbolType::Import,
        "export" => crate::parser::SymbolType::Export,
        _ => crate::parser::SymbolType::Function,
    }
}

/// Execute the search command with the given arguments.
///
/// # Arguments
///
/// * `args` - The parsed search command arguments
///
/// # Returns
///
/// Returns `Ok(())` on success with exit code 0 for results found,
/// exit code 2 for no results, or an error with exit code 1.
pub fn execute(args: SearchArgs) -> anyhow::Result<()> {
    let start = Instant::now();

    // Resolve output format via OutputRouter (TTY auto-detection)
    let router = OutputRouter::new();
    let resolved_format = router.resolve_format(args.format);

    // Resolve context mode from arguments
    let context_mode = resolve_context_mode(&args);

    // Validate regex pattern first if regex mode is enabled
    let regex_pattern = validate_regex_pattern(&args.symbol, args.regex, &resolved_format);

    // Determine project root (current directory)
    let project_root = std::env::current_dir()?;
    let index_path = project_root.join(INDEX_DIR).join(INDEX_FILE);
    let socket_path = get_socket_path(&project_root);

    // Auto-detection: Try daemon first, then index, then build on-demand
    // Step 1: Try daemon if available
    let (results, files_searched, languages) = if let Some(daemon_results) = try_daemon_search(
        &socket_path,
        &args.symbol,
        args.symbol_type.as_ref(),
        args.ignore_case,
        args.regex,
    ) {
        // Daemon search succeeded
        let languages = collect_languages(&daemon_results);
        let file_count = daemon_results
            .iter()
            .map(|s| s.file_path.as_str())
            .collect::<std::collections::HashSet<_>>()
            .len() as u64;
        (daemon_results, file_count, languages)
    } else {
        // Step 2: Fall back to index
        // Check if index exists, if not, build it automatically
        let files_indexed = if !index_path.exists() {
            build_index(&project_root, &index_path)?
        } else {
            0 // Index already exists
        };

        // Open the index and perform search
        let storage = IndexStorage::open(&index_path)?;

        // Get all symbols from index and filter based on search criteria
        let all_symbols = storage.get_all_symbols()?;

        // Collect languages before filtering
        let languages = collect_languages(&all_symbols);

        let results = filter_symbols(
            all_symbols,
            &args.symbol,
            regex_pattern.as_ref(),
            args.ignore_case,
            args.symbol_type.as_ref(),
        );

        let file_count = storage.get_file_count()? as u64;
        let total_files = file_count.max(files_indexed as u64);

        (results, total_files, languages)
    };

    let elapsed = start.elapsed().as_millis() as u64;

    // Build source cache for line-based context extraction
    let source_cache = if context_mode.is_lines() {
        build_source_cache(&results)
    } else {
        HashMap::new()
    };

    // Convert to search results and output
    let search_results = symbols_to_search_results(&results, &context_mode, &source_cache);
    let has_results = !search_results.is_empty();

    match resolved_format {
        OutputFormat::Json => {
            let search_output = build_search_output(
                &args,
                &context_mode,
                search_results,
                files_searched,
                elapsed,
                languages,
            );
            output_json(&search_output)?;
        }
        OutputFormat::Text => {
            output_text(
                &args.symbol,
                &search_results,
                context_mode.is_signatures(),
                elapsed,
                files_searched,
                router.is_tty(),
            );
        }
    }

    // Exit with appropriate code
    std::process::exit(if has_results { 0 } else { 2 });
}

/// Check if a directory should be skipped during indexing.
///
/// Skips hidden directories and common non-source directories.
fn should_skip_directory(name: &str) -> bool {
    name.starts_with('.')
        || name == "node_modules"
        || name == "target"
        || name == "__pycache__"
        || name == "venv"
}

/// Check if a file is binary by looking for null bytes in the first 8KB.
/// Only reads the bytes needed, not the entire file.
fn is_binary_file(path: &Path) -> bool {
    use std::io::Read;
    const BINARY_CHECK_BYTES: usize = 8000;

    match fs::File::open(path) {
        Ok(file) => {
            let mut buffer = [0u8; BINARY_CHECK_BYTES];
            let mut reader = std::io::BufReader::new(file);
            match reader.read(&mut buffer) {
                Ok(n) => buffer[..n].contains(&0),
                Err(_) => true, // Treat unreadable files as binary
            }
        }
        Err(_) => true, // Treat unreadable files as binary (skip them)
    }
}

/// Collect all source files from the project root.
fn collect_source_files(project_root: &Path) -> Vec<PathBuf> {
    WalkDir::new(project_root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            // Always process root directory
            if e.depth() == 0 {
                return true;
            }
            // Skip excluded directories
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy();
                return !should_skip_directory(&name);
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| Language::from_path(e.path()).is_some())
        .filter(|e| !is_binary_file(e.path()))
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Build the index for the project.
///
/// Returns the number of files indexed.
fn build_index(project_root: &Path, index_path: &Path) -> anyhow::Result<usize> {
    // Create the .treelint directory
    if let Some(parent) = index_path.parent() {
        fs::create_dir_all(parent)?;
    }

    eprintln!("Building index...");
    let stderr = std::io::stderr();

    // Collect source files
    let source_files = collect_source_files(project_root);

    let total_files = source_files.len();

    // Progress bar
    let progress = ProgressBar::new(total_files as u64);
    let style = ProgressStyle::default_bar()
        .template(PROGRESS_BAR_TEMPLATE)
        .map_err(|e| anyhow::anyhow!("Failed to set progress bar template: {}", e))?
        .progress_chars("#>-");
    progress.set_style(style);

    // Create index storage
    let storage = IndexStorage::create(index_path)?;
    let extractor = SymbolExtractor::new();

    let mut indexed_count = 0;
    for file_path in &source_files {
        progress.inc(1);

        // Extract symbols from file
        match extractor.extract_from_file(file_path) {
            Ok(symbols) => {
                for symbol in symbols {
                    if let Err(e) = storage.insert_symbol(&symbol) {
                        // Log error but continue
                        let _ = writeln!(stderr.lock(), "Warning: Failed to index symbol: {}", e);
                    }
                }
                indexed_count += 1;
            }
            Err(e) => {
                // Log error but continue
                let _ = writeln!(
                    stderr.lock(),
                    "Warning: Failed to parse {}: {}",
                    file_path.display(),
                    e
                );
            }
        }
    }

    progress.finish_with_message("Index built successfully");
    eprintln!("Indexed {} files", indexed_count);

    Ok(indexed_count)
}

/// Convert CLI SymbolType to parser SymbolType string for comparison
fn convert_symbol_type(cli_type: &SymbolType) -> crate::parser::SymbolType {
    match cli_type {
        SymbolType::Function => crate::parser::SymbolType::Function,
        SymbolType::Class => crate::parser::SymbolType::Class,
        SymbolType::Method => crate::parser::SymbolType::Method,
        SymbolType::Variable => crate::parser::SymbolType::Variable,
        SymbolType::Constant => crate::parser::SymbolType::Constant,
        SymbolType::Import => crate::parser::SymbolType::Import,
        SymbolType::Export => crate::parser::SymbolType::Export,
    }
}

/// Convert parser SymbolType to string for JSON output
fn symbol_type_to_string(symbol_type: &crate::parser::SymbolType) -> String {
    match symbol_type {
        crate::parser::SymbolType::Function => "function".to_string(),
        crate::parser::SymbolType::Class => "class".to_string(),
        crate::parser::SymbolType::Method => "method".to_string(),
        crate::parser::SymbolType::Variable => "variable".to_string(),
        crate::parser::SymbolType::Constant => "constant".to_string(),
        crate::parser::SymbolType::Import => "import".to_string(),
        crate::parser::SymbolType::Export => "export".to_string(),
    }
}

/// Convert Language to string for JSON output
fn language_to_string(language: &Language) -> String {
    match language {
        Language::Python => "python".to_string(),
        Language::TypeScript => "typescript".to_string(),
        Language::Rust => "rust".to_string(),
        Language::Markdown => "markdown".to_string(),
    }
}
