# Story Management Reference

Complete guide for creating, updating, and tracking stories through the DevForgeAI workflow lifecycle.

## Purpose

This reference provides detailed procedures for story operations: creating story documents, defining acceptance criteria, updating status, managing workflow history, and tracking story progression through all 11 workflow states.

## When to Use

Reference this document when:
- Creating new story documents
- Updating story status through workflow
- Appending workflow history entries
- Managing story checkboxes
- Understanding story document structure

---

## Story Document Structure

### Required Sections

Every story MUST include these sections:

1. **YAML Frontmatter** - Metadata
2. **Description** - User story format
3. **Acceptance Criteria** - Testable conditions
4. **Technical Specification** - Implementation details
5. **Non-Functional Requirements** - Performance, security, scalability
6. **Dependencies** - Prerequisites and external dependencies
7. **Workflow Status** - Phase completion checkboxes
8. **Notes** - Additional context

### Complete Story Template Location

```
Template: .claude/skills/devforgeai-orchestration/assets/templates/story-template.md

Contains:
- Complete YAML frontmatter with all fields
- Acceptance criteria format (Given/When/Then)
- Technical specification templates (API, data models, business rules)
- NFR templates (performance, security, scalability)
- Test strategy templates
```

---

## Story Creation

**For creating new story documents, use the devforgeai-story-creation skill:**

```
Skill(command="devforgeai-story-creation")
```

**The story-creation skill handles:**
- Story ID generation (sequential STORY-XXX numbering)
- Requirements analysis (requirements-analyst subagent)
- Technical specification (api-designer subagent, data models, business rules)
- UI specification (components, mockups, interfaces, accessibility)
- Story file creation (YAML frontmatter + all markdown sections)
- Epic/sprint linking (updates parent documents)
- Quality validation (self-validation with validation-checklists.md)
- Completion reporting (summary and next actions)

**Complete story creation documentation:**
`.claude/skills/devforgeai-story-creation/SKILL.md`

**Story creation reference files:**
- story-structure-guide.md - YAML frontmatter fields, required sections
- acceptance-criteria-patterns.md - Given/When/Then patterns by story type
- technical-specification-guide.md - API contracts, data models, business rules
- ui-specification-guide.md - Component mockups, interfaces, accessibility
- validation-checklists.md - Quality validation procedures
- story-examples.md - Complete story examples (CRUD, auth, workflow, reporting)

**This reference (story-management.md) focuses on story lifecycle management:**
- Story status updates (workflow state transitions)
- Workflow history tracking
- Story estimation
- QA results integration
- Lifecycle best practices

---

## Story Document Structure (Reference)

### Required Sections

Every story MUST include these sections:

1. **YAML Frontmatter** - Metadata (id, title, epic, sprint, status, points, priority, dates)
2. **User Story** - As a/I want/So that format
3. **Acceptance Criteria** - Testable conditions (Given/When/Then, minimum 3)
4. **Technical Specification** - API contracts, data models, business rules, dependencies
5. **Non-Functional Requirements** - Performance, security, scalability (measurable)
6. **Edge Cases & Error Handling** - Boundary conditions, error scenarios
7. **Definition of Done** - Implementation, testing, documentation, security checklists
8. **Workflow History** - Status change log with timestamps

**Optional Sections:**
- **UI Specification** - Components, mockups, interfaces, accessibility (if UI story)

### Story Template Location

```
Primary Template: .claude/skills/devforgeai-story-creation/assets/templates/story-template.md
Backup Template: .claude/skills/devforgeai-orchestration/assets/templates/story-template.md

Template contains:
- Complete YAML frontmatter with all fields
- All required markdown sections
- Placeholders for content
- Format guidance and examples
```

---

## Story Lifecycle Management

**Data Models:**
- Entity classes/tables
- Database schema
- Relationships (one-to-many, many-to-many)
- Indexes
- Constraints

**Business Rules:**
- Domain logic
- Validation rules
- Error handling
- State transitions

**See story template for complete examples and format**

### Step 7: Define Non-Functional Requirements

**Performance:**
```
- Response time targets (p95, p99)
- Throughput requirements
- Concurrent user support
- Load testing criteria
```

**Security:**
```
- Authentication method
- Authorization rules
- Data protection (encryption, PII)
- Security testing checklist
- Rate limiting
```

