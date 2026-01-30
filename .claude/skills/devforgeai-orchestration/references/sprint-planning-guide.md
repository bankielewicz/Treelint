# Sprint Planning Guide

Complete reference for sprint planning within the DevForgeAI framework, including capacity guidelines, story selection, status transitions, and sprint file structure.

---

## Overview

Sprints are **2-week iterations** that bundle related stories for coordinated implementation and release. Sprints are optional but recommended for:
- **Team velocity tracking** (points completed per sprint)
- **Coordinated releases** (releasing multiple stories together)
- **Focused work** (team works on related features in parallel)
- **Capacity planning** (prevent over-commitment)

---

## Sprint Capacity Guidelines

### Standard Capacity

**2-week sprint:** 20-40 story points

This assumes:
- 5 team members working full-time
- 80% utilization (20% for meetings, reviews, unexpected issues)
- Average velocity of 8 points per person per sprint
- Standard story point sizing (Fibonacci: 1, 2, 3, 5, 8, ...)

### Velocity Tracking

**Measure completed points per sprint:**
```
Velocity = Points completed / Sprint duration

Example:
  Sprint-1: 32 points completed = 32 points velocity
  Sprint-2: 28 points completed = 28 points velocity
  Average velocity = 30 points (target for Sprint-3 planning)
```

### Capacity Adjustments

Adjust capacity for known variations:

| Situation | Adjustment | Example |
|-----------|-----------|---------|
| Team vacation / holiday | -20% per person | 1 person out: 32 → 28 points |
| New team members | -30% (learning curve) | 1 new member: 32 → 22 points |
| High-risk stories | -10% per risky item | 1 risky story: 32 → 28 points |
| Release day in sprint | -10% | Release on Friday: 32 → 28 points |
| Infrastructure work | +0% (separate track) | Never reduce capacity for DevOps |

---

## Story Selection Workflow

### 1. Available Stories

Stories eligible for sprint selection must have:
- **Status:** Backlog (not Architecture, In Development, etc.)
- **Completeness:** All acceptance criteria defined
- **Technical Spec:** Clear implementation guidance
- **No blockers:** Dependencies should be resolved (or story includes blocker justification)

### 2. Selection Criteria

**Primary:** Select by priority
```
HIGH priority → MEDIUM priority → LOW priority

Example selection (40-point sprint):
  STORY-001: HIGH, 8 points
  STORY-002: HIGH, 5 points
  STORY-003: MEDIUM, 8 points
  STORY-004: MEDIUM, 5 points
  STORY-005: MEDIUM, 8 points
  STORY-006: LOW, 3 points
  STORY-007: LOW, 2 points
  Total: 39 points (within 40-point limit)
```

**Secondary:** Group by epic
```
Keep sprint focused on 1-2 epics (too many = fragmented work)

Example:
  Epic: User Authentication (4 stories, 20 points)
  Epic: Onboarding Flow (3 stories, 18 points)
  Total: 7 stories, 38 points
```

**Tertiary:** Balance story sizes
```
Mix of large (5-8 points), medium (3 points), and small (1-2 points)

Avoid:
  ❌ All large stories (risk of incomplete sprint)
  ❌ All small stories (no substantial progress)
  ✅ 40% large + 30% medium + 30% small (balanced)
```

### 3. Dependency Analysis

Before finalizing selection, verify no blocking dependencies:

```
For each selected story:
  IF story has prerequisites in "dependencies" section:
    Check if prerequisites are:
      ✅ Already completed (OK to select)
      ⏳ In current sprint (risky - order carefully)
      ❌ Not in sprint (remove story from selection OR move blocker to sprint)

Circular dependency example:
  STORY-A: "Requires STORY-B"
  STORY-B: "Requires STORY-A"
  ❌ Cannot select both in same sprint
  ✅ Resolve: Select STORY-A, defer STORY-B (create ADR explaining scope split)
```

---

## Story Status Transition: Backlog → Ready for Dev

### Prerequisites for Transition

Before sprint planning can move stories to "Ready for Dev", verify:

