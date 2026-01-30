# Treelint Troubleshooting Guide

Solutions for common issues when using or developing Treelint.

---

## Quick Diagnostics

```bash
# Check version
treelint --version

# Check Rust version
rustc --version

# Verify build
cargo check

# Run tests
cargo test

# Check for clippy warnings
cargo clippy
```

---

## Installation Issues

### "Command not found: treelint"

**Problem:** Terminal cannot find the `treelint` command.

**Solutions:**

1. **Use full path:**
   ```bash
   ./target/release/treelint --version
   ```

2. **Add to PATH:**
   ```bash
   # Linux/macOS
   export PATH="$PATH:$(pwd)/target/release"

   # Or copy to bin directory
   cp target/release/treelint ~/.local/bin/
   ```

3. **Windows:**
   ```powershell
   # Add to PATH for current session
   $env:Path += ";$(Get-Location)\target\release"
   ```

---

### Build Failures

#### "error: requires Rust 1.70.0 or newer"

**Problem:** Rust version is too old.

**Solution:**
```bash
# Update Rust
rustup update stable

# Verify version
rustc --version
# Should be 1.70.0 or higher
```

#### "error: could not compile `tree-sitter`"

**Problem:** tree-sitter compilation failed.

**Solutions:**

1. **Ensure C compiler is installed:**
   ```bash
   # Ubuntu/Debian
   sudo apt install build-essential

   # macOS
   xcode-select --install

   # Windows
   # Install Visual Studio Build Tools
   ```

2. **Clean and rebuild:**
   ```bash
   cargo clean
   cargo build --release
   ```

#### "error: failed to download `rusqlite`"

**Problem:** Network error or Cargo cache issue.

**Solution:**
```bash
# Clear Cargo cache
rm -rf ~/.cargo/registry/cache

# Retry build
cargo build
```

---

## Runtime Issues

### "No results found"

**Problem:** Search returns no results when you expect matches.

**Possible causes:**

1. **File not indexed:**
   ```bash
   # Force re-index
   rm -rf .treelint/
   treelint search main
   ```

2. **Wrong file extension:**
   - Only `.py`, `.ts`, `.tsx`, `.js`, `.jsx`, `.rs`, `.md` are supported

3. **Case sensitivity:**
   ```bash
   # Use case-insensitive search
   treelint search main -i
   ```

4. **Symbol type mismatch:**
   ```bash
   # Remove type filter to see all matches
   treelint search main
   # vs
   treelint search main --type function
   ```

---

### Index Errors

#### "Database corrupted"

**Problem:** SQLite index file is corrupted.

**Solution:**
```bash
# Delete and rebuild index
rm -rf .treelint/
treelint search main
```

#### "Permission denied"

**Problem:** Cannot read/write index file.

**Solutions:**

1. **Check directory permissions:**
   ```bash
   ls -la .treelint/
   ```

2. **Fix permissions:**
   ```bash
   chmod 755 .treelint/
   chmod 644 .treelint/index.db
   ```

3. **Check if file is locked:**
   ```bash
   # Another process may be using the database
   lsof .treelint/index.db
   ```

#### "Disk full"

**Problem:** No space for index database.

**Solution:**
```bash
# Check disk space
df -h .

# Free up space or use different location
```

---

## Daemon Issues

### "Socket already in use"

**Problem:** Previous daemon didn't shut down cleanly.

**Solution:**
```bash
# Remove stale socket
rm .treelint/daemon.sock

# Kill any orphan processes
pkill -f "treelint.*daemon"
```

### "Connection refused"

**Problem:** Daemon is not running.

**Solutions:**

1. **Check if daemon is running:**
   ```bash
   ps aux | grep treelint
   ```

2. **Check socket exists:**
   ```bash
   ls -la .treelint/daemon.sock
   ```

3. **Restart daemon:**
   ```bash
   # Kill existing
   pkill -f "treelint.*daemon"

   # Start fresh
   treelint daemon start
   ```

### "E001: Index not ready"

**Problem:** Daemon is still building the index.

**Solution:**
```bash
# Wait and retry
# Check status
echo '{"id":"1","method":"status","params":{}}' | nc -U .treelint/daemon.sock

# Wait for status: "ready"
```

---

## File Watcher Issues

### "Too many watches"

**Problem:** System limit for file watches exceeded (Linux).

**Solution:**
```bash
# Check current limit
cat /proc/sys/fs/inotify/max_user_watches

# Increase limit (temporary)
sudo sysctl fs.inotify.max_user_watches=524288

# Increase limit (permanent)
echo "fs.inotify.max_user_watches=524288" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

### "Watcher not detecting changes"

**Problem:** File changes not triggering re-indexing.

**Possible causes:**

1. **File type not supported:**
   - Only `.py`, `.ts`, `.tsx`, `.rs`, `.md` are watched

2. **File in .gitignore:**
   - Ignored files are not watched

3. **File in .treelint/:**
   - The index directory is always ignored

4. **Debounce delay:**
   - Wait 100ms after saving before searching

**Diagnostics:**
```bash
# Check daemon status
echo '{"id":"1","method":"status","params":{}}' | nc -U .treelint/daemon.sock

