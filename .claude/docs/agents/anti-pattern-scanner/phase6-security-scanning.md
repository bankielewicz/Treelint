# Phase 6: Security Vulnerability Scanning (OWASP Top 10)

**Purpose:** Detect security vulnerabilities
**Severity:** CRITICAL (always blocks QA)
**Duration:** <5 seconds

---

## Check 1: Hard-Coded Secrets (A02:2021)

**Patterns:**
```
password\s*=\s*["'].*["']
apiKey\s*=\s*["'].*["']
secret\s*=\s*["'].*["']
token\s*=\s*["'].*["']
```

**Detection:**
```
secrets = Grep(pattern="(password|apiKey|secret|token)\\s*=\\s*[\"']", glob="**/*.{cs,py,js,ts}")

FOR each match in secrets:
  # Check if right-hand side is string literal
  IF not is_env_var(match) AND not is_function_call(match):
    violations.append({
      "pattern": "Hard-coded secret",
      "evidence": match.line,
      "remediation": "Use environment variables or secure vault (e.g., Azure Key Vault, AWS Secrets Manager)",
      "severity": "CRITICAL",
      "owasp": "A02:2021 – Cryptographic Failures"
    })
```

---

## Check 2: SQL Injection Risk (A03:2021)

**Pattern:** String concatenation in SQL queries

```
sql_concat = Grep(pattern='(SELECT|INSERT|UPDATE|DELETE).*\\+.*', glob="**/*.{cs,py,js}")

FOR each match in sql_concat:
  violations.append({
    "pattern": "SQL injection risk",
    "evidence": match.line,
    "remediation": "Use parameterized queries or ORM",
    "severity": "CRITICAL",
    "owasp": "A03:2021 – Injection"
  })
```

---

## Check 3: XSS Vulnerabilities (A03:2021)

**Pattern:** innerHTML or dangerouslySetInnerHTML without sanitization

```
xss = Grep(pattern="(innerHTML|dangerouslySetInnerHTML)", glob="**/*.{js,jsx,ts,tsx}")

FOR each match in xss:
  IF not has_sanitization(match):
    violations.append({
      "pattern": "XSS vulnerability",
      "evidence": match.line,
      "remediation": "Use DOMPurify or textContent instead",
      "severity": "CRITICAL",
      "owasp": "A03:2021 – Injection"
    })
```

---

## Check 4: Insecure Deserialization (A08:2021)

**Pattern:** Deserialize user input without validation

```
deser = Grep(pattern="(JSON\\.parse|JsonConvert\\.DeserializeObject|pickle\\.loads)", glob="**/*")

FOR each match in deser:
  IF operates_on_user_input(match):
    violations.append({
      "pattern": "Insecure deserialization",
      "evidence": match.line,
      "remediation": "Validate input schema before deserialization",
      "severity": "CRITICAL",
      "owasp": "A08:2021 – Software and Data Integrity Failures"
    })
```

---

## Output

```json
{
  "violations": [
    {
      "file": "src/Config/DatabaseConfig.cs",
      "line": 15,
      "pattern": "Hard-coded secret",
      "evidence": "password=\"MySecret123\"",
      "remediation": "Use environment variables: Environment.GetEnvironmentVariable(\"DB_PASSWORD\")",
      "severity": "CRITICAL",
      "owasp": "A02:2021 – Cryptographic Failures"
    }
  ],
  "blocks_qa": true
}
```
