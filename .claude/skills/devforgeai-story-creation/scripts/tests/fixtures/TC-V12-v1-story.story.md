---
id: STORY-V12-TEST
title: Legacy v1.0 Story with Freeform Tech Spec
status: Backlog
---

# Story: v1.0 Format Test

## User Story

**As a** developer,
**I want** to test v1.0 format detection,
**so that** the validator recognizes legacy stories.

## Acceptance Criteria

**Given** a story with v1.0 freeform tech spec (no YAML block)
**When** the validator runs
**Then** it should warn about legacy format (not error - v1.0 is valid)

## Technical Specification

This is a freeform markdown technical specification (v1.0 format).

### Architecture

Use microservices pattern with:
- User service for authentication
- Payment service for transactions
- Notification service for emails

### API Endpoints

```
POST /api/users/register - Create new user account
POST /api/users/login - Authenticate user
GET /api/users/profile - Retrieve user profile
```

### Data Models

```typescript
interface User {
  id: string;
  email: string;
  passwordHash: string;
  createdAt: Date;
}
```

### Database

PostgreSQL database with users, sessions, and audit_logs tables.

## Definition of Done

- [ ] Validator detects v1.0 format
- [ ] Validator warns (not errors) for backward compatibility
