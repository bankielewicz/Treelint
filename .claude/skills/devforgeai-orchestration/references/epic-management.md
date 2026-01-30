# Epic Management Reference

Complete guide for planning, creating, and managing epics in the DevForgeAI orchestration system.

## Purpose

This reference provides detailed procedures for epic management: creating epics from requirements, decomposing into features and stories, tracking progress, and coordinating multi-sprint initiatives.

## When to Use

Reference this document when:
- Creating new epics from ideation requirements
- Breaking down epics into features and stories
- Estimating epic scope and duration
- Tracking epic progress across sprints
- Managing epic dependencies and risks

---

## Epic Creation Process

### Step 1: Load Requirements from Ideation

**Input:** Requirements from `devforgeai-ideation` skill

**Actions:**
```
Read(file_path="devforgeai/specs/requirements/[project]-requirements.md")

Extract:
- Business goals and objectives
- User personas and needs
- Feature decomposition from ideation
- Success metrics
- Complexity assessment
- Technical considerations
```

### Step 2: Define Epic Scope

**Determine what belongs in this epic:**

**Scope Criteria:**
- Single cohesive business initiative
- Deliverable within 2-6 sprints (typically)
- Clear success metrics
- Distinct from other epics (minimal overlap)

**Example Decision:**
```
Business Need: "Improve checkout experience"

Epic Candidates:
✓ EPIC-001: Checkout Flow Optimization (2-3 sprints)
  - Guest checkout
  - Saved payment methods
  - Progress indicator
  - Error recovery

✗ Don't combine: Checkout + User Profile Management
  → Separate epics (different business goals)
```

### Step 3: Create Epic Document

**Use epic template:**
```
Read(file_path=".claude/skills/devforgeai-orchestration/assets/templates/epic-template.md")

Copy template to:
Write(file_path="devforgeai/specs/Epics/EPIC-{number}-{slug}.md")

Fill in YAML frontmatter:
- id: EPIC-XXX (auto-increment)
- title: Brief, descriptive title
- status: Planning
- start_date: Planned start
- target_date: Planned completion
- total_points: Initial estimate
- owner: Product Owner
- tech_lead: Technical lead
```

### Step 4: Define Business Goal and Success Metrics

**Business Goal:**
- Clear statement of business problem
- Value delivered to users/business
- Measurable outcomes

**Success Metrics Template:**
```
Metric Format:
- Metric Name: [Action] [Subject] by [Percentage/Amount] (from [baseline] to [target])

Examples:
✓ Reduce checkout time by 30% (from 5 min to 3.5 min)
✓ Increase conversion rate by 15% (from 20% to 23%)
✓ Support 10,000 concurrent checkouts without degradation
✓ Reduce cart abandonment by 20% (from 70% to 56%)

Measurement Plan:
- How metrics will be tracked
- Baseline measurements (current state)
- Target values (desired end state)
- Review frequency (weekly, end of epic)
```

### Step 5: Epic → Feature Decomposition (Parallel Analysis)

**Decompose epic into 3-7 high-level features using parallel analysis.**

**Reference:** See `references/feature-analyzer.md` for parallel execution pattern (STORY-111).

**Feature Criteria:**
- User-facing capability
- Independently valuable
- Estimatable (can break into stories)
- Takes 1-3 sprints to complete

**Parallel Decomposition Process:**
```
# Step 1: Load parallel configuration
max_concurrent = config.profiles[profile].max_concurrent_tasks  # 4 for Pro

# Step 2: Detect feature dependencies (see dependency-graph.md)
dependencies = build_dependency_graph(features)

# Step 3: Batch independent features
batches = create_batches(features, max_concurrent)

# Step 4: Execute parallel analysis (single message per batch)
FOR each batch:
    Task(subagent="requirements-analyst", prompt="Analyze Feature 1...")
    Task(subagent="requirements-analyst", prompt="Analyze Feature 2...")
    Task(subagent="requirements-analyst", prompt="Analyze Feature 3...")
    Task(subagent="requirements-analyst", prompt="Analyze Feature 4...")
    # All execute concurrently (implicit parallelization)

# Step 5: Aggregate results
all_stories = merge(batch_results)

Example Output:
Epic: Checkout Optimization
├─ Feature 1: Guest Checkout (8 points, 1 sprint)
├─ Feature 2: Saved Payment Methods (13 points, 1-2 sprints)
├─ Feature 3: Multi-Currency Support (8 points, 1 sprint)
└─ Feature 4: Progress Indicator (3 points, 1 sprint)
```

