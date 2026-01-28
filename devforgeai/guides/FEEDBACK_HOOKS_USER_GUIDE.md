# Feedback Hooks User Guide

**Learn how to use feedback hooks to help improve DevForgeAI**

---

## What Are Feedback Hooks?

Feedback hooks are optional conversations triggered after you use certain DevForgeAI commands. They ask for your thoughts about your experience, helping the framework continuously improve.

**Key Points:**
- ✅ Completely optional (you can skip anytime)
- ✅ Quick (2-3 minutes typically)
- ✅ Non-blocking (doesn't delay your workflow)
- ✅ Anonymous (no personal data collected)
- ✅ Configurable (you control when hooks trigger)

---

## Which Commands Have Feedback Hooks?

### Currently Available

| Command | When Hooks Trigger | Example |
|---------|-------------------|---------|
| **`/dev STORY-ID`** | After TDD implementation completes | You finish implementing a feature, feedback asks how the TDD experience was |
| **`/create-context PROJECT`** | After context files are created | You set up a new project, feedback asks about the context discovery process |

### Coming Soon

- `/ideate [description]` - After requirements discovery
- `/create-ui [spec]` - After UI specification generation
- `/audit-deferrals` - After technical debt review

---

## How Feedback Hooks Work

### 1. Command Completes Successfully

You run a DevForgeAI command and it finishes its primary work:
```bash
/dev STORY-042
# ... TDD cycle executes ...
# ✅ All tests passing, implementation complete
```

### 2. Eligibility Check

The command checks if you're eligible for feedback:
- Respects your configuration (`hooks.yaml`)
- Respects rate limits (don't get asked too frequently)
- Respects skip patterns (you can disable specific operations)

### 3. Optional Feedback Prompt

If you're eligible, you may see a feedback prompt:
```
📝 Your feedback helps us improve DevForgeAI!

Would you like to share your experience with the TDD workflow?
This should take 2-3 minutes.

[Yes, let's go] [Skip for now] [Never ask again]
```

### 4. Feedback Conversation

If you choose "Yes", you'll answer 5-7 questions:
```
1. How would you rate the test generation quality? (1-5)
2. Was the TDD cycle clear and intuitive? (Yes/No/Somewhat)
3. What was most helpful about this workflow?
4. What could we improve?
5. Would you recommend this to a colleague? (Yes/No/Maybe)
```

### 5. Feedback Saved

Your responses are saved to `devforgeai/feedback/sessions/` and analyzed to improve the framework.

---

## Controlling Feedback Hooks

### Configuration File

Create or edit `devforgeai/hooks/hooks.yaml`:

```yaml
# hooks.yaml - Configure feedback hooks

# Global enable/disable
enabled: true

# Which operations should trigger hooks?
# Options: all | success-only | failures-only | never
trigger_on: all

# Skip specific operations
skip_patterns:
  - "dev"           # Don't ask for /dev feedback
  - "create-ui"     # Don't ask for /create-ui feedback

# Rate limiting - max feedback per period
rate_limit:
  max_per_day: 3         # Max 3 feedback prompts per day
  max_per_week: 10       # Max 10 per week
  cooldown_minutes: 60   # Wait 60 min between prompts
```

### Common Configurations

**Disable All Feedback:**
```yaml
enabled: false
```

**Only Feedback on Failures:**
```yaml
enabled: true
trigger_on: failures-only
```

**Feedback for Specific Commands:**
```yaml
enabled: true
skip_patterns:
  - "create-context"    # Only /dev gets feedback
  - "create-ui"
  - "ideate"
  - "audit-deferrals"
```

**Limit Frequency:**
```yaml
enabled: true
rate_limit:
  max_per_day: 1
  cooldown_minutes: 120
```

---

## Privacy & Data

### What We Collect

When you provide feedback:
- Your survey responses (rated/text answers)
- Command operation (which command you used)
- Outcome status (success or failure)
- Timestamp (when you provided feedback)

### What We DON'T Collect

- ❌ Your name or email
- ❌ Your code or project files
- ❌ Your Git repository data
- ❌ Your system information
- ❌ Your personal data

### Where Data Is Stored

Feedback responses saved locally in your project:
```
devforgeai/feedback/sessions/[YYYY-MM-DD]-[operation].json
```

You have full access to this directory and can delete feedback anytime.

---

## Use Cases

### Case 1: Improve Your TDD Workflow

```
Scenario: You're learning the TDD workflow
Action: Use /dev command to implement features
Feedback: Survey asks about test generation, test clarity, implementation flow
Benefit: Your feedback helps improve test quality for everyone
```

### Case 2: Discover Features

```
Scenario: You're new to DevForgeAI
Action: Use /create-context to set up your first project
Feedback: Survey asks what was helpful, what was confusing
Benefit: Your experience helps us improve onboarding
```

### Case 3: Validate New Improvements

```
Scenario: Framework team releases new UI specification feature
Action: Use /create-ui for first time
Feedback: Survey asks about UI quality, code clarity, documentation
Benefit: Real-world validation before rolling out to all users
```

---

## Frequently Asked Questions

### Q: Do I have to provide feedback?

**A:** No, feedback is completely optional. You can skip any prompt, and you can disable hooks entirely via configuration.

### Q: Will feedback slow down my workflow?

**A:** No. The feedback prompt appears AFTER your command completes. If you skip it, it adds <1ms overhead. Engaging with feedback typically takes 2-3 minutes.

### Q: Can I change my feedback after submitting?

**A:** Responses are saved as they're submitted. You can edit the JSON files in `devforgeai/feedback/sessions/` if needed, or delete them.

### Q: Who sees my feedback?

**A:** Feedback is stored locally in your project directory. If you choose to share (not automatic), you control where it goes. The framework doesn't upload feedback without your consent.

### Q: How often will I be asked for feedback?

**A:** Rate limiting prevents feedback fatigue. Default: max 3 prompts per day, with 60 minutes between prompts. You can adjust in `hooks.yaml`.

### Q: How do I disable feedback for specific commands?

**A:** Edit `devforgeai/hooks/hooks.yaml` and add operations to `skip_patterns`:
```yaml
skip_patterns:
  - "dev"
  - "create-context"
```

### Q: What if I don't want any feedback prompts?

**A:** Set `enabled: false` in `hooks.yaml`:
```yaml
enabled: false
```

### Q: Will feedback work if I don't have internet?

**A:** Yes. Feedback is saved locally to `devforgeai/feedback/sessions/`. You can review and share feedback whenever you choose, or never share it.

---

## Troubleshooting

### Hooks Not Appearing

**Problem:** You're not seeing feedback prompts even though hooks are enabled.

**Check:**
1. Verify `hooks.yaml` exists at `devforgeai/hooks/hooks.yaml`
2. Verify `enabled: true` in configuration
3. Check `skip_patterns` - make sure your operation isn't listed
4. Check `rate_limit` - you may have hit the daily limit
5. Check cooldown - you may need to wait before next prompt

**Solution:**
```bash
# View your current configuration
devforgeai feedback-config view

# Reset rate limit to allow feedback now
devforgeai feedback-config edit rate_limit.cooldown_minutes 0
```

### Feedback Not Saving

**Problem:** You provided feedback but don't see it in `devforgeai/feedback/sessions/`.

**Possible Causes:**
- Directory permissions (check read/write access)
- Disk space (file system full)
- Process interrupted (Ctrl+C during feedback)

**Check:**
```bash
# Verify directory exists
ls -la devforgeai/feedback/sessions/

# Check file permissions
ls -la devforgeai/hooks/

# View saved feedback
cat devforgeai/feedback/sessions/*.json
```

### Want to Share Feedback

**To share your feedback with the DevForgeAI team:**

1. Export your feedback sessions:
```bash
devforgeai export-feedback --output-dir=/path/to/export
```

2. Review the exported files (they're human-readable JSON)

3. Upload to the feedback portal or GitHub discussion (link provided by team)

---

## Best Practices

### 1. Be Specific

**Good:** "The test generation was confusing because the test names didn't match the acceptance criteria."

**Poor:** "Test generation wasn't great."

### 2. Explain Context

**Good:** "I'm learning TDD for the first time, and the step-by-step guidance in Phase 1 was really helpful."

**Poor:** "This was helpful."

### 3. Suggest Improvements

**Good:** "I'd find it useful to see a quick example of what passing tests look like before writing them."

**Poor:** "Make it better."

### 4. Mention What Worked

**Good:** "The error messages were really clear when tests failed."

**Poor:** "Just asking for feedback."

---

## Contributing to DevForgeAI

Your feedback helps shape the future of DevForgeAI. Here's how your responses impact the framework:

**Your Feedback → Framework Team Reviews → Improvements Made**

1. **Monthly Analysis:** Team reviews all feedback submitted
2. **Pattern Recognition:** Common issues become improvement priorities
3. **Feature Requests:** Frequently requested features are planned
4. **Validation:** Successful features are confirmed by users like you
5. **Improvements Shipped:** New versions incorporate feedback

**Thank you for helping make DevForgeAI better! 🙌**

---

## Related Commands

- `/dev [STORY-ID]` - Development with TDD (has feedback hooks)
- `/create-context [project-name]` - Context setup (has feedback hooks)
- `devforgeai feedback-config` - Manage feedback configuration
- `devforgeai feedback-search` - Search your feedback history
- `devforgeai export-feedback` - Export feedback for sharing

---

## Next Steps

1. **Configure Hooks:** Create `devforgeai/hooks/hooks.yaml` with your preferences
2. **Try a Command:** Use `/dev` or `/create-context` and see if feedback triggers
3. **Share Feedback:** Engage with a prompt to help improve the framework
4. **Adjust Settings:** Fine-tune rate limits and skip patterns as needed

---

**Last Updated:** 2025-11-17
**Framework Version:** 3.0 (Phase 3 Complete)
**Questions?** See troubleshooting section above or create a GitHub issue