# Look for watcher.events_processed
```

---

## Output Issues

### JSON not pretty-printed

**Problem:** JSON output is hard to read.

**Solution:**
```bash
# Pipe through jq
treelint search main --format json | jq .
```

### Colors not showing in terminal

**Problem:** Text output lacks colors.

**Possible causes:**

1. **Piped output:**
   - Colors are disabled when output is piped
   - Use `--format text` explicitly if needed

2. **Terminal doesn't support colors:**
   ```bash
   # Check TERM
   echo $TERM
   ```

3. **Windows CMD:**
   - Use Windows Terminal or PowerShell for color support

---

## Performance Issues

### "Search takes too long"

**Problem:** Queries taking more than expected.

**Solutions:**

1. **Use the daemon:**
   ```bash
   # Start daemon
   treelint daemon start

   # Queries via daemon are <5ms vs 20-60s on-demand
   ```

2. **Check index size:**
   ```bash
   ls -lh .treelint/index.db
   ```

3. **Rebuild index if very large:**
   ```bash
   rm -rf .treelint/
   treelint search main
   ```

### "High CPU usage"

**Problem:** Daemon using too much CPU.

**Possible causes:**

1. **Initial indexing:**
   - Normal during first index build
   - Wait for completion

2. **Too many file changes:**
   - Rapid saves triggering many re-indexes
   - Debouncing should mitigate this

3. **Large files:**
   - Files >10K lines may take longer to parse

---

## Development Issues

### Test Failures

#### "Thread panicked"

**Problem:** Test panic due to race condition.

**Solution:**
```bash
# Run single-threaded
cargo test -- --test-threads=1
```

#### "Cannot find test fixture"

**Problem:** Test can't locate fixture file.

**Solution:**
```bash
# Run from project root
cd /path/to/treelint
cargo test
```

### Clippy Warnings

#### "warning: this could be simplified"

**Problem:** Clippy has suggestions.

**Solution:**
```bash
# View and fix
cargo clippy --fix

# Or manually fix following clippy suggestions
cargo clippy -- -D warnings
```

### Format Errors

#### "Diff in formatting"

**Problem:** Code doesn't match rustfmt style.

**Solution:**
```bash
# Apply formatting
cargo fmt

# Verify
cargo fmt --check
```

---

## Error Reference

### TreelintError

| Error | Cause | Solution |
|-------|-------|----------|
| `Io` | File system error | Check file exists and permissions |
| `Parse` | Invalid source code | Check file is valid syntax |
| `Cli` | Invalid arguments | Check `--help` for valid options |
| `DaemonError` | IPC failure | Restart daemon |

### StorageError

| Error | Cause | Solution |
|-------|-------|----------|
| `DatabaseCorrupted` | SQLite file corrupted | Delete `.treelint/` and rebuild |
| `PermissionDenied` | Cannot access file | Fix file/directory permissions |
| `DiskFull` | No disk space | Free up disk space |
| `ConnectionFailed` | Cannot open database | Check if file is locked |
| `QueryFailed` | Invalid SQL query | Report as bug |

### WatcherError

| Error | Cause | Solution |
|-------|-------|----------|
| `PermissionDenied` | Cannot access path | Check directory permissions |
| `PathNotFound` | Directory doesn't exist | Verify project path |
| `TooManyWatches` | inotify limit reached | Increase system limit |
| `IoError` | General I/O failure | Check system logs |

### Daemon Error Codes

| Code | Name | Cause | Solution |
|------|------|-------|----------|
| `E001` | Index Not Ready | Still indexing | Wait and retry |
| `E002` | Invalid Method | Unknown RPC method | Check method name |
| `E003` | Invalid Params | Bad request format | Check request JSON |

---

## Getting Help

### Debug Logging

```bash
# Enable debug output
RUST_LOG=debug treelint search main

# Enable trace output (very verbose)
RUST_LOG=trace treelint search main
```

### Reporting Issues

When reporting bugs, include:

1. **Version information:**
   ```bash
   treelint --version
   rustc --version
   uname -a  # or winver on Windows
   ```

2. **Error message:** Full error text

3. **Steps to reproduce:** Minimal example

4. **Expected vs actual behavior**

5. **Relevant files:** (if not sensitive)

### Resources

- [GitHub Issues](https://github.com/treelint/treelint/issues)
- [Developer Guide](developer-guide.md)
- [API Reference](../api/cli-reference.md)

---

**Version:** 0.8.0
**Generated:** 2026-01-30
**Source:** error.rs, storage.rs, watcher.rs, common user issues
