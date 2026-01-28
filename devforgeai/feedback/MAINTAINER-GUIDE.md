# Feedback System Maintainer Guide

Guide for framework maintainers on analyzing and acting on user feedback.

---

## Overview

This guide explains how to analyze user feedback collected through the retrospective conversation system and translate insights into framework improvements.

**Goals:**
- Identify patterns requiring framework changes
- Track user satisfaction over time
- Prioritize improvements based on data
- Maintain user trust through transparent handling

---

## Feedback Analysis Workflow

### 1. Aggregate Feedback by Category

**Weekly aggregation script:**

```bash
#!/bin/bash
# Aggregate feedback by workflow type

python3 << 'EOF'
import json
from pathlib import Path
from collections import defaultdict
from datetime import datetime, timedelta

feedback_dir = Path('devforgeai/feedback')
week_ago = datetime.now() - timedelta(days=7)

aggregated = defaultdict(lambda: defaultdict(list))
stats = defaultdict(lambda: {'count': 0, 'avg_rating': 0})

for story_dir in feedback_dir.glob('STORY-*'):
    for feedback_file in story_dir.glob('*-retrospective.json'):
        with open(feedback_file) as f:
            data = json.load(f)

            # Parse timestamp
            ts = datetime.fromisoformat(data['timestamp'])
            if ts < week_ago:
                continue

            workflow = data['workflow_type']
            status = data['success_status']

            for q in data.get('questions', []):
                if q.get('skip'):
                    continue

                qid = q['question_id']
                response = q.get('response')

                if response is not None:
                    aggregated[workflow][qid].append(response)

                    # Track rating averages
                    if q['response_type'] == 'rating':
                        stats[qid]['count'] += 1
                        stats[qid]['avg_rating'] += int(response)

# Calculate averages
for qid, data in stats.items():
    if data['count'] > 0:
        data['avg_rating'] = data['avg_rating'] / data['count']

# Output results
print("=== Weekly Feedback Summary ===\n")
for workflow, questions in aggregated.items():
    print(f"\n{workflow.upper()} Workflow:")
    for qid, responses in questions.items():
        print(f"  {qid}: {len(responses)} responses")
        if qid in stats:
            print(f"    Average rating: {stats[qid]['avg_rating']:.2f}/5")

print("\n=== Detailed Stats ===")
print(json.dumps(stats, indent=2))
EOF
```

### 2. Pattern Detection (80% Threshold)

When **80%+ of feedback mentions same issue**, flag as HIGH PRIORITY:

**Detection script:**

```python
#!/usr/bin/env python3
import json
from pathlib import Path
from collections import Counter

feedback_dir = Path('devforgeai/feedback')
pattern_threshold = 0.80

# Analyze open text responses for patterns
responses_by_question = {}

for story_dir in feedback_dir.glob('STORY-*'):
    for feedback_file in story_dir.glob('*-retrospective.json'):
        with open(feedback_file) as f:
            data = json.load(f)
            for q in data.get('questions', []):
                if q['response_type'] != 'open_text':
                    continue
                if q.get('skip'):
                    continue

                qid = q['question_id']
                response = q.get('response', '').lower()

                if qid not in responses_by_question:
                    responses_by_question[qid] = []
                responses_by_question[qid].append(response)

# Detect common themes
for qid, responses in responses_by_question.items():
    if len(responses) < 5:
        continue

    # Simple keyword frequency (would use NLP in production)
    keywords = []
    for resp in responses:
        keywords.extend(resp.split())

    keyword_counts = Counter(keywords)
    total = len(responses)

    print(f"\n{qid}:")
    for keyword, count in keyword_counts.most_common(10):
        if len(keyword) < 4:
            continue
        frequency = count / total
        if frequency >= pattern_threshold:
            print(f"  ⚠️  HIGH PRIORITY: '{keyword}' mentioned in {frequency*100:.0f}% of responses")
```

**When pattern detected:**

1. **Document in quarterly insights:**
   ```markdown
   ## High Priority Pattern Detected

   **Issue:** 80%+ of users report "TDD red phase confusing"
   **Workflow:** dev (development)
   **Question:** dev_success_05 (additional feedback)
   **Sample responses:**
     - "Red phase guidance unclear"
     - "Don't know which tests to write first"
     - "Examples would help in red phase"

   **Recommendation:**
   - Enhance test-automator subagent guidance
   - Add step-by-step examples in tdd-red-phase.md
   - Create interactive template for common test scenarios

   **Owner:** devforgeai-development skill maintainer
   **Priority:** HIGH
   **Estimated effort:** 2-3 hours
   **Target date:** 2025-01-20
   ```

