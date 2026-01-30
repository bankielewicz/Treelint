//! Command implementations for Treelint CLI
//!
//! This module contains the implementation for each CLI subcommand.

pub mod daemon;
pub mod index;
pub mod search;

pub use daemon::execute as execute_daemon;
pub use index::execute as execute_index;
pub use search::execute as execute_search;
