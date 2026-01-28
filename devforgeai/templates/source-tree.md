# Source Tree Structure - DevForgeAI Framework

**Status**: LOCKED
**Last Updated**: 2025-11-20
**Version**: 2.0 (STORY-048: Added src/ distribution structure)

## CRITICAL RULE: Framework Organization

This file defines WHERE framework components belong in the DevForgeAI repository. Projects using DevForgeAI will have their own source-tree.md files created by the devforgeai-architecture skill.

---

## Framework Directory Structure

```
DevForgeAI2/
├── .claude/                     # Claude Code Terminal configuration (OPERATIONAL)
│   ├── skills/                  # Framework implementation (16 skills)
│   │   ├── devforgeai-ideation/
│   │   │   ├── SKILL.md         # Main skill (500-800 lines)
│   │   │   ├── references/      # Deep documentation (loaded on demand)
│   │   │   │   ├── discovery-workflow.md
│   │   │   │   ├── requirements-elicitation-workflow.md
│   │   │   │   ├── complexity-assessment-workflow.md
│   │   │   │   ├── epic-decomposition-workflow.md
│   │   │   │   ├── feasibility-analysis-workflow.md
│   │   │   │   ├── artifact-generation.md
│   │   │   │   ├── self-validation-workflow.md
│   │   │   │   ├── completion-handoff.md
│   │   │   │   ├── user-interaction-patterns.md
│   │   │   │   ├── error-handling.md
│   │   │   │   ├── requirements-elicitation-guide.md
│   │   │   │   ├── complexity-assessment-matrix.md
│   │   │   │   ├── domain-specific-patterns.md
│   │   │   │   ├── feasibility-analysis-framework.md
│   │   │   │   ├── validation-checklists.md
│   │   │   │   └── output-templates.md
│   │   │   └── assets/
│   │   │       └── templates/
│   │   │           ├── epic-template.md
│   │   │           ├── requirements-spec-template.md
│   │   │           ├── feature-prioritization-matrix.md
│   │   │           └── user-persona-template.md
│   │   ├── devforgeai-architecture/
│   │   │   ├── SKILL.md
│   │   │   ├── references/
│   │   │   │   ├── project-context-discovery.md
│   │   │   │   ├── context-file-creation.md
│   │   │   │   ├── adr-creation.md
│   │   │   │   ├── spec-validation.md
│   │   │   │   ├── ambiguity-detection.md
│   │   │   │   └── completion-handoff.md
│   │   │   └── assets/
│   │   │       └── templates/
│   │   │           ├── tech-stack-template.md
│   │   │           ├── source-tree-template.md
│   │   │           ├── dependencies-template.md
│   │   │           ├── coding-standards-template.md
│   │   │           ├── architecture-constraints-template.md
│   │   │           ├── anti-patterns-template.md
│   │   │           └── adr-template.md
│   │   ├── devforgeai-orchestration/
│   │   │   ├── SKILL.md
│   │   │   └── references/
│   │   │       ├── skill-invocation.md
│   │   │       ├── workflow-state-machine.md
│   │   │       ├── quality-gates.md
│   │   │       └── story-lifecycle-management.md
│   │   ├── devforgeai-story-creation/
│   │   │   ├── SKILL.md
│   │   │   ├── references/
│   │   │   │   ├── requirements-analysis.md
│   │   │   │   ├── acceptance-criteria-creation.md
│   │   │   │   ├── technical-specification-creation.md
│   │   │   │   ├── ui-specification-creation.md
│   │   │   │   ├── self-validation.md
│   │   │   │   └── completion-handoff.md
│   │   │   └── assets/
│   │   │       ├── templates/
│   │   │       │   └── story-template.md
│   │   │       └── contracts/
│   │   │           ├── requirements-analyst-contract.yaml
│   │   │           └── api-designer-contract.yaml
│   │   ├── devforgeai-ui-generator/
│   │   │   ├── SKILL.md
│   │   │   ├── references/
│   │   │   │   ├── ui-type-detection.md
│   │   │   │   ├── technology-selection.md
│   │   │   │   ├── component-generation.md
│   │   │   │   └── spec-validation.md
│   │   │   └── assets/
│   │   │       └── templates/
│   │   │           ├── web-component-template.md
│   │   │           ├── gui-component-template.md
│   │   │           └── terminal-component-template.md
│   │   ├── devforgeai-development/
│   │   │   ├── SKILL.md
│   │   │   └── references/
│   │   │       ├── preflight-validation.md
│   │   │       ├── tdd-red-phase.md
│   │   │       ├── tdd-green-phase.md
│   │   │       ├── tdd-refactor-phase.md
│   │   │       ├── integration-testing.md
│   │   │       ├── phase-4.5-deferral-challenge.md
│   │   │       ├── dod-update-workflow.md
│   │   │       ├── ac-checklist-update-workflow.md
│   │   │       └── git-workflow-conventions.md
│   │   ├── devforgeai-qa/
│   │   │   ├── SKILL.md
│   │   │   ├── references/
│   │   │   │   ├── coverage-analysis-workflow.md
│   │   │   │   ├── code-quality-workflow.md
│   │   │   │   ├── anti-pattern-detection-workflow.md
│   │   │   │   ├── spec-compliance-workflow.md
│   │   │   │   ├── validation-procedures.md
│   │   │   │   ├── report-generation.md
│   │   │   │   └── dod-protocol.md
│   │   │   ├── assets/
│   │   │   │   ├── config/
│   │   │   │   │   ├── coverage-thresholds.md
│   │   │   │   │   ├── quality-metrics.md
│   │   │   │   │   └── security-policies.md
│   │   │   │   └── templates/
│   │   │   │       └── qa-report-template.md
│   │   │   └── scripts/
│   │   │       ├── analyze_complexity.py
│   │   │       ├── detect_duplicates.py
│   │   │       ├── generate_coverage_report.py
│   │   │       ├── generate_test_stubs.py
│   │   │       ├── security_scan.py
│   │   │       └── validate_spec_compliance.py
│   │   ├── devforgeai-release/
│   │   │   ├── SKILL.md
│   │   │   └── references/
│   │   │       ├── deployment-strategies.md
│   │   │       ├── smoke-testing-guide.md
│   │   │       ├── rollback-procedures.md
│   │   │       ├── monitoring-metrics.md
│   │   │       └── release-checklist.md
│   │   ├── devforgeai-documentation/
│   │   │   ├── SKILL.md
│   │   │   ├── references/
│   │   │   │   ├── mode-selection.md
│   │   │   │   ├── greenfield-workflow.md
│   │   │   │   ├── brownfield-workflow.md
│   │   │   │   ├── architecture-documentation.md
│   │   │   │   ├── api-documentation.md
│   │   │   │   ├── user-guides.md
│   │   │   │   └── coverage-validation.md
│   │   │   └── assets/
│   │   │       └── templates/
│   │   │           ├── readme-template.md
│   │   │           ├── api-doc-template.md
│   │   │           ├── developer-guide-template.md
│   │   │           └── architecture-diagram-template.md
│   │   ├── devforgeai-feedback/
│   │   │   ├── SKILL.md
│   │   │   ├── references/
│   │   │   │   ├── session-initialization.md
│   │   │   │   ├── challenge-detection.md
│   │   │   │   ├── insight-extraction.md
│   │   │   │   ├── improvement-recommendation.md
│   │   │   │   └── session-finalization.md
│   │   │   └── assets/
│   │   │       ├── config/
│   │   │       │   └── feedback-config.json
│   │   │       └── templates/
│   │   │           └── session-template.md
│   │   ├── devforgeai-rca/
│   │   │   ├── SKILL.md
│   │   │   ├── references/
│   │   │   │   ├── file-discovery.md
│   │   │   │   ├── five-whys-analysis.md
│   │   │   │   ├── evidence-collection.md
│   │   │   │   ├── recommendation-generation.md
│   │   │   │   └── rca-document-creation.md
│   │   │   └── assets/
│   │   │       └── templates/
│   │   │           └── rca-template.md
│   │   ├── devforgeai-subagent-creation/
│   │   │   ├── SKILL.md
│   │   │   └── references/
│   │   │       ├── requirements-gathering.md
│   │   │       ├── agent-generation.md
│   │   │       └── validation.md
│   │   ├── devforgeai-mcp-cli-converter/
│   │   │   ├── SKILL.md
│   │   │   └── references/
│   │   │       └── conversion-workflow.md
│   │   ├── claude-code-terminal-expert/
│   │   │   ├── SKILL.md
│   │   │   └── references/
│   │   │       └── terminal-knowledge-base.md
│   │   ├── internet-sleuth-integration/
│   │   │   └── SKILL.md (incomplete - use internet-sleuth subagent instead)
│   │   └── skill-creator/
│   │       └── SKILL.md
│   │
│   ├── agents/                  # Specialized subagents (30 agents)
│   │   ├── agent-generator.md
│   │   ├── anti-pattern-scanner.md
│   │   ├── api-designer.md
│   │   ├── architect-reviewer.md
│   │   ├── backend-architect.md
│   │   ├── code-analyzer.md
│   │   ├── code-quality-auditor.md
│   │   ├── code-reviewer.md
│   │   ├── context-validator.md
│   │   ├── coverage-analyzer.md
│   │   ├── deferral-validator.md
│   │   ├── deployment-engineer.md
│   │   ├── dev-result-interpreter.md
│   │   ├── documentation-writer.md
│   │   ├── frontend-developer.md
│   │   ├── git-validator.md
│   │   ├── integration-tester.md
│   │   ├── internet-sleuth.md
│   │   ├── pattern-compliance-auditor.md
│   │   ├── qa-result-interpreter.md
│   │   ├── refactoring-specialist.md
│   │   ├── requirements-analyst.md
│   │   ├── security-auditor.md
│   │   ├── sprint-planner.md
│   │   ├── story-requirements-analyst.md
│   │   ├── tech-stack-detector.md
│   │   ├── technical-debt-analyzer.md
│   │   ├── test-automator.md
│   │   └── ui-spec-formatter.md
│   │
│   ├── commands/                # User-facing workflows (23 commands)
│   │   ├── audit-budget.md
│   │   ├── audit-deferrals.md
│   │   ├── audit-hooks.md
│   │   ├── create-agent.md
│   │   ├── create-context.md
│   │   ├── create-epic.md
│   │   ├── create-sprint.md
│   │   ├── create-story.md
│   │   ├── create-ui.md
│   │   ├── dev.md
│   │   ├── document.md
│   │   ├── export-feedback.md
│   │   ├── feedback-config.md
│   │   ├── feedback-export-data.md
│   │   ├── feedback-reindex.md
│   │   ├── feedback-search.md
│   │   ├── feedback.md
│   │   ├── ideate.md
│   │   ├── import-feedback.md
│   │   ├── orchestrate.md
│   │   ├── qa.md
│   │   ├── rca.md
│   │   └── release.md
│   │
│   ├── memory/                  # Progressive disclosure references
│   │   ├── skills-reference.md
│   │   ├── subagents-reference.md
│   │   ├── commands-reference.md
│   │   ├── documentation-command-guide.md
│   │   ├── qa-automation.md
│   │   ├── context-files-guide.md
│   │   ├── ui-generator-guide.md
│   │   ├── token-efficiency.md
│   │   ├── epic-creation-guide.md
│   │   ├── token-budget-guidelines.md
│   │   └── skill-execution-troubleshooting.md
│   │
│   └── scripts/                 # DevForgeAI CLI tools
│       ├── devforgeai_cli/
│       │   ├── __init__.py
│       │   ├── cli.py
│       │   ├── validators/
│       │   │   ├── dod_validator.py
│       │   │   ├── git_validator.py
│       │   │   └── context_validator.py
│       │   └── README.md
│       ├── install_hooks.sh
│       └── setup.py
│
├── devforgeai/                 # Framework's own context (OPERATIONAL - meta-level)
│   ├── context/                 # Framework architectural constraints
│   │   ├── tech-stack.md        # Framework implementation constraints
│   │   ├── source-tree.md       # This file
│   │   ├── dependencies.md      # Framework dependencies
│   │   ├── coding-standards.md  # Framework coding patterns
│   │   ├── architecture-constraints.md  # Framework design rules
│   │   └── anti-patterns.md     # Framework anti-patterns
│   │
│   ├── RCA/                     # Root Cause Analysis documents
│   │   ├── RCA-006-autonomous-deferrals.md
│   │   ├── RCA-007-multi-file-story-creation.md
│   │   ├── RCA-008-autonomous-git-stashing.md
│   │   ├── RCA-009-skill-invocation-stopped-prematurely.md
│   │   └── [additional RCA documents...]
│   │
│   ├── adrs/                    # Architecture Decision Records
│   │   ├── ADR-001-markdown-for-documentation.md
│   │   ├── ADR-002-skills-over-monolithic-workflows.md
│   │   ├── ADR-003-subagents-for-parallelism.md
│   │   └── [additional ADRs...]
│   │
│   ├── protocols/               # Framework protocols and patterns
│   │   ├── lean-orchestration-pattern.md
│   │   ├── refactoring-case-studies.md
│   │   ├── command-budget-reference.md
│   │   ├── hook-integration-pattern.md
│   │   └── troubleshooting-lean-orchestration-violations.md
│   │
│   ├── qa/                      # QA validation configuration
│   │   ├── coverage-thresholds.md
│   │   ├── quality-metrics.md
│   │   └── reports/             # Per-story QA reports (generated)
│   │
│   ├── feedback/                # Feedback system data
│   │   ├── sessions/            # Individual feedback sessions
│   │   └── feedback-index.json  # Session index
│   │
│   ├── specs/                   # Planning and specifications
│   │   ├── requirements/
│   │   ├── ui/                  # UI specifications (generated by /create-ui)
│   │   └── STRUCTURED-FORMAT-SPECIFICATION.md
│   │
│   ├── deployment/              # Deployment configurations
│   ├── tests/                   # Framework test files
│   ├── scripts/                 # Utility scripts
│   ├── backups/                 # Backup files
│   └── validation/              # Validation artifacts
│
├── src/                         # DISTRIBUTION SOURCE (installer deployment)
│   ├── claude/                  # Claude Code configuration (source)
│   │   ├── agents/              # All 30 subagents (source copies)
│   │   ├── commands/            # All 23 commands (source copies)
│   │   ├── skills/              # All 16 skills (source copies)
│   │   ├── memory/              # Progressive disclosure references
│   │   └── scripts/             # DevForgeAI CLI tools
│   │
│   ├── devforgeai/              # DevForgeAI configuration (source)
│   │   ├── config/              # Configuration templates
│   │   ├── protocols/           # Framework protocols
│   │   ├── specs/               # Specification templates
│   │   ├── templates/           # Document templates
│   │   └── feedback/            # Feedback system templates
│   │
│   ├── scripts/                 # Installer scripts
│   │   ├── install.sh           # Main installer
│   │   ├── update-framework.sh  # Framework updater
│   │   ├── validate-installation.sh
│   │   ├── rollback-*.sh        # Version rollback scripts
│   │   └── audit-*.sh           # Audit scripts
│   │
│   ├── CLAUDE.md                # Template CLAUDE.md (installer merges with user's)
│   ├── README.md                # Distribution README
│   ├── version.json             # Version metadata
│   └── checksums.txt            # File integrity checksums
│
├── devforgeai/specs/                    # Project management and research
│   ├── Epics/                   # High-level business initiatives
│   ├── Sprints/                 # 2-week iteration plans
│   ├── Stories/                 # Atomic work units
│   ├── Terminal/                # Claude Code Terminal research
│   │   ├── sub-agents.md
│   │   ├── agent-skills.md
│   │   ├── slash-commands-best-practices.md
│   │   ├── native-tools-vs-bash-efficiency-analysis.md
│   │   └── ...
│   ├── Workflows.md             # Workflow architecture research
│   └── prompt-engineering-best-practices.md
│
├── docs/                        # Framework documentation
│   ├── architecture/            # Architecture documentation
│   │   ├── decisions/           # Architecture Decision Records (ADRs)
│   │   │   ├── ADR-001-markdown-for-documentation.md
│   │   │   ├── ADR-002-skills-over-monolithic-workflows.md
│   │   │   ├── ADR-003-subagents-for-parallelism.md
│   │   │   └── ADR-NNN-[decision-name].md
│   │   ├── diagrams/            # Architecture diagrams (Mermaid)
│   │   └── patterns/            # Design patterns documentation
│   │
│   ├── guides/                  # User guides
│   │   ├── quickstart.md
│   │   ├── skill-development.md
│   │   └── subagent-development.md
│   │
│   └── api/                     # API specifications
│       ├── skill-api.md
│       ├── subagent-api.md
│       └── command-api.md
│
├── CLAUDE.md                    # Claude Code project instructions (main entry point)
├── README.md                    # Framework overview and quick start
├── ROADMAP.md                   # Implementation phases and timelines
├── LICENSE                      # MIT License
└── .gitignore                   # Git ignore rules
```

