//! Index command implementation
//!
//! This module implements the `treelint index` command which builds
//! the symbol index for the current project.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;

use crate::cli::args::IndexArgs;
use crate::index::storage::IndexStorage;
use crate::parser::{Language, SymbolExtractor};

/// The index directory name
const INDEX_DIR: &str = ".treelint";
/// The index database filename
const INDEX_FILE: &str = "index.db";
/// Progress bar template for indexing
const PROGRESS_BAR_TEMPLATE: &str =
    "{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} files ({eta})";
/// Minimum files for showing progress bar
const PROGRESS_BAR_THRESHOLD: usize = 10;

/// Execute the index command with the given arguments.
///
/// # Arguments
///
/// * `args` - The parsed index command arguments
///
/// # Returns
///
/// Returns `Ok(())` on success with exit code 0.
pub fn execute(args: IndexArgs) -> anyhow::Result<()> {
    let start = Instant::now();

    // Determine project root (current directory)
    let project_root = std::env::current_dir()?;
    let index_dir = project_root.join(INDEX_DIR);
    let index_path = index_dir.join(INDEX_FILE);

    // Create the .treelint directory if needed
    if !index_dir.exists() {
        fs::create_dir_all(&index_dir)?;
    }

    // If force flag is set and index exists, remove it first
    if args.force && index_path.exists() {
        fs::remove_file(&index_path)?;
    }

    // Build the index
    let (files_indexed, symbols_found) = build_index(&project_root, &index_path, args.force)?;

    let elapsed = start.elapsed();
    let elapsed_str = if elapsed.as_secs() > 0 {
        format!("{:.2}s", elapsed.as_secs_f64())
    } else {
        format!("{}ms", elapsed.as_millis())
    };

    // Print completion summary
    println!("Index built successfully");
    println!("Files indexed: {}", files_indexed);
    println!("Symbols found: {}", symbols_found);
    println!("Duration: {}", elapsed_str);

    Ok(())
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
        || name == ".git"
}

/// Check if a file is binary by looking for null bytes in the first 8KB.
fn is_binary_file(path: &Path) -> bool {
    use std::io::Read;
    const BINARY_CHECK_BYTES: usize = 8000;

    match fs::File::open(path) {
        Ok(file) => {
            let mut buffer = [0u8; BINARY_CHECK_BYTES];
            let mut reader = std::io::BufReader::new(file);
            match reader.read(&mut buffer) {
                Ok(n) => buffer[..n].contains(&0),
                Err(_) => true,
            }
        }
        Err(_) => true,
    }
}

/// Collect all source files from the project root.
fn collect_source_files(project_root: &Path) -> Vec<PathBuf> {
    WalkDir::new(project_root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 {
                return true;
            }
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
/// Returns (files_indexed, symbols_found).
fn build_index(
    project_root: &Path,
    index_path: &Path,
    force: bool,
) -> anyhow::Result<(usize, usize)> {
    let stderr = std::io::stderr();

    // Collect source files
    let source_files = collect_source_files(project_root);
    let total_files = source_files.len();

    // Show progress bar if we have enough files and stdout is a TTY
    let show_progress = total_files >= PROGRESS_BAR_THRESHOLD && atty::is(atty::Stream::Stderr);

    let progress = if show_progress {
        let pb = ProgressBar::new(total_files as u64);
        if let Ok(style) = ProgressStyle::default_bar()
            .template(PROGRESS_BAR_TEMPLATE)
            .map(|s| s.progress_chars("#>-"))
        {
            pb.set_style(style);
        }
        Some(pb)
    } else {
        None
    };

    // Create or open index storage
    let storage = if force || !index_path.exists() {
        IndexStorage::create(index_path)?
    } else {
        IndexStorage::open(index_path)?
    };

    let extractor = SymbolExtractor::new();

    let mut indexed_count = 0;
    let mut symbol_count = 0;

    for file_path in &source_files {
        if let Some(ref pb) = progress {
            pb.inc(1);
        }

        // Extract symbols from file
        match extractor.extract_from_file(file_path) {
            Ok(symbols) => {
                let file_symbol_count = symbols.len();
                for symbol in symbols {
                    if let Err(e) = storage.insert_symbol(&symbol) {
                        let _ = writeln!(stderr.lock(), "Warning: Failed to index symbol: {}", e);
                    }
                }
                indexed_count += 1;
                symbol_count += file_symbol_count;
            }
            Err(e) => {
                let _ = writeln!(
                    stderr.lock(),
                    "Warning: Failed to parse {}: {}",
                    file_path.display(),
                    e
                );
            }
        }
    }

    if let Some(pb) = progress {
        pb.finish_and_clear();
    }

    Ok((indexed_count, symbol_count))
}
