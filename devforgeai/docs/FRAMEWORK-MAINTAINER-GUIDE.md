# DevForgeAI Framework Maintainer Guide

**Audience:** Framework developers, contributors, maintainers
**Purpose:** Internal reference for maintaining and extending the DevForgeAI framework
**Version:** 1.0
**Last Updated:** 2025-11-16

---

## Hook System Lifecycle

**Overview:** The DevForgeAI hook system enables event-driven retrospective feedback after key operations complete. This section documents the complete hook lifecycle for framework maintainers.

### Hook Architecture

**Components:**
1. **Hook Configuration** (`devforgeai/config/hooks.yaml`) - Defines which operations trigger feedback
2. **Hook Registry** (`devforgeai CLI`) - Check and invoke hooks based on configuration
3. **Hook Integration Points** (Slash commands) - Phase N in commands triggers hooks after operation success
4. **Feedback Sessions** (`devforgeai/feedback/sessions/`) - Stores user responses for analysis

**Hook Execution Flow:**
```
User executes command → Operation completes → Phase N executes →
check-hooks (enabled?) → invoke-hooks (if enabled) → Feedback questions →
User responds → Session saved → Command exits 0
```

---

### Hook Lifecycle for Sprint Creation (STORY-029)

**Phase 1: User Executes Command**
```bash
/create-sprint "Sprint-5"
```

**Phase 2: Sprint Planning Workflow**
- Phase 0: Argument validation, epic discovery
- Phase 1: Story selection (user selects stories from Backlog)
- Phase 2: Sprint metadata collection
- Phase 3: Invoke orchestration skill → sprint-planner subagent
- Phase 4: Display results (sprint file created, stories updated)

**Phase 3: Hook Check (Phase N)**
```bash
# Command executes:
devforgeai check-hooks --operation=create-sprint --status=success
```

**Check Logic:**
1. Read `devforgeai/config/hooks.yaml`
2. Find hook with `operation_pattern: "create-sprint"`
3. Check `enabled: true/false`
4. Check `trigger_status` contains "success"
5. Check `trigger_conditions` (optional filters)
6. Return: Exit code 0 (trigger) or 1 (skip)

**Performance:** <100ms (NFR-001)

**Phase 4: Hook Invocation (Conditional)**

**If check-hooks returns 0:**
```bash
# Command executes:
devforgeai invoke-hooks \
  --operation=create-sprint \
  --sprint-name="Sprint-5" \
  --story-count=8 \
  --capacity=35
```

**Invocation Logic:**
1. Extract sprint context from command variables
2. Load hook configuration for create-sprint operation
3. Prepare feedback questions with sprint data interpolation
4. Present questions to user (AskUserQuestion)
5. Collect responses
6. Save to `devforgeai/feedback/sessions/create-sprint-{timestamp}-sprint-5.json`
7. Return: Exit code 0 (success)

**Performance:** <3s setup (NFR-002), total <3.5s (NFR-003)

**Phase 5: Graceful Degradation**

**If invoke-hooks fails:**
```bash
# Command logs error
echo "[ERROR] Hook invocation failed" >> devforgeai/feedback/logs/hook-errors.log

# Command displays warning (non-blocking)
Display: "⚠️ Feedback collection failed (sprint creation succeeded)"

# Command continues to exit 0 (sprint creation succeeded)
exit 0
```

**Phase 6: Command Completion**
```bash
# Sprint file exists: devforgeai/specs/Sprints/Sprint-5.md
# Stories updated: status = "Ready for Dev"
# Feedback session: devforgeai/feedback/sessions/create-sprint-*.json (if hooks enabled)
# Exit code: 0 (success)
```

---

### Hook Integration Points by Command

**Currently Integrated (3 commands):**

1. **`/dev` (STORY-023)** - Development workflow feedback
   - Hook check: `--operation=dev --status=success`
   - Context: `--story=${STORY_ID}`
   - Triggers: After Phase 5 (git commit/file tracking)

2. **`/create-epic` (STORY-028)** - Epic creation feedback
   - Hook check: `--operation=create-epic --status=success`
   - Context: `--epic-name="${EPIC_NAME}" --feature-count=${FEATURE_COUNT} --complexity=${COMPLEXITY_SCORE}`
   - Triggers: After Phase 4A.9 (epic validation complete)

