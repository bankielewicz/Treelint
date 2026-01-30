# Sprint Retrospective Workflow

**Skill Reference:** devforgeai-orchestration
**Phase:** Phase 7 (NEW - RCA-006 Phase 2)
**Trigger:** Last story in sprint reaches "Released" status
**Loaded:** When sprint completion detected
**Token Cost:** ~5,000-8,000 tokens

**Purpose:** Automatically audit technical debt at sprint end, identify resolvable deferrals, and provide actionable recommendations for debt reduction.

**RCA-006 Context:** Phase 2 Task 2.3 - Proactive technical debt monitoring at natural workflow boundary (sprint completion).

---

## Trigger Conditions

### When to Invoke Sprint Retrospective

**Automatic trigger:**
- Last story in sprint reaches "Released" status
- All stories in sprint are in terminal states ("Released" or "QA Failed")

**Manual trigger:**
- User runs `/orchestrate sprint-retrospective` (future enhancement)

**Not triggered:**
- Mid-sprint (some stories still in development)
- Sprint has no stories
- Sprint file doesn't exist

---

## Skill Context Available

When this reference is loaded:

- **SPRINT_ID:** Sprint identifier (e.g., Sprint-1, Sprint-2)
- **SPRINT_FILE:** Path to sprint file (devforgeai/specs/Sprints/Sprint-1.md)
- **Sprint Content:** Sprint YAML frontmatter and story list
- **Completion Detected:** All stories in terminal states

**Variables available:**
- `sprint_stories` - List of story IDs in this sprint
- `sprint_start_date` - Sprint start date
- `sprint_end_date` - Sprint end date (actual)
- `sprint_duration` - Planned duration in days

---

## Execution Workflow

### Step 1: Detect Sprint Completion

**Check if sprint is complete:**

```
# Read sprint file (already loaded in conversation)
Extract: story list from YAML frontmatter

sprint_complete = true

FOR each story_id in sprint_stories:
    Glob(pattern="devforgeai/specs/Stories/${story_id}*.story.md")

    IF file found:
        Read YAML frontmatter (first 20 lines)
        Extract: status

        IF status NOT IN ["Released", "QA Failed"]:
            sprint_complete = false
            break

IF NOT sprint_complete:
    Display: "Sprint retrospective skipped - not all stories complete"
    Display: "Pending stories: ${list stories NOT in terminal states}"

    Exit retrospective
    Return to calling workflow

ELSE:
    Display:
    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
     SPRINT RETROSPECTIVE
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    Sprint: ${SPRINT_ID}
    Completion detected: All {len(sprint_stories)} stories in terminal states

    Initiating technical debt audit..."

    Proceed to Step 2
```

---

### Step 2: Invoke /audit-deferrals Command

**Auto-invoke deferral audit:**

```
Display: "Running /audit-deferrals for sprint technical debt analysis..."

# Invoke /audit-deferrals command
# The command will:
# - Scan all QA Approved/Released stories (includes sprint stories)
# - Run Phase 2.5: Blocker Validation (checks if deferrals resolvable)
# - Run Phase 3: Validate deferrals with deferral-validator subagent
# - Generate comprehensive audit report with actionable insights

SlashCommand(command="/audit-deferrals")

# Wait for command to complete...
# Command returns audit results

# The /audit-deferrals command will display:
# - Resolvable deferrals (can be attempted now)
# - Valid deferrals (blockers still present)
# - Invalid deferrals (missing targets, etc.)
# - Technical debt metrics (age, trends)
```

**Note:** /audit-deferrals scans ALL stories (not just current sprint), but sprint-specific metrics will be extracted in Step 3.

---

### Step 3: Extract Sprint-Specific Metrics

**Filter audit results to current sprint:**