1. **Story Status is Backlog**
   - Cannot add stories that are "In Development", "QA Approved", etc.
   - Reset deferred stories to "Backlog" if returning from another sprint

2. **Acceptance Criteria Defined**
   - Story must have ≥ 2 acceptance criteria (Given/When/Then format)
   - Criteria must be testable and unambiguous
   - All criteria must be applicable to this sprint (not deferred)

3. **Technical Specification Complete**
   - API contracts defined (if applicable)
   - Data models documented (if applicable)
   - Business rules specified
   - Integration points identified

4. **Non-Functional Requirements Documented**
   - Performance targets (if applicable)
   - Security requirements (if applicable)
   - Scalability constraints (if applicable)
   - Accessibility requirements (if applicable)

### Transition Operation

When moving story from Backlog to Ready for Dev:

**Update YAML frontmatter:**
```yaml
status: Ready for Dev  # Changed from: Backlog
sprint: SPRINT-{N}    # Changed from: Backlog or null
```

**Add workflow history entry:**
```markdown
## Workflow History

### YYYY-MM-DD HH:MM:SS - Status: Ready for Dev
- Added to SPRINT-{N}: {Sprint Name}
- Transitioned from Backlog to Ready for Dev
- Sprint capacity: {total_points} points
- Priority in sprint: [{rank} of {total}]

### [Previous entry...]
```

### Validation After Transition

After updating story, verify:
- [ ] YAML frontmatter updated correctly
- [ ] status = "Ready for Dev"
- [ ] sprint = "SPRINT-{N}"
- [ ] Workflow history entry present
- [ ] Story links referenced correctly

---

## Sprint File Structure

### YAML Frontmatter

```yaml
---
id: SPRINT-{N}
name: {Sprint Name}
epic: {EPIC-ID or "Multiple" or "Standalone"}
start_date: YYYY-MM-DD
end_date: YYYY-MM-DD
duration_days: 14
status: Active
total_points: {sum of story points}
completed_points: 0
stories:
  - STORY-001
  - STORY-002
  - STORY-003
created: YYYY-MM-DD HH:MM:SS
---
```

**Field explanations:**

| Field | Format | Example | Notes |
|-------|--------|---------|-------|
| `id` | SPRINT-{N} | SPRINT-1 | Numbered sequentially |
| `name` | Text | User Authentication | User-provided theme |
| `epic` | EPIC-ID, "Multiple", "Standalone" | EPIC-001 | Single epic, cross-epic, or independent |
| `start_date` | YYYY-MM-DD | 2025-11-10 | Sprint start (Monday recommended) |
| `end_date` | YYYY-MM-DD | 2025-11-23 | Sprint end (Friday recommended) |
| `duration_days` | Number | 14 | Total calendar days (14, 7, or 21) |
| `status` | Active, Completed, Paused | Active | Current sprint state |
| `total_points` | Number | 32 | Sum of all selected stories |
| `completed_points` | Number | 0 | Updated as stories complete (initially 0) |
| `stories` | YAML list | - STORY-001 | All selected story IDs |
| `created` | YYYY-MM-DD HH:MM:SS | 2025-11-03 12:34:56 | Creation timestamp |

### Markdown Structure

