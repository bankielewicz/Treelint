---
paths: "src/api/**/*.ts, src/api/**/*.py, **/routes/**/*"
description: API endpoint development rules
version: "1.0"
created: 2025-12-10
---

# API Development Rules

These rules apply to API endpoint files.

## Input Validation
- Validate ALL user input
- Use schema validation (Pydantic, Zod, Joi)
- Return 400 for invalid input with details

## Error Handling
- Use consistent error response format
- Never expose internal errors to clients
- Log errors with correlation IDs

## Security
- Parameterized queries only (no SQL concatenation)
- Rate limiting on all endpoints
- Authentication required (unless explicitly public)

## Documentation
- OpenAPI/Swagger annotations
- Request/response examples
- Error response documentation

## Testing
- 95% coverage minimum
- Test success and failure paths
- Test authentication/authorization
