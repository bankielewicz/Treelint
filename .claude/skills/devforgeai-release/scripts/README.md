# Release Skill Automation Scripts

Comprehensive automation scripts for the DevForgeAI release workflow, supporting health checks, smoke testing, metrics monitoring, rollback automation, and release documentation.

## Overview

These scripts automate critical release validation and deployment tasks:

- **health_check.py** - HTTP health endpoint validation with retry logic
- **smoke_test_runner.py** - Orchestrates pytest smoke test suite
- **metrics_collector.py** - Collects and compares production metrics
- **rollback_automation.sh** - Automated rollback for multiple platforms
- **release_notes_generator.py** - Generates release notes and updates changelog

---

## Installation

### Required Dependencies

```bash
# Python scripts require Python 3.7+
python --version

# Install Python dependencies
pip install requests pytest pytest-html pytest-xdist

# Optional: Install monitoring backend libraries
pip install boto3              # AWS CloudWatch
pip install datadog            # Datadog
pip install prometheus-client  # Prometheus
pip install azure-monitor-query  # Azure Monitor
```

### Platform Tools

Ensure the following tools are installed based on your deployment platform:

```bash
# Kubernetes
kubectl version

# Docker
docker version

# Azure CLI
az version

# AWS CLI
aws --version

# Helm (optional)
helm version
```

---

## Configuration

### Directory Structure

Create the required directory structure:

```bash
mkdir -p devforgeai/smoke-tests
mkdir -p devforgeai/monitoring/baselines
mkdir -p devforgeai/releases/rollback-logs
```

### Smoke Test Configuration

Create `devforgeai/smoke-tests/config.json`:

```json
{
  "environments": {
    "staging": {
      "base_url": "https://staging.example.com",
      "test_user": "staging_test@example.com",
      "test_password": "staging_password"
    },
    "production": {
      "base_url": "https://api.example.com",
      "test_user": "prod_test@example.com",
      "test_password": "prod_password"
    }
  }
}
```

### Monitoring Configuration

Create `devforgeai/monitoring/config.json`:

```json
{
  "backend": "cloudwatch",
  "region": "us-east-1",
  "namespace": "AWS/ApplicationELB",
  "dimensions": [
    {
      "Name": "LoadBalancer",
      "Value": "app/my-lb/xxxxx"
    }
  ]
}
```

### Baseline Metrics

Create baseline metrics file `devforgeai/monitoring/baselines/production-baseline.json`:

```json
{
  "version": "v1.2.2",
  "baseline_period": "2025-10-20 to 2025-10-27",
  "metrics": {
    "error_rate": 0.12,
    "response_time_p95": 185,
    "response_time_p99": 420,
    "request_rate": 1250,
    "cpu_utilization": 45,
    "memory_usage": 62,
    "db_connection_usage": 35,
    "cache_hit_rate": 87
  }
}
```

---

## Scripts

### 1. health_check.py

**Purpose**: Validates HTTP health endpoints with retry logic and exponential backoff.

**Usage:**
```bash
python health_check.py --url https://api.example.com/health --retries 5 --timeout 10
```

**Arguments:**
- `--url` (required): Health endpoint URL (can specify multiple times)
- `--retries` (optional): Maximum retry attempts (default: 5)
- `--timeout` (optional): Request timeout in seconds (default: 10)
- `--backoff` (optional): Exponential backoff base delay (default: 1.0)
- `--quiet` (optional): Suppress info logs (errors only)

**Exit Codes:**
- `0`: Success - Health check passed
- `1`: Failure - Health check failed

**Examples:**

```bash
# Basic health check
python health_check.py --url https://api.example.com/health

# With custom retries and timeout
python health_check.py --url https://staging.example.com/health --retries 10 --timeout 30

# Check multiple endpoints
python health_check.py \
  --url https://api.example.com/health \
  --url https://api.example.com/ready \
  --retries 5
```

**Features:**
- Exponential backoff retry (1s, 2s, 4s, 8s, 16s)
- JSON response validation and pretty-printing
- Support for HTTP and HTTPS
- Multiple endpoint support
- Detailed error messages

