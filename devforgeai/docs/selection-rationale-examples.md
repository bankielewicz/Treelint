# Question Selection Rationale Examples

## Example 1: Standard Success (Dev Passed)

**Context:**
- operation_type: dev
- success_status: passed
- No errors, normal user (2 previous ops)

**Selection:**
```
Base count: 6 (dev + passed)
Modifiers: None applied
Final count: 6
Rationale: "Base(6) for dev/passed"
```

**Questions Selected:** 6 from dev/passed set, sorted by priority

---

## Example 2: First-Time User (Release Passed)

**Context:**
- operation_type: release
- success_status: passed
- operation_history: 0 release operations (first time)

**Selection:**
```
Base count: 6 (release + passed)
Modifiers: +2 (first-time user)
Final count: 8
Rationale: "Base(6) + first_time_user(+2) = 8"
```

**Questions Selected:** 8 from release/passed set, includes educational questions

---

## Example 3: Failed with Errors (QA Failed)

**Context:**
- operation_type: qa
- success_status: failed
- error_logs: ["Coverage below 95% threshold"]

**Selection:**
```
Base count: 8 (qa + failed)
Modifiers: +2 (error context)
Final count: 10 (capped at maximum)
Rationale: "Base(8) + error_context(+2) = 10"
```

**Questions Selected:** 10 from qa/failed set, investigation questions prioritized

---

## Example 4: Repeat User (Dev Passed, 5 Previous Ops)

**Context:**
- operation_type: dev
- success_status: passed
- operation_history: 5 dev operations

**Selection:**
```
Base count: 6 (dev + passed)
Modifiers: ×0.7 (repeat user), min 4
Final count: 4 (6 × 0.7 = 4.2 → 4)
Rationale: "Base(6) * repeat_user(0.7) = 4"
```

**Questions Selected:** 4 from dev/passed set, only high-priority questions

---

## Example 5: Rapid Mode (3 Ops in 10 Minutes)

**Context:**
- operation_type: orchestrate
- success_status: passed
- operation_history: 3 operations in last 10 minutes

**Selection:**
```
Base count: 6 (orchestrate + passed)
Modifiers: -3 (rapid mode)
Final count: 3
Rationale: "Base(6) - rapid_mode(3) = 3"
```

**Questions Selected:** 3 from orchestrate/passed set, ONLY priority 1-2 (critical)