```markdown
# Sprint {N}: {Sprint Name}

## Overview

**Duration:** {start_date} to {end_date} ({duration_days} days)
**Capacity:** {total_points} story points
**Epic:** {Epic Name} (Link: EPIC-ID)
**Status:** Active

[Brief description of sprint goals and themes]

## Sprint Goals

[High-level objectives synthesized from story themes]

Example:
"Complete user authentication system with email/password login and account creation to enable MVP user onboarding and security foundation for payment integration."

## Stories

### In Progress (X points)
[Initially empty - populated during sprint]

List of stories currently being worked:
- STORY-001: {Title} ({points} points)
- ...

### Ready for Dev ({total_points} points)

[Initially contains all selected stories, in priority order]

#### STORY-001: {Story Title}
- **Points:** {points}
- **Priority:** HIGH|MEDIUM|LOW
- **Epic:** {EPIC-ID}
- **Acceptance Criteria:** {count} criteria
- **Status:** Ready for Dev

[Repeat for each story, in priority order]

### Completed (X points)
[Initially empty - populated as stories complete]

List of completed stories:
- STORY-001: {Title} (completed {date})
- ...

## Sprint Metrics

- **Planned Velocity:** {total_points} points
- **Current Velocity:** {completed_points} points ({percentage}%)
- **Stories Planned:** {total_stories}
- **Stories Completed:** {completed_stories}
- **Days Remaining:** {calculated from end_date}
- **Burn-down Status:** [On track / At risk / Behind]

## Daily Progress

[Updated each day with cumulative progress]

### Day 1 (Monday, {date})
- Stories in progress: [count]
- Points completed: {points}
- Notes: [any blockers or decisions]

### Day 2 (Tuesday, {date})
- Stories in progress: [count]
- Points completed: {points} cumulative
- Notes: [any blockers or decisions]

[Continue through sprint end]

## Retrospective Notes

[Filled at sprint end]

### What Went Well
- [Team achievements, successes]

### What Could Be Improved
- [Process improvements, learning items]

### Velocity Analysis
- Planned: {planned_points} points
- Completed: {completed_points} points
- Variance: {+/- points}

### Action Items for Next Sprint
- [Process improvements to implement]

## Next Steps

1. Review sprint goals and story priorities
2. Start first story: `/dev STORY-[ID]`
3. Track progress daily
4. Complete sprint with: `/close-sprint`
```

---

## Sprint Duration Options

### 2-Week Sprint (Standard)

**Duration:** 14 calendar days
**Recommended:** Most teams
**Capacity:** 20-40 story points
**Pros:**
- Standard Scrum duration (industry standard)
- Predictable team velocity
- Manageable scope (easy to commit)
- Short feedback cycle
**Cons:**
- Short for large features
- Less time for refinement

**Use when:**
- Building small, focused features
- Rapid iteration needed
- Large feature split across sprints

### 1-Week Sprint (Short)

**Duration:** 7 calendar days
**Recommended:** High-velocity teams or critical projects
**Capacity:** 10-20 story points (roughly half of 2-week)
**Pros:**
- Rapid feedback
- Very focused work
- Low risk (quick course correction)
- Suits critical hotfix sprints
**Cons:**
- Very short for large features
- Frequent planning overhead (2x per month)
- Velocity harder to predict (smaller sample)

**Use when:**
- Hotfix/critical issue sprint
- New team members (rapid onboarding feedback)
- Highly volatile requirements

### 3-Week Sprint (Extended)

**Duration:** 21 calendar days
**Recommended:** Large features requiring extended development
**Capacity:** 40-60 story points (roughly 1.5x of 2-week)
**Pros:**
- Longer runway for large features
- Less planning overhead
- More time for complex integration
- Suitable for waterfall-heavy work
**Cons:**
- Hard to commit (too far out)
- Slow feedback cycle
- Velocity harder to predict (diverse scope)

**Use when:**
- Large features with dependencies
- Legacy system refactoring
- Infrastructure changes

---

## Velocity Tracking and Forecasting

### Calculate Team Velocity

After completing sprints, calculate average velocity:

```
Velocity = Points completed per sprint (time period)

Example (last 5 sprints):
  Sprint-1: 28 points completed
  Sprint-2: 32 points completed
  Sprint-3: 30 points completed
  Sprint-4: 26 points completed
  Sprint-5: 34 points completed

Average velocity = (28+32+30+26+34) / 5 = 30 points/sprint

Use this for next sprint capacity planning
```

### Forecast Completion

Use velocity to forecast when features will complete:

```
Remaining points: 95 points
Average velocity: 30 points/sprint
Estimated sprints needed: 95 / 30 ≈ 3.2 sprints

If in Sprint-10:
  Sprint-10: 28 points remaining (below velocity)
  Sprint-11: 30 points (full sprint)
  Sprint-12: 30 points (full sprint)
  Sprint-13: 7 points (partial sprint)

Forecast completion: End of Sprint-13 (~ 7 weeks)
```

---

## Common Sprint Planning Scenarios