3. **`/create-sprint` (STORY-029)** - Sprint planning feedback
   - Hook check: `--operation=create-sprint --status=success`
   - Context: `--sprint-name="${SPRINT_NAME}" --story-count=${STORY_COUNT} --capacity=${CAPACITY_POINTS}`
   - Triggers: After Phase 4 (sprint file created)

**Pending Integration (7 commands):**
- `/create-story` (STORY-027 - completed, needs Phase N)
- `/qa` (STORY-030)
- `/release` (STORY-031)
- `/orchestrate` (STORY-026 - completed)
- `/ideate` (STORY-032)
- `/create-context` (STORY-033)
- `/create-ui` (STORY-036)

---

### Adding Hooks to New Commands (Maintainer Procedure)

**When implementing hook integration for a new command:**

**Step 1: Add Phase N to Command File**
```markdown
### Phase N: Feedback Hook Integration

**Collect feedback after [operation] completion (non-blocking):**

\```
# Check hooks enabled
Execute: devforgeai check-hooks --operation=[operation-name] --status=success

# Conditional invocation (non-blocking)
IF check-hooks exit == 0:
    Execute: devforgeai invoke-hooks --operation=[operation-name] [context-parameters]

    IF invoke-hooks fails:
        Log to: devforgeai/feedback/logs/hook-errors.log
        Display: "⚠️ Feedback collection failed ([operation] succeeded)"
\```

**Features:**
- Non-blocking ([operation] always succeeds)
- Shell-escaped: Context parameters prevent injection
- [Operation-specific feature, e.g., "Empty sprint: --story-count=0 allowed"]
- **NFR-001:** check-hooks <100ms | **NFR-002:** invoke-hooks <3s | **NFR-003:** Total <3.5s
```

**Step 2: Define Context Parameters**

Identify what context is useful for retrospective feedback:
- `--story-id` (dev, qa, release operations)
- `--epic-name`, `--feature-count`, `--complexity` (create-epic)
- `--sprint-name`, `--story-count`, `--capacity` (create-sprint)
- Choose 2-4 parameters (not too many - keep hook invocation simple)

**Step 3: Add Hook Configuration Example**

Edit `devforgeai/config/hooks.yaml.example`:
```yaml
- id: post-[operation]-feedback
  name: "Post-[Operation] Feedback"
  operation_type: command
  operation_pattern: "[operation-name]"
  trigger_status: [success]
  trigger_conditions:
    user_approval_required: false
  feedback_type: conversation
  feedback_config:
    mode: "focused"
    questions:
      - "[Operation-specific question 1]"
      - "[Operation-specific question 2]"
      - "[Operation-specific question 3]"
  max_duration_ms: 30000
  enabled: false
  tags: [[operation-name], retrospective, story-XXX]
  metadata:
    story_id: "STORY-XXX"
    notes: |
      - Context parameters: [list]
      - Performance targets: [targets]
```

**Step 4: Create Troubleshooting Guide**

Create: `devforgeai/specs/troubleshooting/STORY-XXX-hook-not-triggering-[operation].md`

Include:
- Symptoms (hook not triggering)
- Resolution steps (10 step checklist)
- Common issues (CLI not found, hooks disabled, wrong status parameter)
- Validation checklist
- Manual testing procedure

**Step 5: Update Framework Documentation**

Add hook integration section to relevant skill reference files:
- For create-sprint: `sprint-planning-guide.md`
- For dev: `tdd-patterns.md` or `git-workflow-conventions.md`
- For qa: `qa-workflow-guide.md`
- Include: Hook workflow, context parameters, non-blocking design, performance

**Step 6: Write Comprehensive Tests**

Generate test suite (use test-automator subagent):
- Unit tests: hook check, graceful degradation, context parameters
- Edge case tests: empty data, shell injection, concurrent execution
- Performance tests: NFR-001, NFR-002, NFR-003
- Integration tests: end-to-end workflow with hooks

Target: 40-60 test cases per hook integration

**Step 7: Validate Implementation**

