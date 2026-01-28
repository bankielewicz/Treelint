# claude-code-terminal-expert Skill

**Version:** 3.0.0
**Created:** 2025-11-06
**Last Updated:** 2026-01-18
**Claude Code Version:** 2.1.12
**Status:** ✅ Production Ready
**Type:** Knowledge/Infrastructure Skill
**Agent Skills Spec:** ✅ Compliant (v1.0)

---

## Purpose

Comprehensive expert knowledge of Claude Code Terminal's complete feature set, enabling users to leverage all available capabilities effectively.

**Problem Solved:** Claude often doesn't know about Claude Code Terminal features, leading to user friction when asking "Can Claude Code...?" or "How do I configure...?" questions.

**Solution:** Consolidated 15,788 lines from 25 Terminal docs + 29 official web docs into a self-updating, progressively-loaded skill with 100% topic coverage.

---

## Quick Stats

| Metric | Value |
|--------|-------|
| **Total Files** | 10 (1 SKILL.md + 7 references + 2 assets) |
| **Total Lines** | ~18,000 lines |
| **Topics Covered** | 35+ (including Agent Skills spec) |
| **Source Docs** | 65+ (local + official web docs + agentskills.io) |
| **Claude Code Version** | 2.1.12 |
| **Token Efficiency** | 95% savings (progressive disclosure) |

### December 2025 Update Highlights

- Background Tasks & Agents (Ctrl+B, run_in_background)
- Checkpoints & Rewind (Esc Esc, /rewind)
- Named Sessions (/rename, /resume, /stats)
- Claude Opus 4.5 configuration
- Quick Model Switching (Alt+P, Option+P)
- Remote MCP with OAuth
- IDE Integrations (Chrome, JetBrains, VS Code)
- Claude Agent SDK documentation

---

## Skill Structure

```
claude-code-terminal-expert/
├── SKILL.md                          # Discovery & routing (~700 lines)
├── references/                       # Progressive disclosure (7 files)
│   ├── core-features.md             # ~2,700 lines (subagents, skills, commands, plugins, MCP)
│   ├── configuration-guide.md       # ~1,800 lines (settings, models, CLI, permissions, DevForgeAI)
│   ├── integration-patterns.md      # ~2,950 lines (CI/CD, hooks, headless, containers)
│   ├── troubleshooting-guide.md     # ~2,128 lines (installation, auth, performance, errors)
│   ├── advanced-features.md         # ~3,635 lines (sandboxing, network, monitoring, security)
│   ├── best-practices.md            # ~1,400 lines (workflows, efficiency, Agent Skills practices)
│   └── agent-skills-spec.md         # ~800 lines (Agent Skills specification reference) [NEW]
└── assets/                           # Quick lookup (2 files)
    ├── quick-reference.md           # ~797 lines (commands, shortcuts, configs)
    └── comparison-matrix.md         # ~651 lines (feature comparison, decision trees)
```

---

## When Claude Invokes This Skill

The skill automatically triggers when users ask about:

- **Feature Discovery:** "Can Claude Code...?", "Does Claude Code have...?", "What features..."
- **Component Creation:** Creating subagents, skills, commands, plugins, hooks
- **Configuration:** Settings, models, permissions, MCP servers, CLI options
- **Integration:** GitHub Actions, GitLab CI/CD, headless mode, DevContainers
- **Troubleshooting:** Installation issues, authentication, performance, errors
- **General:** Any Claude Code Terminal questions

**14 specific triggers + catch-all** ensure comprehensive coverage.

---

## Progressive Disclosure Strategy

### Level 1: Metadata (~100 tokens, always loaded)
- YAML frontmatter (name + description)
- Appears in available skills list
- Minimal context impact

### Level 2: Instructions (~2,000 tokens, when skill triggered)
- SKILL.md body with 8 feature overviews
- Routing guidance to appropriate references
- Self-updating procedures

### Level 3: Resources (1,200-3,500 tokens per file, loaded as needed)
- 6 reference files (load 1-2 typically)
- 2 asset files (quick lookup)
- **Token range:** 2,100-6,000 tokens typical (vs 120,000 for all docs)
- **Savings:** 85-98% token reduction

---

## Coverage Summary

### Core Features (5/5) ✅
- Subagents - Specialized AI workers
- Skills - Model-invoked capabilities
- Slash Commands - User-invoked workflows
- Plugins - Bundled extensions
- MCP Servers - External tool integration

