# Structured Technical Specification Format v2.0

**Version:** 2.0
**Status:** Production Ready
**Purpose:** Machine-readable technical specifications for DevForgeAI stories
**Replaces:** Freeform text format (v1.0)

---

## Overview

This document defines the structured YAML format for Technical Specifications in DevForgeAI user stories, enabling deterministic parsing, automated validation, and comprehensive test generation.

**Key Benefits:**
- ✅ **Machine-readable:** YAML parser can extract components programmatically
- ✅ **100% testable:** Every component has explicit test requirements
- ✅ **No ambiguity:** Structured schema eliminates parsing uncertainty
- ✅ **Validation-ready:** Enables automated implementation validation (Phase 3)

---

## Format Version History

| Version | Date | Description | Status |
|---------|------|-------------|--------|
| **1.0** | 2025-10-30 | Freeform markdown text | Deprecated (backward compatible) |
| **2.0** | 2025-11-07 | Structured YAML with test requirements | **Current** |

**Migration path:** `migrate_story_v1_to_v2.py` converts v1.0 → v2.0

---

## Schema Definition

### Top-Level Structure

```yaml
technical_specification:
  format_version: "2.0"

  components:
    - [Component definitions - see types below]

  business_rules:
    - [Business rule definitions]

  non_functional_requirements:
    - [NFR definitions]
```

---

## Component Types (7 Types)

### 1. Service Component

**Purpose:** Backend services, workers, hosted services

**Required Fields:**
- `type`: "Service"
- `name`: Service class name
- `file_path`: Relative path from project root
- `requirements`: Array of testable requirements

**Optional Fields:**
- `interface`: Interface implemented (e.g., `IHostedService`)
- `dependencies`: Array of injected dependencies
- `lifecycle`: Service lifecycle (Singleton, Scoped, Transient)

**Schema:**
```yaml
- type: "Service"
  name: "AlertingService"
  file_path: "src/OmniWatch.Service/Services/AlertingService.cs"
  interface: "IAlertingService"
  lifecycle: "Singleton"
  dependencies:
    - "IAlertDetectionService"
    - "IEmailService"
    - "ILogger<AlertingService>"
  requirements:
    - id: "SVC-001"
      description: "Must implement ServiceBase with OnStart/OnStop methods"
      testable: true
      test_requirement: "Test: Verify OnStart transitions service to Running state"
      priority: "Critical"
    - id: "SVC-002"
      description: "Must coordinate worker lifecycle (start/stop workers)"
      testable: true
      test_requirement: "Test: Verify workers start when service starts, stop when service stops"
      priority: "High"
```

---

### 2. Worker Component

**Purpose:** Background workers, polling loops, scheduled tasks

**Required Fields:**
- `type`: "Worker"
- `name`: Worker class name
- `file_path`: Relative path
- `requirements`: Array of testable requirements

**Optional Fields:**
- `interface`: Base class (e.g., `BackgroundService`)
- `dependencies`: Injected dependencies
- `polling_interval_ms`: Polling interval (if applicable)

**Schema:**
```yaml
- type: "Worker"
  name: "AlertDetectionWorker"
  file_path: "src/OmniWatch.Service/Workers/AlertDetectionWorker.cs"
  interface: "BackgroundService"
  polling_interval_ms: 30000
  dependencies:
    - "IAlertDetectionService"
    - "ILogger<AlertDetectionWorker>"
  requirements:
    - id: "WKR-001"
      description: "Must run continuous polling loop with cancellation token support"
      testable: true
      test_requirement: "Test: Worker polls at 30s intervals until CancellationToken signals stop"
      priority: "Critical"
    - id: "WKR-002"
      description: "Must handle exceptions without stopping worker"
      testable: true
      test_requirement: "Test: Exception in poll iteration doesn't crash worker, logs error, continues"
      priority: "High"
    - id: "WKR-003"
      description: "Must respect graceful shutdown (stop polling within 5s)"
      testable: true
      test_requirement: "Test: StopAsync completes within 5 seconds, cancellation token honored"
      priority: "Medium"
```