Run validation checklist:
- [ ] Phase N exists in command (after operation success, before completion)
- [ ] check-hooks called with correct operation and status
- [ ] invoke-hooks called with all context parameters
- [ ] All parameters shell-escaped (double quotes for strings)
- [ ] Hook failures logged to hook-errors.log
- [ ] Warning displayed on failure
- [ ] Non-blocking design (command exit 0 even if hook fails)
- [ ] Test suite: 40+ tests, 80%+ pass rate
- [ ] Configuration example added to hooks.yaml.example
- [ ] Troubleshooting guide created
- [ ] Framework docs updated

---

### Hook Performance Requirements

**All hook integrations must meet these NFRs:**

| Metric | Target | Validation |
|--------|--------|------------|
| check-hooks execution | <100ms | `time devforgeai check-hooks ...` |
| invoke-hooks setup | <3s | Measure from invocation to first AskUserQuestion |
| Phase N total overhead | <3.5s | Measure Phase N start to completion |
| Sprint creation reliability | 100% | Sprint file created even if hooks fail |
| Graceful error handling | 100% | All hook errors caught and logged |

**If targets not met:**
- Optimize CLI (caching, lazy loading, faster YAML parser)
- Reduce hook configuration complexity
- Simplify trigger conditions evaluation

---

### Hook Testing Strategy

**3-layer testing approach:**

**Layer 1: Unit Tests (40-50% of tests)**
- Hook check command execution
- Hook invocation parameter assembly
- Graceful degradation logic
- Shell escaping validation
- Empty data handling

**Layer 2: Integration Tests (30-40% of tests)**
- End-to-end command workflow with hooks
- Hook enabled vs disabled scenarios
- Hook failure resilience
- Concurrent execution

**Layer 3: E2E Tests (10-20% of tests)**
- Complete user journey (command → hooks → feedback → session saved)
- Multi-hook scenarios (multiple commands in sequence)
- Real-world edge cases (network failures, timeouts, permission errors)

**Coverage target:** 85%+ for hook integration logic

---

### Hook Rollout Strategy (EPIC-006 Pattern)

**Phase 1: Pilot (STORY-023)**
- Implement hook integration for /dev command
- Validate pattern, measure performance
- Identify edge cases

**Phase 2: Core Commands (STORY-027, STORY-028, STORY-029)**
- Integrate hooks into create-story, create-epic, create-sprint
- Refine hook invocation pattern
- Build test suite templates

**Phase 3: Remaining Commands (STORY-030-036)**
- Apply proven pattern to qa, release, orchestrate, ideate, create-context, create-ui, audit-deferrals
- Maintain consistency
- Complete 100% command coverage

**Success Criteria:**
- All 11 commands have Phase N hook integration
- All hooks non-blocking (100% reliability)
- All hooks meet performance targets (<3.5s overhead)
- Comprehensive test coverage (85%+)

---

### Maintenance Procedures

**Monthly Hook Audit:**
```bash
# 1. Check all hooks configured
grep "operation_pattern:" devforgeai/config/hooks.yaml

# 2. Verify all commands have Phase N
grep -l "Phase N" .claude/commands/*.md

# 3. Check hook error log for issues
tail -100 devforgeai/feedback/logs/hook-errors.log

# 4. Review feedback session count
ls -1 devforgeai/feedback/sessions/ | wc -l

# 5. Analyze feedback trends
# (Future: devforgeai analyze-feedback command)
```

**Quarterly Hook Performance Review:**
- Measure check-hooks latency (should be <100ms)
- Measure invoke-hooks setup time (should be <3s)
- Review user feedback on hook experience
- Optimize slow hooks or adjust configurations

**Hook Deprecation:**
- If hook provides no value after 3 months of data collection
- Set `enabled: false` in hooks.yaml
- Archive hook configuration to `hooks.yaml.archive`
- Document deprecation reason in metadata

---

## Related Stories

**Hook System Foundation:**
- STORY-018: Event-driven hook system design
- STORY-019: Operation lifecycle integration
- STORY-020: Feedback CLI commands (check-hooks, invoke-hooks)
- STORY-021: devforgeai check-hooks implementation
- STORY-022: devforgeai invoke-hooks implementation

