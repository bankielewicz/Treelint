# Feedback System User Guide

Welcome to the DevForgeAI Feedback System. This guide explains how feedback works and how to use it effectively.

---

## Overview

The DevForgeAI Feedback System collects your structured feedback after completing commands and workflows, helping the framework improve continuously based on real user experiences.

**Key Benefits:**
- Help shape framework improvements
- Share what works and what doesn't
- Contribute to community knowledge
- Takes 2-3 minutes per session

---

## How It Works

### Feedback Triggered After Operations

After successful or failed operations (/dev, /qa, /orchestrate, /release, etc.), you'll see:

```
✅ Operation Complete!

Would you like to share feedback on this experience?
- 4 questions
- ~2 minutes
- Helps improve the framework

[Yes] [Skip this time] [Disable for session]
```

### When Feedback is Collected

**Automatically triggered after:**
- Development workflows (/dev)
- QA validation (/qa)
- Full orchestration (/orchestrate)
- Release deployments (/release)
- Story/epic creation
- Sprint planning

**Not triggered:**
- Simple queries (list stories, status checks)
- Documentation commands
- Configuration changes

---

## Responding to Questions

### Rating Scale (1-5)

When you see a rating question:

```
How confident are you in the TDD workflow?

1 - Not confident at all
2 - Slightly confident
3 - Neutral
4 - Confident
5 - Very confident

Your rating: _
```

**Interpretation:**
- **1** = Strongly disagree / Very poor
- **3** = Neutral / Average
- **5** = Strongly agree / Excellent

### Multiple Choice

Select the best matching option:

```
Which phase was most challenging?

1. Red (writing failing tests)
2. Green (implementing code to pass)
3. Refactor (improving code quality)
4. Integration (combining components)

Your choice: _
```

### Open Text

Type detailed response (10-5,000 characters):

```
What would improve the development workflow?

Your response:
_
_
_

(Min 10 characters, press Enter twice to submit)
```

**Tips for good open text responses:**
- Be specific (examples help!)
- Mention what worked AND what didn't
- Suggest improvements if you have ideas
- No need to be formal - speak naturally

---

## Skipping Feedback

### Individual Questions

You can skip individual questions:

```
Which phase was most challenging?

1. Red phase
2. Green phase
3. Refactor phase
4. Skip this question

Your choice: 4
```

### Entire Session

Select "Skip this time" to decline without judgment:

```
[Yes] [Skip this time] [Disable for session]
         ↑
    Select this
```

**Note:** After 3+ consecutive skips, system will suggest options:

```
You've skipped feedback 3 times. Would you like to:

1. Continue collecting feedback (default)
2. Disable feedback for this session
3. Switch to failures-only mode (only ask when operations fail)
4. Disable feedback permanently

Your choice: _
```

---

## Skip Patterns and Preferences

### Failures-Only Mode

Only collect feedback when operations fail:

```bash
# Enable failures-only mode
echo "mode: failures_only" >> devforgeai/feedback/config.yaml
```

### Disable Completely

Turn off feedback system:

```bash
# Disable feedback
echo "enable_feedback: false" >> devforgeai/feedback/config.yaml
```

Or set environment variable:

```bash
export DEVFORGEAI_DISABLE_FEEDBACK=true
```

### Re-enable Later

```bash
# Remove disable flag
# Edit devforgeai/feedback/config.yaml:
enable_feedback: true
mode: all  # or failures_only
```

---

## Data Stored

### Your Feedback Includes

✅ **What we store:**
- Workflow type (dev, qa, orchestrate, etc.)
- Story ID and epic ID
- Operation outcome (success, failed, partial)
- Your responses to questions
- Timestamp of submission
- Metadata (duration, questions answered/skipped)

❌ **What we DO NOT store:**
- Personal information (name, email, etc.)
- API keys or credentials
- Sensitive code snippets
- File paths or system info
- Conversation history

### Storage Location

All feedback stored locally on your machine:

```
devforgeai/feedback/
├── STORY-001/
│   └── 20250109_143022-retrospective.json
├── STORY-002/
│   └── 20250109_150145-retrospective.json
└── config.yaml
```

**You control this data** - delete anytime, export anytime, privacy always respected.

---

## Privacy & Data Handling

### Local Storage Only

- Feedback stored in `devforgeai/feedback/` (local machine)
- **Never** automatically transmitted anywhere
- You control when/if to share with maintainers

### Retention Period

- **Retained for:** 12 months
- **After 12 months:** Anonymized or deleted per your preference
- **You choose:** Keep, anonymize, or delete

### Who Sees Your Feedback?