**Time Savings:** 60% reduction vs sequential FOR loop (see feature-analyzer.md).

### Step 6: Map Features to Sprints

**Sprint Capacity Planning:**
```
Typical sprint capacity: 20-25 points
Number of sprints needed = total_points / avg_capacity

Example:
Total points: 50
Avg capacity: 20 points/sprint
Sprints needed: 3 sprints

Sprint Distribution:
- Sprint 1 (20 pts): Features 1-2 (core flow)
- Sprint 2 (18 pts): Features 3-4 (enhancements)
- Sprint 3 (12 pts): Polish, performance, release
```

### Step 7: Identify Dependencies and Risks

**Dependencies:**
```
Internal Dependencies:
- Other epics that must complete first
- Infrastructure/architecture prerequisites
- Team dependencies

External Dependencies:
- Third-party APIs
- Vendor integrations
- Compliance/legal approvals

Document:
- Dependency name
- Owner
- Status (On track / At risk / Blocked)
- ETA
- Impact if delayed
```

**Risks:**
```
Risk Assessment:
- Probability: High / Medium / Low
- Impact: High / Medium / Low
- Mitigation strategy
- Contingency plan

Common Risks:
- Technical complexity underestimated
- External dependency delays
- Resource availability changes
- Scope creep
```

---

## Epic Estimation Techniques

### Story Point Aggregation

**Bottom-up estimation:**
```
1. Break features into high-level user stories
2. Estimate each story (Fibonacci: 1, 2, 3, 5, 8, 13)
3. Sum story points per feature
4. Sum feature points for epic total

Example:
Feature 1: Guest Checkout
├─ Story 1.1: Guest checkout form (3 pts)
├─ Story 1.2: Guest data persistence (2 pts)
├─ Story 1.3: Email confirmation (2 pts)
└─ Story 1.4: Order tracking without login (3 pts)
Total: 10 points

Epic Total: Sum of all features = 50 points
```

### T-Shirt Sizing (Initial Estimate)

**When detailed stories not yet defined:**
```
XS: 5-10 points (1 sprint)
S:  10-20 points (1-2 sprints)
M:  20-40 points (2-3 sprints)
L:  40-60 points (3-4 sprints)
XL: 60+ points (4+ sprints, consider splitting)

Convert to points after decomposition
```

### Reference Class Forecasting

**Compare to similar past epics:**
```
Find similar epics completed previously
Review actual vs estimated points
Apply adjustment factor

Example:
Past Epic: Checkout v1 (estimated 40 pts, actual 52 pts)
Adjustment: +30% for complexity
Current Epic: Estimated 50 pts → Plan for 65 pts
```

---

## Epic Status Tracking

### Status Values

```
Planning:     Epic defined, not started
In Progress:  Active development in sprints
On Hold:      Paused (dependency, priority change)
At Risk:      Behind schedule or blocked
Complete:     All features delivered
Cancelled:    Epic abandoned
```

### Progress Calculation

```
Completed Points / Total Points × 100 = Progress %

Example:
Total: 50 points
Completed: 32 points
Progress: 64%

Update epic frontmatter:
Edit(file_path="devforgeai/specs/Epics/EPIC-001.md",
     old_string="completed_points: 0",
     new_string="completed_points: 32")
```

### Sprint Summary Table

**Update after each sprint:**
```
| Sprint | Status | Points | Stories | Completed | In Progress | Blocked |
|--------|--------|--------|---------|-----------|-------------|---------|
| SPRINT-001 | Complete | 20 | 8 | 8 | 0 | 0 |
| SPRINT-002 | In Progress | 18 | 7 | 4 | 3 | 0 |
| SPRINT-003 | Not Started | 12 | 5 | 0 | 0 | 0 |
| **Total** | **64%** | **50** | **20** | **12** | **3** | **0** |
```

---

## Epic → Story Linking

### Linking Stories to Epic

**In story frontmatter:**
```yaml
---
id: STORY-001
epic: EPIC-001
---
```

