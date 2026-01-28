---
name: claude-code-terminal-expert
description: |
  Comprehensive expert knowledge of Claude Code Terminal features, configuration,
  and capabilities. Use when users ask about Claude Code functionality ("Can Claude
  Code...?", "Does Claude Code have...?"), creating subagents/skills/commands/plugins,
  configuring settings/models/permissions, installing MCP servers, setting up
  hooks/automation, CI/CD integration, troubleshooting issues, or any Claude Code
  Terminal questions. Provides authoritative guidance on all terminal features
  including the Agent Skills specification for creating compliant skills.
model: claude-model: opus-4-5-20251001
license: MIT
compatibility: "Claude Code v2.0+ (tested on v2.1.12)"
metadata:
  author: DevForgeAI
  version: "3.0.0"
  category: knowledge-infrastructure
  last-updated: "2026-01-17"
  agent-skills-spec-version: "1.0"
  topics:
    - subagents
    - skills
    - slash-commands
    - plugins
    - mcp-servers
    - hooks
    - configuration
    - ci-cd
    - agent-skills-specification
allowed-tools: Read Grep Glob WebFetch WebSearch
---

# Claude Code Terminal Expert

**Purpose:** Provide comprehensive, authoritative knowledge about Claude Code Terminal's complete feature set, enabling users to leverage all available capabilities effectively.

---

## ⚠️ EXECUTION MODEL: This Skill Expands Inline

**After invocation, YOU (Claude) execute these instructions phase by phase.**

**When you invoke this skill:**
1. This SKILL.md content is now in your conversation
2. You execute each phase sequentially
3. You display results as you work through phases
4. You complete with success/failure report

**Do NOT:**
- Wait passively for skill to "return results"
- Assume skill is executing elsewhere
- Stop workflow after invocation

**Proceed to "When to Use This Skill" section below and begin execution.**

---

## When to Use This Skill

Invoke this skill when users ask about:

**Feature Discovery:**
- "Can Claude Code do...?" / "Does Claude Code have...?"
- "How do I use [feature] in Claude Code?"
- "What features are available in Claude Code Terminal?"

**Component Creation:**
- Creating custom subagents, skills, or slash commands
- Developing plugins or marketplaces
- Setting up hooks for automation

**Configuration:**
- Configuring settings, models, or permissions
- Installing and managing MCP servers
- Setting up CI/CD integrations
- Network and proxy configuration

**Troubleshooting:**
- Installation issues (WSL, npm, authentication)
- Performance problems or errors
- Feature not working as expected

**Integration:**
- GitHub Actions or GitLab CI/CD setup
- Headless mode automation
- DevContainer configuration
- Hook-based workflows

---

## Core Claude Code Terminal Features

### 1. Subagents (Specialized AI Workers)

**What they are:** Pre-configured AI personalities with isolated context windows for specialized tasks

**Key capabilities:**
- Operate in separate contexts (preserve main conversation)
- Custom system prompts and tool access
- Automatic or explicit invocation
- Project-level (`.claude/agents/`) or user-level (`~/.claude/agents/`)

**When to use:** Complex tasks requiring domain expertise (code review, debugging, security audits)

**Quick start:** `/agents` command → Create New Agent → Define purpose and tools

**Details:** See `references/core-features.md` (Section 1)

---

### 2. Skills (Model-Invoked Capabilities)

**What they are:** Modular capabilities that Claude automatically uses based on context

**Key capabilities:**
- YAML frontmatter + Markdown instructions
- Progressive disclosure (3 levels: metadata → instructions → resources)
- Model-invoked (Claude decides when to use)
- Cannot accept command parameters (context-based only)

**When to use:** Reusable workflows, domain expertise, team-shared utilities

**Quick start:** Create `.claude/skills/my-skill/SKILL.md` with frontmatter

**Details:** See `references/core-features.md` (Section 2)

---

### Agent Skills Specification

**Official Standard:** https://agentskills.io/specification
**GitHub:** https://github.com/agentskills/agentskills

Agent Skills is an open standard (Dec 2025) by Anthropic defining how AI agents discover and use modular capabilities. Adopted by Microsoft, GitHub, Cursor, Spring AI, VS Code.

**Progressive Disclosure (3-Tier Loading):**
| Level | When Loaded | Token Budget | Purpose |
|-------|-------------|--------------|---------|
| **Discovery** | Always (startup) | ~100 tokens | Metadata only (name + description) |
| **Activation** | When triggered | <5,000 tokens | SKILL.md body (instructions) |
| **Execution** | On-demand | Unlimited | references/, scripts/, assets/ |

**YAML Frontmatter Requirements:**
- `name`: **Required**, 1-64 chars, lowercase-with-hyphens only
- `description`: **Required**, 1-1024 chars, MUST include "Use when..." trigger context
- `metadata`: Optional, for version/author/category (NOT at top level)
- `allowed-tools`: Optional (experimental), space-delimited tool whitelist
- `license`: Optional, license reference (e.g., MIT)
- `compatibility`: Optional, version requirements

**Validation:** `pip install skills-ref && skills-ref validate path/to/skill/`

**For complete Agent Skills documentation:**
→ Load `references/agent-skills-spec.md`

---

### 3. Slash Commands (User-Invoked Workflows)

**What they are:** Markdown files containing instructions for Claude to follow

**Key capabilities:**
- User-invoked (explicit `/command` syntax)
- Support arguments via `$ARGUMENTS`, `$1`, `$2`
- YAML frontmatter for model, tools, hints
- Character budget: 15,000 chars limit

**When to use:** Frequently-used prompts, project-specific workflows

**Quick start:** Create `.claude/commands/my-command.md`

**Built-in commands:** `/help`, `/clear`, `/model`, `/agents`, `/mcp`, `/config`, 30+ total

**Details:** See `references/core-features.md` (Section 3)

---

### 4. Plugins (Bundled Extensions)

**What they are:** Packages containing commands, agents, skills, hooks, and MCP servers

**Key capabilities:**
- Marketplace-based distribution
- Team-wide installation via settings.json
- Components auto-discovered by Claude Code
- Version management and updates

**When to use:** Sharing utilities across projects, team collaboration

**Quick start:** `/plugin` command → Install from marketplace

**Details:** See `references/core-features.md` (Section 4)

---

### 5. MCP Servers (External Tool Integration)

**What they are:** Model Context Protocol servers connecting Claude to external services

**Key capabilities:**
- 40+ available servers (GitHub, Jira, Figma, Stripe, etc.)
- HTTP, SSE, or stdio transports
- OAuth authentication support
- Project, user, or local scope

**When to use:** Accessing external APIs, databases, or services

**Quick start:** `claude mcp add --transport http <name> <url>`

**Details:** See `references/core-features.md` (Section 5)

---

### 6. Hooks (Event-Driven Automation)

**What they are:** Shell commands executed at specific lifecycle events

**Key capabilities:**
- 9 event types (PreToolUse, PostToolUse, Stop, SessionStart, etc.)
- Command-based or LLM prompt-based evaluation
- Can block operations (exit code 2)
- Matcher patterns for selective triggering

**When to use:** Auto-formatting, logging, notifications, security checks

**Quick start:** `/hooks` command → Configure event and matcher

**Details:** See `references/integration-patterns.md` (Section 3)

---

### 7. Configuration System

**What it is:** Multi-level settings hierarchy (enterprise → CLI → local → project → user)

**Key capabilities:**
- settings.json files at multiple scopes
- Permission management (allow/ask/deny)
- Model selection and aliases
- Environment variable configuration
- CLAUDE.md files and `.claude/rules/` for project memory

**When to use:** Customizing Claude Code behavior, team standards

**Quick start:** `/config` command or edit `~/.claude/settings.json`

**Details:** See `references/configuration-guide.md`

---

### 8. CI/CD Integration

**What it is:** Automated Claude Code execution in GitHub Actions and GitLab CI/CD

**Key capabilities:**
- GitHub Actions: `@claude` mentions in PRs/issues
- GitLab CI/CD: Automated MR creation and reviews
- AWS Bedrock and Google Vertex support
- Headless mode for automation

**When to use:** Code review automation, PR generation, scheduled tasks

**Quick start:** `/install-github-app` or add workflow YAML

**Details:** See `references/integration-patterns.md`

---

### 9. Memory & Rules System

**What it is:** Hierarchical memory and modular rules for Claude Code behavior

**Key capabilities:**
- 5-level memory hierarchy (Enterprise → Project → Rules → User → Local)
- `.claude/rules/` directory for modular, topic-specific rules
- Conditional rules with `paths:` frontmatter for file-specific behavior
- Import syntax (`@path/to/file`) for file inclusion (max 5 hops)
- `/memory`, `/init`, `#` shortcuts for memory management

**When to use:** Organizing project instructions, team-wide standards, file-specific rules

**Quick start:** Create `.claude/rules/code-style.md` with YAML frontmatter

**DevForgeAI Integration:**
- `.claude/rules/` for Claude Code behavior rules
- `devforgeai/specs/context/` for immutable architectural constraints
- Rules can reference context files: `@devforgeai/specs/context/tech-stack.md`

**Details:** See `references/configuration-guide.md` (Memory & Rules section)

---

### 10. Background Tasks & Agents (December 2025)

**What they are:** Run long-running commands and agents in the background while continuing work

**Key capabilities:**
- `Ctrl+B` - Move any Bash command to background (Tmux users: press twice)
- Background agents with `run_in_background=true` parameter
- Retrieve output via `TaskOutput` tool with task IDs
- Auto-cleanup when session exits
- Concurrent task execution for dev servers, builds, tests

**When to use:** Long-running processes (dev servers, builds, tests), parallel task execution

**Quick start:** Press `Ctrl+B` during command execution, or prefix with `!` for bash mode

**Bash Mode (`!` prefix):**
```bash
! npm test      # Run directly without Claude interpretation
! git status    # Shows real-time progress, supports Ctrl+B
```

**Details:** See `references/core-features.md` (Section 6)

---

### 11. Checkpoints & Rewind (December 2025)

**What they are:** Automatic code state tracking with instant restoration

**Key capabilities:**
- Auto-saves code state before each Claude edit
- `Esc Esc` - Rewind menu (quick access)
- `/rewind` - Restore code and/or conversation
- Persists across sessions (30-day retention)
- Three restore options: conversation only, code only, or both

**When to use:** Risk-free experimentation, recovering from mistakes, exploring alternatives

**Quick start:** Press `Esc Esc` to open rewind menu

**Important limitations:**
- Does NOT track bash command changes (rm, mv, cp)
- Does NOT track external file modifications
- Complements (not replaces) Git version control

**Details:** See `references/core-features.md` (Section 7)

---

### 12. Sessions & History (December 2025)

**What they are:** Named sessions with usage tracking and history management

**Key capabilities:**
- `/rename` - Name current session for easy reference
- `/resume <n>` - Resume previous named sessions
- `/stats` - View usage statistics (model, graph, streak)
- `Ctrl+R` - Reverse search command history
- History stored per working directory

**When to use:** Long projects, resuming work, tracking progress

**Quick start:** `/rename my-feature` → later `/resume my-feature`

**Details:** See `references/configuration-guide.md` (Session Management section)

---

### 13. Model Selection & Switching (December 2025)

**What it is:** Quick model switching and intelligent hybrid modes

**Key capabilities:**
- `Alt+P` (Windows/Linux) or `Option+P` (macOS) - Quick model switch mid-prompt
- Model aliases: `opus`, `sonnet`, `haiku`, `default`, `opusplan`
- `opusplan` mode: Opus for planning, Sonnet for execution (cost-optimized)
- Extended context: `[1m]` suffix for 1M token context window
- Prompt caching for performance optimization

**Available models:**
| Alias | Model | Best For |
|-------|-------|----------|
| `opus` | Claude Opus 4.5 | Complex reasoning, specialized tasks |
| `sonnet` | Claude Sonnet 4.5 | Daily coding (default) |
| `haiku` | Claude Haiku | Simple, fast tasks |
| `opusplan` | Hybrid | Planning with Opus, execution with Sonnet |

**Quick start:** Press `Alt+P` to switch, or `claude --model opus`

**Details:** See `references/configuration-guide.md` (Model Configuration section)

---

## How DevForgeAI Skills Work with User Input

DevForgeAI skills depend on clear, specific user input to generate high-quality outputs. When you provide complete requirements and context, skills can deliver solutions with minimal clarification. Vague or ambiguous input triggers safety mechanisms that ask clarifying questions—important for correctness, but requiring additional iterations.

### Cross-Reference Guidance Documents

**[effective-prompting-guide.md](../../memory/effective-prompting-guide.md)** (User-Facing Guide)

Consult this guide when you want to improve how you communicate with DevForgeAI. It covers all 11 commands with specific examples showing ineffective vs. effective input patterns, command-specific workflows, and common mistakes to avoid. Best for: preparing feature requests, creating user stories, writing error reports, or refining technology decisions. Use this BEFORE interacting with DevForgeAI to optimize your input quality.

**[user-input-guidance.md](../devforgeai-ideation/references/user-input-guidance.md)** (Framework-Internal Reference)

DevForgeAI skills reference this document internally when asking you clarifying questions. It defines discovery workflows, AskUserQuestion templates with examples, and quantification tables for non-functional requirements. You'll encounter these structures in framework clarifying questions—they're intentionally designed to extract complete, measurable specifications. Use this AFTER receiving an AskUserQuestion to understand why specific information is being requested.

### Examples: Effective vs Ineffective Input

**Feature Requests:**
- ❌ Ineffective: "Can Claude Code handle my workflows?"
- ✅ Effective: "Can Claude Code execute custom Python scripts via hooks and capture their output for integration with GitHub Actions?" *(Specificity + use case)*

**Story Creation:**
- ❌ Ineffective: "I need a feature to manage users"
- ✅ Effective: "I need a user registration feature. Acceptance criteria: validate email format, hash passwords with bcrypt, store in PostgreSQL, return 409 if email exists" *(Scope + testable requirements)*

**Error Reporting:**
- ❌ Ineffective: "It's not working"
- ✅ Effective: "When I run `/dev STORY-001`, the test runner fails with 'ModuleNotFoundError: No module named pytest' on Python 3.10 in WSL2" *(Context + reproducibility)*

**Technology Decisions:**
- ❌ Ineffective: "What database should I use?"
- ✅ Effective: "My project has <1M records, <1000 QPS, needs ACID compliance. Should I use SQLite (dev) or PostgreSQL (production)?" *(Constraints + options)*

**Feedback on Outputs:**
- ❌ Ineffective: "This doesn't look right"
- ✅ Effective: "The generated SQL uses string concatenation instead of parameterized queries. This violates the 'no SQL concatenation' rule in anti-patterns.md" *(Evidence + reference)*

**Configuration Questions:**
- ❌ Ineffective: "How do I set up MCP?"
- ✅ Effective: "I want to integrate Claude Code with our internal JIRA instance. Can you show me how to configure OAuth with an MCP server?" *(Goal + integration detail)*

**Testing Specifications:**
- ❌ Ineffective: "Add tests for the order feature"
- ✅ Effective: "Add unit tests for Order.calculate_total() with: valid items (3+ products), empty order, negative quantities, currency mismatches" *(Function + test cases)*

**Scope Definition:**
- ❌ Ineffective: "Make it scalable"
- ✅ Effective: "System must handle 10,000 concurrent users with <200ms p99 latency and 99.9% uptime SLA" *(Measurable metrics)*

**Documentation Requirements:**
- ❌ Ineffective: "Document everything"
- ✅ Effective: "Document the REST API endpoints (URL, method, request/response schema), error codes (400-409), and rate limits (100 req/min per IP)" *(Scope + specificity)*

### The "Ask, Don't Assume" Principle

DevForgeAI follows an explicit "Ask, Don't Assume" principle described in CLAUDE.md. Here's how it works:

**When to Expect Clarifying Questions:**
- **Technology decisions** - No standard specified in tech-stack.md
- **Conflicting requirements** - Spec says X, context file says Y
- **Ambiguous scope** - "Fast", "scalable", "simple" without metrics
- **Missing details** - AC without test cases, design without constraints
- **Preference-dependent choices** - Architecture style, naming conventions, framework preferences

**What Skills Do NOT Assume:**
- Your technology preferences (only uses tech-stack.md)
- Your priority weighting (asks if conflicting)
- Your definition of success metrics (requires explicit targets)
- Your team's available time/budget (requests clarification)
- Your data sensitivity requirements (asks for security constraints)

**Why This Principle Exists:**
Skills operate without human follow-up conversations in some contexts. Making wrong assumptions leads to implementing the wrong feature, violating constraints, or wasting effort on non-goals. By asking explicitly, DevForgeAI ensures outputs match your actual needs rather than the AI's interpretation.

**How This Integrates with Quality Gates:**
DevForgeAI's quality gates check that all decisions are documented with explicit user approval. If a skill makes assumptions and later testing reveals they were wrong, the quality gate blocks progression. This is intentional—catching assumption misalignment early prevents rework.

---

## Progressive Disclosure Strategy

**Load reference files as needed:**

1. **Core Features** - When asked about subagents, skills, commands, plugins, or MCP
   ```
   Read(file_path=".claude/skills/claude-code-terminal-expert/references/core-features.md")
   ```

2. **Configuration** - When asked about settings, models, CLI options, permissions
   ```
   Read(file_path=".claude/skills/claude-code-terminal-expert/references/configuration-guide.md")
   ```

3. **Integration** - When asked about CI/CD, hooks, automation, headless mode
   ```
   Read(file_path=".claude/skills/claude-code-terminal-expert/references/integration-patterns.md")
   ```

4. **Troubleshooting** - When asked about errors, installation issues, performance
   ```
   Read(file_path=".claude/skills/claude-code-terminal-expert/references/troubleshooting-guide.md")
   ```

5. **Advanced** - When asked about sandboxing, networking, monitoring, security
   ```
   Read(file_path=".claude/skills/claude-code-terminal-expert/references/advanced-features.md")
   ```

6. **Best Practices** - When asked about workflows, efficiency, prompt engineering
   ```
   Read(file_path=".claude/skills/claude-code-terminal-expert/references/best-practices.md")
   ```

**Quick References:**
```
Read(file_path=".claude/skills/claude-code-terminal-expert/assets/quick-reference.md")
Read(file_path=".claude/skills/claude-code-terminal-expert/assets/comparison-matrix.md")
```

---

## Self-Updating Mechanism

**When documentation may be outdated:**
- User reports feature not documented
- User asks about new features
- Documentation seems inconsistent with behavior

**Update procedure:**

1. **Fetch latest docs from official sources:**
   ```
   WebFetch(url="https://code.claude.com/docs/en/[topic]", prompt="Extract complete documentation...")
   ```

2. **Compare with current reference file:**
   ```
   Read(file_path=".claude/skills/claude-code-terminal-expert/references/[relevant-file].md")
   # Compare content, identify gaps
   ```

3. **Update reference file with new content:**
   ```
   Edit(file_path=".claude/skills/claude-code-terminal-expert/references/[relevant-file].md",
        old_string="[outdated section]",
        new_string="[updated content from web]")
   ```

4. **Verify update:**
   ```
   Read(file_path=".claude/skills/claude-code-terminal-expert/references/[relevant-file].md")
   # Confirm changes applied correctly
   ```

5. **Notify user:**
   ```
   "Updated [section] in [file] with latest documentation from code.claude.com"
   ```

**Documentation URLs by topic:**
- Subagents: https://code.claude.com/docs/en/sub-agents
- Skills: https://code.claude.com/docs/en/skills
- Agent Skills Spec: https://agentskills.io/specification
- Agent Skills GitHub: https://github.com/agentskills/agentskills
- Agent Skills Overview: https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview
- Slash Commands: https://code.claude.com/docs/en/slash-commands
- Plugins: https://code.claude.com/docs/en/plugins
- Plugin Reference: https://code.claude.com/docs/en/plugins-reference
- Plugin Marketplaces: https://code.claude.com/docs/en/plugin-marketplaces
- MCP: https://code.claude.com/docs/en/mcp
- Settings: https://code.claude.com/docs/en/settings
- Model Config: https://code.claude.com/docs/en/model-config
- CLI Reference: https://code.claude.com/docs/en/cli-reference
- Hooks Guide: https://code.claude.com/docs/en/hooks-guide
- Hooks Reference: https://code.claude.com/docs/en/hooks
- GitHub Actions: https://code.claude.com/docs/en/github-actions
- GitLab CI/CD: https://code.claude.com/docs/en/gitlab-ci-cd
- Headless Mode: https://code.claude.com/docs/en/headless
- Checkpointing: https://code.claude.com/docs/en/checkpointing
- Output Styles: https://code.claude.com/docs/en/output-styles
- Sandboxing: https://code.claude.com/docs/en/sandboxing
- Security: https://code.claude.com/docs/en/security
- Network Config: https://code.claude.com/docs/en/network-config
- Data Usage: https://code.claude.com/docs/en/data-usage
- Monitoring: https://code.claude.com/docs/en/monitoring-usage
- Costs: https://code.claude.com/docs/en/costs
- DevContainer: https://code.claude.com/docs/en/devcontainer
- Statusline: https://code.claude.com/docs/en/statusline
- Memory: https://code.claude.com/docs/en/memory
- Interactive Mode: https://code.claude.com/docs/en/interactive-mode
- Terminal Config: https://code.claude.com/docs/en/terminal-config
- Troubleshooting: https://code.claude.com/docs/en/troubleshooting

---

## Quick Reference

**Common Questions:**
- "How do I create a subagent?" → Section 1 + core-features.md
- "What's the difference between skills and commands?" → comparison-matrix.md
- "How do I set up GitHub Actions?" → integration-patterns.md
- "Claude Code isn't responding, what do I do?" → troubleshooting-guide.md
- "How do I configure permissions?" → configuration-guide.md
- "What MCP servers are available?" → core-features.md (Section 5)

**Keyboard Shortcuts (Updated December 2025):**
- `Ctrl+C` - Cancel generation
- `Ctrl+B` - Run command in background (Tmux: press twice)
- `Esc Esc` - Rewind conversation/code (checkpoint restore)
- `Ctrl+R` - Reverse search command history
- `Ctrl+O` - Toggle verbose output
- `Ctrl+L` - Clear terminal screen
- `Alt+P` / `Option+P` - Quick model switching
- `Tab` - Accept prompt suggestion
- `Shift+Tab` / `Alt+M` - Cycle permission modes (Auto-Accept/Plan/Normal)
- `?` - Show available shortcuts for your terminal
- See `assets/quick-reference.md` for complete list

**Built-in Commands (Updated December 2025):**
- `/help` - List all commands
- `/agents` - Manage subagents
- `/mcp` - Manage MCP servers
- `/config` - Open settings
- `/rewind` - Restore code/conversation checkpoint
- `/rename` - Name current session
- `/resume <n>` - Resume named session
- `/stats` - View usage statistics
- `/plugin` - Install plugins
- `/vim` - Enable Vim editor mode
- `/bug` - Report issues
- See `assets/quick-reference.md` for complete list

---

## Usage Pattern

**Step 1: Identify user's question category**
- Core features → Load core-features.md
- Configuration → Load configuration-guide.md
- Integration/automation → Load integration-patterns.md
- Problems/errors → Load troubleshooting-guide.md
- Advanced topics → Load advanced-features.md
- Best practices → Load best-practices.md

**Step 2: Load appropriate reference file(s)**
- Use Read tool to load specific section
- Provide comprehensive, accurate answer from documentation
- Include code examples from reference files

**Step 3: Suggest related topics**
- Point to other relevant sections
- Recommend next steps or related features

---

## Maintenance Protocol

**Quarterly Review (Every 3 months):**
1. Check code.claude.com for new features or documentation updates
2. Update reference files with latest content
3. Add new features to appropriate sections
4. Update version history in this file

**User-Reported Gaps:**
1. User mentions feature not documented
2. Fetch latest docs from relevant URL
3. Update appropriate reference file
4. Confirm update with user

**Version History:**
- v3.0 (2026-01-18): Agent Skills Specification compliance - updated YAML frontmatter, new agent-skills-spec.md reference, DevForgeAI integration sections
- v2.0 (2025-12-20): Major update with December 2025 features - background tasks, checkpoints, sessions, Opus 4.5, model switching, remote MCP, plugins enhancement
- v1.1 (2025-12-09): Added rules system, new CLI flags, hook events, subagent enhancements, DevForgeAI integration
- v1.0 (2025-11-06): Initial creation, migrated 15,788 lines from devforgeai/specs/Terminal/

---

## Token Efficiency

**This skill:**
- SKILL.md: ~2,000 tokens (lightweight discovery)
- Reference files: Load only as needed (3,500-2,800 tokens each)
- Progressive disclosure: Prevents context overflow

**Best practices:**
- Load single reference file per query when possible
- Use quick-reference.md for simple lookups
- Load multiple references only for cross-cutting questions

---

## Integration with DevForgeAI Framework

**This skill complements DevForgeAI by:**
- Providing Claude Code Terminal expertise
- Enabling self-service feature discovery
- Reducing "Claude doesn't know this feature" friction
- Supporting framework automation with terminal knowledge

**Not a replacement for:**
- DevForgeAI workflow skills (ideation, development, qa, release)
- Framework-specific guidance (see CLAUDE.md)
- Project architecture decisions

---

## Quick Start Examples

**Example 1: Creating a subagent**
```
User: "How do I create a code reviewer subagent?"
Action: Load references/core-features.md (Section 1)
Response: [Detailed steps with YAML frontmatter example]
```

**Example 2: Setting up GitHub Actions**
```
User: "I want Claude Code to review my PRs automatically"
Action: Load references/integration-patterns.md (Section 1)
Response: [Complete GitHub Actions setup with workflow YAML]
```

**Example 3: Troubleshooting**
```
User: "Claude Code keeps asking for permissions, how do I fix this?"
Action: Load references/configuration-guide.md (Permissions section)
Response: [Permission configuration guidance with settings.json examples]
```

**Example 4: Feature comparison**
```
User: "What's the difference between skills and slash commands?"
Action: Load assets/comparison-matrix.md
Response: [Comparison table with use cases]
```

---

## Remember

**Authoritative Source:** All guidance comes from official code.claude.com documentation
**Always Current:** Self-updating mechanism ensures accuracy (v2.0 December 2025)
**Progressive Disclosure:** Load only necessary reference files
**Complete Coverage:** 13 core features + configuration + integration + troubleshooting
**DevForgeAI Integration:** Complements framework with terminal expertise
**Claude Code Version:** 2.0.74 (December 2025)