---

## Directory Purpose and Rules

### Dual-Location Architecture (STORY-048)

**DevForgeAI maintains TWO parallel structures:**

1. **OPERATIONAL folders** (`.claude/` and `devforgeai/`) - Used by Claude Code Terminal during development
2. **DISTRIBUTION source** (`src/`) - Clean copies for external deployment via installer

**Why both exist:**
- Operational folders contain working files, backups, generated outputs, temporary files
- Distribution source contains ONLY framework essentials for installer deployment
- Installer copies from `src/` → target project's `.claude/` and `devforgeai/`
- Keeps distribution clean while allowing messy operational workspace

**Update protocol:**
- Changes made in `.claude/` and `devforgeai/` (operational)
- Periodically sync to `src/` for distribution
- Installer reads from `src/` only

---

### `.claude/` - Claude Code Configuration (OPERATIONAL - LOCKED)

**Purpose**: Claude Code Terminal automatically discovers skills, subagents, and commands from this directory.

**Rules**:
- ✅ ALL skills go in `.claude/skills/[skill-name]/`
- ✅ ALL subagents go in `.claude/agents/[agent-name].md`
- ✅ ALL slash commands go in `.claude/commands/[command-name].md`
- ✅ Contains 16 skills, 30 subagents, 23 commands (as of 2025-11-20)
- ❌ NO executable code in `.claude/` (Markdown documentation only)
- ❌ NO language-specific implementations (framework must be agnostic)

