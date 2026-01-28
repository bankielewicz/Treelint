## CRITICAL: Extracting Parameters from Conversation Context

**IMPORTANT:** Skills CANNOT accept runtime parameters like `--env=staging`. All information must be extracted from conversation context.

### How Slash Commands Pass "Parameters" to Skills

When `/release` command invokes this skill, it:
1. Loads story file via @file reference: `@devforgeai/specs/Stories/STORY-XXX.story.md`
2. States context explicitly: "Story ID: STORY-XXX" and "Environment: staging"
3. Invokes skill WITHOUT arguments: `Skill(command="devforgeai-release")`

**You must extract story ID and environment from the conversation.**

### Story ID Extraction

**Method 1: Read YAML frontmatter**
```
Look for YAML frontmatter in conversation:
  ---
  id: STORY-XXX
  title: ...
  status: ...
  ---

Extract: id field = Story ID
```

**Method 2: Search for file reference**
```
Search conversation for pattern:
  "devforgeai/specs/Stories/STORY-XXX"

Extract STORY-XXX from file path
```

**Method 3: Search for explicit statement**
```
Search conversation for:
  "Story ID: STORY-XXX"
  "Story: STORY-XXX"

Extract STORY-XXX
```

### Environment Extraction

**Look for environment in conversation:**
```
Search for patterns:
  - "Environment: staging" → ENV = staging
  - "Environment: production" → ENV = production
  - "Deploy to production" → ENV = production
  - "Deploy to staging" → ENV = staging
```

**If not found:**
```
Default to staging (safe choice)
Inform user: "No environment specified. Defaulting to staging deployment."
```

### Deployment Strategy Extraction (Optional)

**Look for strategy in conversation:**
```
Search for patterns:
  - "Strategy: blue-green" → STRATEGY = blue-green
  - "Strategy: rolling" → STRATEGY = rolling
  - "Strategy: canary" → STRATEGY = canary
  - "Strategy: recreate" → STRATEGY = recreate
```

**If not found:**
```
Read from deployment config or tech-stack.md:
  - devforgeai/deployment/config.json
  - devforgeai/specs/context/tech-stack.md (deployment section)

Default: rolling (safest for most platforms)
```

### Validation Before Proceeding

Before starting deployment, verify:
- [ ] Story ID extracted successfully
- [ ] Story content available in conversation
- [ ] Environment determined (staging or production)
- [ ] Deployment strategy identified
- [ ] Ready to proceed with release phases

**If extraction fails:**
```
HALT with error:
"Cannot extract required parameters from conversation context.

Expected to find:
  - Story ID: YAML frontmatter with 'id: STORY-XXX' OR file reference
  - Environment: 'Environment: staging/production' OR default to staging

Please ensure story is loaded via slash command or provide parameters explicitly."
```

---