**Scalability:**
```
- Horizontal scaling readiness
- Database growth projections
- Caching strategy
```

---

## Story Status Management

### Status Update Process

**Update Status in Frontmatter:**
```
Edit(file_path="devforgeai/specs/Stories/{story_id}.md",
     old_string="status: Backlog",
     new_string="status: Architecture")
```

### All 11 Status Values

```
1. Backlog        - Not started, awaiting sprint assignment
2. Architecture   - Context files being created/validated
3. Ready for Dev  - Context complete, can start development
4. In Development - Active TDD implementation
5. Dev Complete   - Tests pass, ready for QA
6. QA In Progress - Deep validation running
7. QA Failed      - Quality violations detected
8. QA Approved    - All quality gates passed
9. Releasing      - Deployment in progress
10. Released      - Production deployment complete
11. Blocked       - Waiting for external dependency
```

**For detailed state definitions, see:** `references/workflow-states.md`
**For transition rules, see:** `references/state-transitions.md`

---

## Workflow Checkbox Management

### Checkbox Locations

```markdown
## Workflow Status

- [ ] Architecture phase complete
- [ ] Development phase complete
- [ ] QA phase complete
- [ ] Released
```

### Checking Boxes

**After Architecture Phase:**
```
Edit(file_path="devforgeai/specs/Stories/{story_id}.md",
     old_string="- [ ] Architecture phase complete",
     new_string="- [x] Architecture phase complete")
```

**After Development Phase:**
```
Edit(file_path="devforgeai/specs/Stories/{story_id}.md",
     old_string="- [ ] Development phase complete",
     new_string="- [x] Development phase complete")
```

**After QA Phase:**
```
Edit(file_path="devforgeai/specs/Stories/{story_id}.md",
     old_string="- [ ] QA phase complete",
     new_string="- [x] QA phase complete")
```

**After Release:**
```
Edit(file_path="devforgeai/specs/Stories/{story_id}.md",
     old_string="- [ ] Released",
     new_string="- [x] Released")

# Also add completion timestamp to frontmatter
Edit(file_path="devforgeai/specs/Stories/{story_id}.md",
     old_string="created: {created_date}",
     new_string="created: {created_date}\ncompleted: {timestamp}")
```

---

## Workflow History Management

### Workflow History Section

**Location in Story:**
```markdown
## Workflow History

[History entries appear here in reverse chronological order]

## Notes

[Notes section follows]
```

### Creating Workflow History Section

**If section doesn't exist:**
```
Read(file_path="devforgeai/specs/Stories/{story_id}.md")

IF "## Workflow History" NOT in content:
    # Insert before Notes section
    Edit(file_path="devforgeai/specs/Stories/{story_id}.md",
         old_string="## Notes",
         new_string="## Workflow History\n\n## Notes")
```

### Workflow History Entry Format

**Standard Entry Template:**
```markdown
### {timestamp} - {new_status}
- **Previous Status:** {old_status}
- **Action Taken:** {action_description}
- **Result:** {result_summary}
- **Next Steps:** {next_steps}
```

**Example Entries:**

**Architecture Phase:**
```markdown
### 2025-10-30 14:23:15 - Architecture
- **Previous Status:** Backlog
- **Action Taken:** Story assigned to Sprint 1, moving to architecture validation
- **Result:** devforgeai-architecture skill invoked
- **Next Steps:** Create/validate all 6 context files
```

**Ready for Dev:**
```markdown
### 2025-10-30 14:45:32 - Ready for Dev
- **Previous Status:** Architecture
- **Action Taken:** All context files created and validated
- **Result:** Context files: 6/6 complete (tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns)
- **Next Steps:** Developer can start TDD workflow
```

**Development Start:**
```markdown
### 2025-10-30 15:10:05 - In Development
- **Previous Status:** Ready for Dev
- **Action Taken:** Developer John started TDD workflow
- **Result:** devforgeai-development skill invoked
- **Next Steps:** Execute 6 TDD phases (Context → Test → Implement → Refactor → Integrate → Git)
```

