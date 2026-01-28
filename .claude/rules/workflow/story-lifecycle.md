---
description: Story state transition rules
version: "1.0"
created: 2025-12-10
---

# Story Lifecycle Rules

## States
```
Backlog → Architecture → Ready for Dev → In Development →
Dev Complete → QA In Progress → QA Approved → Releasing → Released
```

## Transition Rules

### Backlog → Architecture
- Story file created with complete acceptance criteria
- Technical specification section present

### Architecture → Ready for Dev
- Context files validated (6 files)
- No blocking ADRs pending

### Ready for Dev → In Development
- Developer assigned (or Claude starting)
- Dependencies available

### In Development → Dev Complete
- All tests written and passing
- Implementation complete per acceptance criteria

### Dev Complete → QA In Progress
- Quality gate 2 passed
- No Critical anti-pattern violations

### QA In Progress → QA Approved
- All acceptance criteria verified
- QA report generated

### QA Approved → Releasing
- Quality gate 3 passed
- Release plan documented

### Releasing → Released
- Deployment successful
- Smoke tests passed
- Rollback verified (if applicable)
