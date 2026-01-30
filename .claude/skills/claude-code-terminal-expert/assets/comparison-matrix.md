# Claude Code Terminal - Feature Comparison Matrix

**Purpose:** Compare and contrast Claude Code Terminal's extensibility features to help choose the right tool for each use case.

---

## Core Feature Comparison

| Feature | Invocation | Context | Complexity | Distribution | Use Case |
|---------|-----------|---------|------------|--------------|----------|
| **Subagents** | Model-invoked or explicit mention | Isolated context window | Medium-High | Project/User files or CLI flag | Specialized AI workers for complex tasks |
| **Skills** | Model-invoked (automatic discovery) | Loads into main context | High (multi-file) | Project/User directories or plugins | Complex capabilities with scripts/templates |
| **Slash Commands** | User-invoked (`/command`) | Main context | Low (single file) | Project/User files or plugins | Quick, frequently-used prompts |
| **Plugins** | Mixed (bundles multiple features) | N/A (container) | Variable | Marketplace installation | Package and distribute multiple components |
| **MCP Servers** | Tool integration | External system | N/A | CLI config or `.mcp.json` | Connect to external tools and data |
| **Hooks** | Event-driven (automatic) | Executes during lifecycle | Low (shell scripts) | Settings.json or plugin hooks | Automate responses to specific events |

---

## When to Use Each Feature

### Use **Subagents** when:
✅ Task requires specialized domain expertise (security, testing, debugging)
✅ Need separate context to preserve main conversation
✅ Different tool permissions for specific tasks
✅ Complex multi-step workflows benefit from focused AI personality
✅ Want to delegate work without polluting main context

❌ Don't use when:
- Task is simple and doesn't need isolation
- Main context has plenty of room
- No specialized expertise required

**Examples:**
- `code-reviewer` - Security and quality reviews
- `debugger` - Root cause analysis
- `test-automator` - Comprehensive test generation
- `data-scientist` - SQL queries and data analysis

---

### Use **Skills** when:
✅ Claude should automatically discover capability based on context
✅ Capability requires multiple files, scripts, or templates
✅ Team needs standardized, detailed guidance
✅ Complex workflows with validation steps
✅ Want reusable expertise across projects

❌ Don't use when:
- Single simple prompt suffices
- Want explicit control over invocation
- No supporting files needed

**Examples:**
- PDF processing with form-filling scripts
- Data analysis with reference docs
- Documentation generation with style guides
- DevForgeAI framework skills (ideation, development, qa, release)

---

### Use **Slash Commands** when:
✅ You frequently invoke the same prompt
✅ Instruction fits in a single markdown file
✅ Want explicit control over when it runs
✅ Need quick reminders or templates
✅ Simple parameterized workflows

❌ Don't use when:
- Workflow is complex with many steps
- Need scripts or utilities
- Want automatic discovery
- Multiple files needed for organization

**Examples:**
- `/review` - Quick code review
- `/optimize` - Performance analysis
- `/security-check` - Security scan
- `/explain` - Code explanation
- DevForgeAI commands (`/dev`, `/qa`, `/release`)

---

### Use **Plugins** when:
✅ Distributing multiple related features together
✅ Team adoption requires bundled installation
✅ Want to share via marketplace
✅ Multiple components work together as package
✅ Need version management for distribution

❌ Don't use when:
- Single feature suffices
- Only need personal configuration
- Not sharing with others

**Examples:**
- Company standards plugin (commands + agents + hooks)
- Testing framework plugin (test subagent + hooks + commands)
- Security suite (scanner agent + hooks + MCP integration)

---

### Use **MCP Servers** when:
✅ Connecting to external tools and APIs
✅ Accessing databases and data sources
✅ Integrating third-party services (GitHub, Jira, Sentry)
✅ Extending Claude beyond the codebase
✅ Need real-time data from external systems

❌ Don't use when:
- Data is local and file-based
- No external integration needed
- Can accomplish with file operations

**Examples:**
- GitHub integration for PR/issue management
- Sentry for error monitoring
- PostgreSQL for database queries
- Figma for design integration
- Jira for project management

---

