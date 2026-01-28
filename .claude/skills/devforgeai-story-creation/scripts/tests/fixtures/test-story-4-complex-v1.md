---
id: STORY-TEST-004
title: Complete Alert Management System (Full Stack)
status: Backlog
points: 13
priority: Critical
created: 2025-11-07
---

# Story: Complete Alert Management System

## User Story

**As a** system administrator,
**I want** a complete alert management system with APIs, background workers, and persistence,
**so that** alerts are detected, processed, stored, and users can be notified automatically.

## Acceptance Criteria

### 1. [x] Alert Detection and Processing

**Given** the alert detection system is running
**When** new alerts are detected in the monitored systems
**Then** alerts are created in the database
**And** notifications are queued for sending

### 2. [x] Alert Management via API

**Given** a user with appropriate permissions
**When** they interact with the alert management API
**Then** they can create, list, retrieve, and resolve alerts
**And** all operations are authenticated and authorized

### 3. [x] Background Processing

**Given** the background workers are running
**When** alerts and emails are queued
**Then** workers process them asynchronously without blocking
**And** failures are retried with exponential backoff

### 4. [x] Complete Logging and Monitoring

**Given** the system is operational
**When** any component logs events
**Then** logs are written to file, Windows Event Log, and database
**And** all logs are structured and searchable

## Technical Specification

### Complete Alert Management System

This story implements a full-stack alert management system with all layers.

### Service Layer

**AlertingService** (IHostedService) - Main orchestrator
- Coordinates all alert operations
- Manages worker lifecycle (starts/stops workers)
- Implements OnStart and OnStop methods
- Registered as Singleton in dependency injection
- Dependencies: IAlertDetectionService, IEmailService, ILogger<AlertingService>

### Background Workers

**1. AlertDetectionWorker** - Detects new alerts
- Polls database every 30 seconds for new alerts from monitored systems
- Inherits from BackgroundService
- Implements ExecuteAsync with continuous polling loop
- Handles exceptions gracefully without crashing
- Supports cancellation tokens for clean shutdown
- Logs all detection activities

**2. EmailSenderWorker** - Sends notification emails
- Polls email queue every 10 seconds
- Processes queued emails and sends via SMTP
- Retries failed sends up to 3 times with exponential backoff (1s, 2s, 4s)
- Marks emails as sent or failed in database
- Supports cancellation for graceful shutdown

### API Endpoints

**POST /api/alerts** - Create new alert
- Request: {severity, message, assignedToUserId (optional)}
- Authentication: Required (Bearer token)
- Authorization: Any authenticated user
- Validation: Severity enum (Info/Warning/Error), Message max 500 chars
- Response 201: Created alert object with generated Id
- Response 400: Validation errors
- Response 401: Unauthorized

**GET /api/alerts** - List all alerts with pagination
- Authentication: Required
- Query params: page (default 1), limit (default 20, max 100), severity (filter), status (filter)
- Response 200: {data: Alert[], pagination: {page, limit, total_items, total_pages}}
- Response 401: Unauthorized

**PATCH /api/alerts/{id}/resolve** - Mark alert as resolved
- Path param: id (UUID)
- Request body: {resolutionNotes (optional)}
- Authentication: Required
- Authorization: Only assigned user or admin
- Updates ResolvedAt timestamp, prevents further modifications
- Response 200: Updated alert
- Response 404: Alert not found
- Response 403: Forbidden (not assigned user or admin)
- Response 422: Alert already resolved (immutable)

### Data Access Layer

**AlertRepository** - CRUD operations for alerts
- Interface: IAlertRepository
- Implementation: Dapper for queries (fast, lightweight)
- Entity: Alert
- Table: dbo.Alerts
- Methods:
  - GetById(id) → Alert or null
  - GetAll(page, limit, filters) → List<Alert> with pagination
  - Create(alert) → Alert with generated Id
  - Update(alert) → Updated alert
  - MarkAsResolved(id, notes) → Updated alert
- Must use parameterized queries (prevent SQL injection)
- All methods async (Task<T> return types)

