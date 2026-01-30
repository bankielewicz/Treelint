---
id: STORY-TEST-002
title: Alert Management Service with Multiple Workers
status: Backlog
points: 8
priority: High
created: 2025-11-07
---

# Story: Alert Management Service with Multiple Workers

## User Story

**As a** system operator,
**I want** a coordinated alert management service that manages multiple background workers,
**so that** alerts are detected, processed, and notifications sent reliably.

## Acceptance Criteria

### 1. [x] Service Manages Worker Lifecycle

**Given** the AlertingService is started
**When** OnStart is called
**Then** all managed workers (AlertDetectionWorker, EmailSenderWorker) start
**And** service transitions to Running state

### 2. [x] Workers Poll Independently

**Given** both workers are running
**When** their respective polling intervals elapse
**Then** AlertDetectionWorker polls every 30 seconds
**And** EmailSenderWorker polls every 10 seconds

### 3. [x] Logging Configured

**Given** the service and workers are running
**When** any component logs an event
**Then** log entries are written to file, EventLog, and database

## Technical Specification

### Architecture

AlertingService (hosted service) coordinates the alert detection and email sending workers.
It implements IHostedService with OnStart and OnStop methods to manage worker lifecycle.

The service is registered as a Singleton and depends on IAlertDetectionService and IEmailService.

### Workers

1. **AlertDetectionWorker** - Polls database every 30 seconds for new alerts
   - Inherits from BackgroundService
   - Implements ExecuteAsync with continuous polling loop
   - Must handle database connection failures gracefully

2. **EmailSenderWorker** - Polls email queue every 10 seconds to send notifications
   - Inherits from BackgroundService
   - Processes queued emails and sends via SMTP
   - Retries failed sends up to 3 times with exponential backoff

### Configuration

appsettings.json contains:
- **AlertDetection.PollingIntervalSeconds** (default: 30, range: 10-300)
- **EmailSender.PollingIntervalSeconds** (default: 10, range: 5-60)
- **ConnectionStrings.OmniWatchDb** (required, SQL Server connection string)
- **Smtp.Host** (required, SMTP server hostname)
- **Smtp.Port** (default: 587)
- **Smtp.Username** (required for authentication)
- **Smtp.Password** (required, stored encrypted)

### Logging

Configure Serilog with three sinks:
- **File sink:** logs/omniwatch-.txt with daily rolling interval, retain 30 days
- **Windows Event Log:** source name "OmniWatch Service", log name "Application"
- **Database sink:** table dbo.Logs with structured logging

Minimum log level: Information (configurable via appsettings.json Logging.LogLevel.Default)

### Non-Functional Requirements

- Service startup time must be under 5 seconds (from OnStart call to Running state)
- Workers must start within 2 seconds of service start
- Log writes should not block worker execution (async logging)
- Service must handle worker exceptions without crashing (worker failure isolation)

## Definition of Done

- [ ] AlertingService implemented with lifecycle methods
- [ ] AlertDetectionWorker polling loop functional
- [ ] EmailSenderWorker email sending operational
- [ ] All configuration keys loaded from appsettings.json
- [ ] Serilog configured with all 3 sinks
- [ ] Tests written for all acceptance criteria
- [ ] Code coverage >95%