---

### 3. Configuration Component

**Purpose:** Application configuration, settings files (appsettings.json, .env)

**Required Fields:**
- `type`: "Configuration"
- `name`: Configuration file name
- `file_path`: Relative path
- `required_keys`: Array of configuration keys

**Schema:**
```yaml
- type: "Configuration"
  name: "appsettings.json"
  file_path: "src/OmniWatch.Service/appsettings.json"
  required_keys:
    - key: "ConnectionStrings.OmniWatchDb"
      type: "string"
      example: "Server=localhost;Database=OmniWatch;Trusted_Connection=true;"
      required: true
      test_requirement: "Test: Configuration loads ConnectionStrings.OmniWatchDb without exception"
    - key: "AlertingService.PollingIntervalSeconds"
      type: "int"
      default: 30
      validation: "Range 10-300"
      required: false
      test_requirement: "Test: PollingIntervalSeconds default is 30 when not specified"
    - key: "Logging.LogLevel.Default"
      type: "string"
      default: "Information"
      validation: "Enum: Trace, Debug, Information, Warning, Error, Critical"
      required: false
      test_requirement: "Test: Default log level is Information"
```

---

### 4. Logging Component

**Purpose:** Logging configuration (Serilog, NLog, log4net)

**Required Fields:**
- `type`: "Logging"
- `name`: Logging framework name
- `file_path`: Configuration location
- `sinks`: Array of log sinks

**Schema:**
```yaml
- type: "Logging"
  name: "Serilog"
  file_path: "src/OmniWatch.Service/Program.cs"
  sinks:
    - name: "File"
      path: "logs/omniwatch-.txt"
      rolling_interval: "Day"
      retention_days: 30
      test_requirement: "Test: Log file created in logs/ directory with date suffix"
    - name: "EventLog"
      source: "OmniWatch Service"
      log_name: "Application"
      test_requirement: "Test: Entry written to Windows Event Log under 'OmniWatch Service' source"
    - name: "Database"
      table: "Logs"
      connection_string_key: "ConnectionStrings.OmniWatchDb"
      test_requirement: "Test: Log entry written to Logs table with correct schema"
```

---

### 5. Repository Component

**Purpose:** Data access layer, repositories, database interactions

**Required Fields:**
- `type`: "Repository"
- `name`: Repository class name
- `file_path`: Relative path
- `interface`: Repository interface
- `data_access`: Data access technology (Dapper, EF Core, Prisma)
- `requirements`: Array of testable requirements

**Schema:**
```yaml
- type: "Repository"
  name: "AlertRepository"
  file_path: "src/OmniWatch.Infrastructure/Repositories/AlertRepository.cs"
  interface: "IAlertRepository"
  data_access: "Dapper"
  entity: "Alert"
  table: "dbo.Alerts"
  requirements:
    - id: "REPO-001"
      description: "Must use parameterized queries to prevent SQL injection"
      testable: true
      test_requirement: "Test: Query uses @parameters, not string concatenation"
      priority: "Critical"
    - id: "REPO-002"
      description: "Must implement GetById, GetAll, Create, Update, Delete methods"
      testable: true
      test_requirement: "Test: All CRUD methods functional and return correct types"
      priority: "High"
    - id: "REPO-003"
      description: "Must handle database connection failures gracefully"
      testable: true
      test_requirement: "Test: Connection failure returns Result.Failure, not exception"
      priority: "Medium"
```

---

### 6. API Endpoint Component

**Purpose:** REST/GraphQL/gRPC API endpoints

**Required Fields:**
- `type`: "API"
- `name`: Endpoint name
- `endpoint`: URL path
- `method`: HTTP method
- `request`: Request schema
- `response`: Response schema

**Optional Fields:**
- `authentication`: Auth requirements
- `validation`: Request validation rules
- `error_responses`: Error response schemas

