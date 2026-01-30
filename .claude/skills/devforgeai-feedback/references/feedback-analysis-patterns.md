# Feedback Analysis Patterns

**Version:** 1.0
**Date:** 2025-11-20
**Status:** Active

This guide provides systematic approaches to analyzing feedback sessions, extracting trends, and driving continuous improvement in the DevForgeAI framework.

---

## Overview

Feedback analysis transforms raw retrospective data into actionable insights through:
- **Trend detection** - Identify recurring patterns across sessions
- **Sentiment tracking** - Monitor user satisfaction over time
- **Bottleneck identification** - Surface common friction points
- **Impact measurement** - Quantify improvement effectiveness

---

## Analysis Dimensions

### 1. Temporal Analysis

**Purpose:** Track changes over time to measure improvement effectiveness

#### Weekly Trends

```bash
# Get feedback sessions from last 7 days
/feedback-search --date=last-7-days

# Analyze by operation type
grep "operation-type:" sessions/*.md | sort | uniq -c

# Sentiment distribution
grep "sentiment:" sessions/*.md | cut -d: -f2 | sort | uniq -c
```

**Metrics to track:**
- Success rate (passed vs failed operations)
- Average session duration
- Skip rate (questions answered vs skipped)
- Sentiment trend (positive, neutral, negative)

**Pattern:** If sentiment declining week-over-week → investigate recent changes

#### Sprint-over-Sprint Comparison

```bash
# Export feedback from Sprint N
/export-feedback --date=2025-11-01:2025-11-15

# Export feedback from Sprint N+1
/export-feedback --date=2025-11-16:2025-11-30

# Compare:
# - Total sessions (engagement)
# - Failure rate (quality)
# - Common keywords (themes)
# - Average duration (efficiency)
```

**Dashboard metrics:**
| Sprint | Sessions | Success Rate | Avg Duration | Top Issue |
|--------|----------|--------------|--------------|-----------|
| 3 | 23 | 87% | 12m 34s | "Slow tests" |
| 4 | 31 | 91% | 10m 45s | "Unclear errors" |
| 5 | 28 | 94% | 9m 12s | "Documentation gaps" |

**Pattern:** Improving success rate + decreasing duration = workflow maturing

---

### 2. Operation-Based Analysis

**Purpose:** Identify which commands/skills/subagents need improvement

#### Per-Operation Metrics

```bash
# Feedback for /dev command only
/feedback-search --operation=/dev

# Calculate metrics:
# - Total runs
# - Failure rate
# - Average duration vs expected
# - Most common improvement suggestions
```

**Example analysis:**

**`/dev` command (45 sessions):**
- Success rate: 89% (40/45)
- Failures: 5 (11%)
  - 3 due to test generation issues
  - 2 due to dependency conflicts
- Average duration: 23m (expected 15-20m)
- Top improvement: "Better mocking examples" (mentioned 8 times)

**Action:** Enhance test-automator subagent with database mocking examples

#### Cross-Operation Comparison

| Operation | Sessions | Success Rate | Avg Duration | Skip Rate |
|-----------|----------|--------------|--------------|-----------|
| /dev | 45 | 89% | 23m | 12% |
| /qa | 38 | 95% | 8m | 5% |
| /release | 12 | 100% | 4m | 8% |
| /create-story | 22 | 91% | 6m | 18% |

**Insights:**
- /dev has highest failure rate → needs attention
- /create-story has highest skip rate → questions too many or unclear
- /release has perfect success but low usage → investigate adoption

---

### 3. Story-Based Analysis

**Purpose:** Correlate feedback with story characteristics to optimize planning

#### Story Complexity vs Feedback

```bash
# Get feedback for specific story
/feedback-search --story=STORY-001

# Extract:
# - Story points (from story file)
# - Actual duration (from feedback session)
# - Success/failure status
# - User sentiment
```

**Pattern detection:**

