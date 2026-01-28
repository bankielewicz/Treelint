---
description: Creates 4 workflows + 2 config files for GitHub Actions CI/CD
argument-hint: [--force]
model: opus
allowed-tools: Read, Skill, Bash(git:*), Write
---

# /setup-github-actions - GitHub Actions Setup

Create GitHub Actions workflows for headless DevForgeAI execution.

---

## Quick Reference

```bash
# Standard usage
/setup-github-actions

# Force overwrite existing files
/setup-github-actions --force

# Files created:
# .github/workflows/dev-story.yml
# .github/workflows/qa-validation.yml
# .github/workflows/parallel-stories.yml
# .github/workflows/installer-testing.yml
# devforgeai/config/ci/github-actions.yaml
# devforgeai/config/ci/ci-answers.yaml
```

---

## Command Workflow

### Phase 01: Argument Validation

**Parse force flag:**

```
FORCE_FLAG = false

IF "--force" in arguments:
    FORCE_FLAG = true
    Display: "⚠️  Force mode enabled - will overwrite existing files"
```

**Check for existing files:**

```
existing_workflows = Glob(pattern=".github/workflows/dev-story.yml")

IF existing_workflows AND NOT FORCE_FLAG:
    AskUserQuestion:
        Question: "GitHub Actions workflows already exist. Overwrite?"
        Options:
          - "Yes, overwrite all"
          - "No, cancel setup"
          - "Yes, but backup first"
```

### Phase 02: Invoke Skill

**Set context markers:**

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  GitHub Actions Setup"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "Creating workflows for headless DevForgeAI execution..."
Display: ""
```

**Invoke devforgeai-github skill:**

```
Skill(command="devforgeai-github-actions")
```

---

### Phase 03: Display Results

**Skill returns result with created files:**

```
Display: ""
Display: "✅ Setup Complete"
Display: ""
Display: "Workflow files created:"
Display: "  • .github/workflows/dev-story.yml"
Display: "  • .github/workflows/qa-validation.yml"
Display: "  • .github/workflows/parallel-stories.yml"
Display: "  • .github/workflows/installer-testing.yml"
Display: ""
Display: "Configuration files created:"
Display: "  • devforgeai/config/ci/github-actions.yaml"
Display: "  • devforgeai/config/ci/ci-answers.yaml"
Display: ""
Display: "Next Steps:"
Display: "  1. Add ANTHROPIC_API_KEY to GitHub Secrets"
Display: "     Settings > Secrets > Actions > New repository secret"
Display: "  2. Review ci-answers.yaml for headless prompts"
Display: "  3. Trigger /dev workflow: Actions > DevForgeAI Story Development"
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

---

## Error Handling

**Not a Git Repository:**

```
Error: Not a Git repository
This command requires a Git repository to create workflows.
```

**No Write Permission:**

```
Error: Cannot write to .github/workflows/
Check directory permissions.
```

---

**Target: ~150 lines (~4K chars, 27% of 15K budget)**