### Configuration (5/5) ✅
- Settings system - Multi-level hierarchy
- Model configuration - Aliases and selection
- CLI reference - Commands and flags
- Permission management - Allow/ask/deny
- Environment variables - All options

### Integration (6/6) ✅
- GitHub Actions - CI/CD automation
- GitLab CI/CD - Pipeline integration
- Hooks - 9 event types
- Headless mode - SDK automation
- DevContainers - Isolated development
- VS Code - IDE integration

### Advanced (6/6) ✅
- Sandboxing - Filesystem/network isolation
- Security - Enterprise controls
- Monitoring - OpenTelemetry
- Network - Proxy, certificates
- Privacy - Data retention, compliance
- Enterprise - Managed policies

### Support (6/6) ✅
- Installation - WSL, npm, auth
- Troubleshooting - Diagnostics, solutions
- Best practices - Workflows, optimization
- Token efficiency - Native tools (40-73% savings)
- Quick reference - Commands, shortcuts
- Feature comparison - Decision guidance

**Total:** 28 topics, 100% coverage

---

## Self-Updating Mechanism

### How It Works

1. **Update Trigger:** User reports outdated info or asks about new features
2. **Fetch Latest:** `WebFetch(url="https://code.claude.com/docs/en/[topic]")`
3. **Compare Content:** Read current reference file, identify gaps
4. **Update File:** Edit reference with new content from official docs
5. **Notify User:** "✅ Updated [section] with latest from code.claude.com"

### Documentation URLs (29 embedded)

**Core Features:**
- https://code.claude.com/docs/en/sub-agents
- https://code.claude.com/docs/en/skills
- https://code.claude.com/docs/en/slash-commands
- https://code.claude.com/docs/en/plugins
- https://code.claude.com/docs/en/mcp

**Configuration:**
- https://code.claude.com/docs/en/settings
- https://code.claude.com/docs/en/model-config
- https://code.claude.com/docs/en/cli-reference
- https://code.claude.com/docs/en/interactive-mode
- https://code.claude.com/docs/en/terminal-config

**Integration:**
- https://code.claude.com/docs/en/github-actions
- https://code.claude.com/docs/en/gitlab-ci-cd
- https://code.claude.com/docs/en/hooks-guide
- https://code.claude.com/docs/en/hooks
- https://code.claude.com/docs/en/headless
- https://code.claude.com/docs/en/devcontainer

**Advanced:**
- https://code.claude.com/docs/en/sandboxing
- https://code.claude.com/docs/en/security
- https://code.claude.com/docs/en/network-config
- https://code.claude.com/docs/en/monitoring-usage
- https://code.claude.com/docs/en/data-usage
- https://code.claude.com/docs/en/costs

**Additional:**
- https://code.claude.com/docs/en/checkpointing
- https://code.claude.com/docs/en/output-styles
- https://code.claude.com/docs/en/plugins-reference
- https://code.claude.com/docs/en/plugin-marketplaces
- https://code.claude.com/docs/en/memory
- https://code.claude.com/docs/en/statusline
- https://code.claude.com/docs/en/troubleshooting

---

## Testing Results

**Test Suite:** 14 comprehensive tests
**Pass Rate:** 100% (14/14 passed)

**Tested:**
- Feature discovery (5 tests) ✅
- Configuration questions (2 tests) ✅
- Integration setup (2 tests) ✅
- Troubleshooting (1 test) ✅
- Quick lookup (1 test) ✅
- Self-updating (1 test) ✅
- Cross-feature integration (1 test) ✅
- DevForgeAI compliance (1 test) ✅

**See:** SKILL-TESTING-REPORT.md for complete test results

---

## DevForgeAI Integration