**Dev Complete:**
```markdown
### 2025-10-31 16:42:18 - Dev Complete
- **Previous Status:** In Development
- **Action Taken:** TDD workflow complete, all tests passing
- **Result:** Build: Success, Tests: 47/47 passing, Light validation: Clean
- **Next Steps:** Deep QA validation
```

**QA Approved:**
```markdown
### 2025-10-31 17:05:33 - QA Approved
- **Previous Status:** QA In Progress
- **Action Taken:** Deep QA validation PASSED
- **Result:** Coverage: Business 97%, Application 88%, Infrastructure 82%, Overall 85%
           Violations: 0 Critical, 0 High, 2 Medium, 5 Low
           All acceptance criteria validated
- **Next Steps:** Ready for release
```

**QA Failed (example):**
```markdown
### 2025-10-31 17:05:33 - QA Failed
- **Previous Status:** QA In Progress
- **Action Taken:** Deep QA validation FAILED
- **Result:** Critical: 2, High: 3
           Coverage: Business 87% (< 95% threshold)
           Missing tests for edge cases in UserService
- **Next Steps:** Fix violations and return to development
- **Action Items:**
  - [P0] Fix SQL injection vulnerability in search query
  - [P0] Add authorization check to DELETE endpoint
  - [P1] Add unit tests for UserService edge cases (8 tests needed)
```

**Released:**
```markdown
### 2025-11-01 10:15:42 - Released
- **Previous Status:** Releasing
- **Action Taken:** Successfully deployed to production
- **Result:** Version: 1.2.0
           Environment: Production
           Health: All checks passing
           Deployment method: Blue-Green
- **Next Steps:** Monitor production, story lifecycle complete
```

### Appending Workflow History

**Append New Entry (Most Recent First):**
```
Read(file_path="devforgeai/specs/Stories/{story_id}.md")

new_entry = f"""
### {timestamp} - {new_status}
- **Previous Status:** {old_status}
- **Action Taken:** {action}
- **Result:** {result}
- **Next Steps:** {next_steps}
"""

# Insert after ## Workflow History header
Edit(file_path="devforgeai/specs/Stories/{story_id}.md",
     old_string="## Workflow History\n\n",
     new_string=f"## Workflow History\n\n{new_entry}\n")
```

---

## Story Estimation

### Story Point Scale

**Fibonacci Sequence:**
```
1 point:  Trivial (< 2 hours)
2 points: Simple (2-4 hours)
3 points: Moderate (4-8 hours, ~1 day)
5 points: Complex (1-2 days)
8 points: Very complex (2-3 days)
13 points: Too large (consider splitting)
```

### Estimation Guidelines

**Factors to Consider:**
```
- Technical complexity
- Amount of code to write
- Testing effort required
- Unknown/learning curve
- Dependencies and integrations
- Risk and uncertainty
```

**Estimation Process:**
```
1. Team discusses story
2. Each member estimates independently
3. Reveal estimates simultaneously (Planning Poker)
4. Discuss outliers (why high/low?)
5. Re-estimate until consensus
6. Record final estimate in frontmatter
```

### Story Splitting

**When to Split:**
```
- Story > 8 points
- Story spans multiple features
- Story has independent deliverables
- Story has dependencies that can be separate
```

**How to Split:**
```
By workflow: Login → Registration → Password Reset (3 stories)
By persona: Admin view → User view (2 stories)
By CRUD: Create user → Read user → Update user → Delete user (4 stories)
By acceptance criteria: 1 criterion per story
```

---

## QA Results Integration

### Adding QA Results Section

**After QA Validation:**
```
Read(file_path="devforgeai/qa/reports/{story_id}-qa-report.md")
Parse QA results

Add section to story (before Workflow History):
```

**QA Results Section Format (PASS):**
```markdown
## QA Results

**Date:** 2025-10-31 17:05:33
**Status:** ✅ PASS
**Validator:** devforgeai-qa skill

### Summary
- Overall Coverage: 89%
- Business Logic: 97%
- Application Layer: 88%
- Infrastructure: 82%
- Critical Issues: 0
- High Issues: 0
- Medium Issues: 2
- Low Issues: 5

### Coverage by Layer
| Layer | Coverage | Threshold | Status |
|-------|----------|-----------|--------|
| Business Logic | 97% | 95% | ✅ Pass |
| Application | 88% | 85% | ✅ Pass |
| Infrastructure | 82% | 80% | ✅ Pass |
| Overall | 89% | 80% | ✅ Pass |

### Violations
- 0 CRITICAL
- 0 HIGH
- 2 MEDIUM (non-blocking)
- 5 LOW (recommendations)

### Report
Full QA Report: `devforgeai/qa/reports/STORY-001-qa-report.md`

### Next Steps
- Story approved for release
- Medium/Low violations are recommendations only
- Can proceed to production deployment
```

