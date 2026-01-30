# Workflow Generation Reference

This document describes the workflow generation process for DevForgeAI GitHub Actions.

## Workflow Templates

### dev-story.yml Template

```yaml
name: DevForgeAI Story Development

on:
  workflow_dispatch:
    inputs:
      story_id:
        description: 'Story ID (e.g., STORY-001)'
        required: true
        type: string

jobs:
  develop:
    runs-on: ubuntu-latest
    timeout-minutes: 30

    steps:
      - uses: actions/checkout@v4

      - name: Validate Story ID
        run: |
          if [[ ! "${{ inputs.story_id }}" =~ ^STORY-[0-9]+$ ]]; then
            echo "Invalid story ID format. Expected: STORY-NNN"
            exit 1
          fi

      - name: Setup Claude Code
        uses: anthropics/claude-code-action@v1
        with:
          api-key: ${{ secrets.ANTHROPIC_API_KEY }}

      - name: Execute Development
        env:
          CLAUDE_CODE_CACHE_ENABLED: true
          CLAUDE_CODE_MODEL: claude-model: opus-4-5-20251001
        run: |
          claude -p "/dev ${{ inputs.story_id }}" --output-format json > dev-result.json

      - name: Upload Artifacts
        uses: actions/upload-artifact@v4
        with:
          name: dev-results-${{ inputs.story_id }}
          path: |
            dev-result.json
            TestResults/
            devforgeai/specs/Stories/${{ inputs.story_id }}*.story.md
```

### qa-validation.yml Template

```yaml
name: DevForgeAI QA Validation

on:
  pull_request:
    branches: [main]
    types: [opened, synchronize]

jobs:
  validate:
    runs-on: ubuntu-latest
    timeout-minutes: 30

    steps:
      - uses: actions/checkout@v4

      - name: Extract Story ID
        id: story
        run: |
          STORY_ID=$(echo "${{ github.event.pull_request.title }}" | grep -oE 'STORY-[0-9]+' | head -1)
          if [ -z "$STORY_ID" ]; then
            echo "No story ID found in PR title"
            echo "found=false" >> $GITHUB_OUTPUT
          else
            echo "story_id=$STORY_ID" >> $GITHUB_OUTPUT
            echo "found=true" >> $GITHUB_OUTPUT
          fi

      - name: Run QA Validation
        if: steps.story.outputs.found == 'true'
        env:
          CLAUDE_CODE_CACHE_ENABLED: true
        run: |
          claude -p "/qa ${{ steps.story.outputs.story_id }} deep" --output-format json > qa-result.json

      - name: Post Results
        if: steps.story.outputs.found == 'true'
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const result = JSON.parse(fs.readFileSync('qa-result.json', 'utf8'));

            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: `## QA Validation Results\n\n${result.summary || 'See artifacts for details'}`
            });
```

### parallel-stories.yml Template

```yaml
name: DevForgeAI Parallel Stories

on:
  workflow_dispatch:
    inputs:
      story_ids:
        description: 'Story IDs as JSON array (e.g., ["STORY-001", "STORY-002"])'
        required: true
        type: string

jobs:
  develop:
    runs-on: ubuntu-latest
    timeout-minutes: 30
    strategy:
      matrix:
        story_id: ${{ fromJSON(inputs.story_ids) }}
      max-parallel: 5
      fail-fast: false

    steps:
      - uses: actions/checkout@v4

      - name: Setup Claude Code
        uses: anthropics/claude-code-action@v1
        with:
          api-key: ${{ secrets.ANTHROPIC_API_KEY }}

      - name: Execute Development
        env:
          CLAUDE_CODE_CACHE_ENABLED: true
          CLAUDE_CODE_MODEL: claude-model: opus-4-5-20251001
        run: |
          claude -p "/dev ${{ matrix.story_id }}" --output-format json > dev-result.json

      - name: Upload Artifacts
        uses: actions/upload-artifact@v4
        with:
          name: dev-results-${{ matrix.story_id }}
          path: dev-result.json
```

## Generation Process

### Phase 1: Pre-Generation Validation

1. **Git Repository Check**
   - Verify `.git/` directory exists
   - Check for clean working tree (warn on uncommitted changes)

2. **Directory Structure Check**
   - Ensure `.github/workflows/` exists (create if missing)
   - Verify `devforgeai/config/ci/` exists (create if missing)

3. **Context File Validation**
   - Check 6 context files exist in `devforgeai/specs/context/`
   - Warn if missing (workflows may not function correctly)

### Phase 2: Configuration Loading

1. **Load Existing Configuration**
   - Read `devforgeai/config/ci/github-actions.yaml` if exists
   - Read `devforgeai/config/ci/ci-answers.yaml` if exists

2. **Apply Defaults**
   - Fill missing values from skill defaults
   - max_parallel_jobs: 5
   - enable_prompt_caching: true
   - prefer_haiku: true

3. **User Overrides**
   - Apply any user-provided answers from AskUserQuestion

### Phase 3: Template Generation

1. **Generate Workflow Files**
   - Apply configuration to templates
   - Write to `.github/workflows/`

2. **Generate Configuration Files**
   - Create example config files if missing
   - Write to `devforgeai/config/ci/`

### Phase 4: Post-Generation Validation

1. **YAML Syntax Validation**
   - Parse each generated file
   - Report any syntax errors

2. **Required Fields Check**
   - Verify all required inputs defined
   - Check job names and steps

## Customization Points

### Environment Variables

```yaml
env:
  # Model selection
  CLAUDE_CODE_MODEL: claude-model: opus-4-5-20251001  # Cheaper
  # CLAUDE_CODE_MODEL: claude-opus-4-5-20251101  # More capable

  # Caching
  CLAUDE_CODE_CACHE_ENABLED: true  # 90% cost reduction

  # Debugging
  CLAUDE_CODE_DEBUG: false  # Enable for verbose logs
```

### Trigger Customization

```yaml
# Add branch filtering
on:
  push:
    branches: [main, develop, 'feature/*']
    paths: ['src/**', 'devforgeai/specs/**']
```

### Runner Selection

```yaml
# Self-hosted runner
runs-on: [self-hosted, linux, x64]

# Or use GitHub-hosted
runs-on: ubuntu-latest
```

## Error Handling

### Missing API Key

```yaml
- name: Validate API Key
  run: |
    if [ -z "${{ secrets.ANTHROPIC_API_KEY }}" ]; then
      echo "::error::ANTHROPIC_API_KEY secret not configured"
      exit 1
    fi
```

### Rate Limit Retry

```yaml
- name: Execute with Retry
  uses: nick-fields/retry@v2
  with:
    timeout_minutes: 10
    max_attempts: 3
    retry_wait_seconds: 30
    command: claude -p "/dev ${{ inputs.story_id }}"
```