**Schema:**
```yaml
- type: "API"
  name: "CreateUser"
  endpoint: "/api/users"
  method: "POST"
  authentication:
    required: true
    method: "Bearer Token"
    scopes: ["admin:write"]
  request:
    content_type: "application/json"
    schema:
      email:
        type: "string"
        required: true
        validation: "Email format, max 255 chars, unique"
      password:
        type: "string"
        required: true
        validation: "Min 8 chars, must include uppercase, lowercase, number, special char"
      name:
        type: "string"
        required: true
        validation: "2-100 chars"
      role:
        type: "enum"
        required: true
        values: ["customer", "admin", "moderator"]
  response:
    success:
      status_code: 201
      schema:
        id: "UUID"
        email: "string"
        name: "string"
        role: "string"
        created_at: "ISO8601 DateTime"
    errors:
      - status_code: 400
        condition: "Validation failed"
        schema:
          error: "Validation failed"
          details: "Array of field errors"
      - status_code: 401
        condition: "Invalid token"
        schema:
          error: "Unauthorized"
          message: "Invalid or expired token"
  requirements:
    - id: "API-001"
      description: "Must validate email format and uniqueness before creation"
      testable: true
      test_requirement: "Test: Duplicate email returns 400 with 'Email already exists' error"
      priority: "Critical"
    - id: "API-002"
      description: "Must hash password before storage (never store plaintext)"
      testable: true
      test_requirement: "Test: Database contains hashed password, not plaintext"
      priority: "Critical"
```

---

### 7. DataModel Component

**Purpose:** Database entities, DTOs, domain objects

**Required Fields:**
- `type`: "DataModel"
- `name`: Entity/DTO name
- `table`: Database table name (if entity)
- `fields`: Array of field definitions

**Optional Fields:**
- `indexes`: Database indexes
- `relationships`: Foreign key relationships
- `constraints`: Database constraints

**Schema:**
```yaml
- type: "DataModel"
  name: "Alert"
  table: "dbo.Alerts"
  purpose: "Represents an alert instance with severity, message, and timestamps"
  fields:
    - name: "Id"
      type: "UUID"
      constraints: "Primary Key, Auto-generated"
      description: "Unique alert identifier"
    - name: "Severity"
      type: "Enum"
      constraints: "Required, Values: Info|Warning|Error"
      description: "Alert severity level"
      test_requirement: "Test: Invalid severity throws ArgumentException"
    - name: "Message"
      type: "String(500)"
      constraints: "Required, Max 500 chars"
      description: "Alert message text"
      test_requirement: "Test: 501-char message throws ValidationException"
    - name: "CreatedAt"
      type: "DateTime"
      constraints: "Required, Auto-generated"
      description: "Alert creation timestamp"
    - name: "ResolvedAt"
      type: "DateTime"
      constraints: "Optional, Nullable"
      description: "Alert resolution timestamp (null until resolved)"
  indexes:
    - name: "IX_Alerts_Severity"
      fields: ["Severity"]
      unique: false
      purpose: "Fast filtering by severity"
    - name: "IX_Alerts_CreatedAt"
      fields: ["CreatedAt"]
      unique: false
      purpose: "Fast sorting by date"
  relationships:
    - type: "Many-to-One"
      related_entity: "User"
      foreign_key: "AssignedToUserId"
      cascade: "Restrict"
      description: "Alert assigned to user"
  constraints:
    - type: "Check"
      expression: "LEN(Message) <= 500"
      description: "Enforce message length limit"
```

---

## Business Rules Schema

**Required Fields:**
- `id`: Unique business rule identifier
- `rule`: Business rule description
- `validation`: How to validate compliance
- `test_requirement`: How to test this rule

**Optional Fields:**
- `trigger`: When rule is evaluated
- `error_handling`: What happens on violation

