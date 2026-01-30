---
name: context-sanitization
description: Pattern for removing secrets and PII from operation context before feedback use
version: "1.0"
created: 2025-12-18
story: STORY-103
---

# Context Sanitization Pattern

Remove secrets, credentials, and personally identifiable information (PII) from extracted context before using in feedback conversations.

## When to Use

Apply sanitization:
1. After extracting context from TodoWrite
2. Before including context in feedback prompts
3. Before logging or persisting context data
4. Before displaying context to users

Never skip sanitization. Context may contain secrets from:
- Environment variables in error messages
- Credential file paths in stack traces
- User data in operation output

---

## Security Rationale

**Why sanitize?**
- Secrets in feedback could be persisted to session files
- PII in feedback violates data protection requirements
- Credentials in logs create security vulnerabilities
- User trust depends on responsible data handling

**Defense in depth:**
- Sanitize at extraction time
- Sanitize before any output
- Never assume context is clean

---

## Secret Removal Patterns

### Environment Variable Patterns

Detect and remove environment variables containing sensitive keywords.

| Pattern | Matches | Example |
|---------|---------|---------|
| `[A-Z_]*KEY[A-Z_]*\s*=\s*\S+` | Any KEY variable | `API_KEY=abc123` |
| `[A-Z_]*SECRET[A-Z_]*\s*=\s*\S+` | Any SECRET variable | `JWT_SECRET=xyz789` |
| `[A-Z_]*TOKEN[A-Z_]*\s*=\s*\S+` | Any TOKEN variable | `ACCESS_TOKEN=def456` |
| `[A-Z_]*PASSWORD[A-Z_]*\s*=\s*\S+` | Any PASSWORD variable | `DB_PASSWORD=hunter2` |
| `[A-Z_]*CREDENTIAL[A-Z_]*\s*=\s*\S+` | Any CREDENTIAL variable | `AWS_CREDENTIAL=...` |
| `[A-Z_]*AUTH[A-Z_]*\s*=\s*\S+` | Any AUTH variable | `AUTH_TOKEN=ghi789` |

### API Key Patterns

Detect common API key formats.

| Pattern | Description |
|---------|-------------|
| `(?:api[_-]?key\|apikey)\s*[:=]\s*['"]?[A-Za-z0-9_-]{20,}['"]?` | Generic API keys |
| `Bearer\s+[A-Za-z0-9_-]{20,}` | Bearer tokens |
| `(?:AKIA\|ASIA)[A-Z0-9]{16}` | AWS access keys |
| `ghp_[A-Za-z0-9]{36}` | GitHub personal tokens |
| `sk-[A-Za-z0-9]{48}` | OpenAI API keys |

### Credential File Path Patterns

Detect file paths that may contain credentials.

| Pattern | Matches |
|---------|---------|
| `.*\.pem$` | PEM certificate files |
| `.*\.key$` | Private key files |
| `.*id_rsa.*` | SSH private keys |
| `.*\.env.*` | Environment files |
| `.*credentials.*` | Credential files |
| `.*secrets.*` | Secrets files |
| `.*password.*` | Password files |

---

## PII Removal Patterns

Remove personally identifiable information to protect user privacy.

### Email Addresses

Pattern: `[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}`

Examples removed:
- `user@example.com`
- `john.doe@company.org`
- `support+tag@service.io`

### Phone Numbers

Pattern: `(?:\+1[-.\s]?)?(?:\(?[2-9][0-9]{2}\)?[-.\s]?)?[2-9][0-9]{2}[-.\s]?[0-9]{4}`

Examples removed:
- `555-123-4567`
- `(555) 123-4567`
- `+1-555-123-4567`
- `5551234567`

### Social Security Numbers

Pattern: `[0-9]{3}[-\s]?[0-9]{2}[-\s]?[0-9]{4}`

Examples removed:
- `123-45-6789`
- `123 45 6789`
- `123456789`

### Credit Card Numbers

Pattern: `[0-9]{4}[-\s]?[0-9]{4}[-\s]?[0-9]{4}[-\s]?[0-9]{4}`

Examples removed:
- `4111-1111-1111-1111`
- `4111 1111 1111 1111`

---

## Sanitization Workflow

Follow these steps to sanitize extracted context.

### Step 1: Identify Sensitive Content

Scan all string fields in context:
1. Iterate through all OperationContext fields
2. Iterate through all TodoContext items
3. Iterate through ErrorContext if present
4. Check stack_trace carefully (highest risk)

For each string value:
- Apply all secret patterns
- Apply all PII patterns
- Record matches found

### Step 2: Replace with Redaction Markers