### Scenario 1: High-Capacity Sprint

**Situation:** Team has high velocity, wants to commit to more work

```
Available points: 50 points
Standard capacity: 40 points
Adjusted capacity: 50 points

⚠️ Risk: Overcommitment may lead to incomplete sprint

Guidance:
  1. Verify team velocity supports this (historical 45+ points)
  2. Identify contingency (which stories are negotiable?)
  3. Plan for blockers (add buffer story)
  4. Review with team before committing

Decision framework:
  ✅ Accept 50 points IF previous sprint also completed 50+
  ❌ Accept 50 points if last sprint was only 25 (too risky)
```

### Scenario 2: Under-Capacity Sprint

**Situation:** Not enough backlog stories to fill sprint

```
Available points: 15 points
Standard capacity: 20-40 points
Gap: 5-25 points

Guidance:
  1. Check for blocked stories (in other statuses)
  2. Create additional stories from epic (if needed)
  3. Identify technical debt candidates
  4. Proceed with 15 points (OK to under-commit)

Decision options:
  ✅ Proceed with 15 points (partial sprint acceptable)
  ✅ Add infrastructure/tech-debt work (separate capacity track)
  ✅ Defer sprint if too small (combine with next sprint)
  ❌ Force-add unplanned work (reduces focus)
```

### Scenario 3: Cross-Epic Sprint

**Situation:** Stories from multiple epics in one sprint

```
EPIC-001: User Authentication (3 stories, 18 points)
EPIC-002: Dashboard (2 stories, 12 points)
EPIC-003: Reporting (2 stories, 10 points)
Total: 7 stories, 40 points

⚠️ Risk: Fragmented focus (3 teams working on different epics)

Guidance:
  1. Verify stories are truly independent
  2. Assign team members to epic groups (minimize context switching)
  3. Set clear integration points
  4. Coordinate releases (don't release partial epics)

Decision framework:
  ✅ Accept multi-epic sprint if stories independent
  ❌ Accept multi-epic sprint if stories blocked by each other
  ❌ Accept multi-epic sprint if requires same team member (context switch cost)
```

### Scenario 4: Risky Story Selection

**Situation:** Want to include a high-uncertainty story

```
Standard story: STORY-001, 5 points, HIGH certainty
Risky story: STORY-002, 8 points, LOW certainty (may take 13 points)

Guidance:
  1. Identify risk: "Requires third-party API integration (not yet available)"
  2. Add risk buffer: 8 points × 1.5 = 12 points (adjust capacity down)
  3. Document risk in sprint: "STORY-002 is high-risk, contingency: defer to STORY-003"
  4. Plan reviews: Check status daily (can pivot faster if needed)

Decision framework:
  ✅ Accept risky story + buffer capacity + daily reviews
  ❌ Accept risky story with normal capacity (overcommitment risk)
  ❌ Defer risky story until risk mitigated
```

---

## Integration with DevForgeAI Workflow

### Sprint in Story Lifecycle

Stories progress through 11 states, sprints bookmark key stages:

```
Backlog
  ↓ (sprint planning)
Backlog → Ready for Dev [SPRINT-{N} assigned here]
  ↓ (start development)
Ready for Dev → In Development
  ↓ (coding phase)
In Development → Dev Complete
  ↓ (QA phase)
Dev Complete → QA In Progress
  ↓ (QA validation)
QA In Progress → QA Approved / QA Failed
  ↓ (if QA Approved)
QA Approved → Releasing
  ↓ (deployment)
Releasing → Released [Sprint metrics updated here]
```

**Sprint linkage:**
- Sprint assigned when story moves to "Ready for Dev"
- Sprint metrics updated when story reaches "Released"
- Sprint retrospective completed after all sprint stories released

### Coordinated Release Pattern

Deploy multiple stories together:

```
Sprint-{N} contains: STORY-A, STORY-B, STORY-C

Release workflow:
  1. All three stories complete QA → QA Approved
  2. All three stories scheduled for Releasing → release together
  3. All three stories deployed simultaneously
  4. All three stories marked Released
  5. Sprint retrospective run after all complete

Benefit: Coordinate feature launch, marketing messaging, customer support
```

