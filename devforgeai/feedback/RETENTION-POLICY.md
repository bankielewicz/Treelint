# Data Retention Policy

Policy for managing feedback data lifecycle, retention, and user privacy.

---

## Default Policy

- **Active Retention:** 12 months
- **Action after 12 months:** User choice (delete, anonymize, or archive)
- **Default action:** Anonymize (preserves insights, protects privacy)
- **User notification:** 30 days before action
- **Opt-out available:** Yes, at any time

---

## Retention Timeline

### Months 1-12: Active Storage

**What's stored:**
- All feedback details (story ID, responses, metadata)
- User identifiable (by story pattern)
- Full response text
- Timestamps and workflow information

**Usage:**
- Aggregated for insights (anonymized at scale)
- Pattern detection (80% threshold)
- Trend analysis (monthly, quarterly)
- Framework improvement prioritization

**User access:**
- Full history available
- Export on demand
- Delete on request
- Modify preferences

**Compliance:**
- Subject to GDPR (right to be forgotten)
- Subject to CCPA (right to download)
- Encrypted at rest (file system permissions)
- Access audited (maintainer actions logged)

---

### Month 12: Retention Notice

**30 days before 12-month mark:**

```
📧 Feedback Retention Notice

Your feedback for STORY-042 (submitted 2024-01-15) will reach the
12-month retention limit on 2025-01-15.

What would you like to do?

1. Delete - Permanently remove all feedback
2. Anonymize - Keep insights, remove identifiers (recommended)
3. Archive - Keep feedback, mark as archived
4. Extend - Keep active for another 12 months

Choose: [1/2/3/4]

No response = Default action (Anonymize) will be taken.
```

**Reminder schedule:**
- 30 days before: Initial notice
- 14 days before: First reminder
- 7 days before: Second reminder
- 1 day before: Final reminder
- Day of: Action taken, confirmation sent

---

### After Month 12: Action Taken

#### Option A: Delete (Permanent Removal)

**Action:**
```bash
# Remove all feedback for story
rm -rf devforgeai/feedback/STORY-042/

# Log deletion
echo "$(date): Deleted STORY-042 (user requested)" >> \
  devforgeai/feedback/retention-audit.log
```

**Result:**
- All user feedback deleted permanently
- Cannot be recovered
- No aggregated insights retained
- User receives confirmation

**Confirmation:**
```
✅ Feedback Deleted

All feedback for STORY-042 has been permanently deleted.

What was deleted:
- 3 feedback sessions
- 12 question responses
- Associated metadata

This action cannot be undone.
```

---

#### Option B: Anonymize (Default)

**Action:**
```python
# Anonymize feedback
anonymized = {
    "feedback_id": generate_new_uuid(),
    "timestamp": data['timestamp'],
    "story_id": "STORY-ANONYMIZED",
    "epic_id": None,
    "workflow_type": data['workflow_type'],
    "success_status": data['success_status'],
    "questions": data['questions'],  # Keep responses
    "metadata": {
        "duration_seconds": data['metadata']['duration_seconds'],
        "total_questions": data['metadata']['total_questions'],
        "answered": data['metadata']['answered'],
        "skipped": data['metadata']['skipped']
    },
    "anonymized_at": datetime.now().isoformat(),
    "original_retention_date": "2025-01-15"
}

# Save to anonymized directory
save_to('devforgeai/feedback/anonymized/', anonymized)

# Remove original
remove_original(story_id)
```

**Result:**
- Identifiers removed (story ID, epic ID, user patterns)
- Responses preserved (for insights)
- Aggregated statistics still valid
- Cannot trace back to specific user

**What's kept:**
- Workflow type (dev, qa, etc.)
- Success status (success, failed, partial)
- Question responses (ratings, choices, text)
- Timestamps (month granularity, not exact day)
- Metadata (duration, answered count)

**What's removed:**
- Story ID (replaced with STORY-ANONYMIZED)
- Epic ID (set to null)
- Exact timestamp (rounded to month)
- Any user-identifying patterns