### Use **Hooks** when:
✅ Need deterministic automation at specific events
✅ Want auto-formatting after file edits
✅ Require logging or compliance tracking
✅ Need to block operations based on rules
✅ Custom notifications for awaiting input

❌ Don't use when:
- LLM decision-making is acceptable
- No automation needed
- Event timing isn't critical

**Examples:**
- Auto-format TypeScript files after edits
- Run tests after code changes
- Block writes to production files
- Log all bash commands for compliance
- Desktop notifications when Claude awaits input

---

## Detailed Comparison Tables

### Invocation Methods

| Feature | User-Invoked | Model-Invoked | Event-Driven | Explicit Only |
|---------|--------------|---------------|--------------|---------------|
| Subagents | ❌ | ✅ (or explicit: "Use X subagent") | ❌ | ✅ (optional) |
| Skills | ❌ | ✅ (automatic discovery) | ❌ | ❌ |
| Slash Commands | ✅ (`/command`) | ✅ (via SlashCommand tool) | ❌ | ✅ (default) |
| Plugins | Mixed | Mixed | ❌ | ✅ (install/enable) |
| MCP Servers | ❌ | ✅ (tools auto-available) | ❌ | ✅ (slash commands) |
| Hooks | ❌ | ❌ | ✅ (lifecycle events) | ❌ |

---

### File Structure & Storage

| Feature | File Type | Location (Project) | Location (User) | Version Control |
|---------|-----------|-------------------|-----------------|-----------------|
| Subagents | `.md` with YAML frontmatter | `.claude/agents/` | `~/.claude/agents/` | ✅ Commit project |
| Skills | `SKILL.md` + resources | `.claude/skills/[name]/` | `~/.claude/skills/[name]/` | ✅ Commit project |
| Slash Commands | `.md` with optional YAML | `.claude/commands/` | `~/.claude/commands/` | ✅ Commit project |
| Plugins | `.claude-plugin/plugin.json` | Via marketplace install | Via marketplace install | ✅ Plugin repo |
| MCP Servers | `.mcp.json` or CLI config | `.mcp.json` (root) | User config | ✅ Commit `.mcp.json` |
| Hooks | JSON in settings.json | `.claude/settings.json` | `~/.claude/settings.json` | ✅ Commit project |

---

### Capabilities & Limitations

| Feature | Parameter Support | Multiple Files | External Tools | Isolated Context | Budget Limits |
|---------|------------------|----------------|----------------|------------------|---------------|
| Subagents | ✅ Via description + prompt | ❌ Single .md file | ✅ Via tools field | ✅ Separate context | ❌ No limits |
| Skills | ❌ Context-based only | ✅ SKILL.md + references + scripts | ✅ Via allowed-tools | ❌ Loads in main | ❌ No limits |
| Slash Commands | ✅ $ARGUMENTS, $1, $2 | ❌ Single .md file | ✅ Via allowed-tools | ❌ Main context | ✅ 15K char budget |
| Plugins | N/A (container) | ✅ Multiple components | ✅ Bundled MCP | Mixed | ❌ No limits |
| MCP Servers | ✅ Tool arguments | ✅ Server-defined | ✅ External system | ✅ External process | ⚠️ 10K token warning |
| Hooks | ✅ Event data via stdin | ✅ Can call scripts | ✅ Via bash commands | ✅ Separate process | ⚠️ 60s timeout |

---

## Use Case Decision Matrix

### "I want to create a code reviewer"

| Approach | Pros | Cons | Best For |
|----------|------|------|----------|
| **Subagent** | Isolated context, specialized prompt, custom tools | Setup overhead | Complex reviews requiring deep analysis |
| **Skill** | Auto-discovery, multi-file guidance, team-shared | No isolated context | Comprehensive review with checklists |
| **Slash Command** | Quick invocation, simple setup | Manual invocation, limited complexity | Simple "review this code" prompts |
| **Hook** | Automatic on every edit | No review control, deterministic only | Auto-lint, format checks |

**Recommendation:** Subagent for complex reviews, Slash Command for quick reviews, Hook for auto-linting.

---

### "I want to automate testing"