Replace matched content with type-specific markers.

| Content Type | Redaction Marker |
|--------------|------------------|
| Environment variable | `[REDACTED:SECRET]` |
| API key | `[REDACTED:API_KEY]` |
| Bearer token | `[REDACTED:TOKEN]` |
| File path credential | `[REDACTED:CREDENTIAL_PATH]` |
| Email address | `[REDACTED:EMAIL]` |
| Phone number | `[REDACTED:PHONE]` |
| SSN | `[REDACTED:SSN]` |
| Credit card | `[REDACTED:CARD]` |
| Unknown secret | `[REDACTED]` |

Marker format preserves:
- That something was redacted
- What type of data was removed
- Context remains parseable

### Step 3: Log Sanitization Actions

Record what was sanitized (without exposing secrets).

Log format:
```
[SANITIZATION] Field: {field_name}, Type: {redaction_type}, Count: {N}
```

Example log entries:
```
[SANITIZATION] Field: error.stack_trace, Type: SECRET, Count: 2
[SANITIZATION] Field: todos[3].content, Type: EMAIL, Count: 1
```

Never log:
- The actual secret value
- Partial secret value
- Position within string (could narrow down)

### Step 4: Verify No Secrets Remain

After sanitization, verify cleanup:

1. Re-run all patterns on sanitized output
2. If any matches found, log error and re-sanitize
3. Maximum 3 sanitization passes (prevent infinite loop)
4. If still matching after 3 passes, return empty context

Final verification ensures:
- No nested secrets (secret containing secret)
- No pattern evasion (split secrets)
- Clean output guaranteed

---

## Logging Sanitization

### What to Log

Always log:
- Field name that was sanitized
- Type of redaction applied
- Count of redactions per field
- Total redactions per context
- Sanitization duration (for performance)

Example summary log:
```
[SANITIZATION] Complete: 5 redactions in 3 fields, 12ms
  - error.message: 1 SECRET
  - error.stack_trace: 3 SECRET, 1 EMAIL
```

### What NOT to Log

Never log:
- Original secret values
- Partial secret values
- Exact character positions
- Patterns that matched (could reveal format)
- Surrounding context of secrets

### Log Levels

| Situation | Level |
|-----------|-------|
| Normal sanitization | DEBUG |
| High redaction count (>10) | INFO |
| Verification required re-sanitize | WARNING |
| 3-pass limit reached | ERROR |

---

## Examples

### Example 1: API Key Removal

Before:
```json
{
  "error": {
    "message": "Authentication failed with API_KEY=sk-abc123xyz789 for endpoint"
  }
}
```

After:
```json
{
  "error": {
    "message": "Authentication failed with [REDACTED:SECRET] for endpoint"
  }
}
```

Log: `[SANITIZATION] Field: error.message, Type: SECRET, Count: 1`

### Example 2: Email and Phone Removal

Before:
```json
{
  "todos": [
    {"content": "Notify user@example.com at 555-123-4567"}
  ]
}
```

After:
```json
{
  "todos": [
    {"content": "Notify [REDACTED:EMAIL] at [REDACTED:PHONE]"}
  ]
}
```

Log: `[SANITIZATION] Field: todos[0].content, Type: EMAIL, Count: 1`
Log: `[SANITIZATION] Field: todos[0].content, Type: PHONE, Count: 1`

### Example 3: Stack Trace Sanitization

Before:
```
Error: Connection failed
  at connect (db.js:42)
  with password=secret123
  at Database.open (db.js:15)
  config: /home/user/.credentials/db.key
```

After:
```
Error: Connection failed
  at connect (db.js:42)
  with [REDACTED:SECRET]
  at Database.open (db.js:15)
  config: [REDACTED:CREDENTIAL_PATH]
```

### Example 4: No Secrets Found

When no sensitive content detected:

Log: `[SANITIZATION] Complete: 0 redactions, 8ms`

No changes to context, but verification still runs.

---

## Integration with Context Extraction

Sanitization integrates with context-extraction.md workflow:

1. Extract context (context-extraction.md steps 1-6)
2. Apply sanitization (this document steps 1-4)
3. Return sanitized context for feedback use

Call sanitization immediately after extraction, before:
- Logging context
- Passing to feedback skill
- Persisting to session files

---

## Related Documentation

- `context-extraction.md` - Extraction patterns before sanitization
- `feedback-persistence-guide.md` - Safe storage of sanitized context
- `../SKILL.md` - Feedback skill main documentation

---

**Document Version:** 1.0
**Last Updated:** 2025-12-18
**Story:** STORY-103