---

### 2. smoke_test_runner.py

**Purpose**: Orchestrates pytest smoke test suite with environment-specific configuration.

**Usage:**
```bash
python smoke_test_runner.py --environment production --tests critical_path
```

**Arguments:**
- `--environment` (required): Target environment (staging, production, production-green, production-canary)
- `--tests` (optional): Test categories to run (default: all)
  - Options: `all`, `critical_path`, `api`, `database`, `authentication`, `integration`, `health`
- `--config` (optional): Path to config file (default: `devforgeai/smoke-tests/config.json`)
- `--parallel` (optional): Number of parallel workers (requires pytest-xdist)
- `--html-report` (optional): Generate HTML report (requires pytest-html)
- `--junit-xml` (optional): Generate JUnit XML report
- `--quiet` (optional): Suppress verbose pytest output

**Exit Codes:**
- `0`: Success - All tests passed
- `1`: Failure - One or more tests failed

**Examples:**

```bash
# Run all smoke tests in production
python smoke_test_runner.py --environment production

# Run critical path tests in staging
python smoke_test_runner.py --environment staging --tests critical_path

# Run API and database tests with parallel execution
python smoke_test_runner.py --environment production --tests api,database --parallel 4

# Generate HTML and JUnit XML reports
python smoke_test_runner.py --environment production \
  --html-report results.html \
  --junit-xml results.xml
```

**Test Categories:**
- `all` - All smoke tests
- `critical_path` - Critical user flows
- `api` - API endpoint tests
- `database` - Database connectivity
- `authentication` - Auth flow tests
- `integration` - External integrations
- `health` - Health checks

---

### 3. metrics_collector.py

**Purpose**: Collects metrics from monitoring systems and compares against baseline.

**Usage:**
```bash
python metrics_collector.py --environment production --duration 900 --baseline-compare
```

**Arguments:**
- `--environment` (required): Environment name (staging, production)
- `--duration` (optional): Collection duration in seconds (default: 900 = 15 minutes)
- `--metrics` (optional): Comma-separated metrics to collect (default: all)
  - Options: `error_rate`, `response_time_p95`, `response_time_p99`, `request_rate`, `cpu`, `memory`, `db_connections`, `cache_hit_rate`
- `--baseline-compare` (optional): Compare against baseline metrics
- `--baseline` (optional): Custom baseline file path
- `--output` (optional): Output JSON report to file
- `--config` (optional): Path to monitoring config (default: `devforgeai/monitoring/config.json`)

**Exit Codes:**
- `0`: Success - Metrics healthy
- `1`: Warning - Metrics show degradation
- `2`: Critical - Rollback recommended

**Examples:**

```bash
# Collect metrics for 15 minutes
python metrics_collector.py --environment production --duration 900

# Compare against baseline
python metrics_collector.py --environment production --duration 900 --baseline-compare

# Collect specific metrics and output to file
python metrics_collector.py --environment production \
  --metrics error_rate,response_time_p95 \
  --output report.json

# Use custom baseline
python metrics_collector.py --environment production \
  --baseline custom-baseline.json \
  --baseline-compare
```

**Supported Backends:**
- AWS CloudWatch
- Datadog
- Prometheus
- Azure Monitor
- Mock (for testing)

**Thresholds:**
- **Error Rate**: Warning at 1.2x baseline, Critical at 2.0x
- **Response Time (p95)**: Warning at 1.3x, Critical at 1.5x
- **CPU**: Warning at 75%, Critical at 80%
- **Memory**: Warning at 80%, Critical at 85%

---

### 4. rollback_automation.sh

**Purpose**: Automated rollback for multiple deployment platforms.

**Usage:**
```bash
./rollback_automation.sh --platform kubernetes --deployment myapp --version v1.9.0 --namespace production
```

**Arguments:**
- `--platform` (required): Deployment platform (kubernetes, azure, aws_ecs, docker)
- `--deployment` (required): Deployment/application name
- `--version` (optional): Version to rollback to (default: previous)
- `--namespace` (optional): Kubernetes namespace (default: production)
- `--cluster` (optional): AWS ECS cluster name
- `--resource-group` (optional): Azure resource group name
- `--rollback-db` (optional): Also rollback database migrations
- `--help`: Display help message

