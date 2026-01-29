//! STORY-007: Daemon Core Architecture with IPC Communication - Integration Test Entry Point
//!
//! This file serves as the entry point for running all STORY-007 acceptance
//! criteria tests. Each AC is implemented in a separate module under
//! tests/STORY-007/.
//!
//! # Running Tests
//!
//! ```bash
//! # Run all STORY-007 tests
//! cargo test --test story_007
//!
//! # Run specific AC tests
//! cargo test --test story_007 ac1  # Daemon process lifecycle
//! cargo test --test story_007 ac2  # IPC socket/pipe creation
//! cargo test --test story_007 ac3  # NDJSON message protocol
//! cargo test --test story_007 ac4  # Search method handler
//! cargo test --test story_007 ac5  # Status method handler
//! cargo test --test story_007 ac6  # Graceful shutdown
//! cargo test --test story_007 ac7  # Error response format
//! ```
//!
//! # Test Organization
//!
//! - **test_ac1_daemon_lifecycle.rs**: Daemon starts, enters ready state within 2 seconds
//! - **test_ac2_ipc_socket_creation.rs**: Unix socket at .treelint/daemon.sock, Windows named pipe
//! - **test_ac3_ndjson_protocol.rs**: Parse requests, serialize responses with id/result/error
//! - **test_ac4_search_handler.rs**: Search via daemon returns standard format
//! - **test_ac5_status_handler.rs**: Returns 7 status fields
//! - **test_ac6_graceful_shutdown.rs**: Signal handling, cleanup
//! - **test_ac7_error_response.rs**: E001, E002, E003 codes

#[path = "STORY-007/mod.rs"]
mod story_007_tests;

// Re-export all test modules for test discovery
pub use story_007_tests::*;