**Confirmation:**
```
✅ Feedback Anonymized

Feedback for STORY-042 has been anonymized.

What happened:
- Identifiers removed (story/epic IDs)
- Insights preserved (responses, metadata)
- Cannot be traced back to you

Your feedback still helps improve the framework,
but your privacy is protected.
```

---

#### Option C: Archive (Keep with Flag)

**Action:**
```python
# Mark as archived
data['archived'] = True
data['archived_at'] = datetime.now().isoformat()
data['archive_reason'] = 'user_requested'

# Save back to original location
save_feedback(data)
```

**Result:**
- All details preserved
- Marked as archived (excluded from active analysis)
- Counts toward user's storage quota
- Can be deleted later

**Storage impact:**
```
Archived feedback counts toward quota:
- Current quota: 100 MB per user
- Your usage: 5 MB (20 stories)
- Archived: 2 MB (8 stories)
- Available: 95 MB
```

**Confirmation:**
```
✅ Feedback Archived

Feedback for STORY-042 has been archived.

What this means:
- Full details preserved
- Excluded from active analysis
- Counts toward storage quota (2 MB)
- You can delete anytime

To delete: /devforgeai feedback-delete STORY-042
```

---

#### Option D: Extend (12 More Months)

**Action:**
```python
# Extend retention
data['retention_extended'] = True
data['extended_at'] = datetime.now().isoformat()
data['new_retention_date'] = (datetime.now() + timedelta(days=365)).isoformat()

# Save back
save_feedback(data)

# Schedule next notice
schedule_retention_notice(story_id, new_retention_date)
```

**Result:**
- Active retention extended to 2026-01-15
- Full access continues
- Same policy applies after 24 months
- New notice sent at 23 months

**Confirmation:**
```
✅ Retention Extended

Feedback for STORY-042 will be retained until 2026-01-15.

What's next:
- Active retention continues
- Same policy applies after 24 months
- You'll receive notice at 23 months

You can change your preference anytime.
```

---

## Sensitive Data (Extended Retention)

**Special handling for sensitive feedback:**

### What Qualifies as Sensitive

- Security vulnerabilities reported
- Privacy concerns documented
- Compliance issues raised
- Legal questions about framework

### Extended Retention (18 Months)

**Why longer:**
- Security tracking requires longer timeline
- Compliance validation needs history
- Vulnerability disclosure may take time
- Legal review processes are lengthy

**Storage:**
```
devforgeai/feedback/sensitive/
├── security/
│   └── STORY-042-vulnerability.json.enc
├── privacy/
│   └── STORY-087-gdpr.json.enc
└── compliance/
    └── STORY-123-sox.json.enc
```

**Encryption:**
- All sensitive feedback encrypted at rest
- AES-256 encryption
- Keys managed by framework maintainers
- Access requires justification + audit log

**Access Control:**
```python
def access_sensitive_feedback(story_id: str, accessor: str, reason: str):
    """Access sensitive feedback with audit trail"""
    # Log access attempt
    log_access(story_id, accessor, reason, timestamp=now())

    # Require approval for access
    if not is_approved(accessor, story_id):
        raise PermissionError("Access requires approval")

    # Decrypt and return
    return decrypt_feedback(story_id)
```

**Retention notice:**
```
📧 Sensitive Feedback Retention Notice

Your sensitive feedback for STORY-042 (security vulnerability)
will reach 18-month retention on 2025-07-15.

Due to sensitive nature, extended retention was applied.

Options:
1. Delete - Remove after issue resolved
2. Archive - Keep encrypted for reference
3. Extend - Keep active for security tracking

Issue status: ✅ Resolved (2025-02-20)
Recommend: Delete (issue resolved, no longer needed)

Choose: [1/2/3]
```

---

## User Control

### Export Feedback

**Command:**
```bash
devforgeai feedback-export [story-id]
devforgeai feedback-export --all
```

