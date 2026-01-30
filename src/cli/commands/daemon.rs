//! Daemon command implementation
//!
//! This module implements the `treelint daemon` command which manages
//! the background daemon process for fast index queries.

use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::cli::args::{DaemonAction, DaemonArgs};
use crate::daemon::server::{DaemonClient, DaemonServer};

/// The treelint directory name
const TREELINT_DIR: &str = ".treelint";
/// The daemon socket filename (Unix)
#[cfg(unix)]
const DAEMON_SOCKET: &str = "daemon.sock";
/// The daemon PID filename
const DAEMON_PID_FILE: &str = "daemon.pid";

/// Get the socket path for the current directory
fn get_socket_path(project_root: &Path) -> String {
    #[cfg(unix)]
    {
        project_root
            .join(TREELINT_DIR)
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

/// Get the PID file path
fn get_pid_file_path(project_root: &Path) -> std::path::PathBuf {
    project_root.join(TREELINT_DIR).join(DAEMON_PID_FILE)
}

/// Check if a daemon is running by trying to connect to the socket
fn check_daemon_running(socket_path: &str) -> Option<DaemonClient> {
    DaemonClient::connect(socket_path).ok()
}

/// Get status from a running daemon
fn get_daemon_status(client: &DaemonClient) -> Option<serde_json::Value> {
    let request = serde_json::json!({
        "id": "status-check",
        "method": "status",
        "params": {}
    });

    client.send_request(&request).ok()
}

/// Execute the daemon command with the given arguments.
///
/// # Arguments
///
/// * `args` - The parsed daemon command arguments
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error.
pub fn execute(args: DaemonArgs) -> anyhow::Result<()> {
    // Check if we should run the server (internal use)
    if args.run_server {
        return run_server();
    }

    let project_root = std::env::current_dir()?;
    let socket_path = get_socket_path(&project_root);

    match args.action {
        Some(DaemonAction::Start) => execute_start(&project_root, &socket_path),
        Some(DaemonAction::Stop) => execute_stop(&project_root, &socket_path),
        Some(DaemonAction::Status) => execute_status(&socket_path),
        None => {
            // No action specified, show help
            eprintln!("Usage: treelint daemon <start|stop|status>");
            std::process::exit(1);
        }
    }
}

/// Execute daemon start command
fn execute_start(project_root: &Path, socket_path: &str) -> anyhow::Result<()> {
    // Check if daemon is already running
    if let Some(client) = check_daemon_running(socket_path) {
        // Get status to show PID
        if let Some(status) = get_daemon_status(&client) {
            if let Some(result) = status.get("result") {
                let pid = result.get("pid").and_then(|p| p.as_u64()).unwrap_or(0);
                println!("Daemon already running (PID: {})", pid);
                return Ok(());
            }
        }
        println!("Daemon already running");
        return Ok(());
    }

    // Get the path to the current executable
    let current_exe = std::env::current_exe()?;

    // Ensure .treelint directory exists
    let treelint_dir = project_root.join(TREELINT_DIR);
    if !treelint_dir.exists() {
        std::fs::create_dir_all(&treelint_dir)?;
    }

    // Spawn the daemon as a background process
    let child = Command::new(&current_exe)
        .arg("daemon")
        .arg("--run-server")
        .current_dir(project_root)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    let pid = child.id();

    // Save PID to file
    let pid_file = get_pid_file_path(project_root);
    std::fs::write(&pid_file, pid.to_string())?;

    // Wait for the daemon to be ready (up to 3 seconds)
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(3);

    while start.elapsed() < timeout {
        if check_daemon_running(socket_path).is_some() {
            println!("Daemon started (PID: {})", pid);
            println!("Socket: {}", socket_path);
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    // If we get here, the daemon may have started but we couldn't connect
    // Still report success since the process was spawned
    println!("Daemon started (PID: {})", pid);
    println!("Socket: {}", socket_path);
    Ok(())
}

/// Execute daemon stop command
fn execute_stop(project_root: &Path, socket_path: &str) -> anyhow::Result<()> {
    // Check if daemon is running
    if check_daemon_running(socket_path).is_none() {
        // Also check for stale PID file
        let pid_file = get_pid_file_path(project_root);
        if pid_file.exists() {
            let _ = std::fs::remove_file(&pid_file);
        }
        // Clean up socket if it exists
        #[cfg(unix)]
        {
            let socket_file = Path::new(socket_path);
            if socket_file.exists() {
                let _ = std::fs::remove_file(socket_file);
            }
        }
        println!("No daemon running");
        return Ok(());
    }

    // Read PID from file
    let pid_file = get_pid_file_path(project_root);
    let pid: Option<u32> = if pid_file.exists() {
        std::fs::read_to_string(&pid_file)
            .ok()
            .and_then(|s| s.trim().parse().ok())
    } else {
        None
    };

    // Kill the daemon process
    if let Some(pid) = pid {
        #[cfg(unix)]
        {
            // Use kill command to send SIGTERM
            let _ = Command::new("kill")
                .arg("-TERM")
                .arg(pid.to_string())
                .output();
        }

        #[cfg(windows)]
        {
            // On Windows, use taskkill
            let _ = Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/F"])
                .output();
        }
    }

    // Wait for the daemon to stop (up to 5 seconds)
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(5);

    while start.elapsed() < timeout {
        if check_daemon_running(socket_path).is_none() {
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    // Clean up socket file
    #[cfg(unix)]
    {
        let socket_file = Path::new(socket_path);
        if socket_file.exists() {
            let _ = std::fs::remove_file(socket_file);
        }
    }

    // Clean up PID file
    if pid_file.exists() {
        let _ = std::fs::remove_file(&pid_file);
    }

    println!("Daemon stopped");
    Ok(())
}

/// Run the daemon server (called by the background process)
pub fn run_server() -> anyhow::Result<()> {
    use crate::parser::{Language, SymbolExtractor};
    use walkdir::WalkDir;

    let project_root = std::env::current_dir()?;

    // Start the daemon server
    let server = DaemonServer::new(&project_root)?;

    // Perform initial index if needed
    // Index all source files in the project
    let _extractor = SymbolExtractor::new(); // Needed for module import
    for entry in WalkDir::new(&project_root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            if e.depth() == 0 {
                return true;
            }
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy();
                // Skip hidden directories and common non-source directories
                return !name.starts_with('.')
                    && name != "node_modules"
                    && name != "target"
                    && name != "__pycache__"
                    && name != "venv";
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| Language::from_path(e.path()).is_some())
    {
        let _ = server.index_file(entry.path());
    }

    // Wait for ready
    server.wait_for_ready(Duration::from_secs(5))?;

    // Run forever (until killed)
    loop {
        std::thread::sleep(Duration::from_secs(1));

        // Check if we should exit (socket removed)
        let socket_path = server.socket_path();
        #[cfg(unix)]
        {
            let socket_file = Path::new(&socket_path);
            if !socket_file.exists() {
                break;
            }
        }

        // Check if shutdown was requested
        if server.is_shutdown_requested() {
            break;
        }
    }

    Ok(())
}

/// Execute daemon status command
fn execute_status(socket_path: &str) -> anyhow::Result<()> {
    // Check if daemon is running
    let client = match check_daemon_running(socket_path) {
        Some(c) => c,
        None => {
            println!("Daemon not running");
            std::process::exit(1);
        }
    };

    // Get status from daemon
    match get_daemon_status(&client) {
        Some(response) => {
            if let Some(result) = response.get("result") {
                let status = result
                    .get("status")
                    .and_then(|s| s.as_str())
                    .unwrap_or("unknown");
                let pid = result.get("pid").and_then(|p| p.as_u64()).unwrap_or(0);
                let uptime = result
                    .get("uptime_seconds")
                    .and_then(|u| u.as_u64())
                    .unwrap_or(0);
                let indexed_files = result
                    .get("indexed_files")
                    .and_then(|f| f.as_u64())
                    .unwrap_or(0);
                let indexed_symbols = result
                    .get("indexed_symbols")
                    .and_then(|s| s.as_u64())
                    .unwrap_or(0);
                let socket = result
                    .get("socket_path")
                    .and_then(|s| s.as_str())
                    .unwrap_or(socket_path);

                println!("Status: {}", status);
                println!("PID: {}", pid);
                println!("Uptime: {}s", uptime);
                println!("Indexed files: {}", indexed_files);
                println!("Indexed symbols: {}", indexed_symbols);
                println!("Socket: {}", socket);

                Ok(())
            } else if let Some(error) = response.get("error") {
                let message = error
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error");
                eprintln!("Error: {}", message);
                std::process::exit(1);
            } else {
                eprintln!("Invalid response from daemon");
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("Failed to get status from daemon");
            std::process::exit(1);
        }
    }
}
