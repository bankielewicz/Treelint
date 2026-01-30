# API Documentation - {{project_name}}

Complete API reference for {{project_name}}.

---

## Table of Contents

- [Overview](#overview)
- [Authentication](#authentication)
- [Endpoints](#endpoints)
- [Data Models](#data-models)
- [Error Codes](#error-codes)
- [Rate Limiting](#rate-limiting)
- [Examples](#examples)

---

## Overview

**Base URL:** `{{base_url}}`

**API Version:** `{{api_version}}`

**Content Type:** `application/json`

---

## Authentication

{{authentication_description}}

### Authentication Methods

{{authentication_methods}}

### Example

```http
{{authentication_example}}
```

---

## Endpoints

{{api_endpoints_list}}

---

## Data Models

### {{model_name_1}}

```json
{{model_schema_1}}
```

**Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
{{model_fields_1}}

---

## Error Codes

| Code | Message | Description |
|------|---------|-------------|
{{error_codes_table}}

---

## Rate Limiting

{{rate_limiting_description}}

---

## Examples

### {{example_workflow_1}}

```http
{{example_request_1}}
```

Response:
```json
{{example_response_1}}
```

---

**Last Updated:** {{last_updated}}
**API Version:** {{api_version}}
