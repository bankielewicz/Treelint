# Technical Debt Register

**Purpose:** Track deferred Definition of Done items to ensure follow-up and prevent lost work

**Maintained by:** devforgeai-development skill (auto-updates when deferrals occur)
**Analyzed by:** technical-debt-analyzer subagent (sprint planning, retrospectives)

---

## Debt Inventory

### Open Debt Items

**Last Updated:** [Auto-updated by dev skill]
**Total Open Items:** 0

<!-- Template for new entries:

---
#### [STORY-ID]: [Deferred Item Description]

**Date Deferred:** YYYY-MM-DD
**Type:** [Story Split | Scope Change | External Blocker]
**Justification:** [Deferral reason from DoD Status]
**Follow-up:** [STORY-XXX | ADR-XXX | Blocker condition]
**Priority:** [High | Medium | Low]
**Status:** Open
**Resolution Target:** [Sprint-X | YYYY-MM-DD | When {condition}]
**Estimated Effort:** [X points | Y hours]

---

-->

### In Progress Debt Items

<!-- Items being actively worked on in current sprint -->

### Resolved Debt Items

<!-- Items completed - moved here for historical tracking -->

---

## Analysis

**Last Analyzed:** Never
**Total Debt (Open + In Progress):** 0 items
**Average Age:** N/A
**Oldest Item:** N/A

**Debt by Type:**
- Story Splits: 0 (0%)
- Scope Changes: 0 (0%)
- External Blockers: 0 (0%)

**Critical Issues:**
- Circular Deferrals: None detected
- Stale Debt (>90 days): None

**Recommendations:** None at this time

---

## Usage Guidelines

### When Debt is Added

**Automatic Addition:**
- devforgeai-development skill adds entries when DoD items deferred with "External blocker" reason
- Entry includes: Story ID, item, blocker description, date, status

**Manual Addition:**
- Use template above to log technical debt from any source
- Follow format for consistency
- Update status regularly

### When Debt is Analyzed

**Automated Analysis:**
- technical-debt-analyzer subagent runs during sprint planning
- Generates reports in `devforgeai/technical-debt-analysis-{date}.md`
- Updates "Analysis" section above

**Manual Analysis:**
- Run `/create-sprint` command (invokes analyzer)
- Review quarterly for long-term trends
- Check before major releases

### When Debt is Resolved

**Move to "Resolved Debt Items" section:**
- Update status to "Resolved"
- Add resolution date and story ID
- Keep for 6 months for trend analysis
- Archive annually

---

## Debt Trends (Historical)

<!-- Track deferral rates over time -->

**Sprint Deferral Rates:**
- Sprint-1: TBD
- Sprint-2: TBD

**Resolution Velocity:**
- Items resolved per sprint: TBD
- Average time to resolution: TBD days

---

## Example Entry

```markdown
---
#### STORY-004: Exit Code Handling in main.rs

**Date Deferred:** 2025-11-03
**Type:** Story Split
**Justification:** Deferred to STORY-005: Exit code handling will be integrated with error framework
**Follow-up:** STORY-006 (created to close circular deferral gap)
**Priority:** High
**Status:** Open
**Resolution Target:** Sprint-2
**Estimated Effort:** 2 points

**Notes:**
- Original deferral created circular dependency (STORY-004 → STORY-005 → STORY-004)
- STORY-006 created to break cycle and complete integration
- Flagged by RCA-006 deferral validation analysis

---
```

---

## Review Schedule

**Weekly:** Review new debt items added during sprint
**Sprint End:** Analyze debt trends, plan debt reduction if needed
**Quarterly:** Comprehensive debt review, close stale items, update resolution strategies

**Debt Reduction Triggers:**
- Open debt >10 items → Schedule debt reduction sprint
- Any item >90 days → Escalate or close
- Circular deferrals detected → Immediate resolution required
- Deferral rate >20% → Review story planning process

---

## Integration Points

**Auto-Updated By:**
- devforgeai-development skill (Phase 6, Step 1 - when external blockers deferred)

**Auto-Analyzed By:**
- technical-debt-analyzer subagent (sprint planning, retrospectives)

**Referenced By:**
- devforgeai-orchestration skill (Phase 5 - Deferred Work Tracking)
- /create-sprint command (before sprint planning)

---

**Created:** 2025-11-03 (RCA-006 implementation)
**Template Version:** 1.0