**QA Results Section Format (FAIL):**
```markdown
## QA Results

**Date:** 2025-10-31 17:05:33
**Status:** ❌ FAIL
**Validator:** devforgeai-qa skill

### Summary
- Overall Coverage: 78% (below 80% threshold)
- Business Logic: 87% (below 95% threshold)
- Critical Issues: 2
- High Issues: 3

### Blocking Issues
1. **[CRITICAL]** SQL injection vulnerability in UserService.Search()
   - File: src/Application/Services/UserService.cs:142
   - Fix: Use parameterized query instead of string concatenation

2. **[CRITICAL]** Hardcoded API key in PaymentController
   - File: src/API/Controllers/PaymentController.cs:28
   - Fix: Move to configuration/secrets management

3. **[HIGH]** Missing authorization check on DELETE /api/users/{id}
   - File: src/API/Controllers/UserController.cs:85
   - Fix: Add [Authorize(Roles = "Admin")] attribute

### Coverage Gaps
- Business Logic: 87% (need 95%, missing 8% = ~12 unit tests)
- Missing tests for UserService edge cases

### Action Items
- [ ] [P0] Fix SQL injection (UserService.Search)
- [ ] [P0] Remove hardcoded API key (PaymentController)
- [ ] [P0] Add authorization check (DELETE endpoint)
- [ ] [P1] Add 12 unit tests for UserService edge cases
- [ ] [P1] Add integration tests for error handling

### Report
Full QA Report: `devforgeai/qa/reports/STORY-001-qa-report.md`

### Next Steps
- Return to In Development status
- Fix all CRITICAL and HIGH violations
- Add tests to meet coverage thresholds
- Re-run QA validation after fixes
```

---

## Story Lifecycle Best Practices

### Keep Stories Small

**Target:** 3-5 points (1-2 days of work)
**Maximum:** 8 points (split if larger)
**Benefit:** Faster feedback, easier estimation, lower risk

### Clear Acceptance Criteria

**Quality Checklist:**
- [ ] 3-7 criteria per story
- [ ] Testable and specific
- [ ] Covers happy path and edge cases
- [ ] Independent (can test separately)
- [ ] Valuable (user-centric)

### Track Dependencies Early

**Document in Dependencies section:**
- Prerequisite stories
- External dependencies
- Technology dependencies

**Check before sprint:**
- Validate dependencies resolved
- Create mocks/stubs if needed
- Escalate blocking dependencies

### Maintain Clean History

**Workflow history provides:**
- Complete audit trail
- Status transition reasoning
- Problem investigation context
- Team communication record

**Best practices:**
- Append after every status change
- Include actionable details
- Document decisions made
- Reference artifacts (QA reports, ADRs)

---

## Story Document Templates

### Template Location

```
Main Template:
.claude/skills/devforgeai-orchestration/assets/templates/story-template.md

Contains:
- Complete story structure (610 lines)
- All sections with examples
- API endpoint templates (C# examples)
- Data model templates
- Business rules format
- NFR templates
- Test strategy templates
```

### Quick Story Creation

```
# 1. Copy template
Read(file_path=".claude/skills/devforgeai-orchestration/assets/templates/story-template.md")

# 2. Replace placeholders
content = template.replace("STORY-XXX", story_id)
content = content.replace("[Story Title]", story_title)
content = content.replace("EPIC-XXX", epic_id)
content = content.replace("SPRINT-XXX", sprint_id)

# 3. Write story
Write(file_path=f"devforgeai/specs/Stories/{story_id}-{slug}.md", content=content)

# 4. Fill in details manually or via prompts
```

---

**Use this reference when:**
- Creating new story documents
- Updating story status through workflow
- Managing workflow checkboxes and history
- Understanding story document structure
- Integrating QA results into stories
- Tracking story lifecycle and progress