**Exit Codes:**
- `0`: Success - Rollback completed
- `1`: Failure - Rollback failed

**Examples:**

```bash
# Kubernetes rollback to previous version
./rollback_automation.sh --platform kubernetes --deployment myapp --namespace production

# Kubernetes rollback to specific version
./rollback_automation.sh --platform kubernetes --deployment myapp --version v1.9.0 --namespace production

# Azure App Service rollback
./rollback_automation.sh --platform azure --deployment myapp --resource-group myRG

# AWS ECS rollback to specific task definition
./rollback_automation.sh --platform aws_ecs --deployment myapp --cluster production --version 42

# Docker rollback
./rollback_automation.sh --platform docker --deployment myapp --version v1.9.0

# With database rollback
./rollback_automation.sh --platform kubernetes --deployment myapp --namespace production --rollback-db
```

**Supported Platforms:**
- **Kubernetes**: kubectl rollout undo
- **Azure App Service**: Slot swap
- **AWS ECS**: Update service to previous task definition
- **Docker**: Stop current, start previous container

**Rollback Logs:**
All rollback operations are logged to `devforgeai/releases/rollback-logs/rollback-YYYYMMDD-HHMMSS.log`

---

### 5. release_notes_generator.py

**Purpose**: Generates release notes from story documents and templates.

**Usage:**
```bash
python release_notes_generator.py --story STORY-001 --version v1.2.3
```

**Arguments:**
- `--story` (required): Story ID (e.g., STORY-001)
- `--version` (required): Release version (e.g., v1.2.3)
- `--qa-report` (optional): Path to QA report file
- `--metrics-report` (optional): Path to metrics report JSON file
- `--template` (optional): Path to custom release notes template
- `--output` (optional): Output path for release notes (default: `devforgeai/releases/release-{version}.md`)
- `--deployment-strategy` (optional): Deployment strategy used (default: rolling)
- `--previous-version` (optional): Previous version for rollback information
- `--update-changelog` (optional): Update CHANGELOG.md with release entry

**Exit Codes:**
- `0`: Success - Release notes generated
- `1`: Failure - Generation failed

**Examples:**

```bash
# Basic release notes generation
python release_notes_generator.py --story STORY-001 --version v1.2.3

# With QA and metrics reports
python release_notes_generator.py --story STORY-001 --version v1.2.3 \
  --qa-report devforgeai/qa/reports/STORY-001-qa-report.md \
  --metrics-report metrics.json

# Update CHANGELOG.md
python release_notes_generator.py --story STORY-001 --version v1.2.3 --update-changelog

# Custom output location
python release_notes_generator.py --story STORY-001 --version v1.2.3 \
  --output /path/to/release-notes.md
```

**Template Variables:**
- `{{VERSION}}` - Release version
- `{{DATE}}` - Release date
- `{{STORY_ID}}` - Story identifier
- `{{STORY_TITLE}}` - Story title
- `{{CHANGES}}` - Changes description
- `{{ACCEPTANCE_CRITERIA}}` - Acceptance criteria list
- `{{QA_STATUS}}` - QA validation status
- `{{COVERAGE}}` - Test coverage percentage
- `{{DEPLOYMENT_STRATEGY}}` - Deployment strategy
- `{{PREVIOUS_VERSION}}` - Previous version for rollback

---

## Integration with Release Workflow

### Phase 2: Staging Deployment

```bash
# After staging deployment, run health check
python scripts/health_check.py --url https://staging.example.com/health --retries 5

# Run smoke tests
python scripts/smoke_test_runner.py --environment staging --tests critical_path
```

### Phase 4: Post-Deployment Validation

```bash
# Production health check
python scripts/health_check.py --url https://api.example.com/health --retries 5

# Production smoke tests
python scripts/smoke_test_runner.py --environment production

# Metrics monitoring
python scripts/metrics_collector.py --environment production --duration 900 --baseline-compare
```

