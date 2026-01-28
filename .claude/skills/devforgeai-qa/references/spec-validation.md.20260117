# Spec Compliance Validation Reference

## Overview

Methods for validating implementation against story specifications and acceptance criteria.

## Acceptance Criteria Validation

### Mapping Criteria to Tests

```python
def validate_acceptance_criteria(story_file, test_path):
    story = parse_story(story_file)
    criteria = story["acceptance_criteria"]

    validation_report = []

    for criterion in criteria:
        # Extract keywords
        keywords = extract_keywords(criterion)
        # Example: "User can add items to cart"
        # Keywords: ["add", "items", "cart"]

        # Find tests
        test_pattern = f"({'|'.join(keywords)})"
        matching_tests = grep(test_pattern, test_path, ignore_case=True)

        if not matching_tests:
            validation_report.append({
                "criterion": criterion,
                "status": "FAIL",
                "issue": "No tests found",
                "action": f"Add test: Test_Can{keywords[0].title()}_{keywords[-1].title()}"
            })
        else:
            # Run tests
            results = run_tests(matching_tests)
            validation_report.append({
                "criterion": criterion,
                "status": "PASS" if results.all_passed else "FAIL",
                "tests": matching_tests,
                "results": results
            })

    return validation_report
```

## API Contract Validation

### Endpoint Validation

```python
def validate_api_endpoint(spec_endpoint, source_path):
    # Spec: POST /api/cart/items
    # Request: { productId: int, quantity: int }
    # Response: { cartId: string, itemCount: int, total: decimal }

    violations = []

    # Check endpoint exists
    endpoint_pattern = f"(Route|@route|app\\.{spec_endpoint.method.lower()}).*{spec_endpoint.path}"
    found = grep(endpoint_pattern, source_path)

    if not found:
        violations.append({
            "severity": "CRITICAL",
            "issue": f"Endpoint not found: {spec_endpoint.method} {spec_endpoint.path}",
            "action": "Implement missing endpoint"
        })
        return violations

    # Validate request model
    controller_file = found[0].file
    actual_request = extract_request_model(controller_file, spec_endpoint.path)
    expected_request = spec_endpoint.request_model

    if not models_match(actual_request, expected_request):
        violations.append({
            "severity": "HIGH",
            "issue": "Request model mismatch",
            "expected": expected_request,
            "actual": actual_request,
            "endpoint": spec_endpoint.path
        })

    # Validate response model
    actual_response = extract_response_model(controller_file, spec_endpoint.path)
    expected_response = spec_endpoint.response_model

    if not models_match(actual_response, expected_response):
        violations.append({
            "severity": "HIGH",
            "issue": "Response model mismatch",
            "expected": expected_response,
            "actual": actual_response
        })

    return violations
```

## Non-Functional Requirements Validation

### Performance Testing

```python
def validate_performance_nfr(nfr_spec, test_path):
    # NFR: "API response time < 200ms for 1000 concurrent users"

    # Check for performance tests
    perf_tests = glob(f"{test_path}/Performance/**/*")

    if not perf_tests:
        return {
            "status": "FAIL",
            "issue": "No performance tests found",
            "action": "Create performance test suite",
            "template": generate_performance_test_template(nfr_spec)
        }

    # Run performance tests
    results = run_tests(perf_tests)

    if results.avg_response_time > nfr_spec.max_response_time:
        return {
            "status": "FAIL",
            "issue": "Performance NFR not met",
            "expected": f"< {nfr_spec.max_response_time}ms",
            "actual": f"{results.avg_response_time}ms",
            "action": "Optimize implementation"
        }

    return {
        "status": "PASS",
        "actual_performance": {
            "avg_response": results.avg_response_time,
            "p95_response": results.p95_response_time,
            "throughput": results.requests_per_second
        }
    }
```

### Security Requirements

```python
def validate_security_nfr(nfr_spec, source_path):
    # NFR: "All API endpoints must require authentication"

    violations = []

    # Find all API endpoints
    endpoints = find_api_endpoints(source_path)

    for endpoint in endpoints:
        # Check for authentication attribute
        has_auth = check_for_auth_attribute(endpoint)

        if not has_auth and not endpoint.is_public:
            violations.append({
                "endpoint": endpoint.path,
                "issue": "Missing authentication",
                "action": "Add [Authorize] attribute"
            })

    return violations
```

## Database Schema Validation

```python
def validate_database_schema(spec, migration_path):
    # Spec defines tables: Orders, OrderItems
    # Check migrations exist

    for table in spec.tables:
        migration_pattern = f"*{table.name}*.cs"
        migrations = glob(f"{migration_path}/{migration_pattern}")

        if not migrations:
            violations.append({
                "table": table.name,
                "issue": "Migration not found",
                "action": f"Create migration for {table.name} table"
            })
            continue

        # Validate schema matches spec
        migration = read_migration(migrations[0])
        actual_schema = parse_schema(migration)

        if not schema_matches(actual_schema, table.schema):
            violations.append({
                "table": table.name,
                "issue": "Schema mismatch",
                "expected": table.schema,
                "actual": actual_schema
            })

    return violations
```

## Error Handling Validation

```python
def validate_error_handling(coding_standards, source_path):
    # Load expected error handling pattern
    pattern = coding_standards["error_handling"]["pattern"]
    # Example: "Result Pattern"

    violations = []

    if pattern == "Result Pattern":
        # Check methods return Result<T>
        business_logic = glob("src/{Services,Application}/**/*.cs")

        for file in business_logic:
            public_methods = grep("public.*\(", file)

            for method in public_methods:
                if not returns_result_type(method):
                    # Check if throws exceptions for business logic
                    exceptions = grep("throw new.*Exception", file, context=method)

                    if exceptions:
                        violations.append({
                            "file": file,
                            "method": method.name,
                            "issue": "Uses exceptions instead of Result Pattern",
                            "expected": "Return Result<T> for business logic errors"
                        })

    return violations
```

## Traceability Matrix

```python
def generate_traceability_matrix(story, tests, implementation):
    """
    Creates matrix: Requirement → Test → Implementation
    """
    matrix = []

    for criterion in story.acceptance_criteria:
        # Find tests
        tests_for_criterion = find_tests_for_criterion(criterion, tests)

        # Find implementation
        impl_for_criterion = find_implementation(criterion, implementation)

        matrix.append({
            "requirement": criterion,
            "tests": [t.name for t in tests_for_criterion],
            "implementation": [i.file for i in impl_for_criterion],
            "status": "COMPLETE" if tests_for_criterion and impl_for_criterion else "INCOMPLETE"
        })

    return matrix
```

## Quick Reference

### Validation Checklist

- [ ] All acceptance criteria have tests
- [ ] All tests pass
- [ ] API endpoints match spec
- [ ] Request/response models match spec
- [ ] Database schema matches spec
- [ ] Error handling follows standards
- [ ] NFRs validated (performance, security)
- [ ] Traceability complete (requirement → test → code)

This reference should be loaded when validating spec compliance during deep QA validation.