**Output:**
```json
{
  "export_date": "2025-01-09T14:30:22Z",
  "stories": [
    {
      "story_id": "STORY-042",
      "sessions": [
        {
          "timestamp": "2025-01-08T10:15:30Z",
          "workflow_type": "dev",
          "success_status": "success",
          "questions": [ ... ]
        }
      ]
    }
  ],
  "total_sessions": 15,
  "total_stories": 8
}
```

**Saved to:**
```
devforgeai/feedback/exports/export-20250109_143022.json
```

---

### Delete Feedback

**Single story:**
```bash
devforgeai feedback-delete STORY-042
```

**Confirmation prompt:**
```
⚠️  Delete Feedback for STORY-042?

This will permanently delete:
- 3 feedback sessions
- 12 question responses
- Associated metadata

This action cannot be undone.

Are you sure? [yes/no]: _
```

**All feedback:**
```bash
devforgeai feedback-delete --all
```

**Confirmation:**
```
⚠️  Delete ALL Feedback?

This will permanently delete:
- 15 feedback sessions
- 60 question responses
- Across 8 stories

This action cannot be undone.
Anonymized insights will also be removed.

Type 'DELETE ALL' to confirm: _
```

**Audit log:**
```bash
# All deletions logged
cat devforgeai/feedback/deletion-audit.log

2025-01-09 14:30:22: Deleted STORY-042 (user requested)
2025-01-10 09:15:45: Deleted STORY-087 (retention policy)
2025-01-12 16:20:10: Deleted all feedback (user requested)
```

---

### Opt Out of Collection

**Disable feedback system:**
```bash
devforgeai feedback disable
```

**Effect:**
- No more feedback prompts
- Existing feedback retained
- Can re-enable anytime

**Re-enable:**
```bash
devforgeai feedback enable
```

---

## Compliance

### GDPR (General Data Protection Regulation)

**Right to be forgotten:**
- User can request deletion anytime
- All feedback removed within 30 days
- Anonymized data also removed (cannot trace back)

**Right to data portability:**
- User can export all feedback
- Machine-readable format (JSON)
- Includes all metadata

**Right to access:**
- User can view all stored feedback
- View how data is used
- View retention status

**Consent:**
- Explicit consent required before collection
- User can withdraw consent anytime
- Withdrawal stops collection, keeps existing data (unless deleted)

---

### CCPA (California Consumer Privacy Act)

**Right to know:**
- What data is collected (see USER-GUIDE.md)
- How data is used (aggregated insights)
- Who has access (maintainers only)

**Right to delete:**
- User can request deletion
- Deletion processed within 45 days
- Confirmation provided

**Right to opt-out:**
- Disable feedback system
- No penalty for opting out
- Commands still work normally

**Right to non-discrimination:**
- Framework works identically with/without feedback
- No features restricted
- No degraded experience

---

### SOC 2 (Service Organization Control)

**Security:**
- Feedback encrypted at rest
- Access audited and logged
- Sensitive data segregated
- Backups encrypted

**Availability:**
- Graceful degradation (commands never blocked)
- Backup and recovery procedures
- Health monitoring

**Confidentiality:**
- Anonymization after retention period
- No sharing without consent
- Maintainers bound by confidentiality

**Privacy:**
- Retention policy enforced
- User control honored
- Deletion audit trail

---

## Audit Trail

### What's Logged

**User actions:**
```
2025-01-09 14:30:22: User exported feedback for STORY-042
2025-01-09 14:35:10: User deleted feedback for STORY-087
2025-01-09 14:40:55: User disabled feedback system
```

**Retention actions:**
```
2025-01-10 02:00:00: Anonymized STORY-042 (12-month policy)
2025-01-10 02:01:15: Deleted STORY-087 (user preference)
2025-01-10 02:02:30: Archived STORY-123 (user preference)
```

**Anonymization events:**
```
2025-01-10 02:00:00: Anonymized STORY-042
  Original: {story_id: STORY-042, epic_id: EPIC-001}
  Anonymized: {story_id: STORY-ANONYMIZED, epic_id: null}
  Timestamp rounded: 2024-01 (from 2024-01-15)
```