### Phase 5: Release Documentation

```bash
# Generate release notes
python scripts/release_notes_generator.py \
  --story STORY-001 \
  --version v1.2.3 \
  --qa-report devforgeai/qa/reports/STORY-001-qa-report.md \
  --metrics-report metrics.json \
  --update-changelog
```

### Emergency Rollback

```bash
# Kubernetes rollback
./scripts/rollback_automation.sh --platform kubernetes --deployment myapp --namespace production

# With database rollback
./scripts/rollback_automation.sh --platform kubernetes --deployment myapp --namespace production --rollback-db
```

---

## Troubleshooting

### Health Check Issues

**Problem**: Health check fails with connection error

**Solution**:
```bash
# Verify URL is accessible
curl -v https://api.example.com/health

# Check firewall/network
ping api.example.com

# Try with increased timeout
python health_check.py --url https://api.example.com/health --timeout 30
```

### Smoke Test Issues

**Problem**: pytest not found

**Solution**:
```bash
pip install pytest pytest-html pytest-xdist
```

**Problem**: Tests fail with authentication error

**Solution**:
```bash
# Verify credentials in config
cat devforgeai/smoke-tests/config.json

# Set environment variables
export TEST_USER="test@example.com"
export TEST_PASSWORD="password"
```

### Metrics Collection Issues

**Problem**: Metrics collection returns no data

**Solution**:
```bash
# Check monitoring config
cat devforgeai/monitoring/config.json

# Verify credentials (AWS example)
aws sts get-caller-identity

# Test with longer duration
python metrics_collector.py --environment production --duration 1800
```

### Rollback Issues

**Problem**: Rollback script fails with "command not found"

**Solution**:
```bash
# Install required CLI tools
# Kubernetes
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"

# Azure CLI
curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash

# AWS CLI
pip install awscli
```

**Problem**: Permission denied executing rollback script

**Solution**:
```bash
chmod +x scripts/rollback_automation.sh
```

---

## Best Practices

### Health Checks
1. Always run health checks with retries (minimum 3)
2. Use exponential backoff to avoid overwhelming unhealthy services
3. Check multiple endpoints if available
4. Run health checks immediately after deployment and again after 5 minutes

### Smoke Tests
5. Keep smoke test suite under 5 minutes total execution time
6. Use parallel execution for faster results
7. Run critical path tests first, then comprehensive tests
8. Generate HTML reports for debugging failures

### Metrics Monitoring
9. Establish baselines from 7-14 days of historical data
10. Monitor for at least 15 minutes post-deployment
11. Compare against same-time-of-day baselines (account for traffic patterns)
12. Set conservative thresholds initially, adjust based on false positives

### Rollback Automation
13. Always test rollback procedures in staging first
14. Document previous version before attempting rollback
15. Create database backups before deployments with migrations
16. Verify health after rollback completes

### Release Documentation
17. Generate release notes immediately after successful deployment
18. Include QA and metrics reports for audit trail
19. Update CHANGELOG.md consistently
20. Archive release notes for compliance

---

## Script Dependencies

### Python Libraries

```txt
# requirements.txt for release skill scripts
requests>=2.28.0
pytest>=7.2.0
pytest-html>=3.2.0
pytest-xdist>=3.1.0

# Optional monitoring backends
boto3>=1.26.0           # AWS CloudWatch
datadog>=0.45.0         # Datadog
prometheus-client>=0.15.0  # Prometheus
azure-monitor-query>=1.1.0  # Azure Monitor
```

Install all dependencies:
```bash
pip install -r scripts/requirements.txt
```

---

## License

These scripts are part of the DevForgeAI framework. See LICENSE.txt for details.

---

## Support

For issues or questions:
- Review script documentation above
- Check troubleshooting section
- Verify configuration files are correct
- Test in staging before production
- Consult DevForgeAI release skill documentation

---

**These scripts automate critical release validation tasks to ensure safe, repeatable production deployments with comprehensive monitoring and rollback capabilities.**