**Rationale**: Claude Code Terminal's discovery mechanism requires this exact structure.

### `.claude/skills/` - Framework Skills (LOCKED)

**Purpose**: Autonomous, model-invoked capabilities for each development phase.

**Rules**:
- ✅ Each skill in its own subdirectory (e.g., `devforgeai-development/`)
- ✅ Main skill file MUST be named `SKILL.md`
- ✅ SKILL.md MUST have YAML frontmatter with `name:` and `description:`
- ✅ Keep SKILL.md under 1,000 lines (target: 500-800 lines)
- ✅ Deep documentation goes in `references/` subdirectory
- ✅ Templates and assets go in `assets/` subdirectory
- ❌ NO skills in root `.claude/` directory
- ❌ NO executable scripts in skill directories (documentation only)

**Naming Convention**: `devforgeai-[phase]` (e.g., `devforgeai-architecture`)

**Example**:
```
.claude/skills/devforgeai-development/
├── SKILL.md                 # Main skill (500-800 lines)
└── references/              # Loaded on demand
    ├── tdd-workflow-guide.md
    └── refactoring-patterns.md
```

### `.claude/agents/` - Specialized Subagents (LOCKED)

**Purpose**: Domain-specific AI workers with separate context windows.

**Rules**:
- ✅ Each subagent is a single `.md` file
- ✅ File name becomes subagent name (e.g., `test-automator.md` → `test-automator`)
- ✅ MUST have YAML frontmatter with `name:`, `description:`, `tools:`, `model:`
- ✅ Keep under 500 lines (target: 100-300 lines)
- ✅ Single responsibility per subagent
- ❌ NO subdirectories in `.claude/agents/`
- ❌ NO multi-responsibility subagents

