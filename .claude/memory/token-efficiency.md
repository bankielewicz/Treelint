# DevForgeAI Token Efficiency Guide

Strategies and best practices for optimizing token usage in Claude Code Terminal.

---

## Token Efficiency Targets

### Per-Operation Targets

- **Light QA validation:** ~10,000 tokens
- **Deep QA validation:** ~65,000 tokens
- **Feature implementation:** ~80,000 tokens
- **Total per story (dev + QA):** ~155,000 tokens
- **UI component generation:** ~35,000 tokens

---

## Native Tools vs Bash: Token Savings

### CRITICAL: Use Native Tools for File Operations

**Native tools provide 40-73% token savings compared to Bash commands.**

### Command Translation

| Operation | ❌ Bash (Inefficient) | ✅ Native Tool (Efficient) | Savings |
|-----------|---------------------|---------------------------|---------|
| Read file | `cat file.py` | `Read(file_path="file.py")` | 40% |
| Search code | `grep -r "pattern"` | `Grep(pattern="pattern")` | 60% |
| Find files | `find . -name "*.ts"` | `Glob(pattern="**/*.ts")` | 73% |
| Edit file | `sed -i 's/old/new/' file` | `Edit(old_string="old", new_string="new")` | 75% |
| Create file | `echo "content" > file` | `Write(file_path="file", content="content")` | 77% |

### Correct Usage

✅ **ALWAYS USE:**
- `Read(file_path="...")` - NOT `cat`, `head`, `tail`
- `Edit(...)` - NOT `sed`, `awk`, `perl`
- `Write(...)` - NOT `echo >`, `cat <<EOF`
- `Glob(pattern="...")` - NOT `find`, `ls -R`
- `Grep(pattern="...")` - NOT `grep` command

❌ **FORBIDDEN:**
- Bash for file reading/editing/searching
- **ONLY use Bash for:** git, npm, pytest, docker, make (terminal operations)

---

## Why Native Tools Are More Efficient

### 1. Direct File System Access
- **Bash:** Claude → Bash Tool → Shell Process → File System → Parse Output → Claude
- **Native:** Claude → Native Tool → File System → Structured Data → Claude
- **Result:** 10-50x faster execution + 40-73% token reduction

### 2. Optimized Output Format
- **Bash `cat`:** Raw text, no line numbers, requires parsing
- **Read tool:** Pre-numbered lines (`cat -n` format), structured, AI-optimized
- **Savings:** ~1-2 tokens per line × 200 lines = 200-400 tokens per file

### 3. No Shell Overhead
- **Bash:** Shell prompts, ANSI codes, formatting metadata
- **Native:** Clean structured JSON responses
- **Savings:** ~30-40% overhead eliminated

### 4. Better Error Handling
- **Bash:** Exit codes + stderr (requires parsing)
- **Native:** Structured errors with suggestions
- **Result:** Faster recovery, fewer retry tokens

---

## Efficiency Strategies

### 1. Progressive Disclosure

**Pattern:** Load only necessary context, expand as needed

```
# Don't load everything at once
❌ Read all 6 context files immediately

# Load progressively
✅ Read context files only when referenced by workflow phase
```

### 2. Parallel Tool Invocations

**Pattern:** Execute independent operations simultaneously

```
# Multiple tool calls in single message
Read(file_path="src/app.py")
Read(file_path="tests/test_app.py")
Grep(pattern="class.*Test", type="py")
Glob(pattern="src/**/*.py")
```

**Benefit:** 4x faster execution, same token cost

### 3. Focused Validation

**Pattern:** Don't re-validate passing components

```
# During refactor phase
- Run tests (already know they pass)
- Skip full coverage analysis (did in Green phase)
- Focus on code quality metrics only
```

**Savings:** ~50% reduction in validation tokens

### 4. Grep Output Modes

**Pattern:** Use minimal output mode for discovery

```
# Discovery phase (minimal tokens)
Grep(pattern="TODO", output_mode="files_with_matches")
# Returns: List of files (~500 tokens)

# Then read selectively
Read(file_path="high_priority_file.py")
# Returns: File content (~1,500 tokens)
```

**vs. Loading full context:**
```
# Inefficient
Grep(pattern="TODO", output_mode="content")
# Returns: Full file contents (~10,000 tokens)
```

**Savings:** 85% token reduction for discovery phase

---

## Session-Level Token Budgets

### Typical QA Review Workflow

| Operation Type | Count | Native Tools | Bash Equivalent | Savings |
|----------------|-------|--------------|-----------------|---------|
| File reads | 50 | ~45,000 | ~75,000 | 30,000 |
| Code searches | 20 | ~32,000 | ~80,000 | 48,000 |
| File edits | 10 | ~4,500 | ~18,000 | 13,500 |
| Pattern matches | 30 | ~24,000 | ~90,000 | 66,000 |
| File creation | 5 | ~2,500 | ~11,000 | 8,500 |
| **TOTAL** | **115** | **~108,000** | **~274,000** | **~166,000 (61%)** |

**Context Window Impact:**
- Claude Code context budget: 200,000 tokens
- With Bash: 274,000 tokens (overflow! ❌)
- With Native: 108,000 tokens (comfortable ✅)

---

## Subagent Context Isolation

**Token Efficiency Pattern:**
- Each subagent operates in isolated context
- Main conversation only pays invocation cost (~5-10K) + summary
- Total workflow can exceed 200K tokens without affecting main context

**Example:**
```
Full dev cycle:
- test-automator: 50K (isolated)
- backend-architect: 50K (isolated)
- code-reviewer: 30K (isolated)
- integration-tester: 40K (isolated)
Total: 170K in subagents

Main conversation sees: ~15K (summaries only)
```

**Efficiency Gain:** 10x effective context capacity

---

## Best Practices Summary

### Golden Rules

1. **🥇 Native Tools for Files:** Read, Edit, Write, Glob, Grep (40-73% savings)
2. **🥈 Bash for Terminal:** git, npm, pytest, docker only
3. **🥉 Progressive Disclosure:** Load context incrementally
4. **🏆 Batch Operations:** Parallel tool calls in single message
5. **🎯 Focused Validation:** Don't re-validate passing components

### Token Budget Management

**Monitor usage:**
- Track tokens per workflow phase
- Target: <100,000 tokens for complex workflows
- Alert if approaching 150,000 tokens
- Optimize if exceeding targets

**Optimization checklist:**
- [ ] Using native tools for all file operations
- [ ] Batching independent operations
- [ ] Progressive disclosure (not loading everything upfront)
- [ ] Appropriate Grep output modes (files vs content)
- [ ] Minimal old_string in Edit operations
- [ ] Subagent isolation for heavy work

---

## Evidence Base

All token savings measurements backed by:
- `devforgeai/specs/native-tools-vs-bash-efficiency-analysis.md` (1,730 lines of research)
- Production session analysis (Story 1.4 QA Review)
- Official Anthropic system prompt specifications
- Real-world benchmarks across 115+ file operations

**This is not theoretical - these are measured production results.**

---

## Remember

Token efficiency is not optional in DevForgeAI—it's mandatory for staying within context windows and enabling complex workflows. Always use native tools for file operations.