**Framework maintainers review:**
- **Aggregated insights** (no individual tracking)
- **Patterns** (80%+ of users report X)
- **Trends** (satisfaction improving over time)

**They do NOT see:**
- Individual user identities
- Personal details
- Story-specific code

### Your Rights

You can:
- **Request deletion:** All your feedback, anytime
- **Export data:** Download your feedback history
- **Opt out:** Disable feedback system
- **Anonymize:** Keep feedback but remove identifiers

---

## Sensitive Information

If your feedback contains sensitive content (security issue, privacy concern):

```
⚠️ This feedback may contain sensitive information.

How should we handle this?

1. Full detail (private - maintainers only)
2. Anonymized (shared for insights, identifiers removed)
3. Redacted (summary only, details withheld)

Your choice: _
```

**Your preference is honored** - we never share sensitive feedback without explicit consent.

### Examples of Sensitive Feedback

- Security vulnerabilities discovered
- Privacy concerns about data handling
- Performance issues revealing infrastructure
- Legal/compliance questions

**Our commitment:** Sensitive feedback handled responsibly, never shared publicly.

---

## Feedback Workflow Example

### Successful Development Session

```bash
/dev STORY-042

# TDD cycle completes successfully...

✅ Development Complete!
   Story status: Dev Complete
   Tests: 15/15 passing
   Coverage: 96%

Would you like to share feedback? (4 questions, ~2 minutes)
[Yes] [Skip this time]

> Yes

Question 1 of 4:
How confident are you in the TDD workflow? (1-5)
> 5

Question 2 of 4:
Which phase was most challenging?
1. Red phase
2. Green phase
3. Refactor phase
4. Integration phase
> 3

Question 3 of 4:
Were test coverage requirements clear? (1-5)
> 4

Question 4 of 4:
Additional feedback on development experience (optional)
> The refactor phase guidance could use more examples. Otherwise great!

Thank you! Feedback saved to devforgeai/feedback/STORY-042/
```

### Failed QA Session

```bash
/qa STORY-042 deep

# QA fails due to coverage...

❌ QA Failed
   Reason: Coverage below threshold (82% actual, 85% required)

Would you like to share feedback? (3 questions, ~2 minutes)
[Yes] [Skip this time]

> Yes

Question 1 of 3:
Coverage not met - which area had gaps?
> Application layer, specifically error handling paths

Question 2 of 3:
Were violation details helpful?
1. Yes, very clear
2. Somewhat helpful
3. No, too vague
> 1

Question 3 of 3:
Is the path forward clear?
1. Yes, I know what to do
2. Partially clear
3. No, need more guidance
> 1

Thank you! Feedback will help improve QA guidance.
```

---

## Benefits of Providing Feedback

### For You

- **Influence improvements** - Your voice shapes framework development
- **Get better tools** - Patterns drive enhancements
- **Feel heard** - Real issues get addressed

### For the Framework

- **Identify pain points** - 80% of users report X → High priority fix
- **Validate changes** - Is new feature actually helpful?
- **Track satisfaction** - Are we improving over time?

### For the Community

- **Shared knowledge** - Anonymized insights help everyone
- **Better documentation** - Common questions drive docs
- **Proven patterns** - What works gets replicated

---

## Frequently Asked Questions

### How long does feedback take?

**Typical:** 2-3 minutes for 3-5 questions

**Depends on:**
- Question type (rating = 5 seconds, open text = 1 minute)
- How much detail you want to share

### Can I skip some questions but answer others?

**Yes!** Skip individual questions you don't want to answer. Partial feedback is still valuable.

### What if I make a mistake?

**You can:**
- Edit responses before final submit
- Delete feedback file after submission
- Request correction from maintainers

### Will my feedback be public?

**No** - Individual feedback is private. Only aggregated, anonymized insights are shared.

### How often is feedback collected?

**After each operation** - But you control frequency:
- Skip individual sessions
- Enable failures-only mode
- Disable entirely

### Can I see my feedback history?

**Yes!**

```bash
# List all your feedback
ls -la devforgeai/feedback/*/

# Read specific feedback
cat devforgeai/feedback/STORY-042/*-retrospective.json
```

---

## Questions or Issues?

If you have questions about the feedback system:

1. **Check maintainer guide:** `devforgeai/feedback/MAINTAINER-GUIDE.md`
2. **Review question bank:** `devforgeai/feedback/questions.md`
3. **Check schema:** `devforgeai/feedback/schema.json`
4. **File an issue:** Report bugs or suggestions via project issue tracker

---

## Thank You!

Your feedback makes DevForgeAI better for everyone. We appreciate you taking the time to share your experience.

**Remember:** Every piece of feedback helps, even quick ratings. Don't worry about perfection - honest feedback is most valuable.