2. **Create action item:**
   - File issue in project tracker
   - Assign to skill/subagent maintainer
   - Track completion

3. **Validate improvement:**
   - After implementing fix, monitor next 10 feedback sessions
   - Target: <20% mention same issue
   - If pattern persists, iterate

---

### 3. Trend Analysis

**Track feedback over time:**

```python
#!/usr/bin/env python3
import json
from pathlib import Path
from datetime import datetime
from collections import defaultdict

feedback_dir = Path('devforgeai/feedback')

# Group by month
monthly_data = defaultdict(lambda: {
    'total_sessions': 0,
    'avg_satisfaction': [],
    'workflows': defaultdict(int)
})

for story_dir in feedback_dir.glob('STORY-*'):
    for feedback_file in story_dir.glob('*-retrospective.json'):
        with open(feedback_file) as f:
            data = json.load(f)

            ts = datetime.fromisoformat(data['timestamp'])
            month_key = ts.strftime('%Y-%m')

            monthly_data[month_key]['total_sessions'] += 1
            monthly_data[month_key]['workflows'][data['workflow_type']] += 1

            # Calculate satisfaction from rating questions
            ratings = [
                q['response'] for q in data.get('questions', [])
                if q['response_type'] == 'rating' and not q.get('skip')
            ]
            if ratings:
                avg = sum(int(r) for r in ratings) / len(ratings)
                monthly_data[month_key]['avg_satisfaction'].append(avg)

# Print trends
print("=== Monthly Trends ===\n")
for month in sorted(monthly_data.keys()):
    data = monthly_data[month]
    satisfactions = data['avg_satisfaction']
    avg_sat = sum(satisfactions) / len(satisfactions) if satisfactions else 0

    print(f"{month}:")
    print(f"  Sessions: {data['total_sessions']}")
    print(f"  Avg Satisfaction: {avg_sat:.2f}/5")
    print(f"  Top workflows: {dict(data['workflows'])}")
```

**Key metrics to track:**