**Access logs:**
```
2025-01-09 10:00:00: maintainer@example.com accessed sensitive/STORY-042
  Reason: Security vulnerability review
  Approved: yes
  Duration: 15 minutes
```

---

### Audit Log Retention

**Audit logs retained:**
- 5 years minimum (regulatory compliance)
- Cannot be deleted by users
- Only accessible to compliance team

**Log location:**
```
devforgeai/feedback/retention-audit.log
devforgeai/feedback/deletion-audit.log
devforgeai/feedback/access-audit.log
devforgeai/feedback/anonymization-audit.log
```

---

## Scheduled Maintenance

### Daily Tasks

```bash
#!/bin/bash
# Check for feedback approaching retention limit

python3 << 'EOF'
from pathlib import Path
from datetime import datetime, timedelta
import json

feedback_dir = Path('devforgeai/feedback')
notice_days = [30, 14, 7, 1]
today = datetime.now()

for story_dir in feedback_dir.glob('STORY-*'):
    for feedback_file in story_dir.glob('*-retrospective.json'):
        with open(feedback_file) as f:
            data = json.load(f)
            ts = datetime.fromisoformat(data['timestamp'])
            age_days = (today - ts).days

            # Check if notice needed
            for notice_day in notice_days:
                if age_days == (365 - notice_day):
                    print(f"Send {notice_day}-day notice for {data['story_id']}")
EOF
```

### Weekly Tasks

```bash
#!/bin/bash
# Apply retention policy

python3 << 'EOF'
from pathlib import Path
from datetime import datetime, timedelta
import json

feedback_dir = Path('devforgeai/feedback')
retention_days = 365
today = datetime.now()

for story_dir in feedback_dir.glob('STORY-*'):
    for feedback_file in story_dir.glob('*-retrospective.json'):
        with open(feedback_file) as f:
            data = json.load(f)
            ts = datetime.fromisoformat(data['timestamp'])
            age_days = (today - ts).days

            if age_days >= retention_days:
                # Apply retention policy
                preference = get_user_preference(data['story_id'])

                if preference == 'delete':
                    delete_feedback(data['story_id'])
                elif preference == 'anonymize':
                    anonymize_feedback(data)
                elif preference == 'archive':
                    archive_feedback(data)
                elif preference == 'extend':
                    extend_retention(data)
                else:
                    # Default: anonymize
                    anonymize_feedback(data)
EOF
```

---

## User Preferences

### Preference Storage

**Config file:**
```yaml
# devforgeai/feedback/retention-preferences.yaml

STORY-042:
  action: anonymize
  preference_set: 2024-12-15

STORY-087:
  action: delete
  preference_set: 2024-12-20

STORY-123:
  action: archive
  preference_set: 2025-01-05

default:
  action: anonymize
```

### Set Preferences

**Per story:**
```bash
devforgeai feedback-preference STORY-042 --action anonymize
```

**Default for all:**
```bash
devforgeai feedback-preference --default delete
```

---

## Summary

**Key Points:**

1. **12-month active retention** (default)
2. **User choice after 12 months** (delete, anonymize, archive, extend)
3. **30-day notice before action** (multiple reminders)
4. **Anonymize by default** (preserves insights, protects privacy)
5. **Sensitive data: 18 months** (encrypted, access controlled)
6. **Full user control** (export, delete, opt-out anytime)
7. **Compliance ready** (GDPR, CCPA, SOC 2)
8. **Audit trail maintained** (all actions logged, 5-year retention)

**User Rights:**
- ✅ Export data anytime
- ✅ Delete data anytime
- ✅ Opt out of collection
- ✅ Choose retention preference
- ✅ Access all stored feedback
- ✅ Withdraw consent

**Framework Commitments:**
- ✅ Transparent handling
- ✅ User control honored
- ✅ Privacy protected
- ✅ Compliance maintained
- ✅ Security enforced