### Finding All Epic Stories

```
Grep(pattern="epic: EPIC-001", glob="**/*.story.md", output_mode="files_with_matches")

Returns:
- devforgeai/specs/Stories/STORY-001-guest-checkout.md
- devforgeai/specs/Stories/STORY-002-saved-payment.md
- devforgeai/specs/Stories/STORY-003-multi-currency.md
...
```

### Epic Story Inventory

**Generate story list:**
```
FOR each story in epic:
    Read(file_path=story)
    Extract: id, title, points, status

Aggregate:
- Total stories
- Completed stories
- Total points
- Completed points
- Status distribution
```

---

## Epic Completion Criteria

### Definition of Done

```
Epic is complete when:
✓ All features delivered
✓ All stories in Released status
✓ Success metrics measured and documented
✓ Retrospective completed
✓ Documentation updated
✓ Technical debt backlog reviewed
```

### Epic Closure Process

**Step 1: Verify All Stories Released**
```
all_stories = find_epic_stories(epic_id)
unreleased = filter(status != "Released")

IF unreleased.count > 0:
    BLOCK: Cannot close epic with unreleased stories
    Action: Release remaining stories or move to next epic
```

**Step 2: Measure Success Metrics**
```
Review success metrics defined at epic start
Collect actual measurements
Document results in epic:

### Metrics Achieved
- Metric 1: Actual vs Target (e.g., 28% vs 30% - close enough)
- Metric 2: Actual vs Target
- Overall: Success / Partial Success / Did Not Meet Goals
```

**Step 3: Conduct Retrospective**
```
What Went Well:
- Successes and wins
- Effective practices

What Could Be Improved:
- Challenges encountered
- Process improvements

Lessons Learned:
- Key takeaways
- Recommendations for future epics
```

**Step 4: Update Epic Status**
```
Edit(file_path="devforgeai/specs/Epics/{epic_id}.md",
     old_string="status: In Progress",
     new_string="status: Complete")

Add completion date to frontmatter
Archive epic (optional, depending on team process)
```

---

## Epic Management Best Practices

### Keep Epics Focused

**Good Epic:**
- Single business goal
- Clear scope boundaries
- 2-6 sprints duration
- Cohesive features

**Epic Too Large (Split):**
- Multiple unrelated goals
- 6+ sprints duration
- Unclear dependencies
- Features could be independent

### Regular Epic Reviews

**Weekly Reviews:**
- Progress vs plan
- Burndown trends
- Risk assessment
- Dependency status updates

**Sprint Reviews:**
- Demo completed features
- Update epic progress
- Adjust plan if needed

### Stakeholder Communication

**Communication Plan:**
```
Frequency: Weekly status updates
Format: Email summary or dashboard
Audience: Product Owner, stakeholders, leadership

Status Update Template:
- Epic: [Name and ID]
- Progress: [X]% complete ([Y] of [Z] points)
- Current Sprint: [Sprint ID and status]
- On Track: Yes / At Risk / Blocked
- Next Milestone: [Date and deliverable]
- Risks/Blockers: [List if any]
```

---

## Epic Templates and Examples

### Epic Document Template Location

```
Template: .claude/skills/devforgeai-orchestration/assets/templates/epic-template.md

Use when:
- Creating new epic
- Need complete epic structure
- Onboarding new team members to epic format
```

### Example Epic Structures

**E-Commerce Epic:**
```
Epic: Checkout Optimization
Features:
- Guest Checkout
- Saved Payment Methods
- Multi-Currency Support
- Checkout Progress Indicator
Duration: 3 sprints, 50 points
```

**Internal Tool Epic:**
```
Epic: Admin Dashboard v2
Features:
- User Management Interface
- Analytics Dashboard
- Report Generation
- Role-Based Access Control
Duration: 4 sprints, 65 points
```

**Infrastructure Epic:**
```
Epic: Platform Migration to Microservices
Features:
- Service Decomposition
- API Gateway Implementation
- Data Migration Strategy
- Deployment Automation
Duration: 6 sprints, 80 points
```

---

**Use this reference when:**
- Planning new epics from requirements
- Estimating epic scope and timeline
- Tracking epic progress across sprints
- Coordinating multi-feature initiatives
- Conducting epic retrospectives
