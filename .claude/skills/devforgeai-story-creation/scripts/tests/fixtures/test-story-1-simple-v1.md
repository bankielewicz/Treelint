---
id: STORY-TEST-001
title: Simple Worker and Configuration Test
status: Backlog
points: 3
priority: Medium
created: 2025-11-07
---

# Story: Simple Worker and Configuration Test

## User Story

**As a** system administrator,
**I want** a background worker that polls for new alerts periodically,
**so that** the system can detect and process alerts automatically without manual intervention.

## Acceptance Criteria

### 1. [x] Worker Polls Database Periodically

**Given** the AlertDetectionWorker is running
**When** 30 seconds have elapsed since the last poll
**Then** the worker queries the database for new alerts
**And** processes any alerts found

---

### 2. [x] Worker Handles Exceptions Gracefully

**Given** the AlertDetectionWorker encounters an exception during polling
**When** the exception occurs
**Then** the worker logs the error
**And** continues polling without crashing
**And** the next poll attempt occurs after the configured interval

---

### 3. [x] Polling Interval Configurable

**Given** the system configuration file
**When** PollingIntervalSeconds is set to a custom value
**Then** the worker uses that interval for polling
**And** defaults to 30 seconds if not specified

---

## Technical Specification

### Service Implementation

AlertDetectionWorker will poll the database every 30 seconds for new alerts.
It should inherit from BackgroundService and implement ExecuteAsync method.
The worker must handle exceptions gracefully and support cancellation tokens for clean shutdown.

### Configuration

appsettings.json should contain:
- PollingIntervalSeconds (integer, default: 30, range: 10-300)
- ConnectionStrings.OmniWatchDb (required, connection string for database)

---

## Non-Functional Requirements

### Performance

- Worker polling should complete within 2 seconds
- Startup time should be under 1 second

### Reliability

- Worker must survive exceptions without crashing
- Graceful shutdown within 5 seconds when cancellation requested

---

## Definition of Done

- [ ] AlertDetectionWorker implemented
- [ ] ExecuteAsync method with polling loop
- [ ] Exception handling implemented
- [ ] Cancellation token support
- [ ] Configuration loading from appsettings.json
- [ ] Tests written for all acceptance criteria
- [ ] Code coverage >95%
