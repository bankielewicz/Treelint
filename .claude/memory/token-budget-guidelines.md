# Token Budget Guidelines for Skills and Subagents

Guidelines for managing token usage in the main conversation vs. isolated contexts.

---

## Context Window Budget

**Main Conversation:** 1,000,000 tokens available

**Check current usage:**
```bash
> /context
# Shows: XXk/1000k tokens (X%)
```

---

## Token Budget Heuristic

### Usage Levels

| Range | Level | Action Guidance |
|-------|-------|----------------|
| **0-25%** | 🟢 GREEN | Plenty of space, no constraints. Complete all work thoroughly. |
| **26-50%** | 🟡 YELLOW | Moderate usage, continue normally. Monitor but don't restrict. |
| **51-75%** | 🟠 ORANGE | High usage, apply efficiency best practices (native tools). |
| **76-90%** | 🔴 RED | Very high, optimize carefully. Use subagents for heavy work. |
| **91-100%** | 🔴 CRITICAL | Near limit, drastic optimization. Use /compact if needed. |

### Example Decisions

**Scenario 1: 10.5% usage (GREEN)**
```
Current: 105,000 / 1,000,000 = 10.5%
Status: 🟢 GREEN

Action: Complete all planned work thoroughly
Rationale: 895K tokens remaining (89.5% free)
Decision: Don't skip epic documents, don't cut corners
```

**Scenario 2: 75% usage (ORANGE)**
```
Current: 750,000 / 1,000,000 = 75%
Status: 🟠 ORANGE

Action: Use efficiency best practices
Rationale: 250K tokens remaining (25% free)
Decision: Use native tools (Read vs cat), minimize verbose output
```

**Scenario 3: 92% usage (CRITICAL)**
```
Current: 920,000 / 1,000,000 = 92%
Status: 🔴 CRITICAL

Action: Drastic optimization needed
Rationale: Only 80K tokens remaining (8% free)
Decision: Use subagents for heavy work, compact conversation if needed
```

---

## Token Efficiency Targets (For Isolated Contexts)

**These targets apply to SUBAGENT OPERATIONS, not main conversation:**

- Light QA validation: ~10,000 tokens (in subagent context)
- Deep QA validation: ~65,000 tokens (in subagent context)
- Feature implementation: ~80,000 tokens (in subagent context)
- Total per story (dev + QA): ~155,000 tokens (across multiple subagent contexts)

**Key Point:** Subagents operate in isolated contexts. Their token usage does NOT count against the main conversation's 1M budget.

---

## Common Misunderstanding

### ❌ INCORRECT Interpretation

"Token efficiency guidance says target <100K, so I should stop working at 105K tokens in the main conversation."

### ✅ CORRECT Interpretation

"Token efficiency guidance is about:
1. **Tool choice** (Read vs cat = 40% savings per operation)
2. **Subagent operations** (each subagent should stay <100K in its isolated context)
3. **NOT about main conversation length** (which has 1M tokens available)"

---

## Decision Framework

### Before Making Efficiency Decisions

1. **Check Facts:**
   ```
   Run: /context
   Note: Current usage percentage
   ```

2. **Apply Heuristic:**
   ```
   If 0-25% (GREEN): No constraints, complete all work
   If 26-50% (YELLOW): Continue normally
   If 51-75% (ORANGE): Use efficiency best practices
   If 76%+ (RED/CRITICAL): Optimize actively
   ```

3. **Consider User Instructions:**
   ```
   Did user say "no time constraints"?
   Did user say "context window is plenty big"?
   → If YES: Ignore efficiency concerns, prioritize thoroughness
   ```

4. **If Uncertain:**
   ```
   Use AskUserQuestion:
   "Current token usage is X%. Should I continue with complete work or optimize?"
   ```

---

## Priority Hierarchy

When making decisions:

**Priority 1: EXPLICIT USER INSTRUCTIONS** (Highest)
- "No time constraints" → Complete all work
- "Be thorough" → Don't skip steps
- "Context window is plenty big" → Ignore token concerns

**Priority 2: CONTEXT FILE CONSTRAINTS**
- tech-stack.md locked technologies
- architecture-constraints.md layer boundaries
- Never violate context files

**Priority 3: SKILL INSTRUCTIONS**
- "Create 7 epic documents" → Create all 7
- "Generate complete requirements" → Be comprehensive

**Priority 4: EFFICIENCY PRINCIPLES** (Lowest)
- "Target <100K tokens" → Applies to subagents, not main conversation
- Use native tools (Read vs cat)
- Efficiency is optimization, not requirement

**When priorities conflict:** Higher priority wins. Never assume lower priority trumps higher.

---

## Remember

- Main conversation: 1M tokens (check with `/context`)
- Subagent contexts: Isolated (separate budgets)
- Token efficiency: Tool choice (Read vs cat), not conversation length
- User instructions: Always highest priority

**"Ask, Don't Assume" applies to efficiency decisions too.**