**Naming Convention**: `[domain]-[role]` (e.g., `test-automator`, `backend-architect`)

**Current components (30 total):**
```
.claude/agents/
├── agent-generator.md
├── anti-pattern-scanner.md
├── api-designer.md
├── architect-reviewer.md
├── backend-architect.md
├── code-analyzer.md
├── code-quality-auditor.md
├── code-reviewer.md
├── context-validator.md
├── coverage-analyzer.md
├── deferral-validator.md
├── deployment-engineer.md
├── dev-result-interpreter.md
├── documentation-writer.md
├── frontend-developer.md
├── git-validator.md
├── integration-tester.md
├── internet-sleuth.md
├── pattern-compliance-auditor.md
├── qa-result-interpreter.md
├── refactoring-specialist.md
├── requirements-analyst.md
├── security-auditor.md
├── sprint-planner.md
├── story-requirements-analyst.md
├── tech-stack-detector.md
├── technical-debt-analyzer.md
├── test-automator.md
└── ui-spec-formatter.md
```

### `.claude/commands/` - Slash Commands (LOCKED)

**Purpose**: User-invoked, parameterized workflows.

**Rules**:
- ✅ Each command is a single `.md` file
- ✅ File name becomes command name (e.g., `dev.md` → `/dev`)
- ✅ MUST have YAML frontmatter with `description:` and `argument-hint:`
- ✅ Keep under 500 lines (target: 200-400 lines)
- ✅ Use `$ARGUMENTS` placeholder for parameters
- ✅ Can invoke skills and subagents
- ❌ NO subdirectories in `.claude/commands/` (flat structure)
- ❌ NO commands exceeding 500 lines (extract to skills)

