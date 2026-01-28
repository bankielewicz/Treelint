---
format_version: "v1.0"
description: "Ongoing accuracy tracking log for DevForgeAI framework"
created: 2025-12-18
purpose: "Track accuracy issues (rule violations, hallucinations, missing citations) to measure progress toward 2x hallucination reduction target (EPIC-016)"
---

# Accuracy Tracking Log

**Purpose:** Maintain a comprehensive log of accuracy issues encountered during DevForgeAI framework operations to track progress toward the 2x hallucination reduction target established in EPIC-016.

**Format Version:** v1.0

**Last Updated:** 2025-12-18

---

## Quick Reference

| Category | Definition | Severity Levels | Examples |
|----------|-----------|-----------------|----------|
| **Rule Violation** | Statement contradicts CLAUDE.md Critical Rules #1-11 | Critical, High, Medium, Low | Using Bash for file operations instead of native tools |
| **Hallucination** | Fabricated information without source; invented facts not in project | Critical, High, Medium, Low | Claiming a file exists when it doesn't; inventing API behavior |
| **Missing Citation** | Recommendation or claim made without referencing source | Critical, High, Medium, Low | Suggesting a library without citing tech-stack.md reference |

---

## Issue Categories

### Category 1: Rule Violations

**Definition:** Any statement or action that violates one of the 11 Critical Rules in CLAUDE.md.

**Rules Referenced:**
1. Check tech-stack.md before technologies
2. Use native tools over Bash for files
3. AskUserQuestion for ALL ambiguities
4. Context files are immutable
5. TDD is mandatory
6. Quality gates are strict
7. No library substitution
8. Anti-patterns forbidden
9. Document decisions in ADRs
10. Ask, don't assume
11. Git operations require user approval

**Severity Matrix:**

| Severity | Definition | Examples |
|----------|-----------|----------|
| **Critical** | Violates Rule #2 (file ops via Bash), #4 (modifies context files), #7 (substitutes locked library), #11 (git without approval) | Using Bash echo instead of Write(); editing tech-stack.md; force-pushing without user consent |
| **High** | Violates Rule #3 (no AskUserQuestion), #5 (skips TDD), #6 (ignores quality gates), #8 (uses anti-pattern) | Autonomous deferral without user question; implementing without tests; using God Object |
| **Medium** | Violates Rule #1 (skips tech-stack check), #9 (no ADR for decision), #10 (assumes instead of asks) | Suggesting tech without checking tech-stack.md; making architectural choice without documenting rationale |
| **Low** | Minor rule violations or unclear applicability | Incomplete documentation; inconsistent formatting |

**Examples:**