```
# Read the generated audit report
audit_report_path = most recent file in devforgeai/qa/deferral-audit-*.md

Read(file_path=audit_report_path)

# Extract deferrals for stories in THIS sprint
sprint_deferrals = []
sprint_resolvable = []
sprint_valid = []

FOR each story_id in sprint_stories:
    # Search audit report for this story
    Grep(pattern=story_id, path=audit_report_path, output_mode="content", -A=10)

    IF story found in audit report:
        Extract: deferrals for this story
        Extract: blocker validation results

        FOR each deferral:
            sprint_deferrals.append(deferral)

            IF deferral.resolvable == true:
                sprint_resolvable.append(deferral)
            ELSE:
                sprint_valid.append(deferral)

# Calculate sprint metrics
stories_with_deferrals = count unique story IDs in sprint_deferrals
total_sprint_deferrals = len(sprint_deferrals)
resolvable_count = len(sprint_resolvable)
valid_count = len(sprint_valid)

# Calculate debt age for sprint
total_debt_age = sum(deferral.age_days for all sprint_deferrals)
average_debt_age = total_debt_age / total_sprint_deferrals if total_sprint_deferrals > 0 else 0
```

---

### Step 4: Generate Sprint Retrospective Summary

**Display comprehensive retrospective:**

```
Display:
"╔═══════════════════════════════════════════════════════════════╗
║                   SPRINT RETROSPECTIVE                        ║
╚═══════════════════════════════════════════════════════════════╝

Sprint: ${SPRINT_ID}
Completed: ${current_date}
Duration: ${sprint_duration} days (${sprint_start_date} to ${sprint_end_date})

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

STORY METRICS:

- Total stories: ${len(sprint_stories)}
- Completed (Released): ${count stories with status "Released"}
- Failed (QA Failed): ${count stories with status "QA Failed"}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DEFERRAL METRICS:

- Stories with deferrals: ${stories_with_deferrals} (${percentage}%)
- Total deferred items: ${total_sprint_deferrals}
- Resolvable deferrals: ${resolvable_count} ✅
- Valid deferrals: ${valid_count} ⏸️

TECHNICAL DEBT:

- Total debt age: ${total_debt_age} days
- Average deferral age: ${average_debt_age} days
- Oldest deferral: ${max_age} days (Story ${oldest_story})

${IF average_debt_age > 14}:
  ⚠️  WARNING: Deferrals aging beyond 2 weeks
      Consider debt reduction sprint

${IF resolvable_count >= 3}:
  💡 OPPORTUNITY: ${resolvable_count} deferrals can be resolved now
      Recommend debt reduction sprint

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RESOLVABLE DEFERRALS (${resolvable_count}):

${FOR each deferral in sprint_resolvable}:
  {index}. {deferral.story_id}: {deferral.item}
     Age: {deferral.age_days} days
     Blocker: {deferral.original_blocker}
     Status: ✅ {deferral.blocker_status}
     Action: {deferral.action}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

VALID DEFERRALS (${valid_count}):

${FOR each deferral in sprint_valid}:
  {index}. {deferral.story_id}: {deferral.item}
     Age: {deferral.age_days} days
     Blocker: {deferral.blocker_description}
     Status: ⏸️ {deferral.blocker_status}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RECOMMENDATIONS:

${IF resolvable_count >= 3}:
  1. 🎯 HIGH PRIORITY: Create debt reduction sprint
     - {resolvable_count} resolvable deferrals ready for implementation
     - Estimated effort: ${estimate_story_points(resolvable_count)} points

${IF resolvable_count > 0 AND resolvable_count < 3}:
  1. 💡 OPPORTUNITY: Resolve {resolvable_count} deferral(s) in next sprint
     - Add to Sprint-${next_sprint_number} backlog

${IF average_debt_age > 14}:
  2. ⚠️  AGING DEBT: Average deferral age is ${average_debt_age} days
     - Review oldest deferrals for alternative approaches
     - Consider re-scoping or creating ADRs

${IF valid_count > 0}:
  3. ℹ️  VALID DEFERRALS: ${valid_count} item(s) still blocked
     - Monitor blockers for resolution
     - Track in technical debt register

Full audit report: devforgeai/qa/deferral-audit-{timestamp}.md

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

---

### Step 5: Offer Debt Reduction Sprint Creation

**IF resolvable_count >= 3:**

```
AskUserQuestion:
  Question: "Create debt reduction sprint for {resolvable_count} resolvable deferrals?"
  Header: "Debt Sprint"
  Options:
    - "Yes, create sprint now"
    - "No, defer to next planning session"
    - "Add to backlog (no sprint)"
  multiSelect: false