| Story | Points | Actual Duration | Expected | Variance | Sentiment |
|-------|--------|-----------------|----------|----------|-----------|
| STORY-001 | 5 | 3h 15m | 2h 30m | +30% | Positive |
| STORY-002 | 8 | 6h 45m | 4h 00m | +69% | Negative |
| STORY-003 | 3 | 1h 50m | 1h 30m | +22% | Positive |

**Insights:**
- 8-point stories consistently over-estimated (STORY-002 took 69% longer)
- Negative sentiment correlates with high variance
- Action: Reduce story point estimates or improve breakdown

#### Feature Type Analysis

```bash
# Group stories by type (CRUD, API, UI, workflow)
grep "feature-type:" devforgeai/specs/Stories/*.md

# Correlate with feedback sentiment
# - CRUD stories: typically positive (well-understood)
# - API stories: mixed (dependency challenges)
# - UI stories: positive (ui-generator helps)
# - Workflow stories: negative (complex, ambiguous)
```

**Action:** Invest more in workflow story analysis phase (/ideate, /create-context)

---

### 4. Keyword and Theme Analysis

**Purpose:** Surface recurring issues through text analysis

#### Manual Keyword Extraction

```bash
# Search for specific keywords
/feedback-search --keyword="slow"
/feedback-search --keyword="confusing"
/feedback-search --keyword="error message"
```

**Frequency analysis:**
```bash
# Extract all feedback content
cat sessions/*.md | grep -v "^---" | grep -v "^#" > all-feedback.txt

# Count keyword occurrences
grep -io "test.*slow\|slow.*test" all-feedback.txt | wc -l  # 12 mentions
grep -io "unclear.*error\|error.*unclear" all-feedback.txt | wc -l  # 8 mentions
grep -io "documentation.*missing\|missing.*documentation" all-feedback.txt | wc -l  # 6 mentions
```

**Top issues by frequency:**
1. Slow tests (12 mentions) → Investigate test-automator performance
2. Unclear errors (8 mentions) → Improve error message quality in skills
3. Missing documentation (6 mentions) → Run /document more frequently

#### Semantic Clustering

**Manual approach:**
1. Read 10-20 recent feedback sessions
2. Group similar comments:
   - **Tooling issues:** "dependency resolution", "package conflicts", "build errors"
   - **Clarity issues:** "ambiguous AC", "unclear spec", "missing examples"
   - **Performance issues:** "slow tests", "long builds", "waiting for subagents"
3. Count occurrences per cluster
4. Prioritize clusters with highest frequency

**Example clustering:**

| Cluster | Occurrences | Top Keywords | Priority |
|---------|-------------|--------------|----------|
| Test Performance | 18 | "slow", "timeout", "waiting" | HIGH |
| Error Clarity | 12 | "confusing", "cryptic", "unclear" | MEDIUM |
| Documentation | 8 | "missing", "incomplete", "need examples" | MEDIUM |
| Dependency Hell | 5 | "conflict", "version", "resolution" | LOW |

**Action:** Address HIGH priority cluster first (optimize test execution)

---

### 5. Sentiment Tracking

**Purpose:** Monitor user satisfaction and detect morale issues early

#### Sentiment Distribution

```bash
# Count by sentiment
grep "sentiment:" sessions/*.md | cut -d: -f2 | sort | uniq -c

# Expected output:
#   45 positive
#   12 neutral
#    3 negative
```

**Healthy ratio:** 70%+ positive, <10% negative

**Warning signs:**
- Negative sentiment >15% → Major usability issues
- Neutral sentiment >40% → Users disengaged or indifferent
- Positive sentiment declining trend → Recent changes hurting UX

#### Sentiment Trend Over Time

```bash
# Monthly sentiment trend
for month in 2025-09 2025-10 2025-11; do
  echo "=== $month ==="
  /feedback-search --date=$month | grep "sentiment:" | cut -d: -f2 | sort | uniq -c
done
```

