### Phase 4: Post-Deployment Validation

**Objective**: Verify production deployment health and stability

#### Step 1: Production Smoke Tests

```
# Wait for application to stabilize
sleep(60)

# Health check
Bash(command="python scripts/health_check.py --url {production_url}/health --retries 5")

HALT if health_check fails:
    "Production health check failed - ROLLBACK"
    execute_production_rollback()

# Smoke tests
Bash(command="python scripts/smoke_test_runner.py --environment production --url {production_url}")

HALT if smoke_tests fail:
    "Production smoke tests failed - ROLLBACK"
    execute_production_rollback()
```

For smoke test guidance, see `references/smoke-testing-guide.md`

#### Step 2: Metrics Monitoring

```
# Monitor key metrics for 15 minutes
Bash(command="python scripts/metrics_collector.py --environment production --duration 900 --baseline-compare")

# Compare against baseline
IF error_rate > baseline * 1.2:
    WARN: "Error rate increased"
    AskUserQuestion about continuing or rolling back

IF response_time > baseline * 1.3:
    WARN: "Response time degraded"
    AskUserQuestion about continuing or rolling back
```

For metrics thresholds, see `references/monitoring-metrics.md`

#### Step 3: Declare Success or Rollback

```
IF all validations pass:
    Report: "✅ Production deployment successful"
    Proceed to Phase 5: Release Documentation
ELSE:
    execute_production_rollback()
    See references/rollback-procedures.md
```

---