**Schema:**
```yaml
business_rules:
  - id: "BR-001"
    rule: "Alert severity must be Info, Warning, or Error (no other values allowed)"
    trigger: "When Alert is created or updated"
    validation: "Enum validation in Alert.Severity property"
    error_handling: "Throw ArgumentException with message 'Invalid severity: {value}'"
    test_requirement: "Test: Creating alert with severity 'Debug' throws ArgumentException"
    priority: "Critical"

  - id: "BR-002"
    rule: "Alert message maximum 500 characters"
    trigger: "When Alert.Message is set"
    validation: "String length check in Alert.SetMessage method"
    error_handling: "Throw ValidationException with message 'Message exceeds 500 characters'"
    test_requirement: "Test: 501-character message throws ValidationException"
    priority: "High"

  - id: "BR-003"
    rule: "Resolved alerts cannot be modified (immutable after resolution)"
    trigger: "When attempting to update alert with ResolvedAt != null"
    validation: "Check ResolvedAt in Update method"
    error_handling: "Return Result.Failure('Cannot modify resolved alert')"
    test_requirement: "Test: Updating resolved alert returns failure result"
    priority: "Medium"
```

---

## Non-Functional Requirements Schema

**Required Fields:**
- `id`: Unique NFR identifier
- `requirement`: NFR description
- `metric`: Measurable target
- `test_requirement`: How to verify

**Optional Fields:**
- `category`: Performance, Security, Scalability, Reliability
- `priority`: Critical, High, Medium, Low

**Schema:**
```yaml
non_functional_requirements:
  - id: "NFR-001"
    category: "Performance"
    requirement: "Service starts within 5 seconds"
    metric: "Startup time < 5s measured from OnStart call to Running state"
    test_requirement: "Test: Measure startup time with Stopwatch, assert < 5 seconds"
    priority: "High"

  - id: "NFR-002"
    category: "Performance"
    requirement: "Worker polling interval configurable"
    metric: "Default 30s, configurable range 10-300s"
    test_requirement: "Test: Worker respects configured interval from appsettings.json"
    priority: "Medium"

  - id: "NFR-003"
    category: "Reliability"
    requirement: "Worker survives exceptions without crashing"
    metric: "Exception recovery time < 100ms, worker continues polling"
    test_requirement: "Test: Throw exception in ExecuteAsync, verify worker continues next iteration"
    priority: "Critical"

  - id: "NFR-004"
    category: "Security"
    requirement: "Database queries use parameterized queries only"
    metric: "Zero string concatenation in SQL queries"
    test_requirement: "Test: Code analysis detects no SQL string concatenation"
    priority: "Critical"
```

---

## Complete Example: Windows Service Story

### v2.0 Structured Format