| Approach | Pros | Cons | Best For |
|----------|------|------|----------|
| **Subagent** | Test generation expertise, isolated context | Manual invocation | Comprehensive test suite generation |
| **Skill** | Auto-discovery when writing tests, multi-file resources | No isolation | TDD workflows, test frameworks |
| **Slash Command** | `/run-tests` quick execution | Manual trigger | Running existing tests |
| **Hook** | Auto-run after code changes | Can't skip, fixed logic | Continuous validation |

**Recommendation:** Skill for test generation, Hook (PostToolUse) for auto-running tests, Slash Command for manual test execution.

---

### "I want to integrate with GitHub"

| Approach | Pros | Cons | Best For |
|----------|------|------|----------|
| **MCP Server** | Full GitHub API access, OAuth auth | Setup complexity | Comprehensive GitHub integration |
| **Slash Command** | Quick PR review commands | Limited to Claude's knowledge | Simple workflows |
| **GitHub Actions** | Automated on PR/issue events | Requires workflow setup | CI/CD automation |
| **Plugin** | Bundle MCP + commands + hooks | Distribution overhead | Team-wide GitHub tooling |

**Recommendation:** MCP for full integration, GitHub Actions for automation, Slash Commands for quick tasks.

---

### "I want to enforce code standards"

| Approach | Pros | Cons | Best For |
|----------|------|------|----------|
| **Hook (PreToolUse)** | Blocks non-compliant edits | Deterministic only (can't use LLM) | Hard rules (formatting, prohibited patterns) |
| **Hook (PostToolUse)** | Auto-format after edits | Can't prevent edits | Auto-formatting, linting |
| **Subagent** | LLM-powered analysis | Manual invocation | Complex standard violations |
| **Skill** | Team-shared standards docs | No enforcement | Documentation and guidance |

**Recommendation:** PreToolUse hook for blocking, PostToolUse hook for auto-fixing, Skill for documenting standards.

---

### "I want to add custom prompts"

| Approach | Pros | Cons | Best For |
|----------|------|------|----------|
| **Slash Command** | Explicit invocation, simple setup | Character budget (15K) | Frequently-used prompts |
| **Skill** | Auto-discovery, no budget limit | Always in context (metadata) | Complex multi-step workflows |
| **CLAUDE.md** | Always available, team-shared | Not parameterized | Project context and guidelines |
| **Output Style** | Changes Claude's behavior globally | Replaces system prompt | Learning mode, documentation focus |

**Recommendation:** Slash Command for quick prompts, Skill for complex capabilities, CLAUDE.md for project context.

---

## Integration Patterns

### Pattern 1: Comprehensive Code Review System

**Components:**
- **Subagent** (`code-reviewer`): Deep analysis with isolated context
- **Hook** (PostToolUse): Auto-lint after edits
- **Slash Command** (`/quick-review`): Fast reviews for small changes
- **MCP** (GitHub): Fetch PR context, post review comments

**Workflow:**
```
1. Edit code → Hook auto-lints
2. /quick-review → Fast check (slash command)
3. Before PR → "Do comprehensive review" → Subagent activated
4. Post review to GitHub → MCP integration
```

---

### Pattern 2: TDD Development Workflow

**Components:**
- **Skill** (test-first-development): Auto-discovery for test writing
- **Hook** (PostToolUse on Edit): Auto-run tests after code changes
- **Subagent** (test-automator): Generate comprehensive test suites
- **Slash Command** (`/tdd-cycle`): Initiate Red-Green-Refactor

**Workflow:**
```
1. /tdd-cycle feature-name → Slash command starts
2. Write failing tests → Skill guides through patterns
3. Implement code → Hook auto-runs tests
4. Tests fail → Subagent helps debug
5. Refactor → Checkpoints for safety
```

---

### Pattern 3: CI/CD Automation

**Components:**
- **GitHub Actions**: Automated PR reviews and feature implementation
- **MCP** (GitHub + Sentry): Integration with issue tracker and monitoring
- **Hooks**: Pre-commit validation
- **Slash Commands**: Manual override commands

**Workflow:**
```
1. Create issue → GitHub Actions triggers Claude Code
2. Claude implements → Uses MCP to read issue
3. Pre-commit hook → Validates code
4. Opens PR → Automated review via GitHub Actions
5. Deploy → Sentry MCP monitors errors
```

---

### Pattern 4: Team Productivity Pack (Plugin)

**Plugin Contents:**
- **Commands** (`/standup-notes`, `/pr-summary`, `/code-review`)
- **Subagents** (reviewer, documenter, tester)
- **Skills** (api-documentation, test-generation)
- **Hooks** (auto-format, auto-test, changelog-update)
- **MCP** (Jira, Confluence, GitHub)

**Distribution:**
```
Company Marketplace
└── productivity-pack@company
    ├── 10 slash commands
    ├── 5 subagents
    ├── 3 skills
    ├── 8 hooks
    └── 3 MCP servers (pre-configured)
```

---

## Configuration Comparison

### Complexity of Setup

| Feature | Initial Setup | Ongoing Maintenance | Learning Curve |
|---------|--------------|---------------------|----------------|
| Subagents | Low (via `/agents`) | Low | Low |
| Skills | Medium (SKILL.md + structure) | Medium | Medium |
| Slash Commands | Low (single .md file) | Low | Low |
| Plugins | High (manifest + components) | Medium | High |
| MCP Servers | Medium (CLI commands) | Low | Medium |
| Hooks | Medium (JSON + scripts) | Medium | Medium-High |

---

### Team Collaboration

| Feature | Sharing Method | Team Adoption | Version Control | Discovery |
|---------|---------------|---------------|-----------------|-----------|
| Subagents | Git (`.claude/agents/`) | Automatic on pull | ✅ Commit | `/agents` interface |
| Skills | Git (`.claude/skills/`) or plugin | Automatic on pull | ✅ Commit | Auto-discovery |
| Slash Commands | Git (`.claude/commands/`) or plugin | Automatic on pull | ✅ Commit | `/help` list |
| Plugins | Marketplace + settings.json | Automatic on trust | ✅ Plugin repo | `/plugin` interface |
| MCP Servers | `.mcp.json` in git | Requires approval | ✅ Commit | `/mcp` interface |
| Hooks | settings.json in git | Automatic on pull | ✅ Commit | Settings review |

---

## Security & Permissions

### Permission Control

| Feature | Tool Restriction | Sandboxing | Audit Trail | Risk Level |
|---------|-----------------|------------|-------------|------------|
| Subagents | ✅ `tools` field | ✅ Via sandbox config | ✅ Session logs | Medium |
| Skills | ✅ `allowed-tools` field | ✅ Via sandbox config | ✅ Session logs | Low-Medium |
| Slash Commands | ✅ `allowed-tools` frontmatter | ✅ Via sandbox config | ✅ Session logs | Low |
| Plugins | ⚠️ All components' permissions | ✅ Via sandbox config | ✅ Session logs | High (untrusted) |
| MCP Servers | ⚠️ Full server capabilities | ❌ External process | ⚠️ Server-dependent | High (untrusted) |
| Hooks | ⚠️ Full shell access | ❌ Executes with user credentials | ⚠️ None built-in | **Very High** |

**Security Best Practices:**
- Review all hooks before enabling (execute with your credentials!)
- Trust MCP servers only from verified sources
- Audit plugins thoroughly before installation
- Use `allowed-tools` to restrict subagents and skills
- Configure deny rules for sensitive files

---

## Performance Characteristics

### Token Efficiency

| Feature | Context Impact | Token Overhead | Caching | Progressive Loading |
|---------|---------------|----------------|---------|---------------------|
| Subagents | ✅ Isolated (minimal) | ~5-10K invocation + summary | ✅ Yes | N/A |
| Skills | ⚠️ Loads in main | ~2K metadata + content when used | ✅ Yes | ✅ Level 1→2→3 |
| Slash Commands | ⚠️ Budget-limited | Up to 15K char budget total | ✅ Yes | ❌ Loads at startup |
| Plugins | Mixed | Depends on components | ✅ Yes | Mixed |
| MCP Servers | ⚠️ Tool outputs | Per tool call | ⚠️ Server-dependent | N/A |
| Hooks | ✅ Minimal | ~100 tokens per event | ❌ No | N/A |

**Key Insight:** Subagents are most token-efficient for heavy work (isolated context). Skills use progressive disclosure. Commands have character budget limits.

---

### Execution Speed

| Feature | Latency | Parallelizable | Background Execution |
|---------|---------|----------------|---------------------|
| Subagents | Medium (context gathering) | ✅ Multiple subagents | ❌ Blocks until complete |
| Skills | Low (in-context) | ❌ Sequential | ❌ Part of conversation |
| Slash Commands | Low (immediate) | ❌ Sequential | ❌ Part of conversation |
| Plugins | N/A (container) | N/A | N/A |
| MCP Servers | Variable (network) | ✅ Multiple tools | ✅ Server-dependent |
| Hooks | Low (local script) | ✅ Parallel hooks | ✅ Can background |

---

## Feature Combinations

### What Can Work Together

| Primary Feature | Can Include | Example |
|----------------|-------------|---------|
| **Plugin** | Commands + Agents + Skills + Hooks + MCP | Complete productivity suite |
| **Skill** | Scripts + Templates + Reference docs | PDF processing with utilities |
| **Slash Command** | Bash execution + File references | `/commit` with git integration |
| **Hook** | Multiple hook commands | Format + lint + test in sequence |
| **Subagent** | N/A (single .md file) | Cannot bundle other features |
| **MCP Server** | Exposed prompts (→ commands) + Resources | GitHub server with slash commands |

---

### Common Combinations

**Combination 1: Testing Suite**
- **Skill**: `test-generation` (guides test writing)
- **Subagent**: `test-automator` (generates comprehensive tests)
- **Hook**: Auto-run tests after edits (PostToolUse)
- **Slash Command**: `/run-tests` (manual execution)

**Combination 2: Documentation System**
- **Skill**: `api-documentation` (API doc patterns)
- **Hook**: Auto-update docs after API changes
- **Slash Command**: `/generate-docs` (manual generation)
- **MCP**: Confluence for publishing

**Combination 3: Security & Compliance**
- **Subagent**: `security-auditor` (OWASP Top 10 checks)
- **Hook**: Block writes to production files (PreToolUse)
- **Hook**: Scan for secrets before commits
- **MCP**: Vault for secrets management
- **Slash Command**: `/security-scan` (manual audit)

---

## Decision Trees

### "Should I create a Subagent or Skill?"

```
Does task need isolated context (preserve main conversation)?
├─ YES → Use Subagent
│         (Examples: code review, debugging, complex analysis)
│
└─ NO → Is task automatically discoverable from context?
          ├─ YES → Use Skill
          │         (Examples: PDF processing, data analysis)
          │
          └─ NO → Want explicit invocation?
                    └─ YES → Use Slash Command
                              (Examples: /review, /explain)
```

---

### "Should I create a Slash Command or add to CLAUDE.md?"

```
Will you invoke this frequently with variations?
├─ YES → Is it parameterized (takes arguments)?
│         ├─ YES → Create Slash Command
│         │         (Example: /review-pr $1 $2)
│         │
│         └─ NO → Does it fit in one file?
│                   ├─ YES → Create Slash Command
│                   │         (Example: /quick-review)
│                   │
│                   └─ NO → Create Skill
│                             (Complex multi-step workflow)
│
└─ NO → Add to CLAUDE.md
          (Always-active project context)
```

---

### "Should I use a Hook or prompt Claude?"

```
Must this ALWAYS happen at a specific event?
├─ YES → Is the logic deterministic (no LLM needed)?
│         ├─ YES → Use Hook (command type)
│         │         (Example: Auto-format with prettier)
│         │
│         └─ NO → Use Hook (prompt type)
│                   (Example: Evaluate if should stop)
│
└─ NO → Can Claude decide when to do this?
          └─ YES → Use Skill or Subagent
                    (Let Claude determine timing)
```

---

### "Should I install an MCP Server or use Bash?"

```
Do I need real-time data from external system?
├─ YES → Is there an official MCP server available?
│         ├─ YES → Install MCP Server
│         │         (Example: GitHub, Jira, Sentry)
│         │
│         └─ NO → Can I build a custom MCP server?
│                   ├─ YES → Build MCP Server
│                   │         (Use MCP SDK)
│                   │
│                   └─ NO → Use Bash + curl/API calls
│                             (One-off integrations)
│
└─ NO → Use Bash for local operations
          (git, npm, file operations via native tools)
```

---

## Quick Feature Selection Guide

| I Want To... | Use This Feature | Configuration Method |
|-------------|-----------------|---------------------|
| Delegate specialized tasks with context isolation | **Subagent** | `/agents` or create `.claude/agents/name.md` |
| Add auto-discovered capabilities | **Skill** | Create `.claude/skills/name/SKILL.md` |
| Create frequently-used prompt | **Slash Command** | Create `.claude/commands/name.md` |
| Automate actions at lifecycle events | **Hook** | Add to `settings.json` hooks section |
| Connect to external tools/APIs | **MCP Server** | `claude mcp add --transport http name url` |
| Package multiple features together | **Plugin** | Create plugin with `.claude-plugin/plugin.json` |
| Remember project conventions | **CLAUDE.md** | Create `CLAUDE.md` at project root |
| Change Claude's communication style | **Output Style** | `/output-style` or create custom style |

---

## Anti-Patterns to Avoid

| ❌ Don't Do This | ✅ Do This Instead | Why |
|-----------------|-------------------|-----|
| Create giant monolithic skill | Split into focused skills | Easier discovery, maintenance |
| Use Bash for file operations | Use Read, Edit, Write, Glob, Grep | 40-73% token savings |
| Put everything in slash commands | Use appropriate feature type | Character budget limits |
| Make hooks that call LLMs | Use command hooks for deterministic logic | Performance, reliability |
| Install untrusted plugins | Audit thoroughly first | Security risk |
| Hardcode secrets in configs | Use environment variables | Security |
| Create vague descriptions | Be specific with keywords | Better auto-discovery |
| Duplicate functionality | Use one feature type per capability | Confusion, maintenance |

---

## Migration Strategies

### From Manual Prompts → Slash Commands
```
Before: Copy-paste same prompt repeatedly
After: Create .claude/commands/my-prompt.md
Benefit: One-time setup, consistent execution
```

### From Slash Commands → Skills
```
Before: /review command (manual invocation)
After: code-review skill (auto-discovery)
Benefit: Claude knows when to review without prompting
```

### From Skills → Subagents
```
Before: Heavy skill loading lots of content
After: Subagent with isolated context
Benefit: Preserve main context, handle complex analysis
```

### From Multiple Tools → Plugin
```
Before: 5 slash commands + 3 subagents + 2 hooks (scattered)
After: Single plugin bundling all features
Benefit: Easy distribution, version management
```

---

## Rules vs CLAUDE.md (2025)

### Comparison

| Aspect | CLAUDE.md | .claude/rules/ |
|--------|-----------|----------------|
| **Structure** | Single file | Multiple files |
| **Organization** | Monolithic | Modular by topic |
| **Path targeting** | No | Yes (via frontmatter) |
| **Team scalability** | Merge conflicts | Parallel editing |
| **Reusability** | Copy/paste | Symlinks supported |
| **Auto-loading** | Yes | Yes (all .md files) |

### When to Use Each

**Use CLAUDE.md when:**
- Simple project with few rules
- Quick setup needed
- All instructions fit in one file
- No file-specific requirements

**Use .claude/rules/ when:**
- Larger projects with many standards
- Team collaboration (avoid merge conflicts)
- File-specific rules needed (via `paths:`)
- Want to share rules across projects (symlinks)
- Modular organization preferred

### Combining Both

They work together! CLAUDE.md provides high-level guidance while rules/ provides detailed standards:

```
.claude/
├── CLAUDE.md                 # Overview and imports
└── rules/
    ├── code-style.md         # Detailed code style
    ├── testing.md            # Test requirements
    └── security.md           # Security rules
```

In CLAUDE.md:
```markdown
For detailed standards, see the rules directory.
- Code style: @.claude/rules/code-style.md
- Testing: @.claude/rules/testing.md
```

---

## Version History

- **v1.0 (2025-11-06)**: Initial creation with comprehensive comparisons
- **v1.1 (2025-12-09)**: Added Rules vs CLAUDE.md comparison
- **Source**: Consolidated from Terminal documentation research and official docs

---

**Use this matrix to choose the right feature for your use case and avoid common pitfalls!**