user_selection = response

IF user selects "Yes, create sprint now":
    # Prepare sprint details
    next_sprint_number = extract from existing sprints + 1
    sprint_name = "Sprint-${next_sprint_number} (Debt Reduction)"
    start_date = ${current_date + 1}  # Tomorrow
    duration = 10  # 2-week sprint

    # For each resolvable deferral, create follow-up story
    debt_stories = []

    FOR each deferral in sprint_resolvable:
        Display: "Creating follow-up story for: {deferral.item} (from {deferral.story_id})"

        Task(
          subagent_type="requirements-analyst",
          description="Create debt reduction story",
          prompt="Create story to resolve deferred work:

                  Parent Story: {deferral.story_id}
                  Deferred DoD Item: '{deferral.item}'
                  Original Blocker: '{deferral.original_blocker}'
                  Blocker Status: RESOLVED ({deferral.blocker_status})
                  Action: {deferral.action}

                  Requirements:
                  - Extract acceptance criteria from original deferred item
                  - Set prerequisite_stories: [{deferral.story_id}]
                  - Set epic: {same as parent story}
                  - Set status: Backlog
                  - Set priority: High (debt reduction)
                  - Set points: {estimate based on complexity}

                  Return new story ID and file path."
        )

        new_story_id = extract from subagent response
        debt_stories.append(new_story_id)

        Display: "✓ Created: {new_story_id}"

    # Create sprint with debt stories
    Invoke: /create-sprint with parameters:
      Sprint Name: "{sprint_name}"
      Selected Stories: {debt_stories}
      Start Date: {start_date}
      Duration: {duration}

    Display:
    "✅ Debt reduction sprint created

    Sprint: {sprint_name}
    Stories: {len(debt_stories)}
    Estimated points: {sum story points}

    Next: Run /dev for each story to resolve deferrals"

ELIF user selects "Add to backlog":
    Display:
    "Adding resolvable deferrals to backlog (no sprint created)

    Resolvable items:
    {FOR each deferral in sprint_resolvable}:
      - {deferral.story_id}: {deferral.item}

    Recommendation: Address these in next sprint planning session"

ELSE:  # "No, defer to next planning"
    Display: "Debt reduction deferred to next planning session"
```

---

### Step 6: Update Sprint File with Retrospective

**Add retrospective section to sprint file:**

```
Read(file_path="${SPRINT_FILE}")

retrospective_section = """

## Sprint Retrospective

**Date:** ${current_date}
**Completed:** ${len(sprint_stories)} stories

### Metrics

**Story Completion:**
- Released: ${count "Released"}
- QA Failed: ${count "QA Failed"}
- Completion rate: ${(count "Released" / len(sprint_stories)) * 100}%

**Technical Debt:**
- Stories with deferrals: ${stories_with_deferrals} (${percentage}%)
- Total deferred items: ${total_sprint_deferrals}
- Resolvable deferrals: ${resolvable_count}
- Valid deferrals: ${valid_count}
- Average debt age: ${average_debt_age} days

### Actionable Items

${IF resolvable_count > 0}:
**Resolvable Deferrals:**
{FOR each deferral in sprint_resolvable}:
  - {deferral.story_id}: {deferral.item}
    Action: {deferral.action}

${IF user created debt reduction sprint}:
**Debt Reduction Sprint Created:**
- Sprint: {sprint_name}
- Stories: {len(debt_stories)}
- Target: Resolve {resolvable_count} deferrals

### Lessons Learned

${IF average_debt_age > 14}:
- ⚠️  Deferrals aging beyond 2 weeks - improve blocker resolution
${IF stories_with_deferrals / len(sprint_stories) > 0.4}:
- ⚠️  High deferral rate (${percentage}%) - stories may be over-scoped
${IF resolvable_count > 0}:
- 💡 ${resolvable_count} blockers resolved during sprint - good progress

### Next Sprint Recommendations

${IF resolvable_count >= 3}:
1. Focus next sprint on debt reduction (${resolvable_count} items ready)
${IF average_debt_age > 14}:
2. Address aging deferrals (avg ${average_debt_age} days)
${IF stories_with_deferrals / len(sprint_stories) > 0.5}:
3. Reduce story scope to minimize deferrals (<50% stories with deferrals)

"""