```yaml
technical_specification:
  format_version: "2.0"

  components:
    - type: "Service"
      name: "AlertingService"
      file_path: "src/OmniWatch.Service/Services/AlertingService.cs"
      interface: "IAlertingService"
      lifecycle: "Singleton"
      dependencies:
        - "IAlertDetectionService"
        - "IEmailService"
      requirements:
        - id: "SVC-001"
          description: "Must implement ServiceBase with OnStart/OnStop"
          testable: true
          test_requirement: "Test: OnStart transitions to Running state"
          priority: "Critical"
        - id: "SVC-002"
          description: "Must coordinate worker lifecycle"
          testable: true
          test_requirement: "Test: Workers start when service starts"
          priority: "High"

    - type: "Worker"
      name: "AlertDetectionWorker"
      file_path: "src/OmniWatch.Service/Workers/AlertDetectionWorker.cs"
      interface: "BackgroundService"
      polling_interval_ms: 30000
      dependencies:
        - "IAlertDetectionService"
      requirements:
        - id: "WKR-001"
          description: "Must run continuous polling loop with cancellation"
          testable: true
          test_requirement: "Test: Worker polls at 30s intervals until cancellation"
          priority: "Critical"
        - id: "WKR-002"
          description: "Must handle exceptions without stopping"
          testable: true
          test_requirement: "Test: Exception in poll doesn't crash worker"
          priority: "High"

    - type: "Configuration"
      name: "appsettings.json"
      file_path: "src/OmniWatch.Service/appsettings.json"
      required_keys:
        - key: "ConnectionStrings.OmniWatchDb"
          type: "string"
          example: "Server=localhost;Database=OmniWatch;Trusted_Connection=true;"
          required: true
          test_requirement: "Test: Configuration loads ConnectionStrings.OmniWatchDb"
        - key: "AlertingService.PollingIntervalSeconds"
          type: "int"
          default: 30
          validation: "Range 10-300"
          required: false
          test_requirement: "Test: PollingIntervalSeconds default is 30"

    - type: "Logging"
      name: "Serilog"
      file_path: "src/OmniWatch.Service/Program.cs"
      sinks:
        - name: "File"
          path: "logs/omniwatch-.txt"
          rolling_interval: "Day"
          test_requirement: "Test: Log file created in logs/ with date suffix"
        - name: "EventLog"
          source: "OmniWatch Service"
          test_requirement: "Test: Entry written to Windows Event Log"
        - name: "Database"
          table: "Logs"
          test_requirement: "Test: Log entry written to Logs table"

    - type: "Repository"
      name: "AlertRepository"
      file_path: "src/OmniWatch.Infrastructure/Repositories/AlertRepository.cs"
      interface: "IAlertRepository"
      data_access: "Dapper"
      entity: "Alert"
      table: "dbo.Alerts"
      requirements:
        - id: "REPO-001"
          description: "Must use parameterized queries (prevent SQL injection)"
          testable: true
          test_requirement: "Test: Query uses @parameters, not string concatenation"
          priority: "Critical"

  business_rules:
    - id: "BR-001"
      rule: "Alert severity must be Info, Warning, or Error"
      validation: "Enum validation in AlertSeverity"
      test_requirement: "Test: Invalid severity throws ArgumentException"
      priority: "Critical"

    - id: "BR-002"
      rule: "Alert message maximum 500 characters"
      validation: "String length check in Alert.SetMessage"
      test_requirement: "Test: 501-char message throws ValidationException"
      priority: "High"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "Service starts within 5 seconds"
      metric: "Startup time < 5s"
      test_requirement: "Test: Measure startup time, assert < 5 seconds"
      priority: "High"

    - id: "NFR-002"
      category: "Performance"
      requirement: "Worker polling interval configurable"
      metric: "Default 30s, configurable 10-300s"
      test_requirement: "Test: Worker respects configured interval"
      priority: "Medium"
```

---

## Validation Rules

### Component Validation

**All components must have:**
- ✅ `type` field (one of 7 valid types)
- ✅ `name` field (non-empty string)
- ✅ `file_path` field (relative path from project root)
- ✅ Type-specific required fields (see schemas above)

**Requirements validation:**
- ✅ Each requirement has `id`, `description`, `testable`, `test_requirement`
- ✅ IDs unique within component
- ✅ `testable` is boolean (true/false)
- ✅ `test_requirement` starts with "Test: " prefix

### Business Rule Validation

- ✅ `id` unique within story
- ✅ `rule` is non-empty string
- ✅ `test_requirement` provided
- ✅ `validation` describes how to validate

### NFR Validation

- ✅ `id` unique within story
- ✅ `metric` is measurable (contains number or threshold)
- ✅ `test_requirement` describes verification method
- ✅ `category` is one of: Performance, Security, Scalability, Reliability

---

## Migration Examples

### Example 1: Worker Component (v1.0 → v2.0)

**v1.0 (Freeform):**
```markdown
### Service Implementation Pattern

AlertDetectionWorker will poll the database every 30 seconds for new alerts. It should inherit from BackgroundService and implement ExecuteAsync. The worker must handle exceptions gracefully and support cancellation tokens for clean shutdown.
```

**v2.0 (Structured):**
```yaml
- type: "Worker"
  name: "AlertDetectionWorker"
  file_path: "src/OmniWatch.Service/Workers/AlertDetectionWorker.cs"
  interface: "BackgroundService"
  polling_interval_ms: 30000
  requirements:
    - id: "WKR-001"
      description: "Must run continuous polling loop with cancellation"
      testable: true
      test_requirement: "Test: Worker polls at 30s intervals until cancellation"
      priority: "Critical"
    - id: "WKR-002"
      description: "Must handle exceptions without stopping"
      testable: true
      test_requirement: "Test: Exception in poll doesn't crash worker"
      priority: "High"
```

