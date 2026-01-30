//! Deps command implementation
//!
//! This module implements the `treelint deps` command which analyzes
//! and displays dependency graphs (call relationships and imports).

use crate::cli::args::{DepsArgs, GraphFormat};
use crate::graph::{CallGraph, CallGraphExtractor, ImportGraph, ImportGraphExtractor};
use crate::index::IndexStorage;
use crate::output::{format_call_graph_mermaid, format_import_graph_mermaid};

/// The index directory name
const INDEX_DIR: &str = ".treelint";
/// The index database filename
const INDEX_FILE: &str = "index.db";

/// Execute the deps command with the given arguments.
///
/// # Arguments
///
/// * `args` - The parsed deps command arguments
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the operation fails.
///
/// # Errors
///
/// Returns an error if:
/// - Neither --calls nor --imports is specified (BR-001)
/// - Index does not exist
/// - Database operations fail
pub fn execute(args: DepsArgs) -> anyhow::Result<()> {
    // BR-001: At least one of --calls or --imports must be specified
    if !args.calls && !args.imports {
        eprintln!("Error: Specify --calls or --imports (or both)");
        std::process::exit(1);
    }

    // Determine project root (current directory)
    let project_root = std::env::current_dir()?;
    let index_path = project_root.join(INDEX_DIR).join(INDEX_FILE);

    // Check if index exists
    if !index_path.exists() {
        eprintln!("Error: No index found. Run 'treelint index' first to build the index.");
        std::process::exit(1);
    }

    // Open the index
    let storage = IndexStorage::open(&index_path)?;

    // Determine output format (default to JSON)
    let format = args.format.unwrap_or(GraphFormat::Json);

    // Handle --calls flag
    if args.calls && !args.imports {
        let call_extractor = CallGraphExtractor::new();
        let mut call_graph = call_extractor.extract(&storage, &project_root)?;

        // Apply symbol filter if specified
        if let Some(ref symbol) = args.symbol {
            call_graph = call_graph.filter_by_symbol(symbol);
        }

        output_call_graph(&call_graph, format)?;
    }
    // Handle --imports flag
    else if args.imports && !args.calls {
        let import_extractor = ImportGraphExtractor::new();
        let mut import_graph = import_extractor.extract(&storage, &project_root)?;

        // Apply symbol filter if specified
        if let Some(ref symbol) = args.symbol {
            import_graph = import_graph.filter_by_symbol(symbol);
        }

        output_import_graph(&import_graph, format)?;
    }
    // Handle both --calls and --imports
    else {
        // Output both graphs
        let call_extractor = CallGraphExtractor::new();
        let mut call_graph = call_extractor.extract(&storage, &project_root)?;

        let import_extractor = ImportGraphExtractor::new();
        let mut import_graph = import_extractor.extract(&storage, &project_root)?;

        // Apply symbol filter if specified
        if let Some(ref symbol) = args.symbol {
            call_graph = call_graph.filter_by_symbol(symbol);
            import_graph = import_graph.filter_by_symbol(symbol);
        }

        // For combined output, we create a combined JSON structure
        match format {
            GraphFormat::Json => {
                let combined = serde_json::json!({
                    "calls": call_graph,
                    "imports": import_graph
                });
                println!("{}", serde_json::to_string_pretty(&combined)?);
            }
            GraphFormat::Mermaid => {
                // Output both diagrams with a separator
                println!("{}", format_call_graph_mermaid(&call_graph));
                println!();
                println!("{}", format_import_graph_mermaid(&import_graph));
            }
        }
    }

    Ok(())
}

/// Output a call graph in the specified format.
fn output_call_graph(graph: &CallGraph, format: GraphFormat) -> anyhow::Result<()> {
    match format {
        GraphFormat::Json => {
            let json = serde_json::to_string_pretty(graph)?;
            println!("{}", json);
        }
        GraphFormat::Mermaid => {
            println!("{}", format_call_graph_mermaid(graph));
        }
    }
    Ok(())
}

/// Output an import graph in the specified format.
fn output_import_graph(graph: &ImportGraph, format: GraphFormat) -> anyhow::Result<()> {
    match format {
        GraphFormat::Json => {
            let json = serde_json::to_string_pretty(graph)?;
            println!("{}", json);
        }
        GraphFormat::Mermaid => {
            println!("{}", format_import_graph_mermaid(graph));
        }
    }
    Ok(())
}
