# Claude Code Bash Approval Friction - Quick Start Guide

**Problem:** Subagents require 15-25+ bash command approvals per story (approval friction)
**Solution:** Pre-configure bash allowlist in `.claude/settings.json`
**Time Required:** 5 minutes
**Impact:** Reduces approvals from 20+ to 1-2 per story (80-90% friction reduction)

---

## Step 1: Create `.claude/settings.json`

Copy this configuration to `.claude/settings.json` in your project root:

```json
{
  "permissions": {
    "allow": [
      "Read",
      "Write(src/**)",
      "Write(tests/**)",
      "Write(scripts/**)",
      "Write(.claude/**)",
      "Write(devforgeai/**)",
      "Edit(src/**)",
      "Edit(tests/**)",
      "Bash(npm run test:*)",
      "Bash(npm run build)",
      "Bash(npm run lint)",
      "Bash(npm run typecheck)",
      "Bash(npm test:*)",
      "Bash(pytest:*)",
      "Bash(python -m pytest:*)",
      "Bash(python -m mypy:*)",
      "Bash(dotnet test:*)",
      "Bash(dotnet build:*)",
      "Bash(git status)",
      "Bash(git diff:*)",
      "Bash(git add:*)",
      "Bash(git commit:*)",
      "Bash(git log:*)",
      "Bash(wc -c:*)",
      "Bash(wc -l:*)"
    ],
    "deny": [
      "Bash(rm -rf *)",
      "Bash(sudo:*)",
      "Bash(curl:*)",
      "Bash(wget:*)",
      "Bash(git push:*)",
      "Bash(npm publish)",
      "Read(.env*)",
      "Read(secrets/**)"
    ],
    "ask": [
      "Bash(npm install:*)",
      "Bash(git checkout:*)",
      "Bash(git merge:*)"
    ]
  }
}
```

## Step 2: Commit to Git

```bash
git add .claude/settings.json
git commit -m "chore: Configure bash command allowlist for DevForgeAI workflows

- Allows: test runners, linters, type checkers, git commands
- Blocks: destructive operations (rm -rf), privileged access (sudo)
- Ask approval for: package installs, branch operations
- Impact: Reduces bash approval prompts 80-90% (20+ → 1-2 per story)"
```

## Step 3: Test

Start Claude Code and run `/dev STORY-ID` with a story. You should see:
- ✅ No approval prompts for: npm test, npm run build, git status, git commit
- ⚠️ Single approval for commands not in allowlist
- ❌ Commands in "deny" list are completely blocked (expected)

## Step 4: Iterate

As new commands emerge:
1. When Claude needs a new bash command, you'll see approval prompt
2. Review the command for safety
3. If safe, add to "allow" list in `.claude/settings.json`
4. If risky, add to "ask" list (manual approval each time)
5. Commit change to git

---

## Understanding the Configuration

**Allow List:** Auto-approved commands (no prompts)
- `npm run test:*` - All npm test variants (test, test:unit, test:integration, etc.)
- `pytest:*` - All pytest commands
- `git commit:*` - Git commits (safe, all changes staged by user)

**Deny List:** Blocked commands (completely forbidden)
- `rm -rf *` - Never allow recursive deletion
- `sudo:*` - Never allow privileged operations
- `curl:*, wget:*` - Never allow external downloads
- `.env*, secrets/**` - Never allow reading sensitive files

**Ask List:** Manual approval required each time
- `npm install:*` - User should review dependencies
- `git checkout:*` - Branch changes require attention
- `git merge:*` - Merges are important, review needed

---

## Common Commands to Add

When you encounter these, add to "allow" list:

```json
{
  "allow": [
    "Bash(npm run format)",
    "Bash(npm run lint:fix)",
    "Bash(go test ./...)",
    "Bash(go build)",
    "Bash(cargo test)",
    "Bash(cargo build)",
    "Bash(java -m org.junit:*)",
    "Bash(ruby -m unittest:*)"
  ]
}
```

---

## Known Issues

**Complex commands with pipes or environment variables may still prompt:**

Example problematic command:
```bash
NODE_OPTIONS="--max-old-space-size=4096" npm test
```

**Workaround:** Create wrapper script

Create `.claude/scripts/run-tests.sh`:
```bash
#!/bin/bash
NODE_OPTIONS="${NODE_OPTIONS}" npm test "$@"
```

Add to allowlist:
```json
{
  "allow": [
    "Bash(bash .claude/scripts/run-tests.sh:*)"
  ]
}
```

Now tell Claude: "Use `.claude/scripts/run-tests.sh` to run tests" → Single approval works reliably

---

## Security Best Practices

1. **Never add `Bash(*:*)` or `Bash(*)`** - This allows ALL bash commands
2. **Use specific patterns** - `Bash(npm test:*)` not `Bash(npm:*)`
3. **Review deny list regularly** - Ensure dangerous operations are blocked
4. **Commit to git** - Team sees what's approved, can audit changes
5. **Document new commands** - Comment in JSON why each command needed

Example with documentation:
```json
{
  "allow": [
    "Bash(npm run test:*)",           // TDD - run tests during development
    "Bash(pytest:*)",                  // Python testing framework
    "Bash(git commit:*)",              // Version control - safe (changes staged by user)
    "Bash(git diff:*)"                 // Code review - read-only
  ]
}
```

---

## Measuring Improvement

**Before allowlist:**
- Approvals per story: 15-20
- Workflow interruptions: Frequent
- Development friction: High

**After allowlist:**
- Approvals per story: 1-2
- Workflow interruptions: Minimal
- Development friction: Low

Track this metric in `devforgeai/metrics/bash-approvals.json` for ongoing optimization.

---

## Next Steps (Optional Enhancements)

**Phase 2: Deferred Test Execution**
- Have subagents generate tests WITHOUT running them
- User reviews code, then approves single "npm test" execution
- Reduces friction another 10% (1-2 approvals → 1 approval)

**Phase 3: Code Review Gate**
- Add review checkpoint before test execution
- User sees generated code, approves with single command
- Achieves single-approval-per-story goal

See full research at: `devforgeai/specs/research/CLAUDE-CODE-BASH-APPROVAL-FRICTION-RESEARCH.md`

---

## Questions?

If approval prompts continue for allowed commands:
1. Check command exactly matches pattern (case-sensitive, spaces matter)
2. Try wrapper script workaround for complex commands
3. Check for pipes (|), redirects (>), or environment variables in command
4. Report issue: https://github.com/anthropics/claude-code/issues

---

**Configuration Type:** Permanent (committed to git, team-shared)
**Update Frequency:** Add new commands as needed, audit quarterly
**Expected Result:** 80-90% reduction in bash approval friction
**Estimated Payoff:** 5-10 minutes saved per story (1-2 hours per sprint)
