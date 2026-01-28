# Smoke Testing Guide

Comprehensive guide for validating deployments with automated smoke tests.

## Purpose

Smoke tests are **quick validation tests** that verify critical functionality after deployment. They are designed to catch deployment issues (configuration errors, missing dependencies, broken integrations) **before users are affected**.

## Characteristics of Good Smoke Tests

✅ **Fast**: Complete in < 5 minutes
✅ **Critical Path Only**: Test must-have functionality, not edge cases
✅ **No Test Data Setup**: Use production-like data or read-only operations
✅ **Idempotent**: Can run repeatedly without side effects
✅ **Clear Pass/Fail**: No ambiguous results

## Standard Smoke Test Checklist

### 1. Health Endpoint Check
```bash
# Basic health check
curl -f https://api.example.com/health

# Expected: HTTP 200
# Response: {"status": "healthy", "timestamp": "2025-10-30T12:00:00Z"}
```

### 2. Database Connectivity
```python
# tests/smoke/test_database_connectivity.py
def test_database_connection():
    """Verify application can connect to database"""
    conn = get_database_connection()
    assert conn.is_connected()

    # Execute simple query
    result = conn.execute("SELECT 1")
    assert result == 1
```

### 3. Authentication Flow
```python
# tests/smoke/test_user_authentication.py
def test_user_login():
    """Verify users can authenticate"""
    response = requests.post(
        f"{BASE_URL}/api/auth/login",
        json={"email": "test@example.com", "password": "test123"}
    )

    assert response.status_code == 200
    assert "token" in response.json()
```

### 4. Critical API Endpoints
```python
# tests/smoke/test_api_endpoints_respond.py
def test_critical_endpoints():
    """Verify critical APIs respond"""
    endpoints = [
        "/api/users",
        "/api/products",
        "/api/orders"
    ]

    for endpoint in endpoints:
        response = requests.get(f"{BASE_URL}{endpoint}", headers=auth_headers)
        assert response.status_code in [200, 201]
```

### 5. External Integrations
```python
# tests/smoke/test_external_integrations.py
def test_payment_gateway_connectivity():
    """Verify payment gateway is accessible"""
    # Use test API call that doesn't charge
    response = requests.get(
        "https://api.stripe.com/v1/account",
        headers={"Authorization": f"Bearer {STRIPE_TEST_KEY}"}
    )
    assert response.status_code == 200

def test_email_service_connectivity():
    """Verify email service is accessible"""
    # Check SMTP connection without sending
    smtp = smtplib.SMTP(EMAIL_HOST, EMAIL_PORT)
    smtp.starttls()
    smtp.login(EMAIL_USER, EMAIL_PASSWORD)
    smtp.quit()  # Success if no exception
```

### 6. Static Assets Loading
```python
# tests/smoke/test_static_assets.py
def test_frontend_loads():
    """Verify frontend application loads"""
    response = requests.get(f"{BASE_URL}/")
    assert response.status_code == 200
    assert "<html" in response.text.lower()

def test_critical_assets():
    """Verify critical static assets load"""
    assets = [
        "/static/css/main.css",
        "/static/js/app.js"
    ]

    for asset in assets:
        response = requests.get(f"{BASE_URL}{asset}")
        assert response.status_code == 200
```

## Environment-Specific Configuration

### Staging Environment
```python
# tests/smoke/conftest.py
import pytest

@pytest.fixture
def environment(request):
    env = request.config.getoption("--environment")

    if env == "staging":
        return {
            "base_url": "https://staging.example.com",
            "test_user": "staging_test@example.com",
            "test_password": "staging_password"
        }
    elif env == "production":
        return {
            "base_url": "https://api.example.com",
            "test_user": "prod_test@example.com",
            "test_password": "prod_password"
        }
```

### Production Environment

**CRITICAL**: Production smoke tests must NOT:
- Create/modify/delete user data
- Process real payments
- Send emails to real users
- Trigger rate limiting

**Use read-only operations or dedicated test accounts**.

## Running Smoke Tests

### Manual Execution
```bash
# Staging
pytest tests/smoke/ --environment=staging --verbose

# Production
pytest tests/smoke/ --environment=production --verbose
```

### Automated via Script
```bash
# Use the smoke_test_runner.py script
python scripts/smoke_test_runner.py \
  --environment production \
  --url https://api.example.com \
  --tests critical_path
```

## Test Organization

```
tests/smoke/
├── __init__.py
├── conftest.py                    # Environment configuration
├── test_health_checks.py          # Health endpoints
├── test_database_connectivity.py  # Database tests
├── test_user_authentication.py    # Auth flow
├── test_api_endpoints_respond.py  # API tests
├── test_external_integrations.py  # Third-party services
└── test_static_assets.py          # Frontend loading
```

## Critical Path Testing

Identify the **most important user flows** and test them:

### E-Commerce Example
1. User can browse products ✓
2. User can add to cart ✓
3. User can checkout (without payment) ✓
4. Order confirmation email sent (to test account) ✓

### SaaS Platform Example
1. User can log in ✓
2. User can access dashboard ✓
3. User can create project (basic) ✓
4. API responds to authenticated requests ✓