**Naming Convention**: `[action]` or `[action]-[object]` (e.g., `dev`, `create-context`)

**Current commands (23 total):**
```
.claude/commands/
├── audit-budget.md          # /audit-budget
├── audit-deferrals.md       # /audit-deferrals
├── audit-hooks.md           # /audit-hooks [--validate|--performance|--check-circular]
├── create-agent.md          # /create-agent [name] [options]
├── create-context.md        # /create-context [project-name]
├── create-epic.md           # /create-epic [epic-name]
├── create-sprint.md         # /create-sprint [sprint-name]
├── create-story.md          # /create-story [feature-description | epic-id]
├── create-ui.md             # /create-ui [STORY-ID or component-description]
├── dev.md                   # /dev [STORY-ID]
├── document.md              # /document [STORY-ID | --type=TYPE | --mode=MODE]
├── export-feedback.md       # /export-feedback [--date-range RANGE] [--sanitize true/false]
├── feedback-config.md       # /feedback-config [view|edit|reset] [field] [value]
├── feedback-export-data.md  # /feedback-export-data [--format] [--date-range] [--story-ids]
├── feedback-reindex.md      # /feedback-reindex
├── feedback-search.md       # /feedback-search [query] [--severity] [--status] [--limit]
├── feedback.md              # /feedback [context]
├── ideate.md                # /ideate [business-idea-description]
├── import-feedback.md       # /import-feedback <archive-path>
├── orchestrate.md           # /orchestrate [STORY-ID]
├── qa.md                    # /qa [STORY-ID] [mode]
├── rca.md                   # /rca [issue-description] [severity]
└── release.md               # /release [STORY-ID] [environment]
```