# Append retrospective to sprint file
Look for end of file
Append: retrospective_section

Edit to save changes

Display: "✓ Retrospective added to sprint file: ${SPRINT_FILE}"
```

---

## Success Criteria

Sprint retrospective succeeds when:
- [ ] Sprint completion detected (all stories terminal)
- [ ] /audit-deferrals invoked automatically
- [ ] Audit results filtered to sprint stories
- [ ] Sprint-specific metrics calculated
- [ ] Retrospective summary displayed
- [ ] User offered debt reduction sprint (if ≥3 resolvable)
- [ ] Debt stories created (if user approved)
- [ ] Sprint file updated with retrospective section

**On success:** Sprint marked complete with retrospective data

---

## Integration Notes

**Invoked by:** devforgeai-orchestration skill (when last story reaches "Released")

**Invokes:**
- `/audit-deferrals` command (SlashCommand tool)
- requirements-analyst subagent (if creating debt stories)
- `/create-sprint` command (if user approves debt reduction sprint)

**Updates:**
- Sprint file (retrospective section)
- New story files (debt reduction stories, if created)
- New sprint file (debt reduction sprint, if created)

**References:**
- `/audit-deferrals` command (performs the audit)
- `sprint-planning-guide.md` (sprint creation)
- RCA-006 Phase 2 Task 2.3

**Token Efficiency:**
- Sprint completion check: ~1,000 tokens
- /audit-deferrals invocation: ~15,000 tokens (in command's context)
- Metric extraction: ~2,000 tokens
- Retrospective display: ~1,000 tokens
- Debt story creation: ~10,000 tokens per story (if created)
- Total: ~5,000-8,000 tokens (without debt story creation)
- Total: ~20,000-35,000 tokens (if creating 3 debt stories)

---

## Error Handling

### Sprint File Not Found
```
ERROR: Sprint file not available

Expected: ${SPRINT_FILE}

Actions:
1. Verify sprint file exists: Glob(pattern="devforgeai/specs/Sprints/${SPRINT_ID}.md")
2. Check SPRINT_ID extracted correctly from context
3. Re-invoke with correct sprint ID
```

### /audit-deferrals Command Fails
```
ERROR: /audit-deferrals command failed to complete

Possible causes:
- No QA Approved/Released stories (nothing to audit)
- deferral-validator subagent failed
- File system issues

Actions:
1. Review /audit-deferrals error output above
2. Run /audit-deferrals manually to debug
3. Skip retrospective for this sprint (log error)

Fallback: Display basic sprint summary without debt audit
```

### Debt Story Creation Fails
```
ERROR: requirements-analyst subagent failed to create debt story

Possible causes:
- Subagent timeout
- Invalid deferral description
- Story ID collision

Actions:
1. Review subagent error output
2. User can create debt stories manually:
   /create-story {description}
3. Skip debt sprint creation, log deferrals for manual follow-up
```

---

## Configuration Options (Future Enhancement)

**Retrospective thresholds (configurable):**
- `resolvable_threshold`: 3 (trigger debt sprint recommendation)
- `age_warning_threshold`: 14 days (flag aging deferrals)
- `deferral_rate_warning`: 40% (flag high deferral rate)

**Auto-creation flags:**
- `auto_create_debt_sprint`: false (default: ask user)
- `auto_create_debt_stories`: false (default: ask user)

**Configuration file:** `devforgeai/config/retrospective-config.yaml` (future)

---

## Protocol Compliance

**This retrospective enforces:**
- Proactive technical debt monitoring (no manual intervention)
- Sprint-level visibility (see debt trends across sprints)
- Actionable recommendations (specific commands, estimated effort)
- User control (always asks before creating sprints/stories)

**Aligns with:**
- RCA-006 Phase 2 recommendations
- "Ask, Don't Assume" framework principle
- Quality gate philosophy (catch issues early)
- Zero technical debt policy (proactive debt reduction)

---

## Example Output

### Sprint with Resolvable Deferrals

```
╔═══════════════════════════════════════════════════════════════╗
║                   SPRINT RETROSPECTIVE                        ║
╚═══════════════════════════════════════════════════════════════╝