**Transformation:**
- Text description → structured component
- Implicit requirements → explicit requirements with IDs
- No test info → explicit test requirements
- Ambiguous "handle exceptions" → testable requirement with specific test

---

### Example 2: Configuration (v1.0 → v2.0)

**v1.0 (Freeform):**
```markdown
### Configuration

appsettings.json should contain ConnectionStrings.OmniWatchDb for database access and AlertingService.PollingIntervalSeconds (default 30) for worker configuration.
```

**v2.0 (Structured):**
```yaml
- type: "Configuration"
  name: "appsettings.json"
  file_path: "src/OmniWatch.Service/appsettings.json"
  required_keys:
    - key: "ConnectionStrings.OmniWatchDb"
      type: "string"
      required: true
      test_requirement: "Test: Configuration loads ConnectionStrings.OmniWatchDb"
    - key: "AlertingService.PollingIntervalSeconds"
      type: "int"
      default: 30
      required: false
      test_requirement: "Test: PollingIntervalSeconds default is 30"
```

**Transformation:**
- Freeform sentence → structured config keys
- Implicit types → explicit type declarations
- Default values extracted → explicit default field
- No test info → test requirements per key

---

## Parsing Algorithm

### Extracting Tech Spec from Story File

```python
import yaml
import re

def extract_tech_spec(story_content: str) -> dict:
    """
    Extract structured technical specification from story file.

    Args:
        story_content: Complete story file content

    Returns:
        Parsed YAML as dictionary

    Raises:
        ValueError: If tech spec section not found or invalid YAML
    """
    # Find tech spec section
    match = re.search(
        r"## Technical Specification\s+```yaml\s+(.*?)\s+```",
        story_content,
        re.DOTALL
    )

    if not match:
        raise ValueError("Technical Specification section not found")

    yaml_content = match.group(1)

    try:
        tech_spec = yaml.safe_load(yaml_content)
    except yaml.YAMLError as e:
        raise ValueError(f"Invalid YAML in tech spec: {e}")

    # Validate format version
    if tech_spec.get("technical_specification", {}).get("format_version") != "2.0":
        raise ValueError("Format version must be 2.0")

    return tech_spec
```

---

## Test Requirement Format Standards

### Standard Format

All test requirements must follow this format:

```
Test: [Action] [Expected Outcome]
```

**Examples:**
- ✅ "Test: Worker polls at 30s intervals until cancellation"
- ✅ "Test: Invalid severity throws ArgumentException"
- ✅ "Test: Configuration loads ConnectionStrings.OmniWatchDb"
- ❌ "Should test the worker" (vague, not actionable)
- ❌ "Verify polling works" (no specific assertion)

### Test Requirement Categories

**1. Behavior Tests (Action → Outcome):**
```
"Test: [Method] [Input] [Expected Output]"
Example: "Test: CreateUser with duplicate email returns 400 error"
```

**2. Exception Tests (Invalid Input → Exception):**
```
"Test: [Invalid Input] throws [ExceptionType]"
Example: "Test: 501-char message throws ValidationException"
```

**3. State Tests (Action → State Change):**
```
"Test: [Action] transitions [Object] to [State]"
Example: "Test: OnStart transitions service to Running state"
```

**4. Configuration Tests (Load → Verify):**
```
"Test: Configuration loads [Key] without exception"
Example: "Test: Configuration loads ConnectionStrings.OmniWatchDb"
```

---

## Component Type Selection Guide

| If Spec Mentions... | Use Component Type | Example |
|---------------------|-------------------|---------|
| Background task, polling, scheduled job | **Worker** | AlertDetectionWorker |
| Hosted service, lifecycle management | **Service** | AlertingService |
| API endpoint, REST route, GraphQL mutation | **API** | POST /api/users |
| Database table, entity, domain object | **DataModel** | Alert entity |
| Configuration file, appsettings, environment | **Configuration** | appsettings.json |
| Logging setup, log sinks | **Logging** | Serilog configuration |
| Data access, repository, DAO | **Repository** | AlertRepository |