- *Critical:* "I'll use `echo >> file.txt` to write the configuration" (should use Write tool per Rule #2)
- *High:* "Let me implement this without tests first" (violates TDD mandate - Rule #5)
- *Medium:* "I'll use Redis for caching" (without checking if tech-stack.md allows Redis - Rule #1)
- *Low:* "The code comment is incomplete" (minor documentation issue)

---

### Category 2: Hallucinations

**Definition:** Fabricated information, invented facts, or statements presenting false information not present in any project file or publicly established fact.

**Severity Matrix:**

| Severity | Definition | Examples |
|----------|-----------|----------|
| **Critical** | Fabricated security-critical information; false claims about capabilities; invented API behavior | Claiming "user is authenticated" when no auth check happened; inventing function signatures; claiming an endpoint exists |
| **High** | Invented project structure; false file references; made-up framework features | Claiming file `X.md` exists when it doesn't; inventing a framework capability |
| **Medium** | Minor factual errors; misremembered minor details | Getting a filename slightly wrong; misremembering a count |
| **Low** | Ambiguous or context-dependent fabrications; claims that could be verified | Unclear statement that might be hallucination or imprecision |

**Examples:**

- *Critical:* "The /dev command executes automatically without user input" (false - /dev is a slash command that requires explicit invocation)
- *High:* "The file `devforgeai/metrics/baseline-2025-12-01.md` already exists in the repository" (without verifying - could be false)
- *Medium:* "There are 100 tests passing" (when actually there are 95)
- *Low:* "The framework supports three deployment environments" (should verify exact count)

---

### Category 3: Missing Citations

**Definition:** Recommendations or claims made without referencing the source file, documentation, or authority backing the statement.

**Severity Matrix:**

| Severity | Definition | Examples |
|----------|-----------|----------|
| **Critical** | Security or architectural recommendations without citing authority; critical business logic without source | Recommending authentication approach without citing CLAUDE.md rules; suggesting data storage without citing architecture docs |
| **High** | Feature recommendations without tech-stack.md reference; process recommendations without citing ADRs | Suggesting a library without citing tech-stack.md; recommending workflow without referencing DevForgeAI process |
| **Medium** | Minor technical guidance without clear source; cited source is vague | "According to best practices..." without specifying which practices or document |
| **Low** | Implicit knowledge that could benefit from source citation; context-obvious recommendations | Suggesting file naming convention without citing coding-standards.md |

**Citation Formats (Acceptable):**
- Full format: `(Source: file.md, lines X-Y)`
- Abbreviated format: `(See: file.md, section "Title")`
- Inline reference: `per CLAUDE.md Critical Rule #2`
- File reference: `(STORY-NNN requirement)`

**Examples:**

- *Critical:* "Implement JWT authentication" (should cite: "per CLAUDE.md, implement auth as specified in architecture constraints")
- *High:* "Use Jest for testing" (should cite: "per tech-stack.md, Jest 30.2.0 is the mandated test framework")
- *Medium:* "Format this as YAML" (should cite: "per coding-standards.md, configuration files use YAML format")
- *Low:* "Use meaningful variable names" (could reference: "per coding-standards.md, section 'Naming Conventions'")

---

## Entry Template with 7 Required Fields

### Template

```markdown
| Field | Value |
|-------|-------|
| Date | YYYY-MM-DD (ISO 8601) |
| Category | Rule Violation \| Hallucination \| Missing Citation |
| Severity | Critical \| High \| Medium \| Low |
| Command/Context | /dev STORY-NNN, /qa STORY-NNN, architecture question, etc. |
| Description | 50-500 characters explaining the specific issue |
| Evidence | Quote or reference demonstrating the issue |
| Resolution Status | Open \| Resolved \| Deferred |
```

### Example Entry

```markdown
| Field | Value |
|-------|-------|
| Date | 2025-12-18 |
| Category | Hallucination |
| Severity | High |
| Command/Context | /dev STORY-100 |
| Description | Claude claimed the accuracy-log.md file would be >500KB when actual requirement is <50KB. Off by 10x order of magnitude. |
| Evidence | "I'll create a large template file around 500KB to accommodate all scenarios" vs. actual NFR "File size limit < 50KB" |
| Resolution Status | Resolved |
```

### Field Definitions

- **Date (ISO 8601):** Date when issue was discovered (YYYY-MM-DD format)
- **Category:** Exactly one of: Rule Violation, Hallucination, Missing Citation (case-sensitive)
- **Severity:** Exactly one of: Critical, High, Medium, Low (case-sensitive)
- **Command/Context:** The operation during which issue occurred (e.g., "/dev STORY-001", "architecture question about storage")
- **Description:** 50-500 character explanation of the specific issue. Minimum 50 chars to force clarity; maximum 500 chars to keep focused.
- **Evidence:** Direct quote showing the issue or file reference + line number. Cannot be empty.
- **Resolution Status:** Exactly one of: Open (not yet addressed), Resolved (fixed), Deferred (intentionally postponed with reason)

---

## Usage Guidance

### When to Log an Issue

Use this decision tree to determine if an issue should be logged:

**1. Is the statement or action inaccurate or incomplete?**
   - YES → Continue to step 2
   - NO → No logging needed

**2. Could this inaccuracy affect framework maintainers or future development?**
   - YES → Continue to step 3
   - NO → No logging needed (too minor)

**3. Which category best describes the issue?**

   **Rule Violation?** (Statement contradicts CLAUDE.md #1-11)
   - Violating file operation rules (Rule #2)
   - Skipping user confirmation (Rule #3)
   - Modifying immutable files (Rule #4)
   - Skipping TDD (Rule #5)
   - Ignoring quality gates (Rule #6)
   - Using prohibited libraries (Rule #7)
   - Using anti-patterns (Rule #8)
   - Making undocumented architectural decisions (Rule #9)
   - Assuming instead of asking (Rule #10)
   - Git operations without approval (Rule #11)
   - → **LOG as Rule Violation**

   **Hallucination?** (Invented facts not in project files)
   - False claims about what's already implemented
   - Made-up function signatures or behavior
   - Invented file paths or project structure
   - Fabricated capability claims
   - → **LOG as Hallucination**

   **Missing Citation?** (Recommendation without source)
   - Technology suggestion without tech-stack.md reference
   - Process recommendation without ADR or CLAUDE.md reference
   - Architecture suggestion without authority
   - → **LOG as Missing Citation**

**4. Severity Assessment** - See matrix in each category section above

**5. Log the entry** using the template format

### How to Determine Severity Level

**Critical Issues** (Immediate impact on accuracy and trust)
- Security or authentication-related fabrications
- False claims about user approval or consent
- File operation violations (Rule #2 - use Bash instead of native tools)
- Modifying locked context files (Rule #4)
- Substituting prohibited libraries (Rule #7)
- Force-pushing or other destructive git operations without approval (Rule #11)

**High Issues** (Significant accuracy impact)
- Hallucinating entire features or capabilities
- Skipping TDD workflow
- Autonomous deferrals without user AskUserQuestion
- Violating anti-patterns list
- Missing critical architectural decisions (no ADR)
- Ignoring quality gates without justification

**Medium Issues** (Moderate accuracy impact)
- Minor technological claims without tech-stack verification
- Incomplete architectural documentation
- Assumptions made without asking user
- Off-by-one or similar small factual errors

**Low Issues** (Minimal accuracy impact)
- Ambiguous statements that might be fabrication
- Minor documentation gaps
- Context-dependent inaccuracies
- Borderline cases where intent is clear but wording is imprecise

### How to Write Effective Descriptions

**GOOD:**
- "Claude claimed file would be 500KB when requirement is <50KB (10x overestimate)"
- "Recommended library without checking tech-stack.md constraints first"
- "Implemented feature without writing tests (violated TDD Rule #5)"

**BAD:**
- "Something was wrong" (too vague)
- "Made a mistake" (unclear what mistake)
- "Didn't follow rules" (which rules? how specifically?)

**Guidelines:**
- Be specific: reference exact statements, file names, or rule numbers
- Be measurable: quantify the inaccuracy where possible
- Be actionable: make it clear what would correct the issue
- Be concise: fit explanation within 50-500 character limit

### How to Reference Evidence

**Preferred formats:**
- `(Source: file.md, lines X-Y)` - Most specific
- `(See: CLAUDE.md, Rule #2)` - For rules
- `(Ref: STORY-NNN, AC#1)` - For acceptance criteria
- Direct quote: `"exact text from response"`
- Commit reference: `(Commit abc1234: message)`

**Example:**
```
Evidence: "I'll use Bash echo to create the file" (should use Write tool per CLAUDE.md Rule #2)
```

### Review Cadence

**Recommended Schedule:**
- **Weekly:** Review open issues to identify patterns
- **Sprint Planning:** Review deferred issues, assess if they should be resolved
- **Post-QA:** Review any accuracy issues identified during QA validation
- **Monthly:** Generate trend report (see Summary Statistics Format below)

**Questions to Ask During Review:**
- Are issues being resolved or accumulating?
- Do certain command types have more issues? (/dev vs /qa vs /create-story)
- Are severity levels appropriate?
- Are there patterns in hallucination types?
- Is citation compliance improving?

---

## Baseline Reference Section

### Link to Baseline Metrics

**Baseline Document:** See `devforgeai/metrics/baseline-YYYY-MM-DD.md` for the baseline metrics collection for STORY-099.

**Baseline Status:** Baseline pending - see STORY-099 completion for initial baseline metrics. Once STORY-099 baseline is available, use it as the comparison point for measuring improvement toward the 2x hallucination reduction target.

**Baseline Data to Compare:**
- Total rule violations captured at baseline
- Total hallucinations captured at baseline
- Citation compliance rate at baseline
- Accuracy issues per operation type (ratio of issues to operations)

### Comparison Instructions

When STORY-099 baseline is available:

1. **Count current issues** by type and severity from this log
2. **Compare to baseline** counts from `baseline-YYYY-MM-DD.md`
3. **Calculate improvement:**
   - Violation rate change: (current - baseline) / baseline * 100%
   - Hallucination rate change: (current - baseline) / baseline * 100%
   - Citation compliance change: (current - baseline) / baseline * 100%
4. **Track progress toward 2x reduction** - Target: Reduce hallucinations by 50%
5. **Document in EPIC-016** progress toward accuracy goals

### Summary Statistics Format

Track these metrics monthly:

```
## Monthly Summary (YYYY-MM)

| Metric | Count | vs. Baseline | Trend |
|--------|-------|--------------|-------|
| Total Issues Logged | N | +/-% | ↑↓→ |
| Rule Violations | N | +/-% | ↑↓→ |
| Hallucinations | N | +/-% | ↑↓→ |
| Missing Citations | N | +/-% | ↑↓→ |
| **Severity Breakdown** | | | |
| Critical | N | +/-% | ↑↓→ |
| High | N | +/-% | ↑↓→ |
| Medium | N | +/-% | ↑↓→ |
| Low | N | +/-% | ↑↓→ |
| **Per-Operation Type** | | | |
| /dev operations | N issues | - | - |
| /qa operations | N issues | - | - |
| /create-story operations | N issues | - | - |
| Architecture questions | N issues | - | - |
| **Compliance** | | | |
| Issues with Evidence | N% | +/-% | ↑↓→ |
| Issues with STORY/ADR ref | N% | +/-% | ↑↓→ |
| Resolved Issues | N% | +/-% | ↑↓→ |
```

---

## Edge Case Guidance

### 1. First-Time Logging with No Baseline

**Situation:** Accuracy log is ready, but STORY-099 baseline hasn't been completed yet.

**Handling:**
- Start logging issues immediately - do NOT wait for baseline
- In "Baseline Reference" section above, the placeholder text "Baseline pending - see STORY-099" indicates this state
- Once STORY-099 baseline is available, update "Baseline Status" to link to the actual baseline document
- Continue logging without interruption

**Example:**
```
Baseline Status: Baseline pending - see STORY-099 completion.
Currently logging issues. Will compare against baseline once STORY-099 completes.
```

### 2. Multi-Category Issues

**Situation:** An issue could be classified as multiple categories (e.g., both a Rule Violation and a Hallucination).

**Handling:**
- Log under PRIMARY category (choose the most severe)
- In Description field, cross-reference secondary categories: "Also related to: [Category]"
- Count issue only ONCE in statistics (use primary category for counting)

**Example:**
```
Category: Rule Violation (PRIMARY)
Description: "Used Bash echo instead of Write() - violates Rule #2. Also contains hallucination about file size."
```

### 3. High-Volume Logging Periods

**Situation:** During intensive framework development, many issues may be logged in a single day.

**Handling:**
- Use Daily Summary format to batch entries:

```markdown
## Daily Summary - 2025-12-18

**Total Issues Logged Today:** 5
**Critical:** 1 | **High:** 2 | **Medium:** 2 | **Low:** 0

| Time | Category | Severity | Command | Summary |
|------|----------|----------|---------|---------|
| 09:15 | Rule Violation | Critical | /dev STORY-095 | Used Bash instead of Write() |
| 10:30 | Hallucination | High | /qa STORY-096 | False claim about coverage % |
| ...  | ... | ... | ... | ... |
```

### 4. Historical Issue Backfill

**Situation:** You discover an issue that occurred in the past (not today) and want to log it retroactively.

**Handling:**
- Use SEPARATE fields to track when issue occurred vs. when it was logged:

```
Date: 2025-12-15 (when issue actually occurred)
Added Date: 2025-12-18 (when added to log) [OPTIONAL FIELD FOR BACKFILL]
Category: Hallucination
...
```

**Note:** When tracking trends, use "Date" field for historical accuracy, not "Added Date".

### 5. Issue Resolution Tracking

**Situation:** An issue has been resolved. How do you track the resolution?

**Handling:**
- Update Resolution Status to "Resolved"
- Add optional resolution fields to entry:

```
Resolution Status: Resolved
Resolution Date: 2025-12-19
Resolution Reference: STORY-101 (or RCA-001 for root cause analysis)
Resolution Notes: Fixed by implementing X, verified by test Y
```

**Example Entry:**
```
| Field | Value |
|-------|-------|
| Date | 2025-12-18 |
| Category | Rule Violation |
| Severity | High |
| Command/Context | /dev STORY-100 |
| Description | Claude used autonomous deferral instead of AskUserQuestion |
| Evidence | "(Note: Fixed in Phase 06 by invoking AskUserQuestion for user approval)" |
| Resolution Status | Resolved |
| Resolution Date | 2025-12-18 |
| Resolution Reference | STORY-100 Phase 06 Deferral Challenge |
| Resolution Notes | Added AskUserQuestion for all deferrals, user provided explicit approval |
```

---

## Data Validation Rules

### Rule BR-001: Date Format (ISO 8601)

**Format:** `YYYY-MM-DD`

**Validation Regex:** `^\d{4}-\d{2}-\d{2}$`

**Examples:**
- ✅ Valid: `2025-12-18`, `2025-01-01`, `2025-12-31`
- ❌ Invalid: `12/18/2025`, `18-12-2025`, `2025-12-18T10:30:00Z` (don't include time)

### Rule BR-002: Category Values (Exact Match, Case-Sensitive)

**Allowed:** `Rule Violation`, `Hallucination`, `Missing Citation`

**Validation:** Must be EXACTLY one of these three values. No abbreviations or variations.

**Examples:**
- ✅ Valid: `Rule Violation`, `Hallucination`, `Missing Citation`
- ❌ Invalid: `rule violation`, `Rule_Violation`, `Violation`, `Hallucinate`, `Missing Cit.`

### Rule BR-003: Severity Values (Exact Match, Case-Sensitive)

**Allowed:** `Critical`, `High`, `Medium`, `Low`

**Validation:** Must be EXACTLY one of these four values. Case-sensitive.

**Examples:**
- ✅ Valid: `Critical`, `High`, `Medium`, `Low`
- ❌ Invalid: `critical`, `CRITICAL`, `Crit`, `Sev1`, `1 - Critical`

### Rule BR-004: Description Length

**Length Range:** Minimum 50 characters, Maximum 500 characters

**Validation:** Count characters (excluding field label and spaces).

**Examples:**
- ✅ Valid (98 chars): "Claude recommended using Redis for caching without checking tech-stack.md first, violating Rule #1"
- ❌ Too short (23 chars): "Didn't check tech stack"
- ❌ Too long (512 chars): "[Very long description exceeding 500 character limit...]"

### Rule BR-005: Evidence Cannot Be Empty

**Validation:** Evidence field must contain something - a quote, file reference, or explanation.

**Examples:**
- ✅ Valid: `"I'll use Bash echo"`, `(Source: CLAUDE.md, Rule #2)`, `(Commit abc1234)`
- ❌ Invalid: (empty), `N/A`, `(none)`

### Rule BR-006: Resolution Status

**Allowed:** `Open`, `Resolved`, `Deferred`

**Validation:** Must be EXACTLY one of these values. Case-sensitive.

**Examples:**
- ✅ Valid: `Open`, `Resolved`, `Deferred`
- ❌ Invalid: `open`, `RESOLVED`, `In Progress`, `Pending`

---

## Notes

- **Framework Version:** DevForgeAI (Claude Code Terminal 1.0+)
- **EPIC Reference:** EPIC-016 (Memory Architecture & Accuracy)
- **Related Stories:** STORY-099 (Baseline Metrics), STORY-101 (Citation Format Standards)
- **Last Review:** 2025-12-18
- **Next Review Due:** 2025-12-25 (weekly cadence)

---

**Template Version:** v1.0
**Last Updated:** 2025-12-18