### `devforgeai/` - Framework Context (OPERATIONAL - LOCKED)

**Purpose**: Framework's own architectural constraints (meta-level).

**Rules**:
- ✅ Framework's context files go in `devforgeai/specs/context/`
- ✅ QA configuration goes in `devforgeai/qa/`
- ✅ Specifications go in `devforgeai/specs/`
- ✅ RCA documents go in `devforgeai/RCA/`
- ✅ Protocols go in `devforgeai/protocols/`
- ✅ Feedback data goes in `devforgeai/feedback/`
- ❌ NO project-specific files in `devforgeai/` (this is framework meta-context)
- ❌ NO executable code in `devforgeai/` (documentation only)

**Rationale**: Projects using DevForgeAI will have their own `devforgeai/specs/context/` files created by devforgeai-architecture skill.

---

### `src/` - Distribution Source (STORY-048 - LOCKED)

**Purpose**: Clean, version-controlled source files for installer deployment to external projects.

**Rules**:
- ✅ Contains ONLY essential framework files (no backups, no generated outputs)
- ✅ `src/claude/` mirrors `.claude/` structure (skills, agents, commands, memory, scripts)
- ✅ `src/devforgeai/` contains distribution templates (config, protocols, specs, templates)
- ✅ `src/scripts/` contains installer and update scripts
- ✅ Version metadata in `version.json` and integrity checks in `checksums.txt`
- ✅ Template `CLAUDE.md` for installer merge with user's existing file
- ❌ NO operational files (backups, test outputs, generated reports)
- ❌ NO .backup files or temporary files
- ❌ NO user-specific configurations

**Update workflow:**
1. Make changes in `.claude/` or `devforgeai/` (operational folders)
2. Test changes thoroughly
3. Sync essential files to `src/` (excluding backups, test outputs)
4. Update `version.json` with new version number
5. Regenerate `checksums.txt` for integrity validation
6. Commit `src/` changes for distribution

