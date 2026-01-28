# Phase 4: Technical Specification Workflow

Create high-level technical specifications that transform requirements into detailed implementation guidance.

## Overview

Phase 4 produces comprehensive technical specifications that guide development teams. These specs bridge the gap between business requirements and implementation code.

**References Used:**
- **system-design-patterns.md** - Architecture pattern library and design guidance

---

## Specification Components

Technical specifications should include:

### 1. Functional Specifications

**Use cases with actors and flows:**
```markdown
## Use Case: User Registration

**Actor:** Anonymous User
**Preconditions:** None
**Trigger:** User clicks "Sign Up" button

**Main Flow:**
1. User provides email and password
2. System validates email format
3. System checks email is unique
4. System hashes password (bcrypt)
5. System creates user record
6. System sends verification email
7. System displays "Check your email" message

**Alternate Flows:**
- 2a. Email invalid → Display error, return to step 1
- 3a. Email exists → Display "Email already registered", offer password reset

**Postconditions:** User created with status "Pending Verification"
```

**Acceptance criteria (measurable):**
```markdown
## Acceptance Criteria

1. ✅ Email validation follows RFC 5322 standard
2. ✅ Password must be 12+ characters with complexity requirements
3. ✅ Verification email sent within 30 seconds
4. ✅ User cannot log in until email verified
5. ✅ Registration completes in <500ms (95th percentile)
```

**Business rules and validation:**
```markdown
## Business Rules

- BR-001: Email must be unique across all users
- BR-002: Passwords must meet complexity: 12+ chars, uppercase, lowercase, number, symbol
- BR-003: Verification links expire after 24 hours
- BR-004: Max 3 registration attempts per email per day (prevent abuse)
- BR-005: Usernames must be 3-20 characters, alphanumeric + underscore only
```

