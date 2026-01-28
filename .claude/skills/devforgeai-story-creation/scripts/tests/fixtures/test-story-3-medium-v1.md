---
id: STORY-TEST-003
title: User Management API with Repository and Data Model
status: Backlog
points: 8
priority: High
created: 2025-11-07
---

# Story: User Management API with Repository and Data Model

## User Story

**As an** API consumer,
**I want** to create and retrieve user accounts via REST API,
**so that** I can manage users programmatically.

## Acceptance Criteria

### 1. [x] Create User via API

**Given** valid user data (email, password, name, role)
**When** POST /api/users is called
**Then** a new user is created in the database
**And** response returns 201 Created with user object

### 2. [x] Email Validation and Uniqueness

**Given** a user creation request
**When** email format is invalid OR email already exists
**Then** API returns 400 Bad Request with validation error details

### 3. [x] Retrieve User by ID

**Given** a user exists in the database
**When** GET /api/users/{id} is called
**Then** API returns 200 OK with user object
**And** password is NOT included in response (security)

## Technical Specification

### API Endpoints

**POST /api/users** - Create new user account
- Request body: {email, password, name, role}
- Validation:
  - Email format (valid email pattern)
  - Email uniqueness (no duplicates)
  - Password strength (min 8 chars, must include uppercase, lowercase, number, special char)
  - Name length (2-100 chars)
  - Role enum (customer, admin, moderator)
- Response 201 Created: {id, email, name, role, created_at}
- Response 400 Bad Request: Validation errors with field-level details
- Response 409 Conflict: Email already exists

**GET /api/users/{id}** - Retrieve user by ID
- Path parameter: id (UUID)
- Response 200 OK: User object (WITHOUT password)
- Response 404 Not Found: User not found

### Data Access

UserRepository implements IUserRepository with Dapper for data access:
- **GetById(id)** → User or null
- **GetByEmail(email)** → User or null (for uniqueness check)
- **Create(user)** → User with generated ID
- **Update(user)** → Updated user
- **Delete(id)** → bool success

Must use parameterized queries to prevent SQL injection.
All database operations must use async methods for performance.

### Data Model

**User entity** - Represents a user account

Database table: dbo.Users

Fields:
- **Id** (UUID, primary key, auto-generated)
- **Email** (string, unique index, max 255 chars, required)
- **PasswordHash** (string, 255 chars, required) - Never store plaintext passwords
- **Name** (string, max 100 chars, required)
- **Role** (enum: customer, admin, moderator, required)
- **CreatedAt** (datetime, auto-generated, required)
- **UpdatedAt** (datetime, auto-updated on changes, nullable)

Indexes:
- Unique index on Email (fast lookup, enforce uniqueness)
- Index on CreatedAt (for sorting by registration date)

### Business Rules

1. **Email Uniqueness:** Email must be unique across all users (enforced by unique index)
2. **Password Security:** Passwords must be hashed before storage (use BCrypt with salt)
3. **Role Validation:** Role must be one of: customer, admin, moderator (enum validation)

### Non-Functional Requirements

- API response time: <500ms at p95 percentile
- Database queries: <100ms for indexed lookups
- Password hashing: BCrypt with cost factor 12 (secure but performant)

## Definition of Done

- [ ] POST /api/users endpoint implemented
- [ ] GET /api/users/{id} endpoint implemented
- [ ] UserRepository with all CRUD methods
- [ ] User entity with proper constraints
- [ ] Email validation and uniqueness checks
- [ ] Password hashing (BCrypt)
- [ ] All API tests passing
- [ ] Code coverage >95%