**Installer behavior:**
- Reads framework files from `src/claude/` and `src/devforgeai/`
- Deploys to target project's `.claude/` and `devforgeai/`
- Merges template `CLAUDE.md` with user's existing file (preserves user instructions)
- Creates timestamped backups before deployment
- Validates checksums for integrity
- Supports rollback to previous version

**Rationale**: Separates clean distribution source from messy operational workspace, enables versioned external deployment.

### `devforgeai/specs/` - Project Management (LOCKED)

**Purpose**: Epics, sprints, stories, and research documentation.

**Rules**:
- ✅ Epics go in `devforgeai/specs/Epics/`
- ✅ Sprints go in `devforgeai/specs/Sprints/`
- ✅ Stories go in `devforgeai/specs/Stories/`
- ✅ Research documentation in `devforgeai/specs/Terminal/`
- ✅ Stories MUST have YAML frontmatter with id, title, epic, sprint, status, points, priority
- ❌ NO code in `devforgeai/specs/` (documentation only)

**Story Naming**: `STORY-NNN-[title].md` (e.g., `STORY-001-user-authentication.md`)
**Epic Naming**: `EPIC-NNN-[title].md` (e.g., `EPIC-001-user-management.md`)
**Sprint Naming**: `SPRINT-NNN.md` (e.g., `SPRINT-001.md`)

### `docs/` - Framework Documentation (LOCKED)

**Purpose**: Architecture documentation, ADRs, guides, API specs.

**Rules**:
- ✅ ADRs go in `docs/architecture/decisions/`
- ✅ Diagrams go in `docs/architecture/diagrams/`
- ✅ User guides go in `docs/guides/`
- ✅ API specifications go in `docs/api/`
- ❌ NO generated documentation (commit only source)
- ❌ NO language-specific docs (framework must be agnostic)

**ADR Naming**: `ADR-NNN-[decision-title].md` (e.g., `ADR-001-markdown-for-documentation.md`)

---

## File Naming Conventions

### Skills

**Pattern**: `devforgeai-[phase]`
**Examples**:
- ✅ `devforgeai-ideation`
- ✅ `devforgeai-architecture`
- ✅ `devforgeai-development`
- ❌ `IdeationSkill` (no CamelCase)
- ❌ `dev-skill` (use full phase name)

### Subagents

**Pattern**: `[domain]-[role]`
**Examples**:
- ✅ `test-automator`
- ✅ `backend-architect`
- ✅ `deployment-engineer`
- ❌ `TestAutomator` (no CamelCase)
- ❌ `test_automator` (use hyphens, not underscores)

### Slash Commands

**Pattern**: `[action]` or `[action]-[object]`
**Examples**:
- ✅ `dev`
- ✅ `qa`
- ✅ `create-context`
- ✅ `create-story`
- ❌ `DevCommand` (no CamelCase)
- ❌ `create_context` (use hyphens, not underscores)

### Context Files

**Pattern**: `[purpose].md` (all lowercase, hyphens)
**Required Files**:
- `tech-stack.md`
- `source-tree.md`
- `dependencies.md`
- `coding-standards.md`
- `architecture-constraints.md`
- `anti-patterns.md`

**Examples**:
- ✅ `tech-stack.md`
- ✅ `anti-patterns.md`
- ❌ `TechStack.md` (no CamelCase)
- ❌ `tech_stack.md` (use hyphens, not underscores)

### Documentation Files

**Pattern**: `[topic].md` or `[topic]-[subtopic].md`
**Examples**:
- ✅ `README.md`
- ✅ `ROADMAP.md`
- ✅ `tdd-workflow-guide.md`
- ✅ `complexity-assessment-matrix.md`
- ❌ `readme.md` (use UPPERCASE for root docs)
- ❌ `TDDWorkflowGuide.md` (no CamelCase for reference docs)

---

## Forbidden Patterns

### ❌ FORBIDDEN: Monolithic Skills

**Wrong**:
```
.claude/skills/
└── devforgeai-all-in-one/
    └── SKILL.md    # 5,000 lines doing everything
```

**Correct**:
```
.claude/skills/
├── devforgeai-ideation/
├── devforgeai-architecture/
├── devforgeai-development/
├── devforgeai-qa/
└── devforgeai-release/
```

