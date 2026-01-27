---
id: BRAINSTORM-001
title: "Treelint: AST-Aware Code Search CLI for AI Token Efficiency"
status: complete
created: 2026-01-27
confidence: HIGH
stakeholder_count: 4
opportunity_count: 6
constraint_count: 5
hypothesis_count: 4
related_research: [RESEARCH-005]
tags: [tree-sitter, AST, token-efficiency, code-search, rust, cli]
---

# BRAINSTORM-001: Treelint - AST-Aware Code Search CLI

## Key Files for Context

| File | Purpose |
|------|---------|
| devforgeai/specs/research/RESEARCH-001-claude-code-search-token-efficiency.research.md | Market research validating 40-83% token reduction opportunity |
| devforgeai/specs/context/tech-stack.md | Technology constraints for this project |
| .claude/agents/internet-sleuth.md | Research subagent used for competitive analysis |

## Glossary

- **AST**: Abstract Syntax Tree - structured representation of code parsed by tree-sitter
- **tree-sitter**: Incremental parsing system that generates ASTs for 100+ languages
- **Token**: Unit of text consumed by AI models; search results consume input tokens
- **Context window**: Maximum tokens an AI can process in one request (~128K-200K)
- **MCP**: Model Context Protocol - standard for AI tool integration
- **Repository map**: Summary of all symbols in a codebase (Aider's approach)
- **Symbol**: Named code construct (function, class, method, variable, etc.)

---

## Executive Summary

**Problem:** AI coding assistants waste 40-83% of context window on false positives from text-based code search (grep/ripgrep). Comments, strings, and variable names match keywords indiscriminately, consuming tokens without adding value.

**Solution:** Treelint - a Rust-based CLI that uses tree-sitter AST parsing to return semantic code units (functions, classes) instead of raw text lines. JSON output enables AI assistants to intelligently select what context to include.

**Value Proposition:** 40-80% token reduction, improved accuracy, faster responses.

**Timeline:** < 2 weeks (ASAP)

**Confidence:** HIGH (validated by RESEARCH-005 market data)

---

## Stakeholder Analysis

### Primary Stakeholders

| Stakeholder | Role | Goals | Concerns |
|-------------|------|-------|----------|
| AI coding assistants (Claude, GPT) | Primary User | Token efficiency, accuracy, speed | Structured output format compatibility |
| Individual developers | Decision Maker | Personal productivity gains | Easy installation, no friction |

### Secondary Stakeholders

| Stakeholder | Role | Goals | Concerns |
|-------------|------|-------|----------|
| Open-source AI tooling ecosystem | Beneficiary | Ecosystem enhancement | Interoperability standards |
| Tree-sitter community | Enabler | Grammar adoption | Language coverage |

### Stakeholder Conflicts

None identified - all stakeholders benefit from improved code search efficiency.

---

## Problem Statement

### The Problem

> AI coding assistants experience **excessive token consumption** when searching code because text-based tools (ripgrep/grep) return **false positives from comments and strings**, resulting in **40-83% wasted context window** and degraded response quality.

### Root Cause Analysis (5 Whys)

1. **Why** do AI assistants waste tokens on code search?
   → Text-based search returns excessive false positives

2. **Why** does text search return false positives?
   → Comments and strings contain code-like text that matches keywords

3. **Why** is this especially problematic for AI?
   → All false positives waste tokens + confuse reasoning = wrong answers

4. **Why** hasn't this been solved already?
   → Existing tools (Aider, Cursor) use AST but for internal use or proprietary
   → No purpose-built CLI optimized for AI consumption exists

5. **Why** is a CLI the right solution?
   → Works with any AI tool via Bash/subprocess, no vendor lock-in

### Current State

- Claude Code uses ripgrep (fast but text-based)
- 20-30% of context window consumed by search results
- Competing tools achieve 4-18% context utilization with AST

### Pain Points

| Pain Point | Business Impact |
|------------|-----------------|
| False positives (comments/strings match) | Wasted tokens, wrong context |
| No semantic understanding | AI reasons about irrelevant code |
| Line-based results | No function/class boundaries |
| No relevance scoring | All matches treated equally |

### Failed Solutions

- **Ripgrep filtering:** Faster but same false positive problem
- **Tree-sitter MCP server:** Generic, not optimized for AI output format
- **Manual context curation:** Not scalable

---

## Opportunities

### Vision Statement

> If Treelint works perfectly, AI coding assistants will fit **2-4x more code in context**, achieve **higher accuracy** from reduced noise, and respond **faster** with less data to process.

### Opportunity Inventory

| ID | Opportunity | Impact | Effort | Phase |
|----|------------|--------|--------|-------|
| O1 | Symbol-based code search | HIGH | MEDIUM | v1 MVP |
| O2 | JSON structured output | HIGH | LOW | v1 MVP |
| O3 | Configurable context lines | HIGH | LOW | v1 MVP |
| O4 | Background indexing service | HIGH | HIGH | v1 MVP |
| O5 | Repository map generation | HIGH | MEDIUM | v1 MVP |
| O6 | Dependency/call graph | MEDIUM | HIGH | v1 MVP |

### Technology Approach

- **Language:** Rust (single binary, cross-platform, high performance)
- **Parser:** tree-sitter with embedded grammars
- **Output:** JSON with symbol metadata
- **Interface:** CLI with hybrid flags (`--function foo` OR `foo --type function`)

---

## Constraints

### Hard Constraints

| Category | Constraint | Rationale |
|----------|-----------|-----------|
| Timeline | < 2 weeks | Business need for DevForgeAI integration |
| Binary | Single binary, no dependencies | Easy installation for developers |
| Platforms | Windows, macOS, Linux | Cross-platform requirement |
| Grammars | Embedded in binary | No external files to manage |

### Soft Constraints

| Category | Constraint | Flexibility |
|----------|-----------|-------------|
| Languages | Python, TypeScript, Rust, Markdown | Could add more post-MVP |
| Resources | Solo developer | Community contributions post-launch |

### Technical Constraints

- Must work without internet connection (offline-first)
- Binary size should be < 50MB with embedded grammars
- Query latency < 100ms for typical searches

---

## Hypotheses

### Critical Hypotheses to Validate

| ID | Hypothesis | Success Criteria | Validation Method | Risk if Wrong |
|----|-----------|------------------|-------------------|---------------|
| H1 | Tree-sitter grammars can embed in Rust binary | Compile succeeds, binary < 50MB | Build test | Need external grammar files |
| H2 | JSON output reduces tokens by 40%+ vs grep | Measured in Claude Code sessions | A/B comparison | Value prop doesn't hold |
| H3 | Claude Code can parse and use JSON output | Works via Bash tool | Integration test | Need MCP server |
| H4 | 4 languages achievable in 2 weeks | MVP with Py/TS/Rust/MD | Development sprint | Scope reduction |

### Assumptions

1. Tree-sitter Rust bindings are stable and well-documented
2. Embedded grammars won't cause licensing issues
3. JSON parsing overhead is negligible for AI models
4. CLI invocation via Bash is fast enough (<200ms)

---

## Prioritization

### MoSCoW Classification

| Priority | Features |
|----------|----------|
| **Must Have** | Symbol search, JSON output, context lines, indexing, repo map, dep graph |
| **Should Have** | (none - all promoted to Must Have) |
| **Could Have** | Natural language queries, fuzzy matching |
| **Won't Have (v1)** | GUI, IDE plugins, web interface |

### Impact-Effort Matrix

```
HIGH IMPACT
    │
    │  ┌─────────────────┐  ┌─────────────────┐
    │  │ Symbol Search   │  │ Background      │
    │  │ JSON Output     │  │ Indexing        │
    │  │ Context Lines   │  │ Repo Map        │
    │  │ (Quick Wins)    │  │ (Major Projects)│
    │  └─────────────────┘  └─────────────────┘
    │
    ├──────────────────────────────────────────────
    │
    │  ┌─────────────────┐  ┌─────────────────┐
    │  │ (Fill-Ins)      │  │ Dep Graph       │
    │  │                 │  │ (Plan Carefully)│
    │  └─────────────────┘  └─────────────────┘
    │
LOW IMPACT
    └──────────────────────────────────────────────
         LOW EFFORT                    HIGH EFFORT
```

### Recommended Sequence

1. **Week 1 - Track A (Core CLI):**
   - Symbol-based search (function, class, method)
   - JSON structured output
   - Context lines configuration

2. **Week 1-2 - Track B (Parallel):**
   - Background indexing daemon
   - File watcher for incremental updates

3. **Week 2:**
   - Repository map generation
   - Dependency graph extraction

---

## Output Format Specification

### JSON Schema (Recommended)

```json
{
  "query": {
    "symbol": "validateUser",
    "type": "function",
    "context_lines": 5
  },
  "results": [
    {
      "type": "function",
      "name": "validateUser",
      "file": "src/auth/validator.py",
      "lines": [10, 45],
      "signature": "def validateUser(email: str, password: str) -> bool",
      "body": "...",
      "references": 12,
      "complexity": 8
    }
  ],
  "stats": {
    "files_searched": 150,
    "symbols_matched": 3,
    "elapsed_ms": 47
  }
}
```

### Query Interface

```bash
# Primary style (keyword + type filter)
treelint validateUser --type function

# Shorthand style (convenience flags)
treelint --function validateUser
treelint --class AuthService
treelint --method login

# With context
treelint --function validateUser --context 10

# Output control
treelint --function validateUser --format json
treelint --function validateUser --format signature-only
```

---

## Indexing Architecture (Hybrid Mode)

### Design Decision: Daemon + On-Demand Fallback

The indexing system uses a **hybrid approach** that auto-detects the best mode:

```
┌─────────────────────────────────────────────────────────┐
│                    treelint CLI                         │
├─────────────────────────────────────────────────────────┤
│  treelint search validateUser --type function           │
│     ↓                                                   │
│  [Check: Is daemon running?]                            │
│     ├─ YES → Query daemon's live index (fastest)        │
│     └─ NO  → Build index on-demand (still fast)         │
└─────────────────────────────────────────────────────────┘
```

### Environment-Based Mode Selection

| Environment | Recommended Mode | Rationale |
|-------------|------------------|-----------|
| Dev machine | Daemon | Always-fresh index, hardware handles it easily |
| CI/CD | Manual | No daemon needed, index once before tests |
| Containers | Manual | Short-lived, daemon overhead unnecessary |
| Claude Code session | Either | Daemon if available, else manual trigger |

### CLI Commands

```bash
# Search (works with or without daemon)
treelint search validateUser --type function

# Force manual re-index (CI/CD or daemon not running)
treelint index --force

# Daemon management
treelint daemon start
treelint daemon stop
treelint daemon status
```

### Daemon Architecture

```
┌─────────────────────────────────────────────────────────┐
│               treelint daemon                           │
├─────────────────────────────────────────────────────────┤
│  treelint daemon start                                  │
│     • Watch file changes (notify/inotify)               │
│     • Incremental re-index modified files only          │
│     • Serve queries via Unix socket / TCP               │
│                                                         │
│  Resource Usage (typical):                              │
│     • CPU: ~0.1% idle (file watcher)                    │
│     • RAM: ~100-500MB (index cache)                     │
│     • Disk: Incremental writes only                     │
└─────────────────────────────────────────────────────────┘
```

### Key Benefits

1. **Same command works both ways** - CLI auto-detects daemon availability
2. **Zero configuration** - Defaults work everywhere
3. **Optimal for each environment** - Daemon for interactive, manual for automation
4. **Trivial resource overhead** - Modern hardware handles daemon easily

---

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| Tree-sitter embedding fails | LOW | HIGH | Fallback to external grammar files |
| Binary size too large | MEDIUM | MEDIUM | Lazy-load grammars, compress |
| 2-week timeline not achievable | MEDIUM | MEDIUM | Deprioritize dep graph to v1.1 |
| JSON overhead significant | LOW | LOW | Benchmark early, optimize if needed |

---

## Success Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Token reduction | ≥ 40% | Compare grep vs treelint output size |
| Query latency | < 100ms | Benchmark on 10K file codebase |
| Accuracy (precision) | ≥ 95% | Manual review of search results |
| Binary size | < 50MB | Build artifact measurement |
| Platform coverage | 3/3 | CI matrix (Win/Mac/Linux) |

---

## Next Steps

1. **Run `/ideate`** to transform this brainstorm into formal requirements
2. **Validate H1** - Spike on tree-sitter grammar embedding in Rust
3. **Run `/create-context`** to establish architectural constraints
4. **Create Epic** with 2-week sprint structure

---

## Session Metadata

| Attribute | Value |
|-----------|-------|
| Session ID | BRAINSTORM-001 |
| Duration | ~30 minutes |
| Questions Asked | 18 |
| Confidence Level | HIGH |
| Research Used | RESEARCH-005 |

---

## Change Log

| Date | Change |
|------|--------|
| 2026-01-27 | Initial brainstorm session completed |
| 2026-01-27 | Added Hybrid Indexing Architecture section (daemon + on-demand fallback) |