**UserRepository** - User lookups for alert assignment
- Interface: IUserRepository
- Implementation: Dapper
- Methods: GetById(id), GetByEmail(email)
- Used for validating assignedToUserId in alert creation

### Database Schema

**Alert table** (dbo.Alerts)
- Id (UUID, primary key, auto-generated)
- Severity (enum: Info/Warning/Error, required)
- Message (string, max 500 chars, required)
- AssignedToUserId (UUID, foreign key to dbo.Users, nullable)
- CreatedAt (datetime, auto-generated, required)
- ResolvedAt (datetime, nullable, set when alert resolved)
- ResolutionNotes (string, max 1000 chars, nullable)

Indexes:
- Primary key on Id
- Index on Severity (for filtering)
- Index on CreatedAt (for sorting)
- Index on AssignedToUserId (for user alert queries)

### Configuration

appsettings.json must contain:
- **ConnectionStrings.OmniWatchDb** (required) - SQL Server connection string
- **AlertDetection.PollingIntervalSeconds** (default: 30, range: 10-300)
- **EmailSender.PollingIntervalSeconds** (default: 10, range: 5-60)
- **EmailSender.MaxRetries** (default: 3, range: 1-5)
- **Smtp.Host** (required) - SMTP server
- **Smtp.Port** (default: 587)
- **Smtp.Username** (required)
- **Smtp.Password** (required, encrypted in production)
- **Logging.LogLevel.Default** (default: Information)

### Logging

Configure Serilog with three sinks:
1. **File sink:** logs/omniwatch-.txt
   - Daily rolling interval
   - Retain 30 days
   - JSON structured format

2. **Windows Event Log:**
   - Source: "OmniWatch Service"
   - Log name: "Application"
   - Minimum level: Warning

3. **Database sink:**
   - Table: dbo.Logs
   - Structured logging with timestamp, level, message, properties
   - Async writes (non-blocking)

### Business Rules

1. **Alert Severity Validation:** Alert severity must be Info, Warning, or Error (no other values allowed). Enforced by enum validation.

2. **Alert Message Length:** Alert message maximum 500 characters. Enforced by string length validation.

3. **Resolved Alert Immutability:** Resolved alerts cannot be modified (ResolvedAt != null means immutable). Any update attempt to resolved alert returns 422 Unprocessable Entity.

4. **Assignment Validation:** If assignedToUserId provided, user must exist in database. Enforced by foreign key constraint and validation.

### Performance Requirements

- **Service startup:** Must start within 5 seconds from OnStart call to Running state
- **API response time:** p95 latency < 500ms for all endpoints
- **Worker polling:** Each poll iteration completes within 2 seconds
- **Database queries:** Indexed queries < 100ms, non-indexed < 500ms
- **Concurrent users:** Support 100 concurrent API requests without degradation

### Security Requirements

- **SQL Injection Prevention:** All database queries use parameterized queries (no string concatenation)
- **Password Security:** Never store plaintext passwords (BCrypt hashing required)
- **Authentication:** All API endpoints require valid Bearer token
- **Authorization:** PATCH /api/alerts/{id}/resolve requires assigned user or admin role

### Reliability Requirements

- **Worker exception handling:** Workers survive exceptions without crashing service
- **Graceful shutdown:** Service and workers stop within 10 seconds when shutdown requested
- **Database connection resilience:** Workers handle temporary database outages (retry with exponential backoff)
- **Email send resilience:** Failed emails retried 3 times before marking as failed

## Definition of Done

- [ ] AlertingService implemented with lifecycle management
- [ ] AlertDetectionWorker and EmailSenderWorker functional
- [ ] All 3 API endpoints implemented
- [ ] AlertRepository and UserRepository with all methods
- [ ] Alert entity with all fields and constraints
- [ ] All configuration keys loaded correctly
- [ ] Serilog configured with all 3 sinks
- [ ] All business rules enforced
- [ ] All performance targets met
- [ ] All security requirements implemented
- [ ] Tests written for all acceptance criteria
- [ ] Code coverage >95%