**Rationale**: Modularity enables independent updates and token efficiency.

### ❌ FORBIDDEN: Executable Code in Framework

**Wrong**:
```
.claude/skills/devforgeai-development/
├── SKILL.md
└── scripts/
    └── implement.py    # Python implementation code
```

**Correct**:
```
.claude/skills/devforgeai-development/
├── SKILL.md
└── references/
    └── tdd-workflow-guide.md    # Documentation only
```

**Rationale**: Framework must be language-agnostic. Skills provide instructions, not code.

### ❌ FORBIDDEN: Flat Command Structure

**Wrong**:
```
.claude/commands/
├── dev-backend.md
├── dev-frontend.md
├── dev-database.md
└── dev-tests.md
```

**Correct**:
```
.claude/commands/
└── dev.md    # Single command that handles all development
```

**Rationale**: Commands should orchestrate subagents for specialization, not duplicate command for each domain.

### ❌ FORBIDDEN: Context Files Outside `devforgeai/specs/context/`

**Wrong**:
```
.claude/
├── tech-stack.md    # ❌ Wrong location
└── skills/
```

**Correct**:
```
devforgeai/specs/context/
├── tech-stack.md    # ✅ Correct location
```

**Rationale**: Consistent location for AI agents to discover constraints.

---

## Progressive Disclosure Pattern

**Principle**: Keep main files concise, deep details in references.

**Example**:
```
.claude/skills/devforgeai-ideation/
├── SKILL.md (500 lines)
│   # Phase 1: Discovery
│   # Phase 2: Requirements Elicitation
│   # For detailed questions by domain, see references/requirements-elicitation-guide.md
│   # Phase 3: Complexity Assessment
│   # For scoring rubric, see references/complexity-assessment-matrix.md
│
└── references/
    ├── requirements-elicitation-guide.md (1,000 lines)
    ├── complexity-assessment-matrix.md (800 lines)
    ├── domain-specific-patterns.md (1,200 lines)
    └── feasibility-analysis-framework.md (600 lines)
```

**Benefit**: SKILL.md loads immediately (~20K tokens), references load only when needed (saving 60-80% tokens).

---

## Project Context Pattern (For Projects Using DevForgeAI)

When devforgeai-architecture skill creates context for a **project** using DevForgeAI:

```
my-project/
├── devforgeai/
│   └── context/
│       ├── tech-stack.md        # Project's tech choices (e.g., C#, React, PostgreSQL)
│       ├── source-tree.md       # Project's structure (e.g., Clean Architecture)
│       ├── dependencies.md      # Project's packages (e.g., Dapper 2.1.28)
│       ├── coding-standards.md  # Project's patterns (e.g., async/await rules)
│       ├── architecture-constraints.md  # Project's layer rules
│       └── anti-patterns.md     # Project's forbidden patterns
```

**Distinction**:
- **DevForgeAI's `devforgeai/specs/context/`**: Framework's own constraints (meta-level)
- **Project's `devforgeai/specs/context/`**: Project-specific constraints (implementation-level)

---

## Enforcement Checklist

Before committing framework changes:
- [ ] Skills are in `.claude/skills/[skill-name]/` with SKILL.md
- [ ] Subagents are in `.claude/agents/[agent-name].md`
- [ ] Commands are in `.claude/commands/[command-name].md`
- [ ] Context files are in `devforgeai/specs/context/`
- [ ] ADRs are in `docs/architecture/decisions/`
- [ ] NO executable code in `.claude/` or `devforgeai/`
- [ ] ALL components use Markdown format (not JSON/YAML)
- [ ] File naming follows conventions (lowercase, hyphens)
- [ ] Main files under size limits (skills <1000 lines, commands <500 lines)
- [ ] Reference documentation uses progressive disclosure

---

## References

- [CLAUDE.md](src/CLAUDE.md) - Project instructions for Claude Code
- [README.md](README.md) - Framework overview
- [tech-stack.md](tech-stack.md) - Technology constraints
- [Claude Code Skills Documentation](https://docs.claude.com/en/docs/claude-code/agent-skills)

---

**REMEMBER**: This source-tree.md defines the **framework's own structure**. Projects using DevForgeAI will have their own source-tree.md files created by the devforgeai-architecture skill based on project architecture patterns.
