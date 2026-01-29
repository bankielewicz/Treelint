//! Daemon Module
//!
//! This module provides the background daemon service that maintains the symbol
//! index and handles IPC requests from clients.
//!
//! ## Architecture
//!
//! The daemon uses platform-specific IPC:
//! - Unix/macOS: Unix domain socket at `.treelint/daemon.sock`
//! - Windows: Named pipe at `\\.\pipe\treelint-daemon`
//!
//! ## Protocol
//!
//! NDJSON (Newline-Delimited JSON) protocol:
//! - Request: `{"id": "...", "method": "...", "params": {...}}`
//! - Response: `{"id": "...", "result": {...}, "error": null}` or
//!   `{"id": "...", "result": null, "error": {"code": "...", "message": "..."}}`
//!
//! ## Methods
//!
//! - `search`: Search for symbols in the index
//! - `status`: Get daemon status information
//! - `index`: Trigger re-indexing
//!
//! ## Error Codes
//!
//! - `E001`: Index not ready
//! - `E002`: Invalid method
//! - `E003`: Invalid parameters

pub mod protocol;
pub mod server;

pub use protocol::{DaemonRequest, DaemonResponse, ErrorInfo, ProtocolHandler};
pub use server::{DaemonClient, DaemonError, DaemonServer, DaemonState};