**Hook Integration Rollout (EPIC-006):**
- STORY-023: /dev command (pilot)
- STORY-027: /create-story command
- STORY-028: /create-epic command
- STORY-029: /create-sprint command ← Current
- STORY-030: /qa command
- STORY-031: /release command
- STORY-026: /orchestrate command
- STORY-032: /ideate command
- STORY-033: /create-context command
- STORY-036: /create-ui command
- STORY-[TBD]: /audit-deferrals command

---

## Best Practices for Hook Maintainers

**1. Non-Blocking Design (Critical)**
- Hook failures must NEVER break the operation
- All errors caught and logged
- Command always exits 0 on operation success
- Warning displayed, but execution continues

**2. Performance Targets (Required)**
- check-hooks: <100ms
- invoke-hooks setup: <3s
- Total overhead: <3.5s
- Test performance with every hook integration

**3. Security (Required)**
- Shell-escape all string parameters: `"${VARIABLE}"`
- Validate all numeric parameters before passing
- Use hardcoded operation names (not user input)
- Never execute user-provided commands

**4. Context Parameters (Recommended)**
- Limit to 2-4 parameters per hook
- Choose meaningful data (sprint-name, story-count, not low-value data)
- Document parameter purpose in hooks.yaml.example
- Ensure parameters available at Phase N execution point

**5. Testing (Required)**
- 40+ test cases per hook integration
- Unit, integration, E2E coverage
- Security tests (7 injection attack vectors minimum)
- Performance tests (validate all 3 NFRs)
- Edge case tests (empty data, concurrent execution, timeouts)

**6. Documentation (Required)**
- Add hook section to relevant skill reference guide
- Create hooks.yaml.example configuration
- Write troubleshooting guide
- Update this maintainer guide

---

## Troubleshooting Hook Issues

**Issue:** Hook integration phase not executing

**Diagnosis:**
```bash
# Check if Phase N exists
grep -n "Phase N" .claude/commands/[command].md

# Check command file size (should be <15K chars)
wc -c .claude/commands/[command].md
```

**Solution:**
- If Phase N missing: Implement per STORY-XXX requirements
- If file too large: Apply lean orchestration pattern (extract to skill/subagent)

---

**Issue:** Hook check always returns exit code 1 (disabled)

**Diagnosis:**
```bash
# Check hook configuration
cat devforgeai/config/hooks.yaml | grep -A 20 "post-[operation]-feedback"

# Verify enabled field
grep "enabled:" devforgeai/config/hooks.yaml
```

**Solution:**
- Change `enabled: false` to `enabled: true`
- Restart terminal to reload config

---

**Issue:** Hook invocation timeout (>3s)

**Diagnosis:**
```bash
# Measure invoke-hooks performance
time devforgeai invoke-hooks --operation=[operation] [context-params]
```

**Solution:**
- Check CLI installation (pip-installed vs python -m devforgeai)
- Optimize hook configuration (reduce question count)
- Check disk I/O (feedback files on slow storage?)

---

## Hook System Roadmap

**Completed (EPIC-006 Phase 1-2):**
- ✅ Event-driven hook architecture (STORY-018)
- ✅ CLI commands (check-hooks, invoke-hooks) (STORY-020-022)
- ✅ Pilot integration (/dev) (STORY-023)
- ✅ Core command integrations (/create-story, /create-epic, /create-sprint) (STORY-027-029)

**In Progress (EPIC-006 Phase 3):**
- ⏳ Remaining command integrations (STORY-030-036)

**Future Enhancements:**
- 🔮 Feedback analysis dashboard (visualize trends)
- 🔮 Hook recommendation engine (suggest new hooks based on patterns)
- 🔮 Automated hook configuration generator
- 🔮 Hook A/B testing (compare different question sets)

---

## Contact & Support

**Questions or Issues:**
- File GitHub issue: [repository-url]/issues
- Contact: Framework maintainers
- Documentation: `devforgeai/docs/` directory

**Contributing:**
- Follow hook integration pattern (STORY-029 as reference)
- Run full test suite before PR
- Update this guide with new patterns discovered
- Maintain backward compatibility

---

**Last Updated:** 2025-11-16 (Added hook lifecycle section for STORY-029)
**Next Review:** 2025-12-16 (after remaining 7 command integrations complete)
