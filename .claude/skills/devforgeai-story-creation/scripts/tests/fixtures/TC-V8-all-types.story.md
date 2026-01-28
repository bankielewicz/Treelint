---
id: STORY-V8-TEST
title: Story with All 7 Component Types
status: Backlog
format_version: "2.0"
---

# Story: All Component Types Test

## User Story

**As a** developer,
**I want** to test validation with all 7 component types,
**so that** the validator handles complete stories.

## Acceptance Criteria

**Given** a story with Service, Worker, Configuration, Logging, Repository, API, and DataModel
**When** the validator runs
**Then** it should pass validation for all component types

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"

  components:
    - type: "Service"
      name: "AuthService"
      file_path: "src/Services/AuthService.cs"
      interface: "IAuthService"
      requirements:
        - id: "SVC-001"
          description: "JWT token authentication"
          testable: true
          test_requirement: "Test: Verify JWT token validation accepts valid tokens"
          priority: "Critical"

    - type: "Worker"
      name: "EmailWorker"
      file_path: "src/Workers/EmailWorker.cs"
      interface: "BackgroundService"
      polling_interval_ms: 30000
      requirements:
        - id: "WKR-001"
          description: "Process email queue"
          testable: true
          test_requirement: "Test: Verify emails sent from queue at 30s intervals"
          priority: "High"

    - type: "Configuration"
      name: "appsettings.json"
      file_path: "src/appsettings.json"
      required_keys:
        - key: "ConnectionStrings.DefaultConnection"
          type: "string"
          required: true
          test_requirement: "Test: Configuration loads connection string"

    - type: "Logging"
      name: "Serilog"
      file_path: "src/Program.cs"
      sinks:
        - name: "File"
          path: "logs/app-.txt"
          test_requirement: "Test: Log file created in logs/ directory"

    - type: "Repository"
      name: "UserRepository"
      file_path: "src/Infrastructure/Repositories/UserRepository.cs"
      interface: "IUserRepository"
      data_access: "Dapper"
      requirements:
        - id: "REPO-001"
          description: "CRUD operations for users"
          testable: true
          test_requirement: "Test: Verify GetById, Create, Update, Delete methods functional"
          priority: "High"

    - type: "API"
      name: "GetUsers"
      endpoint: "/api/users"
      method: "GET"
      authentication:
        required: true
        method: "Bearer Token"
      requirements:
        - id: "API-001"
          description: "List users with pagination"
          testable: true
          test_requirement: "Test: GET /api/users returns 200 with user list"
          priority: "High"

    - type: "DataModel"
      name: "User"
      table: "dbo.Users"
      purpose: "User entity for authentication"
      fields:
        - name: "Id"
          type: "UUID"
          constraints: "Primary Key"
          description: "User identifier"
        - name: "Email"
          type: "String(255)"
          constraints: "Required, Unique"
          description: "User email"
          test_requirement: "Test: Email uniqueness enforced by database constraint"
```

## Definition of Done

- [x] All 7 component types present
- [x] Validator should accept all types
