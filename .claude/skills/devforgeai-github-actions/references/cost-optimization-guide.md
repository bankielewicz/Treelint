# Cost Optimization Guide

Strategies for minimizing Claude API costs in GitHub Actions workflows.

## Cost Components

### Token Pricing (as of 2025)

| Model | Input Tokens | Output Tokens |
|-------|--------------|---------------|
| Claude Opus 4.5 | $15/MTok | $75/MTok |
| Claude Haiku 4.5 | $0.25/MTok | $1.25/MTok |

**Key insight:** Haiku is 60x cheaper for input, 60x cheaper for output.

### Typical Story Cost Breakdown

```
Story: STORY-001 (Simple CRUD feature)
─────────────────────────────────────
Context loading:     ~50K tokens (input)
Conversation:        ~20K tokens (output)
Total tokens:        ~70K tokens

Without optimization:
  Opus:   (50K × $15 + 20K × $75) / 1M = $2.25

With optimization:
  Haiku:  (50K × $0.25 + 20K × $1.25) / 1M = $0.0375
  Caching: 90% reduction on context = $0.00375 + $0.025 = $0.03

Savings: 98.7% ($2.25 → $0.03)
```

## Optimization Strategies

### 1. Prompt Caching (90% Savings)

Enable caching to reuse framework context across turns:

```yaml
env:
  CLAUDE_CODE_CACHE_ENABLED: true
```

**What gets cached:**
- CLAUDE.md project instructions
- Context files (tech-stack.md, etc.)
- Skill SKILL.md content
- Story file content (after initial load)

**Cache duration:** 5 minutes
**Break-even:** 2+ turns per story

### 2. Haiku Model (60x Cheaper)

Use Haiku for routine operations:

```yaml
env:
  CLAUDE_CODE_MODEL: claude-model: opus-4-5-20251001
```

**When Haiku is sufficient:**
- Test generation (TDD Red phase)
- Code implementation (TDD Green phase)
- File operations and refactoring
- QA validation (pattern matching)

**When Opus may be needed:**
- Complex architectural decisions
- Novel problem solving
- Multi-file refactoring with dependencies
- Security-sensitive implementations

### 3. Turn Limits

Prevent runaway costs with turn limits:

```yaml
# In github-actions.yaml
cost_optimization:
  max_turns:
    simple: 10      # Simple CRUD stories
    complex: 20     # Standard implementations
    architecture: 30 # Major refactoring
```

### 4. Early Termination

Stop workflows that exceed budget:

```yaml
- name: Check Cost
  run: |
    COST=$(jq -r '.total_cost_usd // 0' dev-result.json)
    if (( $(echo "$COST > 0.15" | bc -l) )); then
      echo "::error::Cost exceeded threshold: $${COST}"
      exit 1
    fi
```

## Cost Monitoring

### Per-Workflow Tracking

```yaml
- name: Log Cost Summary
  run: |
    COST=$(jq -r '.total_cost_usd // 0' dev-result.json)
    echo "### Cost Summary" >> $GITHUB_STEP_SUMMARY
    echo "- Story: ${{ inputs.story_id }}" >> $GITHUB_STEP_SUMMARY
    echo "- Cost: \$$COST" >> $GITHUB_STEP_SUMMARY
    echo "- Model: ${CLAUDE_CODE_MODEL:-default}" >> $GITHUB_STEP_SUMMARY
```

### Monthly Budgeting

| Stories/Month | Without Optimization | With Optimization |
|---------------|---------------------|-------------------|
| 10 | $22.50 | $0.30 |
| 50 | $112.50 | $1.50 |
| 100 | $225.00 | $3.00 |

### Cost Alerts

Configure GitHub Actions budget alerts:
1. Go to Settings > Billing
2. Set spending limit
3. Enable email notifications

## Configuration Reference

### github-actions.yaml

```yaml
cost_optimization:
  # Enable prompt caching for repeated context
  enable_prompt_caching: true

  # Use cheaper model for routine operations
  prefer_haiku: true

  # Maximum cost per story (USD)
  max_cost_per_story: 0.15

  # Turn limits by story complexity
  max_turns:
    simple: 10
    complex: 20
    architecture: 30
```

### Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| CLAUDE_CODE_CACHE_ENABLED | Enable prompt caching | false |
| CLAUDE_CODE_MODEL | Override model | claude-opus-4-5 |
| CLAUDE_CODE_MAX_TURNS | Turn limit | unlimited |

## Best Practices

### 1. Right-Size Stories

| Story Size | Points | Expected Cost |
|------------|--------|---------------|
| Small | 1-3 | $0.03-0.05 |
| Medium | 5-8 | $0.08-0.12 |
| Large | 13+ | $0.12-0.20 |

Break large stories (>13 points) into smaller units.

### 2. Batch Parallel Runs

Run multiple stories together to share cache:
- Stories 1-5 in single workflow
- Shared context cached across matrix jobs
- 50%+ additional savings

### 3. Off-Peak Execution

Schedule workflows during off-peak hours:
- Faster response times
- No rate limit delays
- More consistent costs

```yaml
on:
  schedule:
    - cron: '0 2 * * *'  # 2 AM UTC
```

### 4. Cache Story Context

Pre-load story context before development:

```yaml
- name: Pre-cache Context
  env:
    CLAUDE_CODE_CACHE_ENABLED: true
  run: |
    # Read context files to prime cache
    claude -p "Read devforgeai/specs/context/*.md"
```

## Troubleshooting

### High Costs

**Symptom:** Story costs >$0.20
**Causes:**
- Haiku not enabled
- Caching disabled
- Complex story requiring many turns

**Solution:**
```yaml
env:
  CLAUDE_CODE_MODEL: claude-model: opus-4-5-20251001
  CLAUDE_CODE_CACHE_ENABLED: true
```

### Rate Limit Delays

**Symptom:** 429 errors adding to cost
**Cause:** Too many parallel jobs

**Solution:**
```yaml
strategy:
  max-parallel: 3  # Reduce from 5
```

### Inconsistent Costs

**Symptom:** Same story has variable costs
**Causes:**
- Cache misses (>5 min between turns)
- Different models used
- Retry loops

**Solution:**
- Enable caching
- Lock model version
- Add retry limits
