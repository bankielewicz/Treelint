---
description: Prevent hardcoded secrets in code
version: "1.0"
created: 2025-12-10
---

# No Hardcoded Secrets

## Forbidden Patterns
- API keys in source code
- Passwords in configuration files
- Private keys in repository
- Connection strings with credentials

## Required Approach
- Use environment variables
- Use secrets management (Vault, AWS Secrets Manager)
- Use `.env` files (gitignored)
- Reference secrets by name, not value

## Detection Patterns
```
# Block commits containing:
password\s*=\s*['"][^'"]+['"]
api[_-]?key\s*=\s*['"][^'"]+['"]
secret\s*=\s*['"][^'"]+['"]
-----BEGIN (RSA|DSA|EC|OPENSSH) PRIVATE KEY-----
```

## Exception Process
1. Raise in AskUserQuestion
2. Document justification
3. Add to .gitignore if file-based
4. Use environment variable injection