### Role in Framework
**Type:** Infrastructure/support skill
**Purpose:** Provide Claude Code Terminal expertise
**Relationship:** Complements DevForgeAI workflow skills (doesn't replace)

### Integration Points
- **Used by:** Users asking Claude Code feature questions
- **Not used by:** DevForgeAI workflow skills (separate domains)
- **Benefit:** Reduces "Claude doesn't know this" friction
- **Impact:** Minimal token overhead (~100-6,000 tokens)

### Framework Compliance
- ✅ Progressive disclosure pattern (3 levels)
- ✅ Native tool usage (Read, Edit, WebFetch)
- ✅ Token efficient (95% savings average)
- ✅ Official sources only (evidence-based)
- ✅ Self-updating capability
- ✅ Team-shareable (project-level skill)

---

## Token Efficiency Analysis

### Usage Scenarios

| Scenario | Files Loaded | Tokens | vs All Docs | Savings |
|----------|-------------|--------|-------------|---------|
| Metadata only | Frontmatter | 100 | 120,000 | 99.9% |
| Simple question | SKILL.md | 2,100 | 120,000 | 98.2% |
| + Quick lookup | + assets | 2,800 | 120,000 | 97.7% |
| + 1 reference | + ref file | 4,500 | 120,000 | 96.3% |
| + 2 references | + 2 ref files | 7,000 | 120,000 | 94.2% |
| + 3 references | + 3 ref files | 9,500 | 120,000 | 92.1% |
| All loaded | Everything | 18,000 | 120,000 | 85.0% |

**Average:** 95% token reduction through progressive disclosure

**Comparison:**
- Loading all Terminal docs: ~120,000 tokens
- Using this skill: ~2,100-6,000 tokens (typical)
- **Result:** 20-57x more efficient

---

## Content Sources

### Migrated from devforgeai/specs/Terminal/ (25 files)
- sub-agents.md
- agent-skills.md
- slash-commands.md
- plugins.md
- mcp.md
- settings.md
- model-config.md
- cli-reference.md
- hooks-guide.md
- hooks-reference.md
- github-actions.md
- gitlab-ci-cd.md
- headless-mode.md
- checkpointing.md
- output-styles.md
- plugins-reference.md
- statusline-integration.md
- slash-commands-best-practices.md
- prompt-engineering-best-practices.md
- system-prompt-best-practices.md
- native-tools-vs-bash-efficiency-analysis.md
- common-workflows.md
- plan-usage-policy.md
- session-state-research-report.md
- vs-code.md
- index.md

### Fetched from code.claude.com (29 web docs)
All current official documentation retrieved and verified.

**Total Sources:** 54 documents consolidated

---

## Maintenance

### Update Schedule
- **Quarterly:** Check code.claude.com for new features (next: 2026-02-06)
- **On user report:** Update specific sections immediately
- **Annual:** Complete documentation refresh (next: 2026-11-06)

### Update Procedure
1. User reports feature gap or outdated info
2. WebFetch latest from appropriate code.claude.com URL
3. Compare with current reference file
4. Edit reference file with new content
5. Verify update applied
6. Notify user of changes

### Version History

- **v3.0.0 (2026-01-18):** Agent Skills Specification Compliance
  - Updated YAML frontmatter to Agent Skills spec (agentskills.io)
  - Added `license`, `compatibility`, `metadata`, `allowed-tools` fields
  - New reference: `agent-skills-spec.md` (complete specification docs)
  - Updated `core-features.md` with Agent Skills compliance section
  - Updated `configuration-guide.md` with DevForgeAI integration section
  - Updated `best-practices.md` with Agent Skills best practices
  - Claude Code Version: 2.1.12

- **v2.0 (2025-12-20):** December 2025 Features Update
  - Added Background Tasks & Agents documentation
  - Added Checkpoints & Rewind features
  - Added Named Sessions documentation
  - Added Claude Opus 4.5 configuration
  - Added Remote MCP with OAuth
  - Added IDE Integrations (Chrome, JetBrains, VS Code)
  - Added Claude Agent SDK documentation
  - Claude Code Version: 2.0.74

- **v1.0 (2025-11-06):** Initial creation
  - Migrated 15,788 lines from devforgeai/specs/Terminal/
  - Fetched 29 official web docs from code.claude.com
  - Created 9-file skill with progressive disclosure
  - 100% topic coverage (28/28)
  - 100% test pass rate (14/14)

---

## Usage Examples

### Example 1: Feature Discovery
```
User: "Can Claude Code create custom subagents?"

Skill triggers → Loads SKILL.md → Routes to core-features.md Section 1

Response: Complete subagent documentation with:
- What they are and how they work
- File locations and configuration
- YAML frontmatter structure
- Tool access and model selection
- Creation workflow (via /agents)
- 3 complete examples (code-reviewer, debugger, data-scientist)
- Best practices and troubleshooting
```

### Example 2: Configuration Help
```
User: "How do I configure permissions in Claude Code?"

Skill triggers → Loads configuration-guide.md Section 4

Response: Comprehensive permission guide with:
- Allow/ask/deny pattern explanation
- Tool-specific permission rules
- JSON configuration examples
- Security best practices
- Common permission scenarios
- Troubleshooting permission issues
```

### Example 3: Integration Setup
```
User: "How do I set up Claude Code in GitHub Actions?"

Skill triggers → Loads integration-patterns.md Section 1

Response: Complete GitHub Actions guide with:
- Quick setup vs manual installation
- Workflow YAML examples
- AWS Bedrock and Google Vertex configs
- Security and cost considerations
- Troubleshooting common issues
- Best practices for CI/CD
```

### Example 4: Troubleshooting
```
User: "Claude Code won't start on WSL - node not found error"

Skill triggers → Loads troubleshooting-guide.md Section 1

Response: Specific WSL solutions with:
- Root cause explanation (Windows Node vs WSL Node)
- Multiple fix options (nvm setup, PATH configuration)
- Diagnostic commands (which npm, which node)
- Step-by-step resolution
- Prevention guidance
```

### Example 5: Quick Lookup
```
User: "What are the keyboard shortcuts in Claude Code?"

Skill triggers → Loads quick-reference.md

Response: Complete shortcut reference:
- General controls (Ctrl+C, Ctrl+D, etc.)
- Input modes (newline shortcuts)
- Special functions (Tab, Shift+Tab, Esc Esc)
- Vim mode shortcuts (if enabled)
- Platform-specific variations
```

---

## Key Features

### 1. Comprehensive Knowledge Base
- **28 topics** covering all Claude Code Terminal features
- **100% official content** from Anthropic documentation
- **No speculation** - evidence-based only
- **Up-to-date** - web docs verified current (2025-11-06)

### 2. Progressive Disclosure
- **Level 1:** Metadata (100 tokens, always)
- **Level 2:** SKILL.md (2,000 tokens, when triggered)
- **Level 3:** References (1,200-3,500 tokens, as needed)
- **Result:** 95% token savings vs loading all docs

### 3. Self-Updating
- **29 official URLs** embedded for auto-updates
- **WebFetch mechanism** to retrieve latest docs
- **Comparison & update** procedure documented
- **User notification** on successful updates

### 4. Quick Access
- **Cheat sheet** for commands and shortcuts
- **Decision matrices** for feature selection
- **Troubleshooting** quick lookup tables
- **Configuration examples** for common scenarios

### 5. DevForgeAI Compliant
- Follows framework patterns
- Token-efficient architecture
- Native tool usage
- Team-shareable project skill

---

## Critical Content Highlights

### Token Efficiency (best-practices.md Section 4)

**MUST READ:** Native tools provide **40-73% token savings** vs Bash commands.

| Operation | Bash | Native Tool | Savings |
|-----------|------|-------------|---------|
| Read file | `cat file.py` | `Read(file_path="file.py")` | 40% |
| Search code | `grep -r "pattern"` | `Grep(pattern="pattern")` | 60% |
| Find files | `find . -name "*.ts"` | `Glob(pattern="**/*.ts")` | 73% |

**Real session impact:** 274K tokens (Bash) → 108K tokens (Native) = **61% savings**

**This is mandatory** - Claude Code system prompt requires native tools for file operations.

---

## Common Questions

### Q: When should I use this skill vs DevForgeAI skills?

**A:** Different purposes:
- **claude-code-terminal-expert:** Answers questions about Claude Code Terminal features ("How do I create a subagent?", "What MCP servers exist?")
- **DevForgeAI skills:** Execute development workflows (ideation, development, qa, release)

**This skill provides knowledge. DevForgeAI skills execute work.**

### Q: How do I trigger this skill?

**A:** Ask Claude any question about Claude Code Terminal. The skill's description includes 14 specific triggers + catch-all, so it activates automatically.

### Q: Will this slow down my conversations?

**A:** No. Progressive disclosure loads only needed content:
- Simple questions: ~2,100 tokens (just SKILL.md)
- Complex questions: ~4,500-7,000 tokens (SKILL.md + 1-2 references)
- Still 94-98% more efficient than loading all docs

### Q: How does the self-updating work?

**A:** When docs seem outdated, the skill:
1. Fetches latest from code.claude.com
2. Compares with current reference
3. Updates reference file
4. Notifies you of changes

### Q: Can I customize this skill for my team?

**A:** Yes! The skill is in `.claude/skills/` (project-level):
- Add team-specific examples to references
- Customize for your tech stack
- Add internal tool integrations
- Commit changes to git for team sharing

---

## Verification Reports

**Structure & Content:**
- See `SKILL-VERIFICATION.md` for complete verification
- All files created, all content migrated
- DevForgeAI pattern compliance verified
- Character budget analyzed (SKILL.md: 15,060 chars, 0.4% over acceptable)

**Testing:**
- See `SKILL-TESTING-REPORT.md` for test results
- 14/14 tests passed (100%)
- Progressive disclosure validated
- Self-updating mechanism verified
- Token efficiency measured

---

## Usage in DevForgeAI Framework

### Before This Skill
```
User: "Can Claude Code create subagents?"
Claude: "I'm not sure about Claude Code's capabilities. Let me search the documentation..."
[May not find answer, gives uncertain response]
```

### After This Skill
```
User: "Can Claude Code create subagents?"
Claude: [Skill triggers automatically]
"Yes! Claude Code has comprehensive subagent support. Here's how to create one..."
[Authoritative, complete answer from official docs]
```

### Framework Integration
- Reduces user friction when using terminal
- Enables self-service for feature questions
- Supports DevForgeAI workflows with terminal knowledge
- Minimal token impact on framework operations

---

## Production Readiness Checklist

- [x] All source content migrated (97% coverage)
- [x] All topics documented (100% coverage)
- [x] Progressive disclosure implemented
- [x] Self-updating mechanism functional
- [x] Testing complete (14/14 passed)
- [x] DevForgeAI integration verified
- [x] Token efficiency validated (95% savings)
- [x] Documentation updated (skills-reference.md, CLAUDE.md)
- [x] Source folder deleted (devforgeai/specs/Terminal/ removed)
- [x] Verification reports created

**Status:** ✅ READY FOR PRODUCTION USE

---

## Next Steps for Users

1. **Restart Claude Code Terminal** - Skill will be auto-discovered
2. **Ask Claude Code questions** - Skill triggers automatically
3. **Explore features** - Ask "What can Claude Code do?"
4. **Create components** - Ask "How do I create a subagent/skill/command?"
5. **Troubleshoot issues** - Ask "Claude Code won't start, help?"

---

## Contributing

### Reporting Gaps
If you find missing or outdated information:
1. Ask Claude: "The documentation for [topic] seems outdated"
2. Skill will fetch latest and update automatically
3. Or manually update the appropriate reference file

### Adding Content
To add team-specific examples or customizations:
1. Edit appropriate reference file in `references/`
2. Maintain official content, add team sections
3. Commit changes to git
4. Team members get updates on git pull

---

## Files in This Skill

| File | Purpose | Lines | Load Frequency |
|------|---------|-------|----------------|
| SKILL.md | Discovery & routing | 440 | When skill triggers |
| core-features.md | 5 main features | 2,428 | High |
| configuration-guide.md | Settings & config | 1,513 | High |
| integration-patterns.md | CI/CD & automation | 2,790 | Medium |
| troubleshooting-guide.md | Problem solving | 2,128 | Medium |
| advanced-features.md | Enterprise & security | 3,553 | Low |
| best-practices.md | Optimization & patterns | 1,230 | Medium |
| quick-reference.md | Cheat sheet | 726 | High |
| comparison-matrix.md | Decision guidance | 600 | Medium |

---

## Support

**Questions about this skill:**
- Check SKILL.md for usage guidance
- Review SKILL-VERIFICATION.md for structure details
- See SKILL-TESTING-REPORT.md for test coverage

**Questions about Claude Code Terminal:**
- Just ask Claude - this skill provides the answers!

**Report issues:**
- Use `/bug` in Claude Code Terminal
- Or open issue in DevForgeAI project

---

**The claude-code-terminal-expert skill makes Claude an authoritative expert on all Claude Code Terminal features, eliminating the "Claude doesn't know this" problem and enabling self-service feature discovery.**

**Version:** 1.0 | **Status:** Production Ready | **Last Updated:** 2025-11-06