1. **Participation Rate** (% operations with feedback)
   - Target: 30-50% (don't be intrusive)
   - Trend: Should stabilize or increase over time

2. **Response Quality** (avg fields answered)
   - Target: 70%+ of offered questions answered
   - Trend: Higher = questions are relevant

3. **User Satisfaction** (avg rating across success questions)
   - Target: 4+/5 (users feel framework is improving)
   - Trend: Should increase over time

4. **Skill Gaps** (workflows with lowest ratings)
   - Identify: Which workflows need most improvement
   - Trend: Gaps should decrease as improvements made

5. **Feature Requests** (common themes in open text)
   - Identify: What's missing from framework
   - Trend: New themes emerge, old ones resolved

---

### 4. Export Reports

**Generate quarterly insights:**

```bash
#!/bin/bash
# Generate quarterly report

QUARTER=$(date +%Y-Q$(($(date +%-m)/3+1)))
OUTPUT="devforgeai/feedback/quarterly-insights-${QUARTER}.md"

python3 << 'EOF' > "$OUTPUT"
import json
from pathlib import Path
from datetime import datetime, timedelta
from collections import defaultdict

feedback_dir = Path('devforgeai/feedback')
three_months_ago = datetime.now() - timedelta(days=90)

# Collect all feedback from last 3 months
patterns = defaultdict(list)
trends = defaultdict(lambda: {'ratings': [], 'count': 0})
feature_requests = []

for story_dir in feedback_dir.glob('STORY-*'):
    for feedback_file in story_dir.glob('*-retrospective.json'):
        with open(feedback_file) as f:
            data = json.load(f)
            ts = datetime.fromisoformat(data['timestamp'])

            if ts < three_months_ago:
                continue

            workflow = data['workflow_type']
            trends[workflow]['count'] += 1

            for q in data.get('questions', []):
                if q.get('skip'):
                    continue

                qid = q['question_id']
                response = q.get('response')

                if q['response_type'] == 'rating':
                    trends[workflow]['ratings'].append(int(response))
                elif q['response_type'] == 'open_text':
                    if 'improve' in response.lower() or 'need' in response.lower():
                        feature_requests.append({
                            'workflow': workflow,
                            'request': response
                        })

# Generate report
print(f"# Quarterly Insights - {datetime.now().strftime('%Y Q%q')}")
print(f"\nGenerated: {datetime.now().isoformat()}")
print(f"\n---\n")

print("## Executive Summary\n")
print(f"- Total feedback sessions: {sum(t['count'] for t in trends.values())}")
print(f"- Workflows analyzed: {len(trends)}")
print(f"- Feature requests collected: {len(feature_requests)}")

print("\n## Workflow Satisfaction\n")
for workflow, data in sorted(trends.items()):
    if data['ratings']:
        avg = sum(data['ratings']) / len(data['ratings'])
        print(f"- **{workflow}**: {avg:.2f}/5 ({data['count']} sessions)")

print("\n## Top Feature Requests\n")
for i, req in enumerate(feature_requests[:10], 1):
    print(f"{i}. [{req['workflow']}] {req['request'][:100]}...")

print("\n## Recommended Actions\n")
print("(Maintainer: Add specific action items based on patterns detected)")
EOF

echo "Quarterly report generated: $OUTPUT"
```

**Report distribution:**
- Share with framework maintainers
- Review in quarterly planning meeting
- Prioritize top 3-5 improvements
- Track action items to completion

---

## Data Integrity

### Validation Rules

**On feedback submission:**

```python
def validate_feedback(data: dict) -> bool:
    """Validate feedback data integrity"""
    # Timestamp must be ISO 8601
    try:
        datetime.fromisoformat(data['timestamp'])
    except ValueError:
        return False

    # Story ID must exist
    story_path = Path(f'devforgeai/specs/Stories/{data["story_id"]}.story.md')
    if not story_path.exists():
        return False

    # Workflow type must be valid
    valid_workflows = [
        'dev', 'qa', 'orchestrate', 'release',
        'ideate', 'create-story', 'create-epic', 'create-sprint'
    ]
    if data['workflow_type'] not in valid_workflows:
        return False

    # Responses must pass content validation
    for q in data.get('questions', []):
        response = q.get('response')
        if response and not q.get('skip'):
            # Check for spam (very short, gibberish)
            if isinstance(response, str):
                if len(response) < 5:
                    return False

    return True
```

### Corrupted Feedback Recovery

**If corruption detected:**

```bash
#!/bin/bash
# Restore from weekly backup

STORY_ID="$1"
BACKUP_DIR="devforgeai/backups/feedback"

# Find latest backup
LATEST_BACKUP=$(ls -t "$BACKUP_DIR"/feedback_backup_*.tar.gz | head -1)

if [ -z "$LATEST_BACKUP" ]; then
    echo "Error: No backups found"
    exit 1
fi

# Extract feedback for story
echo "Restoring from $LATEST_BACKUP"
tar -xzf "$LATEST_BACKUP" "devforgeai/feedback/$STORY_ID/"

echo "Feedback restored for $STORY_ID"
echo "Please review: devforgeai/feedback/$STORY_ID/"
```

**Log incident:**

```bash
echo "$(date): Corrupted feedback restored for $STORY_ID from $LATEST_BACKUP" >> \
  devforgeai/feedback/corruption-incidents.log
```

---

## Sensitive Feedback Handling

**If feedback contains sensitive content:**

### Detection

```python
def is_sensitive(response: str) -> bool:
    """Detect potentially sensitive content"""
    sensitive_keywords = [
        'security', 'vulnerability', 'exploit', 'breach',
        'password', 'api key', 'secret', 'credential',
        'privacy', 'gdpr', 'compliance', 'legal'
    ]

    response_lower = response.lower()
    return any(keyword in response_lower for keyword in sensitive_keywords)
```

### Handling Process

1. **Flag for review:**
   ```bash
   # Move to secure directory
   mkdir -p devforgeai/feedback/sensitive/
   mv "devforgeai/feedback/$STORY_ID/$FILENAME" \
      "devforgeai/feedback/sensitive/"
   ```

2. **Notify appropriate stakeholders:**
   - Security issues → Security team
   - Privacy concerns → Legal/compliance
   - Vulnerabilities → Development lead

3. **Take action:**
   - Address framework vulnerability if exposed
   - Update documentation if needed
   - Thank user for responsible disclosure

4. **Store securely:**
   - Encrypt sensitive feedback
   - Separate from public insights
   - Access audited and logged

---

## User Data Access

### Export User Feedback

```bash
#!/bin/bash
# Export all feedback for a user (by story pattern)

USER_PATTERN="$1"  # e.g., "STORY-0*" for stories 001-099

OUTPUT_DIR="devforgeai/feedback/exports/$(date +%Y%m%d)"
mkdir -p "$OUTPUT_DIR"

for story_dir in devforgeai/feedback/$USER_PATTERN/; do
    cp -r "$story_dir" "$OUTPUT_DIR/"
done

tar -czf "${OUTPUT_DIR}.tar.gz" "$OUTPUT_DIR"
rm -rf "$OUTPUT_DIR"

echo "Feedback exported to: ${OUTPUT_DIR}.tar.gz"
```

### Delete User Feedback

```bash
#!/bin/bash
# Delete all feedback for user

USER_PATTERN="$1"

read -p "Delete all feedback matching $USER_PATTERN? (yes/no) " confirm
if [ "$confirm" != "yes" ]; then
    echo "Cancelled"
    exit 0
fi

# Log deletion
echo "$(date): Deleted feedback for $USER_PATTERN" >> \
  devforgeai/feedback/deletion-audit.log

# Remove feedback
rm -rf devforgeai/feedback/$USER_PATTERN/

echo "Feedback deleted for $USER_PATTERN"
```

---

## Scheduled Maintenance

### Weekly Tasks

```bash
#!/bin/bash
# Weekly maintenance (run Saturdays 2 AM)

# 1. Backup feedback files
devforgeai/scripts/backup-feedback.sh

# 2. Validate integrity
python3 devforgeai/scripts/validate-feedback-integrity.py

# 3. Generate weekly summary
python3 devforgeai/scripts/aggregate-feedback.py --period=week

# 4. Check for patterns
python3 devforgeai/scripts/detect-patterns.py --threshold=0.80
```

### Monthly Tasks

```bash
#!/bin/bash
# Monthly maintenance (1st of month)

# 1. Aggregate by category
python3 devforgeai/scripts/aggregate-feedback.py --period=month

# 2. Identify emerging patterns
python3 devforgeai/scripts/detect-patterns.py --period=month

# 3. Flag urgent issues
python3 devforgeai/scripts/flag-urgent-issues.py

# 4. Update tracking metrics
python3 devforgeai/scripts/update-metrics.py
```

### Quarterly Tasks

```bash
#!/bin/bash
# Quarterly maintenance

# 1. Generate quarterly report
devforgeai/scripts/generate-quarterly-report.sh

# 2. Review high-priority items
# (Manual: Schedule meeting with maintainers)

# 3. Update framework documentation
# (Manual: Implement top 3-5 improvements)

# 4. Check retention policy
python3 devforgeai/scripts/check-retention.py
```

---

## Success Metrics

### Track These Metrics

**1. Participation Rate** (% operations with feedback)
```python
participation_rate = feedback_sessions / total_operations
# Target: 30-50%
```

**2. Response Quality** (avg fields answered)
```python
response_quality = answered_questions / total_questions
# Target: 70%+
```

**3. Actionable Insights** (patterns identified per quarter)
```python
# Target: 3-5 framework improvements per quarter
```

**4. User Sentiment** (avg rating across success questions)
```python
user_sentiment = avg(all_rating_responses)
# Target: 4+/5
```

**5. Issue Resolution Time** (days from pattern detection to fix)
```python
resolution_time = fix_date - detection_date
# Target: <30 days for high-priority
```

### Dashboard Example

```markdown
# Feedback Metrics Dashboard

## Current Quarter (2025 Q1)

- **Participation Rate:** 42% (↑ from 38% last quarter)
- **Response Quality:** 73% (↑ from 68%)
- **User Sentiment:** 4.2/5 (↑ from 3.9)
- **Patterns Detected:** 4 high-priority
- **Issues Resolved:** 3/4 (75%)
- **Avg Resolution Time:** 18 days (↓ from 25)

## Actions This Quarter

1. ✅ Enhanced TDD red phase guidance (pattern: 82% confusion)
2. ✅ Improved QA violation messages (pattern: 79% unclear)
3. ✅ Added orchestrate checkpoint examples (pattern: 76% requests)
4. 🔄 Working on: Release rollback clarity (pattern: 81% unclear)

## Next Quarter Priorities

- Continue monitoring TDD improvements (validate 82% → <20%)
- Address release workflow concerns
- Explore new feature requests (top 3)
```

---

## Related Documentation

- **User Guide:** `devforgeai/feedback/USER-GUIDE.md`
- **Question Bank:** `devforgeai/feedback/questions.md`
- **JSON Schema:** `devforgeai/feedback/schema.json`
- **Questions YAML:** `devforgeai/feedback/questions.yaml`
- **Retention Policy:** `devforgeai/feedback/RETENTION-POLICY.md`
- **Graceful Degradation:** `devforgeai/feedback/GRACEFUL-DEGRADATION.md`

---

## Summary

**Maintainer responsibilities:**
1. **Weekly:** Backup, validate, summarize
2. **Monthly:** Aggregate, detect patterns, flag issues
3. **Quarterly:** Generate insights, plan improvements, track metrics
4. **Always:** Respect user privacy, handle sensitive data responsibly

**Key principle:** User feedback drives framework evolution. Patterns = priorities.
