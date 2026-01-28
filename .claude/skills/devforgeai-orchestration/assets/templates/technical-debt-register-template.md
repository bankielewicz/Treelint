---
version: "2.0"
last_updated: "YYYY-MM-DD"
analytics:
  total_open: 0
  total_in_progress: 0
  total_resolved: 0
  by_type:
    story_split: 0
    scope_change: 0
    external_blocker: 0
  by_priority:
    critical: 0
    high: 0
    medium: 0
    low: 0
  by_source:
    dev_phase_06: 0
    qa_discovery: 0
thresholds:
  warning_count: 5
  critical_count: 10
  blocking_count: 15
  stale_days: 90
---

# Technical Debt Register v2.0

**Purpose:** Track deferred Definition of Done items to ensure follow-up and prevent lost work

**Maintained by:** devforgeai-development skill (auto-updates when deferrals occur)
**Analyzed by:** technical-debt-analyzer subagent (sprint planning, retrospectives)

---

## Debt Item Format

Each debt item follows the DEBT-NNN structured format:

| ID | Date | Source | Type | Priority | Status | Effort | Follow-up |
|----|------|--------|------|----------|--------|--------|-----------|
| DEBT-NNN | YYYY-MM-DD | dev_phase_06 \| qa_discovery | Story Split \| Scope Change \| External Blocker | Critical \| High \| Medium \| Low | Open \| In Progress \| Resolved | N points | STORY-XXX \| ADR-XXX |

**Field Definitions:**
- **ID**: 3-digit zero-padded identifier (DEBT-001, DEBT-002, etc.)
- **Date**: ISO 8601 format (YYYY-MM-DD)
- **Source**: Origin of debt (`dev_phase_06` = Phase 06 Deferral, `qa_discovery` = QA validation)
- **Type**: Category (Story Split, Scope Change, External Blocker)
- **Priority**: Urgency level (Critical, High, Medium, Low)
- **Status**: Current state (Open, In Progress, Resolved)
- **Effort**: Estimated work (points or hours)
- **Follow-up**: Reference to follow-up story or ADR (STORY-XXX or ADR-XXX)

---

## Open Debt Items

| ID | Date | Source | Type | Priority | Status | Effort | Follow-up |
|----|------|--------|------|----------|--------|--------|-----------|
<!-- No open debt items - add entries using DEBT-NNN format above -->

---

## In Progress Debt Items

| ID | Date | Source | Type | Priority | Status | Effort | Follow-up |
|----|------|--------|------|----------|--------|--------|-----------|
<!-- No items in progress -->

---

## Resolved Debt Items

| ID | Date | Source | Type | Priority | Status | Effort | Follow-up |
|----|------|--------|------|----------|--------|--------|-----------|
<!-- No resolved items -->

---

## Usage Instructions

### Auto-Creation by technical-debt-analyzer

This template is automatically used by the technical-debt-analyzer subagent when:
1. `devforgeai/technical-debt-register.md` does not exist
2. Subagent is invoked during sprint planning or /create-sprint command

**Auto-creation workflow:**
```
IF devforgeai/technical-debt-register.md not found:
    Read this template
    Replace "YYYY-MM-DD" with current ISO date
    Write to devforgeai/technical-debt-register.md
    Log: "Created technical debt register from template"
```

### Adding New Debt Items

**ID Assignment:**
1. Use next available ID: `max(existing IDs) + 1`
2. If register is empty, start with `DEBT-001`
3. Always use 3-digit zero-padding (DEBT-001, not DEBT-1)
4. Validation regex: `^DEBT-[0-9]{3}$`

**Required Fields:**
All 8 fields (ID, Date, Source, Type, Priority, Status, Effort, Follow-up) are mandatory.

**Source Field Values:**
| Value | Description |
|-------|-------------|
| `dev_phase_06` | Debt added during Phase 06 (Deferral Challenge) of /dev workflow |
| `qa_discovery` | Debt discovered during /qa validation |

### Threshold Alerting

| Threshold | Level | Action |
|-----------|-------|--------|
| 5+ open items | Warning | Review debt in sprint planning |
| 10+ open items | Critical | Schedule debt reduction sprint |
| 15+ open items | Blocking | Block new feature work until debt reduced |
| >90 days stale | Escalate | Review and close or create follow-up |

---

## Integration Points

| Role | Component |
|------|-----------|
| Auto-Updated By | devforgeai-development skill (Phase 06), devforgeai-qa skill |
| Auto-Analyzed By | technical-debt-analyzer subagent (sprint planning, retrospectives) |
| Referenced By | devforgeai-orchestration skill, /create-sprint, /review-qa-reports |

---

**Template Version:** 2.0 (STORY-285)
**Template Location:** .claude/skills/devforgeai-orchestration/assets/templates/technical-debt-register-template.md