**Example output:**
```
=== 2025-09 ===
  12 positive
   5 neutral
   2 negative

=== 2025-10 ===
  18 positive
   4 neutral
   1 negative

=== 2025-11 ===
  25 positive
   3 neutral
   0 negative
```

**Insight:** Sentiment improving month-over-month → changes working

#### Sentiment vs Operation

```bash
# Correlate sentiment with operation type
for op in dev qa release; do
  echo "=== $op ==="
  /feedback-search --operation=/$op | grep "sentiment:" | cut -d: -f2 | sort | uniq -c
done
```

**Pattern:** If one operation consistently negative → focus improvement there

---

### 6. Improvement Impact Measurement

**Purpose:** Validate that changes based on feedback actually help

#### Before/After Analysis

**Step 1: Baseline (before improvement)**
```bash
# Get feedback before change (e.g., test-automator enhancement)
/feedback-search --operation=/dev --date=before-2025-11-15

# Calculate baseline metrics:
# - Average duration
# - Success rate
# - "slow test" mention frequency
```

**Baseline metrics (Oct 15 - Nov 14):**
- Average /dev duration: 23m 45s
- Success rate: 87%
- "Slow test" mentions: 12

**Step 2: Implement improvement**
- Enhanced test-automator with parallel execution
- Added caching for dependency installation
- Deployed on Nov 15

**Step 3: Post-change (after improvement)**
```bash
# Get feedback after change
/feedback-search --operation=/dev --date=after-2025-11-15

# Calculate post-change metrics:
# - Average duration
# - Success rate
# - "slow test" mention frequency
```

**Post-change metrics (Nov 15 - Nov 30):**
- Average /dev duration: 18m 12s
- Success rate: 91%
- "Slow test" mentions: 2

**Impact:**
- Duration reduced 23% (23m → 18m)
- Success rate improved 4% (87% → 91%)
- Complaints reduced 83% (12 → 2)

**Conclusion:** Improvement effective, keep monitoring

---

### 7. Question Effectiveness Analysis

**Purpose:** Optimize feedback questions to maximize insight

#### Skip Rate by Question

```bash
# Extract question IDs and skip status
grep "question-id:" sessions/*.md | sort | uniq -c

# Example:
#   45 cmd_passed_01  # Asked 45 times
#   45 cmd_passed_02  # Asked 45 times
#   38 cmd_passed_03  # Asked 45 times, skipped 7 (15% skip rate)
#   23 cmd_passed_04  # Asked 45 times, skipped 22 (49% skip rate)
```