Sprint: Sprint-1
Completed: 2024-11-06
Duration: 14 days (2024-10-24 to 2024-11-06)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

STORY METRICS:

- Total stories: 8
- Completed (Released): 7
- Failed (QA Failed): 1

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DEFERRAL METRICS:

- Stories with deferrals: 3 (37.5%)
- Total deferred items: 7
- Resolvable deferrals: 3 ✅
- Valid deferrals: 4 ⏸️

TECHNICAL DEBT:

- Total debt age: 45 days
- Average deferral age: 6.4 days
- Oldest deferral: 14 days (Story STORY-002)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RESOLVABLE DEFERRALS (3):

1. STORY-008.1: Integration tests
   Age: 1 day
   Blocker: Requires .so/.dylib/.dll from STORY-008
   Status: ✅ RESOLVED (STORY-008 complete)
   Action: Re-run /dev STORY-008.1

2. STORY-008.1: Miri validation
   Age: 1 day
   Blocker: Requires nightly toolchain
   Status: ✅ RESOLVED (nightly installed)
   Action: cargo +nightly miri test

3. STORY-003: API documentation
   Age: 7 days
   Blocker: Requires API stabilization
   Status: ✅ RESOLVED (API released in STORY-005)
   Action: Re-run /dev STORY-003

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

VALID DEFERRALS (4):

[List of 4 valid deferrals with blockers...]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RECOMMENDATIONS:

1. 🎯 HIGH PRIORITY: Create debt reduction sprint
   - 3 resolvable deferrals ready for implementation
   - Estimated effort: 8 points

2. ℹ️  VALID DEFERRALS: 4 item(s) still blocked
   - Monitor blockers for resolution

Full audit report: devforgeai/qa/deferral-audit-2024-11-06-1700.md

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Create debt reduction sprint for 3 resolvable deferrals?
[ ] Yes, create sprint now
[ ] No, defer to next planning session
[ ] Add to backlog (no sprint)

[User selects "Yes, create sprint now"]

Creating debt reduction sprint...
✓ Created: STORY-009 (Integration tests from STORY-008.1)
✓ Created: STORY-010 (Miri validation from STORY-008.1)
✓ Created: STORY-011 (API documentation from STORY-003)

✅ Debt reduction sprint created

Sprint: Sprint-2 (Debt Reduction)
Stories: 3
Estimated points: 8

Next: Run /dev for each story to resolve deferrals
```

---

## Retrospective Best Practices

**Frequency:**
- Every sprint (automatic)
- Quarterly (deep dive with trends)

**Focus Areas:**
1. **Velocity:** Are deferrals impacting story completion?
2. **Debt trends:** Is debt growing or shrinking?
3. **Blocker patterns:** What types of blockers are common?
4. **Resolution speed:** How quickly are deferrals resolved?

**Actions:**
- Create debt reduction sprints when resolvable deferrals accumulate
- Review aging deferrals (>30 days) for alternative approaches
- Adjust story scoping if high deferral rate (>40%)

---

## Difference from /audit-deferrals Command

**Sprint Retrospective (This Workflow):**
- **Scope:** Single sprint (filtered to sprint stories)
- **Timing:** Automatic (triggered by sprint completion)
- **Purpose:** Sprint-specific debt analysis and planning
- **Output:** Sprint metrics + recommendations for next sprint
- **Creates:** Debt reduction sprint (optional)

**/audit-deferrals Command:**
- **Scope:** All stories (entire project)
- **Timing:** Manual or scheduled (quarterly)
- **Purpose:** Comprehensive deferral audit across all work
- **Output:** Full audit report with all violations
- **Creates:** Nothing (read-only audit)

**Both work together:**
- Retrospective uses /audit-deferrals for data
- Retrospective filters to sprint scope
- Retrospective adds sprint-specific actions
