# ADR-002: SHA-256 Hashing with sha2 Crate

## Status

**Accepted** - 2026-01-30

## Context

STORY-008 (File Watcher and Incremental Index Updates) requires hash-based change detection to determine if file content has actually changed when modification events are received. This prevents unnecessary re-indexing when files are "touched" without actual content changes.

The initial implementation used `std::collections::hash_map::DefaultHasher` (SipHash) combined with multiple seeds to produce a 64-character hex string. However, code review identified this as a critical issue:

1. **Misleading documentation**: Function was named `sha256_hash` but didn't use SHA-256
2. **Not cryptographically secure**: DefaultHasher is optimized for hash tables, not content verification
3. **Higher collision risk**: SipHash has higher collision probability than SHA-256 for file content

## Decision

Add the `sha2` crate (version 0.10) to provide proper SHA-256 hashing for file change detection.

### Rationale

1. **Correctness**: SHA-256 is the standard for content-based change detection
2. **Reliability**: Cryptographically secure with negligible collision probability
3. **Performance**: sha2 crate is highly optimized, comparable to OpenSSL
4. **Maintenance**: Well-maintained by the RustCrypto project
5. **No external dependencies**: Pure Rust implementation, no system libraries required

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| Rename to `content_hash` | Still higher collision risk, technical debt |
| Use `ring` crate | Heavier, includes more than needed |
| Use `md5` crate | MD5 is deprecated for security |
| Keep DefaultHasher | Not cryptographically sound for change detection |

## Consequences

### Positive

- Accurate SHA-256 hashing as documented
- Zero collision probability for practical file sizes
- Reduced technical debt
- Documentation matches implementation

### Negative

- Additional ~200KB to binary size (acceptable)
- Slightly slower than SipHash (negligible for file I/O)

### Neutral

- No API changes (same 64-char hex output)
- All existing tests pass

## Implementation

```toml
# Cargo.toml
sha2 = "0.10"
```

```rust
// src/daemon/watcher.rs
use sha2::{Digest, Sha256};

pub fn sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}
```

## References

- STORY-008: File Watcher and Incremental Index Updates
- Code Review: SHA-256 Implementation Not Cryptographically Secure
- RustCrypto sha2 crate: https://crates.io/crates/sha2