**Actionable thresholds:**
- Skip rate <20% → Good question, keep it
- Skip rate 20-40% → Consider rephrasing
- Skip rate >40% → Remove or rework (users don't find it valuable)

**Example action:** `cmd_passed_04` has 49% skip rate → question unclear or irrelevant

#### Response Length by Question

```bash
# Extract responses for specific question
grep -A 5 "## What Went Well" sessions/*.md | wc -w

# Average words per response:
# - cmd_passed_01 (What Went Well): 42 words avg
# - cmd_passed_02 (Improvement): 38 words avg
# - cmd_passed_03 (Challenges): 15 words avg  ← Short responses
# - cmd_passed_04 (Efficiency): 8 words avg   ← Very short
```

**Pattern:** Short responses may indicate:
- Question too broad (users don't know what to say)
- Question redundant (covered by previous questions)
- Question not actionable (users give generic answers)

**Action:** Rephrase cmd_passed_03 to be more specific, remove cmd_passed_04

---

### 8. Actionable Insights Extraction

**Purpose:** Convert feedback into concrete tasks

#### Manual Extraction Process

1. **Read recent feedback** (last 10-20 sessions)
2. **Identify concrete suggestions:**
   - "Add mocking examples to test-automator"
   - "Include refactoring checklist in Phase 3"
   - "Improve dependency resolution error messages"
3. **Group by component:**
   - test-automator subagent: 3 suggestions
   - devforgeai-development skill: 2 suggestions
   - Error handling: 2 suggestions
4. **Prioritize by frequency:**
   - Mentioned >5 times → HIGH
   - Mentioned 2-4 times → MEDIUM
   - Mentioned once → LOW (or ignore)
5. **Create stories:**
   - STORY-XXX: Enhance test-automator with database mocking examples
   - STORY-YYY: Add refactoring guidance checklist to Phase 3

#### Automated Extraction (Future)

```bash
# Extract all "Suggestions for Next Time" sections
grep -A 10 "## Suggestions for Next Time" sessions/*.md > suggestions.txt

# Manual review + clustering
# Create stories for top 3-5 suggestions
```

---

## Analysis Workflows

### Weekly Review (30 minutes)

**Goal:** Monitor health and catch issues early

1. **Run search:**
   ```bash
   /feedback-search --date=last-7-days
   ```

2. **Check metrics:**
   - Total sessions (engagement)
   - Success rate (quality)
   - Sentiment distribution (satisfaction)

3. **Skim recent feedback:**
   - Read 5 most recent sessions
   - Note any new patterns or surprising feedback

4. **Quick actions:**
   - If any critical issues mentioned → create story immediately
   - If sentiment declining → investigate recent changes

### Monthly Review (2 hours)

**Goal:** Deep analysis and improvement planning

1. **Trend analysis:**
   - Compare to previous month (sessions, success rate, duration)
   - Identify improving and declining areas

2. **Keyword extraction:**
   - Run keyword searches for known issues
   - Count frequency of mentions
   - Identify new themes emerging

3. **Per-operation review:**
   - Calculate metrics for each operation
   - Identify worst-performing operations
   - Compare to baselines

4. **Action planning:**
   - Create 3-5 stories for top improvement areas
   - Schedule into next sprint
   - Assign owners (skills, subagents, documentation)

### Quarterly Review (4 hours)

**Goal:** Strategic assessment and major improvements

1. **Trend visualization:**
   - Create charts (success rate over 3 months, sentiment trend)
   - Identify long-term patterns

2. **Before/after impact:**
   - Review improvements made last quarter
   - Measure impact using before/after metrics
   - Document successes and failures

3. **Question effectiveness:**
   - Calculate skip rates for all questions
   - Remove or rework high-skip questions
   - Add new questions for emerging themes

4. **Strategic initiatives:**
   - Identify systemic issues (not just tactical fixes)
   - Plan larger improvements (e.g., redesign workflow, new subagent)
   - Update roadmap based on feedback trends

---

## Common Patterns

### Pattern: "Test Generation Too Slow"

**Symptoms:**
- Frequent mentions in "What Could Improve"
- Duration variance (expected 5m, actual 15m)
- Negative sentiment when tests mentioned

**Root causes:**
- Subagent generating redundant tests
- No caching of common test patterns
- Excessive API calls for test suggestions

**Solutions:**
1. Add test pattern caching
2. Optimize subagent prompt (more specific test requirements)
3. Parallel test generation for independent modules

**Validation:**
- Before: 15m avg test generation
- After: 5m avg test generation
- "Slow test" mentions reduced 80%

---

### Pattern: "Acceptance Criteria Ambiguous"

**Symptoms:**
- Mentions in "Challenges" section
- Rework required in Phase 2 (implementation)
- Questions to user during development

**Root causes:**
- Story creation missing edge cases
- Technical specs too high-level
- No examples provided

**Solutions:**
1. Enhance devforgeai-story-creation with edge case checklist
2. Add requirement: All ACs must have 1+ example
3. Story template includes "Example Scenario" section

**Validation:**
- Before: 30% of stories have AC questions during /dev
- After: 10% of stories have AC questions during /dev

---

### Pattern: "Unclear Error Messages"

**Symptoms:**
- Mentions in failure feedback
- Low rating for "error message clarity"
- Time wasted debugging

**Root causes:**
- Generic error messages ("Validation failed")
- No context (which field, what value)
- No suggested fix

**Solutions:**
1. Error message template: "{What failed} - {Why} - {How to fix}"
2. Add validation details to all error logs
3. Link to documentation in error messages

**Validation:**
- Before: Error clarity rating 2.3/5
- After: Error clarity rating 4.1/5

---

## Reporting Templates

### Weekly Summary Email

```
Subject: DevForgeAI Feedback Summary - Week of Nov 13-20

**Activity:**
- 12 feedback sessions collected
- 9 /dev, 2 /qa, 1 /release

**Health Metrics:**
- Success rate: 92% (11/12 passed)
- Sentiment: 83% positive, 17% neutral, 0% negative
- Avg duration: 10m 45s (within expected range)

**Top Themes:**
- Test generation speed improved (4 positive mentions)
- Documentation gaps noted (2 mentions)
- One dependency conflict (STORY-042)

**Actions:**
- Created STORY-XXX: Add API mocking examples to docs
- Investigating STORY-042 dependency issue

**Next Review:** Nov 27
```

### Monthly Retrospective Report

```markdown
# Feedback Analysis - November 2025

## Summary

- **Total Sessions:** 45
- **Success Rate:** 91% (41/45 passed)
- **Sentiment:** 78% positive, 18% neutral, 4% negative
- **Engagement:** 22% increase from October

## Top Improvements Needed

1. **Test Performance** (18 mentions)
   - Average test gen time: 12m (expected 5m)
   - Action: STORY-XXX - Parallel test generation

2. **Error Message Clarity** (12 mentions)
   - Clarity rating: 2.8/5
   - Action: STORY-YYY - Structured error templates

3. **Documentation Coverage** (8 mentions)
   - Missing: API mocking, async patterns
   - Action: Run /document, add examples

## Success Stories

- **/dev workflow maturity:** Duration down 15% (23m → 19m)
- **Sentiment improvement:** Positive up from 65% → 78%
- **Question optimization:** Removed 2 high-skip questions

## Action Plan - December

- Address test performance (HIGH priority)
- Launch error message improvement initiative
- Schedule documentation sprint

## Long-Term Trends

- Success rate: Oct 87% → Nov 91% ✅
- Duration efficiency: Oct 23m → Nov 19m ✅
- User satisfaction: Oct 3.2/5 → Nov 3.8/5 ✅

**Status:** Framework health improving
```

---

## Tools and Automation

### Command-Line Analysis

```bash
# Session count by month
ls sessions/ | cut -c1-6 | sort | uniq -c

# Success rate
grep "status:" sessions/*.md | grep -c "passed"
grep "status:" sessions/*.md | grep -c "failed"

# Average duration (requires jq)
grep "duration-ms:" sessions/*.md | cut -d: -f2 | awk '{sum+=$1; count++} END {print sum/count/1000/60 " minutes"}'
```

### Future Automation Ideas

- **Dashboard:** Web UI showing real-time metrics
- **Alerts:** Slack notification if sentiment <50% positive
- **Trend charts:** Matplotlib graphs of key metrics
- **NLP clustering:** Auto-group feedback by theme
- **Anomaly detection:** Flag unusual patterns (sudden spike in failures)

---

## Related Documentation

- **Feedback Question Templates:** `feedback-question-templates.md`
- **Feedback Persistence Guide:** `feedback-persistence-guide.md`
- **Feedback Export Formats:** `feedback-export-formats.md`
- **Template Format Specification:** `template-format-specification.md`

---

**Effective feedback analysis requires consistent effort. Start with weekly reviews (30min), graduate to monthly deep dives (2h), and run quarterly strategic assessments (4h). The insights gained will compound over time, driving continuous improvement in the DevForgeAI framework.**
