//! STORY-009: Daemon CLI Commands and Auto-Detection - Integration Test Entry Point
//!
//! This file serves as the entry point for running all STORY-009 acceptance
//! criteria tests. Each AC is implemented in a separate module under
//! tests/STORY-009/.
//!
//! # Running Tests
//!
//! ```bash
//! # Run all STORY-009 tests
//! cargo test --test story_009
//!
//! # Run specific AC tests
//! cargo test --test story_009 ac1  # Daemon start command
//! cargo test --test story_009 ac2  # Daemon start when already running
//! cargo test --test story_009 ac3  # Daemon stop command
//! cargo test --test story_009 ac4  # Daemon stop when not running
//! cargo test --test story_009 ac5  # Daemon status command
//! cargo test --test story_009 ac6  # Index command
//! cargo test --test story_009 ac7  # Index --force flag
//! cargo test --test story_009 ac8  # Auto-detection
//! ```
//!
//! # Test Organization
//!
//! - **test_ac1_daemon_start.rs**: Daemon starts, returns PID and socket path
//! - **test_ac2_daemon_start_already_running.rs**: Idempotent start, shows "already running"
//! - **test_ac3_daemon_stop.rs**: Stop running daemon, waits up to 5 seconds
//! - **test_ac4_daemon_stop_not_running.rs**: Idempotent stop, shows "no daemon running"
//! - **test_ac5_daemon_status.rs**: Shows metrics when running, exit codes 0/1
//! - **test_ac6_index_command.rs**: Full index build with progress
//! - **test_ac7_index_force.rs**: Force flag bypasses hash cache
//! - **test_ac8_auto_detection.rs**: Daemon if available -> index.db -> on-demand build
//!
//! # TDD Phase: RED
//!
//! These tests are designed to FAIL initially because:
//! - The `daemon` subcommand does not exist yet in CLI args
//! - The `index` subcommand does not exist yet in CLI args
//! - Auto-detection logic is not implemented in search command
//!
//! After implementation (TDD Green phase), all tests should pass.

#[path = "STORY-009/mod.rs"]
mod story_009_tests;

// Re-export all test modules for test discovery
pub use story_009_tests::*;