## API Contract Validation

Smoke tests should verify **API contracts match specification**:

```python
# tests/smoke/test_api_contracts.py
def test_user_endpoint_contract():
    """Verify /api/users endpoint matches contract"""
    response = requests.get(f"{BASE_URL}/api/users/1")

    assert response.status_code == 200

    user = response.json()
    # Verify response structure matches spec
    assert "id" in user
    assert "email" in user
    assert "firstName" in user
    assert "lastName" in user
    assert "createdAt" in user
```

## Performance Smoke Tests

Quick performance checks (not comprehensive load testing):

```python
# tests/smoke/test_performance.py
import time

def test_response_time_acceptable():
    """Verify API responds within acceptable time"""
    start = time.time()
    response = requests.get(f"{BASE_URL}/api/users")
    elapsed = time.time() - start

    assert response.status_code == 200
    assert elapsed < 1.0  # Should respond within 1 second
```

## Security Smoke Tests

Basic security checks:

```python
# tests/smoke/test_security.py
def test_https_enforced():
    """Verify HTTP redirects to HTTPS"""
    http_url = BASE_URL.replace("https://", "http://")
    response = requests.get(http_url, allow_redirects=False)

    assert response.status_code in [301, 302]  # Redirect
    assert response.headers["Location"].startswith("https://")

def test_authentication_required():
    """Verify protected endpoints require auth"""
    response = requests.get(f"{BASE_URL}/api/users")  # No auth header

    assert response.status_code == 401  # Unauthorized
```

## Failure Handling

### Test Retry Logic
```python
# Retry flaky tests (network issues)
@pytest.mark.flaky(reruns=3, reruns_delay=2)
def test_external_api():
    """Test with retry for flaky network calls"""
    response = requests.get(f"{EXTERNAL_API}/health")
    assert response.status_code == 200
```

### Timeouts
```python
# Always use timeouts to prevent hanging
response = requests.get(url, timeout=10)  # 10 second timeout
```

## Smoke Test Automation

### Integration with Release Skill

```python
# scripts/smoke_test_runner.py
"""
Orchestrates smoke test suite with retry logic and reporting.

Usage:
  python smoke_test_runner.py --environment production --url https://api.example.com
"""
import subprocess
import sys

def run_smoke_tests(environment, base_url):
    result = subprocess.run(
        [
            "pytest",
            "tests/smoke/",
            f"--environment={environment}",
            f"--base-url={base_url}",
            "--verbose",
            "--tb=short"
        ],
        capture_output=True,
        text=True
    )

    if result.returncode != 0:
        print("SMOKE TESTS FAILED")
        print(result.stdout)
        print(result.stderr)
        sys.exit(1)

    print("✓ All smoke tests passed")
    return True

if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--environment", required=True)
    parser.add_argument("--url", required=True)
    args = parser.parse_args()

    run_smoke_tests(args.environment, args.url)
```

## Monitoring Smoke Test Results

Track smoke test history to identify patterns:

```
Deployment | Smoke Tests | Duration | Failures | Rollback
-----------|-------------|----------|----------|----------
v1.0.0     | 15 passed   | 2m 10s   | 0        | No
v1.0.1     | 15 passed   | 2m 15s   | 0        | No
v1.1.0     | 12 passed   | 1m 50s   | 3        | Yes (API contract breach)
v1.1.1     | 15 passed   | 2m 05s   | 0        | No
```

## Best Practices

1. **Keep Tests Minimal** - Only critical path, not exhaustive
2. **Use Dedicated Test Accounts** - Never use real user data
3. **Run in Staging First** - Catch issues before production
4. **Automate Execution** - Part of deployment pipeline
5. **Monitor Test Duration** - Should complete in < 5 minutes
6. **Document Expected Behavior** - Clear assertions
7. **Handle Flaky Tests** - Retry mechanism for network calls
8. **Version Test Suite** - Track with application version
9. **Alert on Failures** - Immediate notification if tests fail
10. **Regular Test Maintenance** - Update when APIs change

## Common Pitfalls

- ❌ **Testing too much** - Smoke tests become slow
- ❌ **No idempotency** - Tests create data that conflicts
- ❌ **Production data modification** - Dangerous!
- ❌ **No timeouts** - Tests hang indefinitely
- ❌ **Ignoring flaky tests** - Should investigate and fix
- ❌ **No test data cleanup** - Test accounts fill up

## Integration with CI/CD

```yaml
# .github/workflows/deploy.yml
- name: Run Smoke Tests (Staging)
  run: |
    python scripts/smoke_test_runner.py \
      --environment staging \
      --url https://staging.example.com

- name: Deploy to Production
  run: |
    # deployment commands...

- name: Run Smoke Tests (Production)
  run: |
    python scripts/smoke_test_runner.py \
      --environment production \
      --url https://api.example.com

- name: Rollback if Tests Fail
  if: failure()
  run: |
    # rollback commands...
```

## References

- [Pytest Documentation](https://docs.pytest.org/)
- [Requests Library](https://docs.python-requests.org/)
- [Martin Fowler: Smoke Testing](https://martinfowler.com/bliki/SmokeTest.html)