---

## Best Practices Checklist

**Before Sprint Planning:**
- [ ] Verify backlog stories are groomed (acceptance criteria, tech spec defined)
- [ ] Calculate team velocity from last 3 sprints
- [ ] Identify any known blockers or risks
- [ ] Reserve capacity for unplanned work (~20%)
- [ ] Coordinate with stakeholders on priorities

**During Sprint Planning:**
- [ ] Select stories in priority order (HIGH → MEDIUM → LOW)
- [ ] Verify dependencies (no circular dependencies)
- [ ] Mix story sizes (avoid all large or all small)
- [ ] Target 20-40 points for 2-week sprint
- [ ] Document capacity adjustments (holidays, new team members)
- [ ] Get team commitment (not manager-imposed)

**After Sprint Planning:**
- [ ] Verify sprint file created successfully
- [ ] Confirm all story statuses updated to "Ready for Dev"
- [ ] Announce sprint goals to team
- [ ] Schedule daily standups/progress reviews
- [ ] Post sprint metrics (points, goals, etc.)

**During Sprint Execution:**
- [ ] Update sprint metrics daily (progress against capacity)
- [ ] Move stories through workflow states (In Progress → Completed)
- [ ] Identify blockers immediately (don't wait for end of day)
- [ ] Adjust plan if needed (reprioritize, defer, add stories)
- [ ] Document notes for retrospective

**At Sprint End:**
- [ ] Calculate velocity (points completed / sprint duration)
- [ ] Document what went well and what could improve
- [ ] Plan action items for next sprint
- [ ] Complete sprint retrospective notes
- [ ] Update sprint status to "Completed"

---

## References

**DevForgeAI Framework:**
- **Story Definition:** `devforgeai/specs/Stories/{STORY-ID}.story.md` - Individual story format
- **Epic Definition:** `devforgeai/specs/Epics/{EPIC-ID}.epic.md` - Epic parent format
- **Sprint Definition:** `devforgeai/specs/Sprints/Sprint-{N}.md` - Generated by sprint planner

**Workflow States:**
- **Current state definitions:** `references/workflow-states.md`
- **Valid transitions:** `references/state-transitions.md`
- **Quality gates:** `references/quality-gates.md`

**Related Skills:**
- **devforgeai-orchestration:** Sprint planning coordination
- **devforgeai-development:** Story implementation (after sprint assignment)
- **requirements-analyst subagent:** Story decomposition and refinement

---

## Hook Integration (STORY-029)

**After sprint creation, feedback hooks can be triggered to capture planning insights.**

### Hook Workflow

**Phase N (Feedback Hook Integration):**
- Executes after sprint file creation (Phase 4 in /create-sprint command)
- Checks if hooks enabled: `devforgeai-validate check-hooks --operation=create-sprint --status=success`
- If enabled (exit code 0), invokes hooks with sprint context

**Sprint Context Passed to Hooks:**
- `--sprint-name="${SPRINT_NAME}"` (shell-escaped for security)
- `--story-count=${STORY_COUNT}` (number of stories selected)
- `--capacity=${CAPACITY_POINTS}` (total story points)

**Non-Blocking Design:**
- Sprint creation ALWAYS succeeds (exit code 0)
- Hook failures logged to `devforgeai/feedback/logs/hook-errors.log`
- Warning displayed: "⚠️ Feedback collection failed (sprint creation succeeded)"

**Performance:**
- Hook check: <100ms (NFR-001)
- Hook invocation setup: <3s (NFR-002)
- Total overhead: <3.5s (NFR-003)

**Use Cases:**
- Capture retrospective insights on story selection
- Identify capacity planning challenges
- Track sprint goal clarity issues
- Document team velocity trends

**Configuration:**
- Controlled via `devforgeai/config/hooks.yaml`
- See `hooks.yaml.example` for sprint-create configuration

---

**Token Budget**: Progressive disclosure (load as needed)
**Reference Usage**: Load during sprint-planner subagent invocation
**Framework Integration**: Core artifact in Epic → Sprint → Story hierarchy
