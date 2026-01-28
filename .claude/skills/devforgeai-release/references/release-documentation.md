### Phase 5: Release Documentation

**Objective**: Document release for audit trail and communication

#### Step 1: Generate Release Notes

```
Read(file_path=".claude/skills/devforgeai-release/assets/templates/release-notes-template.md")

release_notes = populate_template(
    version, story_id, story_title, changes, qa_status,
    coverage, deployment_strategy, timestamps, metrics, rollback_command
)

Write(file_path="devforgeai/releases/release-{version}.md", content=release_notes)
```

#### Step 2: Update Story Status and Change Log

```
Read(file_path="devforgeai/specs/Stories/{story_id}.story.md")

# Update YAML status
Edit(file_path="devforgeai/specs/Stories/{story_id}.story.md",
     old_string="status: QA Approved",
     new_string="status: Released")

# Append Change Log entry with claude/deployment-engineer author
# Reference: .claude/references/changelog-update-guide.md
Edit(file_path="devforgeai/specs/Stories/{story_id}.story.md",
     old_string="| {last_date} | {last_author} | {last_action} | {last_change} | {last_files} |",
     new_string="| {last_date} | {last_author} | {last_action} | {last_change} | {last_files} |\n| {timestamp} | claude/deployment-engineer | Released | Deployed {version} to production | CHANGELOG.md |")

# Update Current Status in Change Log section
Edit(file_path="devforgeai/specs/Stories/{story_id}.story.md",
     old_string="**Current Status:** QA Approved",
     new_string="**Current Status:** Released")
```

#### Step 3: Update Project CHANGELOG.md

```
IF file_exists("CHANGELOG.md"):
    # Add story entry under [Unreleased] section
    Edit(file_path="CHANGELOG.md",
         old_string="## [Unreleased]",
         new_string="## [Unreleased]\n\n- {story_title} ([{story_id}])")

    # Add reference link at bottom of file
    # Format: [STORY-XXX]: devforgeai/specs/Stories/archive/STORY-XXX.story.md
    Append reference link
```

#### Step 4: Archive Story File

**Move story file to archive directory after release:**

```
# Create archive directory if not exists (handled by .gitkeep)
# Archive path: devforgeai/specs/Stories/archive/

Bash(command="mv devforgeai/specs/Stories/{story_id}.story.md devforgeai/specs/Stories/archive/")

Display: "✓ Story archived to devforgeai/specs/Stories/archive/{story_id}.story.md"
```

**Why Archive:**
- Keeps active Stories/ directory clean
- Maintains full history in archive/
- CHANGELOG.md links to archived stories for traceability

---

