---
name: devforgeai-github-actions
description: Generate GitHub Actions workflows for headless DevForgeAI /dev and /qa execution. Creates dev-story.yml, qa-validation.yml, parallel-stories.yml workflows with cost optimization, prompt caching, and Haiku preference. Use when setting up CI/CD automation for DevForgeAI projects.
model: claude-model: opus-4-5-20251001
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - AskUserQuestion
  - Bash(git:*)
  - Task
---

# DevForgeAI GitHub Skill

Generate GitHub Actions workflows for headless DevForgeAI execution with cost optimization.

---

## ⚠️ EXECUTION MODEL: This Skill Expands Inline

**After invocation, YOU (Claude) execute these instructions phase by phase.**

Do NOT wait passively - execute the workflow now.

---

## Parameter Extraction

Extracts configuration from context (github-actions.yaml if exists, or uses defaults).

**Configuration Priority:**
1. User-provided answers (AskUserQuestion)
2. Existing devforgeai/config/ci/github-actions.yaml
3. Defaults from skill (max_parallel_jobs: 5, enable_prompt_caching: true)

---

## Security Prerequisites

All workflows require GitHub Secrets to be configured before execution.

### Required Secrets

| Secret Name | Description | Source |
|-------------|-------------|--------|
| `ANTHROPIC_API_KEY` | Claude API authentication | [console.anthropic.com](https://console.anthropic.com) |
| `GITHUB_TOKEN` | Repository access (auto-provided by GitHub Actions) | Automatic |

### Setup Instructions

1. Navigate to repository **Settings > Secrets and variables > Actions**
2. Click **New repository secret**
3. Name: `ANTHROPIC_API_KEY`
4. Value: Your API key from console.anthropic.com
5. Click **Add secret**

### Security Validation

Workflows fail-fast if secrets are missing:
- Missing `ANTHROPIC_API_KEY`: "Error: ANTHROPIC_API_KEY secret not configured"
- Check GitHub Actions logs for validation errors

### Best Practices

- **Never commit API keys** to repository
- Use **repository secrets** for sensitive values
- Rotate keys periodically
- Use **environment protection rules** for production deployments

---

## Purpose

Generate GitHub Actions workflows that:
1. Execute /dev and /qa in headless mode
2. Support parallel story development via matrix
3. Optimize costs with prompt caching and Haiku model
4. Handle AskUserQuestion via ci-answers.yaml
5. Track execution costs for compliance with <$0.15/story target

---

## Generation Workflow (5 Phases)

### Phase 1: Pre-Generation Validation
**Purpose:** Validate prerequisites exist
**Validates:** Git repo, .github/ directory, context files
**Output:** Validation passed or failure details

### Phase 2: Configuration Loading
**Purpose:** Load or create configuration files
**Creates:** devforgeai/config/ci/github-actions.yaml, ci-answers.yaml if missing
**Output:** Configuration loaded and valid

### Phase 3: Workflow Template Generation
**Purpose:** Generate 4 workflow YAML files
**Templates:**
- dev-story.yml (AC#1: /dev execution)
- qa-validation.yml (AC#2: PR quality gate)
- parallel-stories.yml (AC#3: Matrix parallel)
- installer-testing.yml (optional)
**Output:** Workflows created in .github/workflows/

### Phase 4: Cost Optimization Configuration
**Purpose:** Apply cost optimization settings
**Applies:** Prompt caching, Haiku model, max-turns limits
**Output:** Workflows optimized

### Phase 5: Validation & Summary
**Purpose:** Validate generated files and display summary
**Output:** Setup complete, next steps documented

---

## Success Criteria

- [ ] 4 workflow files created with valid YAML syntax
- [ ] 2 config files created (github-actions.yaml, ci-answers.yaml)
- [ ] All 5 acceptance criteria covered by workflows
- [ ] Cost optimization enabled (prompt caching, Haiku)
- [ ] ANTHROPIC_API_KEY documented in setup steps
- [ ] max-parallel configured appropriately
- [ ] All workflow triggers configured
- [ ] Artifact uploads configured for test results
- [ ] Story status updated

---

## Reference Files

Load on-demand during execution:
- parameter-extraction.md - Configuration loading
- workflow-generation.md - Template generation
- cost-optimization-guide.md - Cost strategies
- ci-answers-protocol.md - Headless prompts
- validation-procedures.md - Pre-checks
