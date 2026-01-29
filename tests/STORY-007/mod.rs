//! STORY-007: Daemon Core Architecture with IPC Communication Tests
//!
//! This module contains test suites for verifying the daemon server
//! functionality, including lifecycle management, IPC communication,
//! NDJSON protocol, method handlers, and error handling.
//!
//! # Test Organization
//!
//! - AC#1: Daemon Process Lifecycle
//! - AC#2: IPC Socket/Pipe Creation
//! - AC#3: NDJSON Message Protocol
//! - AC#4: Search Method Handler
//! - AC#5: Status Method Handler
//! - AC#6: Graceful Shutdown
//! - AC#7: Error Response Format
//!
//! # Dependencies
//!
//! - interprocess for cross-platform IPC
//! - serde_json for NDJSON parsing
//! - tempfile for isolated test directories

pub mod test_ac1_daemon_lifecycle;
pub mod test_ac2_ipc_socket_creation;
pub mod test_ac3_ndjson_protocol;
pub mod test_ac4_search_handler;
pub mod test_ac5_status_handler;
pub mod test_ac6_graceful_shutdown;
pub mod test_ac7_error_response;
