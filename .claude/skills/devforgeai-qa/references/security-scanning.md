# Security Scanning Reference

## SAST (Static Application Security Testing)

### Tools by Language

**.NET:**
```bash
dotnet tool install --global security-scan
security-scan src/
```

**Python:**
```bash
pip install bandit
bandit -r src/ -f json
```

**JavaScript:**
```bash
npm audit
npm install -g retire
retire --path src/
```

## Common Vulnerabilities

### SQL Injection (CWE-89)

**Detection patterns:**
- `ExecuteRawSql(.*+`
- `string.Format.*SELECT`
- `f"SELECT.*{`

**Secure fix:**
```csharp
// ❌ Vulnerable
var sql = $"SELECT * FROM Users WHERE Id = {userId}";

// ✅ Secure
var sql = "SELECT * FROM Users WHERE Id = @Id";
var user = await conn.QuerySingleAsync<User>(sql, new { Id = userId });
```

### XSS (CWE-79)

**Detection patterns:**
- `innerHTML =`
- `dangerouslySetInnerHTML`
- `document.write(`

**Secure fix:**
```javascript
// ❌ Vulnerable
element.innerHTML = userInput;

// ✅ Secure
element.textContent = userInput; // Or use sanitization library
```

### Hardcoded Secrets

**Detection patterns:**
- `password\s*=\s*["'][^"']+["']`
- `api_?key\s*=\s*["'][^"']+["']`
- `connectionstring.*password=`

**Secure fix:**
```csharp
// ❌ Hardcoded
var password = "MySecretPassword123";

// ✅ Configuration
var password = config["Database:Password"];
// OR environment variable
var password = Environment.GetEnvironmentVariable("DB_PASSWORD");
```

### Weak Cryptography

**Avoid:** MD5, SHA1, DES, TripleDES

**Use:** SHA256, SHA512, AES256, bcrypt (for passwords)

## Dependency Vulnerabilities

```bash
# .NET
dotnet list package --vulnerable --include-transitive

# Python
pip-audit

# JavaScript
npm audit
# OR
yarn audit
```

## OWASP Top 10 Checks

1. Broken Access Control
2. Cryptographic Failures
3. Injection (SQL, NoSQL, OS, LDAP)
4. Insecure Design
5. Security Misconfiguration
6. Vulnerable Components
7. Authentication Failures
8. Software/Data Integrity Failures
9. Logging/Monitoring Failures
10. Server-Side Request Forgery

## Quick Reference

| Vulnerability | Pattern | CWE | Severity |
|---------------|---------|-----|----------|
| SQL Injection | String concat in queries | CWE-89 | CRITICAL |
| XSS | innerHTML, document.write | CWE-79 | CRITICAL |
| Hardcoded Secret | password="..." | CWE-798 | CRITICAL |
| Weak Crypto | MD5, SHA1 | CWE-327 | HIGH |
| Path Traversal | ../  in paths | CWE-22 | HIGH |