**Data models and relationships:**
```markdown
## Data Models

### User Entity

```csharp
public class User
{
    public int Id { get; set; }
    public string Email { get; set; }
    public string PasswordHash { get; set; }
    public UserStatus Status { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime? VerifiedAt { get; set; }
}

public enum UserStatus
{
    PendingVerification = 0,
    Active = 1,
    Suspended = 2,
    Deleted = 3
}
```

### Relationships

- User 1:N EmailVerificationTokens
- User 1:N PasswordResetTokens
```
```

---

### 2. API Specifications

**Endpoint definitions (OpenAPI/Swagger):**
```markdown
## API Endpoints

### POST /api/users/register

**Description:** Create new user account

**Request:**
```json
{
  "email": "user@example.com",
  "password": "SecurePass123!",
  "username": "johndoe"
}
```

**Response (201 Created):**
```json
{
  "id": 42,
  "email": "user@example.com",
  "username": "johndoe",
  "status": "PendingVerification",
  "createdAt": "2025-01-06T10:30:00Z"
}
```

**Response (400 Bad Request):**
```json
{
  "error": "ValidationError",
  "message": "Email already registered",
  "field": "email"
}
```

**Response (429 Too Many Requests):**
```json
{
  "error": "RateLimitExceeded",
  "message": "Too many registration attempts. Try again in 24 hours.",
  "retryAfter": 86400
}
```
```

**Authentication/authorization:**
```markdown
## Authentication

**Endpoint Security:**
- POST /api/users/register → Public (no auth)
- GET /api/users/{id} → Authenticated (JWT required)
- PUT /api/users/{id} → Authenticated + Owner or Admin role

**JWT Configuration:**
- Algorithm: HS256
- Expiration: 1 hour (access token)
- Refresh token: 7 days
- Issuer/Audience validation required
```

**Error codes and handling:**
```markdown
## Error Codes

| Code | Status | Meaning | Example |
|------|--------|---------|---------|
| 1001 | 400 | Email invalid format | "user@" |
| 1002 | 400 | Email already registered | Duplicate email |
| 1003 | 400 | Password too weak | "password123" |
| 1004 | 429 | Rate limit exceeded | 4th attempt in 24h |
| 1005 | 500 | Email send failed | SMTP error |
```

---

### 3. Database Specifications

**Entity-Relationship Diagrams (ERD):**
```markdown
## Database Schema

### Users Table

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| Id | INT | PRIMARY KEY, IDENTITY | Auto-incrementing ID |
| Email | NVARCHAR(255) | UNIQUE, NOT NULL | User email |
| PasswordHash | NVARCHAR(255) | NOT NULL | bcrypt hash |
| Username | NVARCHAR(50) | UNIQUE, NOT NULL | Display name |
| Status | TINYINT | NOT NULL, DEFAULT 0 | UserStatus enum |
| CreatedAt | DATETIME2 | NOT NULL, DEFAULT GETUTCDATE() | Registration time |
| VerifiedAt | DATETIME2 | NULL | Email verification time |

**Indexes:**
- IX_Users_Email (UNIQUE, NONCLUSTERED)
- IX_Users_Username (UNIQUE, NONCLUSTERED)
- IX_Users_Status (NONCLUSTERED) - For filtering active users
```

**Migration strategy:**
```markdown
## Migration Strategy

Use FluentMigrator for all schema changes:

```csharp
[Migration(20250106001)]
public class CreateUsersTable : Migration
{
    public override void Up()
    {
        Create.Table("Users")
            .WithColumn("Id").AsInt32().PrimaryKey().Identity()
            .WithColumn("Email").AsString(255).NotNullable().Unique()
            .WithColumn("PasswordHash").AsString(255).NotNullable()
            .WithColumn("Username").AsString(50).NotNullable().Unique()
            .WithColumn("Status").AsByte().NotNullable().WithDefaultValue(0)
            .WithColumn("CreatedAt").AsDateTime2().NotNullable()
                .WithDefault(SystemMethods.CurrentUTCDateTime)
            .WithColumn("VerifiedAt").AsDateTime2().Nullable();
    }

    public override void Down()
    {
        Delete.Table("Users");
    }
}
```

**Migration naming:** `{YYYYMMDDHHMMSS}_{Description}.cs`
```

---

### 4. Non-Functional Requirements (NFRs)

**Performance targets:**
```markdown
## Performance Requirements

| Operation | Target | Measurement |
|-----------|--------|-------------|
| User registration | <500ms | 95th percentile |
| Email send | <5s | 99th percentile |
| Login | <200ms | 95th percentile |
| Password validation | <50ms | 99th percentile |

**Load requirements:**
- Support 1,000 concurrent registrations
- Handle 10,000 users total (initial launch)
- Scale to 100,000 users (year 1)
```

**Scalability requirements:**
```markdown
## Scalability Requirements

**Concurrent users:** 1,000 simultaneous registrations
**Data volume:** 100,000 users (year 1 target)
**Growth rate:** 500 new users per day (average)

**Scaling strategy:**
- Horizontal scaling (multiple API instances)
- Database read replicas (if needed)
- Caching layer (Redis for verification tokens)
```

**Security requirements:**
```markdown
## Security Requirements

**Password Security:**
- bcrypt hashing (cost factor 12)
- Salt generated per-user
- Never store plaintext passwords
- Password reset links expire in 1 hour

**Email Verification:**
- Cryptographically secure tokens (32 bytes)
- One-time use (invalidate after verification)
- Expire after 24 hours

**Rate Limiting:**
- Max 3 registration attempts per email per 24 hours
- Max 10 verification email resends per email per 24 hours
- IP-based rate limiting (100 requests per hour)

**Compliance:**
- GDPR: User consent for email communications
- GDPR: Right to be forgotten (DELETE /api/users/{id})
- Data retention: Delete unverified accounts after 30 days
```

**Availability and reliability:**
```markdown
## Availability Requirements

**Uptime:** 99.9% (43.2 minutes downtime per month)
**Recovery Time Objective (RTO):** 15 minutes
**Recovery Point Objective (RPO):** 5 minutes (max data loss)

**Disaster Recovery:**
- Daily database backups
- Geo-redundant storage
- Automated backup testing weekly
```

---

## Ambiguity Resolution During Spec Creation

When specs contain ambiguous requirements:

### Example: "Handle authentication" without details

```
Question: "The spec requires authentication. Which method should be used?"
Header: "Auth method"
Description: "This will be documented in tech-stack.md"
Options:
  - "JWT tokens (stateless)"
  - "OAuth 2.0 (third-party auth)"
  - "Session-based (stateful)"
  - "Passkeys/WebAuthn (passwordless)"
multiSelect: false
```

### Example: Performance requirements vague ("fast", "scalable")

```
Question: "What are the performance targets for this API?"
Header: "Performance"
Options:
  - "High performance (< 100ms response time)"
  - "Standard (< 500ms response time)"
  - "Acceptable (< 2s response time)"
multiSelect: false
```

### Example: Security requirements unclear

```
Question: "What security compliance requirements apply?"
Header: "Compliance"
Options:
  - "GDPR only (European users)"
  - "HIPAA (healthcare data)"
  - "SOC 2 (enterprise SaaS)"
  - "PCI DSS (payment processing)"
multiSelect: true  # Multiple may apply
```

**Always use AskUserQuestion for ambiguous NFRs** - never assume performance targets, security requirements, or compliance needs.

---

## Output

Phase 4 produces:

1. **Functional specifications:**
   - Use cases with flows
   - Acceptance criteria
   - Business rules
   - Data models

2. **API specifications:**
   - Endpoint definitions
   - Request/response formats
   - Authentication approach
   - Error handling

3. **Database specifications:**
   - Table schemas
   - Indexes
   - Relationships
   - Migration scripts

4. **Non-functional requirements:**
   - Performance targets (specific, measurable)
   - Scalability requirements
   - Security requirements
   - Availability targets

**Write specifications to:** `docs/specs/` or `docs/architecture/` (per source-tree.md)

**Next Phase:** Move to Phase 5 (Validate Spec Against Context)