---

## Backward Compatibility (v1.0 Support)

### Dual Format Detection

```python
def detect_format_version(story_content: str) -> str:
    """Detect story format version."""

    # Check for structured YAML format
    if "```yaml\ntechnical_specification:" in story_content:
        return "2.0"

    # Check frontmatter for format_version
    frontmatter_match = re.search(r"---\n(.*?)\n---", story_content, re.DOTALL)
    if frontmatter_match:
        frontmatter = yaml.safe_load(frontmatter_match.group(1))
        if "format_version" in frontmatter:
            return frontmatter["format_version"]

    # Default to v1.0 (freeform)
    return "1.0"
```

### Format-Aware Parsing

```python
def parse_tech_spec(story_content: str):
    """Parse technical specification (v1.0 or v2.0)."""

    version = detect_format_version(story_content)

    if version == "2.0":
        return parse_structured_tech_spec(story_content)
    elif version == "1.0":
        return parse_freeform_tech_spec(story_content)
    else:
        raise ValueError(f"Unsupported format version: {version}")
```

---

## Benefits of v2.0 Format

### For Test Generation (Phase 1 Step 4)

**v1.0 (85% accuracy):**
- Parse freeform text: "Worker should poll database"
- Guess component type: Is this a Worker or Service?
- Infer test requirements: What should we test?
- **Result:** Some components missed, tests incomplete

**v2.0 (95%+ accuracy):**
- Parse YAML: `type: "Worker"`
- Extract requirements: `requirements[0].test_requirement`
- Generate tests: Direct mapping from test_requirement
- **Result:** All components detected, comprehensive tests

---

### For Implementation Validation (Phase 3)

**v1.0 (Not Possible):**
- Cannot validate "Worker should poll database" programmatically
- No way to check if polling loop exists
- Manual verification required

**v2.0 (Automated):**
```python
# Validate Worker component exists
component = tech_spec["components"][0]
assert component["type"] == "Worker"
assert file_exists(component["file_path"])

# Validate requirements met
for req in component["requirements"]:
    validate_requirement(component["file_path"], req)
```

**Result:** Automated validation possible

---

## Migration Strategy

### Pilot Migration (10 Stories)

**Story selection criteria:**
- 3 simple stories (2-3 components)
- 4 medium stories (4-6 components)
- 3 complex stories (8+ components)

**Migration validation:**
1. Run `migrate_story_v1_to_v2.py <story-file>`
2. Run `validate_tech_spec.py <story-file>`
3. Manual review of YAML quality
4. Test with `/dev <STORY-ID>` to verify compatibility

**Success criteria:**
- 100% migration success (10/10 stories)
- 100% validation passing
- /dev works with all migrated stories

---

### Full Migration (All Stories)

**Process:**
1. Backup all stories: `cp devforgeai/specs/Stories/*.md devforgeai/backups/phase2-full/`
2. Count stories: `find devforgeai/specs/Stories -name "*.story.md" | wc -l`
3. Batch migrate: 10 stories at a time
4. Validate each batch: `validate_tech_spec.py`
5. Manual spot-check: 20% of each batch
6. HALT on validation failures

**Rollback capability:**
```bash
# Restore from backup
cp devforgeai/backups/phase2-full/*.md devforgeai/specs/Stories/
```

---

## Format Specification Summary

**Component types:** 7 (Service, Worker, Configuration, Logging, Repository, API, DataModel)

**Required top-level keys:**
- `technical_specification.format_version`: "2.0"
- `technical_specification.components`: Array (1+ components)

**Optional top-level keys:**
- `technical_specification.business_rules`: Array
- `technical_specification.non_functional_requirements`: Array

**Test requirement format:**
- Prefix: "Test: "
- Structure: Action + Expected Outcome
- Specific and measurable

**Validation tooling:**
- Parser: `validate_tech_spec.py`
- Migration: `migrate_story_v1_to_v2.py`

---

**This specification enables deterministic parsing (95%+ accuracy) and provides foundation for automated validation (Phase 3).**
