# Security Scanning Policies

## SAST (Static Application Security Testing)

### Scan Frequency
- **Every commit:** Light security checks (SQL injection, secrets)
- **Before release:** Comprehensive SAST scan

### Tools by Language
- **.NET:** Security Code Scan, dotnet-security-scan
- **Python:** Bandit
- **JavaScript/TypeScript:** ESLint security plugin, retire.js
- **Java:** SpotBugs, Find Security Bugs

### Severity Levels

| Severity | Action | Examples |
|----------|--------|----------|
| CRITICAL | Block immediately | SQL injection, hardcoded secrets, command injection |
| HIGH | Block release | XSS, weak cryptography, insecure deserialization |
| MEDIUM | Warning, fix soon | Missing security headers, insecure cookies |
| LOW | Advisory | Code quality security implications |

## Vulnerability Categories

### 1. Injection Vulnerabilities (CRITICAL)

**SQL Injection (CWE-89):**
- Pattern: String concatenation in SQL queries
- Detection: `string.Format.*SELECT`, `f"SELECT.*{`, `ExecuteRawSql(.*+`
- Fix: Use parameterized queries

**Command Injection (CWE-78):**
- Pattern: User input in system commands
- Detection: `Process.Start`, `exec()`, `system()` with user input
- Fix: Validate input, use safe APIs

**NoSQL Injection:**
- Pattern: Unvalidated user input in NoSQL queries
- Detection: Direct object embedding in MongoDB, etc.
- Fix: Use query builders, validate input

### 2. Cross-Site Scripting (HIGH)

**Reflected XSS (CWE-79):**
- Pattern: User input directly in HTML
- Detection: `innerHTML =`, `dangerouslySetInnerHTML`, `document.write(`
- Fix: Use textContent, sanitization libraries

**Stored XSS:**
- Pattern: Persisted user input rendered without encoding
- Fix: Output encoding, Content Security Policy

### 3. Authentication/Authorization (HIGH)

**Broken Authentication:**
- Missing authentication on endpoints
- Weak password policies
- Session fixation vulnerabilities

**Broken Access Control:**
- Missing authorization checks
- Insecure direct object references
- Path traversal (CWE-22)

### 4. Cryptographic Failures (HIGH)

**Weak Cryptography (CWE-327):**
- Pattern: MD5, SHA1, DES, TripleDES
- Detection: `MD5.Create()`, `SHA1.Create()`, `DES.Create()`
- Fix: Use SHA256, SHA512, AES256, bcrypt for passwords

**Hardcoded Secrets (CWE-798):**
- Pattern: Passwords, API keys, connection strings in code
- Detection: `password = "..."`, `apiKey = "..."`, regex patterns
- Fix: Environment variables, secret managers (Azure Key Vault, AWS Secrets Manager)

### 5. Deserialization Vulnerabilities (HIGH)

**Insecure Deserialization (CWE-502):**
- Pattern: Deserializing untrusted data
- Detection: `BinaryFormatter`, `pickle.loads()`, `unserialize()`
- Fix: Use safe serializers (JSON), validate input

## Dependency Vulnerability Scanning

### Scan Frequency
- **Weekly:** Automated dependency scans
- **Before release:** Comprehensive dependency audit

### Tools
- **.NET:** `dotnet list package --vulnerable --include-transitive`
- **Python:** `pip-audit`, `safety check`
- **JavaScript:** `npm audit`, `yarn audit`
- **Java:** OWASP Dependency-Check

### Action Thresholds

| CVE Severity | Action | Timeline |
|--------------|--------|----------|
| Critical (9.0-10.0) | Block release, fix immediately | < 24 hours |
| High (7.0-8.9) | Block release, fix before ship | < 1 week |
| Medium (4.0-6.9) | Warning, plan fix | < 1 month |
| Low (0.1-3.9) | Advisory, monitor | When convenient |

### Vulnerability Response

1. **Assess Impact:** Does it affect our usage?
2. **Check Fix Availability:** Is patched version available?
3. **Update Dependencies:** Upgrade to fixed version
4. **Test:** Verify fix doesn't break functionality
5. **Document:** Update dependencies.md, create ADR if needed

## Security Best Practices

### Input Validation
- Validate all user input
- Whitelist > blacklist approach
- Sanitize before use
- Use strong typing

### Output Encoding
- Encode output based on context (HTML, URL, JavaScript)
- Use framework-provided encoding
- Content Security Policy headers

### Authentication
- Use established frameworks (OAuth 2.0, OpenID Connect)
- Multi-factor authentication for sensitive operations
- Secure session management
- Password hashing (bcrypt, Argon2)

### Authorization
- Principle of least privilege
- Role-based access control (RBAC)
- Authorization checks on every request
- Secure direct object references

### Cryptography
- Use TLS 1.2+ for transport
- Use strong algorithms (AES256, SHA256+)
- Proper key management
- Don't roll your own crypto

### Error Handling
- Don't expose stack traces
- Generic error messages to users
- Detailed logging for debugging
- Fail securely

### Logging & Monitoring
- Log security events
- Monitor for anomalies
- Don't log sensitive data
- Centralized logging

## OWASP Top 10 Coverage

### 2021 OWASP Top 10

1. **Broken Access Control** → Authorization checks, path traversal detection
2. **Cryptographic Failures** → Weak crypto detection, secret scanning
3. **Injection** → SQL injection, command injection, XSS detection
4. **Insecure Design** → Architecture review (anti-patterns.md)
5. **Security Misconfiguration** → Dependency scanning, header checks
6. **Vulnerable Components** → Dependency vulnerability scanning
7. **Authentication Failures** → Authentication pattern review
8. **Software/Data Integrity Failures** → Deserialization checks, supply chain
9. **Logging/Monitoring Failures** → Logging pattern review
10. **Server-Side Request Forgery (SSRF)** → URL validation checks

## Compliance Requirements

### GDPR
- Personal data handling
- Right to erasure
- Data encryption
- Audit logging

### PCI DSS
- Cardholder data protection
- Encryption at rest and in transit
- Access control
- Regular security testing

### HIPAA
- Protected Health Information (PHI) security
- Encryption requirements
- Access controls
- Audit trails

## Exemptions & Exceptions

### Test Code
- Security scans may be less strict for test code
- Hardcoded test data acceptable
- Mock credentials allowed

### Development Tools
- Tools not deployed to production
- Development utilities
- Code generators

### Third-Party Code
- Vendored dependencies (not our code)
- Generated code
- Legacy code (with documented exceptions)

## Reporting

### Security Report Format
```json
{
  "scan_date": "2025-10-30",
  "tool": "bandit",
  "vulnerabilities": [
    {
      "severity": "CRITICAL",
      "cwe": "CWE-89",
      "category": "SQL Injection",
      "file": "src/repository.py",
      "line": 42,
      "description": "SQL query uses string formatting",
      "recommendation": "Use parameterized queries"
    }
  ],
  "summary": {
    "critical": 1,
    "high": 3,
    "medium": 5,
    "low": 10
  }
}
```

## References

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Top 25](https://cwe.mitre.org/top25/archive/2023/2023_top25_list.html)
- [NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)
- [Security Scanning Reference](../../references/security-scanning.md)
