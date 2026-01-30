# Monitoring Metrics Reference

This document provides guidance on monitoring production deployments, establishing baselines, and detecting anomalies during release validation.

## Overview

**Purpose**: Define key metrics, baseline establishment, alert thresholds, and comparison logic for post-deployment monitoring.

**Usage**: Reference this during Phase 4 (Post-Deployment Validation) and Phase 6 (Post-Release Monitoring) of the release workflow.

---

## Key Metrics to Monitor

### 1. Error Rate

**Definition**: Percentage of failed requests/operations

**Collection Methods**:
- Application logs (ERROR level count / total requests)
- Load balancer error responses (5xx status codes)
- APM tools (Application Performance Monitoring)

**Calculation**:
```
error_rate = (failed_requests / total_requests) * 100
```

**Acceptable Thresholds**:
- Baseline: < 0.5% (varies by application)
- Warning: > 1.2x baseline
- Critical: > 2.0x baseline

**Example**:
```python
# Baseline: 0.1% error rate
# Warning threshold: 0.12% (1.2x baseline)
# Critical threshold: 0.2% (2x baseline)

if current_error_rate > baseline * 2.0:
    trigger_rollback()
elif current_error_rate > baseline * 1.2:
    alert_team()
```

### 2. Response Time (p95 / p99)

**Definition**: 95th/99th percentile response time (milliseconds)

**Why Percentiles**: Average masks outliers; p95 shows worst-case for 95% of users

**Collection Methods**:
- API Gateway metrics
- Application instrumentation (timing middleware)
- APM tools (New Relic, Datadog, Dynatrace)

**Calculation**:
```
p95_response_time = percentile(response_times, 95)
p99_response_time = percentile(response_times, 99)
```

**Acceptable Thresholds**:
- Baseline: Application-specific (e.g., 200ms p95)
- Warning: > 1.3x baseline
- Critical: > 1.5x baseline

**Example**:
```python
# Baseline: 200ms p95, 500ms p99
# Warning thresholds: 260ms p95 (1.3x), 650ms p99
# Critical thresholds: 300ms p95 (1.5x), 750ms p99

if current_p95 > baseline_p95 * 1.5:
    trigger_rollback()
elif current_p95 > baseline_p95 * 1.3:
    extend_monitoring()
```

### 3. Request Rate (RPS - Requests Per Second)

**Definition**: Number of requests handled per second

**Purpose**: Detect traffic anomalies or capacity issues

**Collection Methods**:
- Load balancer metrics
- Application metrics
- API Gateway logs

**Acceptable Thresholds**:
- Baseline: Historical average (e.g., 1000 RPS)
- Warning: < 0.5x baseline (traffic drop - possible routing issue)
- Warning: > 1.5x baseline (unexpected traffic spike)

**Example**:
```python
# Baseline: 1000 RPS
# Warning thresholds: < 500 RPS or > 1500 RPS

if current_rps < baseline * 0.5:
    alert("Traffic drop detected - possible routing issue")
elif current_rps > baseline * 1.5:
    alert("Traffic spike - check capacity")
```

### 4. CPU Utilization

**Definition**: Percentage of CPU capacity used

**Collection Methods**:
- Kubernetes metrics (kubectl top pods)
- Cloud provider dashboards (AWS CloudWatch, Azure Monitor, GCP Monitoring)
- Node exporters (Prometheus)

**Acceptable Thresholds**:
- Normal: < 70%
- Warning: 70-80%
- Critical: > 80%

**Example**:
```python
if cpu_utilization > 80:
    alert("CPU throttling risk - may need scaling")
elif cpu_utilization > 70:
    monitor_closely()
```

### 5. Memory Usage

**Definition**: Percentage of memory capacity used

**Collection Methods**:
- Kubernetes metrics
- Cloud provider dashboards
- Application heap metrics

**Acceptable Thresholds**:
- Normal: < 75%
- Warning: 75-85%
- Critical: > 85% (OOMKill risk)

**Example**:
```python
if memory_usage > 85:
    alert("Memory exhaustion risk - check for leaks")
elif memory_usage > 75:
    monitor_for_upward_trend()
```

### 6. Database Connection Pool Usage

**Definition**: Percentage of database connections in use

**Purpose**: Detect connection leaks or capacity issues

**Collection Methods**:
- Database metrics (PostgreSQL pg_stat_activity, MySQL processlist)
- ORM/connection pool metrics
- APM database monitoring

**Acceptable Thresholds**:
- Normal: < 60%
- Warning: 60-80%
- Critical: > 80% (connection exhaustion)

**Example**:
```python
# Connection pool size: 100
# Current active: 85
connection_usage = (active_connections / pool_size) * 100

if connection_usage > 80:
    alert("Connection pool exhaustion - check for leaks")
```

### 7. Cache Hit Rate

**Definition**: Percentage of requests served from cache vs. database

**Purpose**: Ensure caching strategy effective after deployment

**Collection Methods**:
- Redis INFO stats (keyspace_hits / keyspace_misses)
- Memcached stats
- Application cache instrumentation

**Acceptable Thresholds**:
- Baseline: Application-specific (e.g., 85% hit rate)
- Warning: < 0.8x baseline (cache warming issue)

**Example**:
```python
# Baseline: 85% hit rate
# Warning threshold: < 68% (0.8x baseline)

cache_hit_rate = (cache_hits / total_requests) * 100

if cache_hit_rate < baseline * 0.8:
    alert("Cache effectiveness degraded - possible cache warming issue")
```

### 8. Dependency Health (External Services)

**Definition**: Availability and response time of external dependencies

**Purpose**: Detect integration failures after deployment

**Collection Methods**:
- Health endpoint polling
- Circuit breaker metrics
- API call success rates

**Acceptable Thresholds**:
- Availability: > 99% (per SLA)
- Response time: < dependency SLA threshold

**Example**:
```python
# Payment gateway dependency
payment_availability = (successful_calls / total_calls) * 100

if payment_availability < 99:
    alert("Payment gateway integration failing")
```

---

## Baseline Establishment

### What is a Baseline?

**Baseline**: Historical metrics representing "normal" application behavior before deployment.

**Purpose**: Provide comparison point to detect degradation after deployment.

### How to Establish Baselines

#### Method 1: Historical Average (Recommended)

**Process**:
1. Collect metrics for 7-14 days before deployment
2. Calculate average, median, p95, p99 for each metric
3. Store baseline values for comparison

**Example**:
```python
# Collect 7 days of production metrics
baseline = {
    "error_rate": 0.12,  # Average: 0.12%
    "response_time_p95": 185,  # p95: 185ms
    "response_time_p99": 420,  # p99: 420ms
    "rps": 1250,  # Average: 1250 RPS
    "cpu": 45,  # Average: 45%
    "memory": 62,  # Average: 62%
}

# Save to file for deployment comparison
write_baseline(baseline, "baseline-pre-release-v1.2.3.json")
```

#### Method 2: Same-Time Comparison

**Process**:
1. Capture metrics at same time of day/week (account for traffic patterns)
2. Compare deployment metrics to same time window from previous week

**Example**:
```python
# Deployment at Monday 10am
# Compare to last Monday 10am metrics
baseline = get_metrics(date="last Monday", time="10:00-11:00")
current = get_metrics(date="this Monday", time="10:00-11:00")

compare(current, baseline)
```

#### Method 3: Staging Environment Baseline

**Process**:
1. Establish baseline from staging environment metrics
2. Use staging baseline as proxy for production
3. Adjust for production traffic volume

**Example**:
```python
# Staging baseline (low traffic)
staging_baseline = {"error_rate": 0.05, "rps": 50}

# Scale to production traffic
production_multiplier = prod_rps / staging_rps  # e.g., 20x
estimated_baseline = adjust_for_scale(staging_baseline, multiplier)
```

### Baseline Storage

**Location**: `devforgeai/monitoring/baselines/`

**Format**:
```json
{
  "version": "v1.2.3",
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
  },
  "traffic_profile": {
    "peak_rps": 2800,
    "avg_rps": 1250,
    "off_peak_rps": 600
  }
}
```

---

## Alert Threshold Configuration

### Threshold Levels

**1. Informational**: Metrics outside normal but not concerning
**2. Warning**: Requires attention, extend monitoring
**3. Critical**: Requires immediate action (potential rollback)

### Threshold Configuration by Metric

| Metric | Informational | Warning | Critical | Rollback Trigger |
|--------|---------------|---------|----------|------------------|
| **Error Rate** | > 1.1x baseline | > 1.2x baseline | > 2.0x baseline | Yes |
| **Response Time (p95)** | > 1.2x baseline | > 1.3x baseline | > 1.5x baseline | Yes |
| **Request Rate** | ±20% baseline | ±30% baseline | ±50% baseline | Investigate |
| **CPU** | > 70% | > 75% | > 80% | Investigate |
| **Memory** | > 70% | > 80% | > 85% | Investigate |
| **DB Connections** | > 60% | > 70% | > 80% | Yes |
| **Cache Hit Rate** | < 0.9x baseline | < 0.8x baseline | < 0.7x baseline | Investigate |

### Dynamic Thresholds (Advanced)

**Concept**: Adjust thresholds based on traffic patterns and time of day

**Example**:
```python
# Peak hours (9am-5pm): stricter thresholds
# Off-peak hours: relaxed thresholds

if is_peak_hours():
    error_rate_threshold = baseline * 1.2
else:
    error_rate_threshold = baseline * 1.5
```

---

## Metrics Collection Tools Integration

### AWS CloudWatch

**Setup**:
```python
import boto3

cloudwatch = boto3.client('cloudwatch')

# Collect error rate metric
response = cloudwatch.get_metric_statistics(
    Namespace='AWS/ApplicationELB',
    MetricName='HTTPCode_Target_5XX_Count',
    Dimensions=[{'Name': 'LoadBalancer', 'Value': 'app/my-lb/xxxxx'}],
    StartTime=start_time,
    EndTime=end_time,
    Period=300,  # 5 minutes
    Statistics=['Sum']
)

error_count = sum([point['Sum'] for point in response['Datapoints']])
```

**Key Metrics**:
- `HTTPCode_Target_5XX_Count` - Error count
- `TargetResponseTime` - Response time
- `RequestCount` - Request rate
- `TargetConnectionErrorCount` - Connection failures

### Datadog

**Setup**:
```python
from datadog import initialize, api

initialize(api_key='YOUR_API_KEY', app_key='YOUR_APP_KEY')

# Query error rate
query = "sum:application.requests.error{env:production}.as_rate()"
results = api.Metric.query(start=start_time, end=end_time, query=query)

error_rate = results['series'][0]['pointlist'][-1][1]  # Latest value
```

**Key Metrics**:
- `application.requests.error` - Error count
- `application.response.time` - Response time
- `system.cpu.user` - CPU usage
- `system.mem.used` - Memory usage

### Prometheus + Grafana

**Setup**:
```python
from prometheus_api_client import PrometheusConnect

prom = PrometheusConnect(url="http://prometheus:9090")

# Query error rate
error_rate_query = 'rate(http_requests_total{status=~"5.."}[5m]) * 100'
error_rate = prom.custom_query(query=error_rate_query)

# Query response time (p95)
p95_query = 'histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))'
p95_response_time = prom.custom_query(query=p95_query)
```

**Key Metrics**:
- `http_requests_total{status="5xx"}` - Error count
- `http_request_duration_seconds` - Response time histogram
- `node_cpu_seconds_total` - CPU usage
- `node_memory_MemAvailable_bytes` - Available memory

### Azure Monitor

**Setup**:
```python
from azure.monitor.query import MetricsQueryClient
from azure.identity import DefaultAzureCredential

credential = DefaultAzureCredential()
client = MetricsQueryClient(credential)

# Query error rate
response = client.query_resource(
    resource_uri=f"/subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.Web/sites/{app}",
    metric_names=["Http5xx"],
    timespan=timedelta(minutes=15)
)

error_count = sum([point.count for point in response.metrics[0].timeseries[0].data])
```

**Key Metrics**:
- `Http5xx` - 5xx error count
- `HttpResponseTime` - Response time
- `Requests` - Request rate
- `CpuPercentage` - CPU usage
- `MemoryPercentage` - Memory usage

### Custom Metrics Collection Script

**Location**: `scripts/metrics_collector.py` (part of release skill)

**Usage**:
```bash
python scripts/metrics_collector.py \
    --environment production \
    --duration 900 \
    --baseline-compare \
    --output metrics-report.json
```

**Output Format**:
```json
{
  "collection_time": "2025-10-30T14:30:00Z",
  "environment": "production",
  "duration_seconds": 900,
  "metrics": {
    "error_rate": 0.14,
    "response_time_p95": 198,
    "response_time_p99": 445,
    "request_rate": 1280,
    "cpu_utilization": 52,
    "memory_usage": 68
  },
  "baseline": {
    "error_rate": 0.12,
    "response_time_p95": 185,
    "response_time_p99": 420,
    "request_rate": 1250,
    "cpu_utilization": 45,
    "memory_usage": 62
  },
  "comparison": {
    "error_rate_change": "+16.7%",
    "error_rate_status": "WARNING",
    "response_time_p95_change": "+7.0%",
    "response_time_p95_status": "NORMAL",
    "cpu_change": "+15.6%",
    "cpu_status": "NORMAL"
  },
  "violations": [
    {
      "metric": "error_rate",
      "severity": "WARNING",
      "message": "Error rate 16.7% above baseline",
      "threshold": "1.2x baseline",
      "recommendation": "Extend monitoring period"
    }
  ],
  "rollback_recommended": false
}
```

---

## Comparison Logic

### Metric-by-Metric Comparison

**Algorithm**:
```python
def compare_metrics(current, baseline, thresholds):
    violations = []

    # Error rate comparison
    if current.error_rate > baseline.error_rate * thresholds.critical:
        violations.append({
            "metric": "error_rate",
            "severity": "CRITICAL",
            "current": current.error_rate,
            "baseline": baseline.error_rate,
            "threshold": thresholds.critical,
            "change_percent": ((current.error_rate - baseline.error_rate) / baseline.error_rate) * 100,
            "rollback": True
        })
    elif current.error_rate > baseline.error_rate * thresholds.warning:
        violations.append({
            "metric": "error_rate",
            "severity": "WARNING",
            "current": current.error_rate,
            "baseline": baseline.error_rate,
            "threshold": thresholds.warning,
            "change_percent": ((current.error_rate - baseline.error_rate) / baseline.error_rate) * 100,
            "rollback": False
        })

    # Response time comparison
    if current.response_time_p95 > baseline.response_time_p95 * thresholds.response_critical:
        violations.append({
            "metric": "response_time_p95",
            "severity": "CRITICAL",
            "current": current.response_time_p95,
            "baseline": baseline.response_time_p95,
            "threshold": thresholds.response_critical,
            "change_percent": ((current.response_time_p95 - baseline.response_time_p95) / baseline.response_time_p95) * 100,
            "rollback": True
        })

    # CPU utilization (absolute threshold, not baseline comparison)
    if current.cpu > 80:
        violations.append({
            "metric": "cpu_utilization",
            "severity": "CRITICAL",
            "current": current.cpu,
            "threshold": 80,
            "message": "CPU utilization above 80% - scaling required",
            "rollback": False  # Investigate, not automatic rollback
        })

    return violations
```

### Trend Analysis

**Detect upward/downward trends over monitoring period**:

```python
def analyze_trend(metric_timeseries, window_size=5):
    """
    Detect if metric is trending upward (degrading) or downward (improving)
    """
    if len(metric_timeseries) < window_size:
        return "INSUFFICIENT_DATA"

    recent = metric_timeseries[-window_size:]
    slope = calculate_linear_regression_slope(recent)

    if slope > 0.05:  # 5% increase per measurement
        return "DEGRADING"
    elif slope < -0.05:
        return "IMPROVING"
    else:
        return "STABLE"

# Example usage
error_rate_timeseries = [0.12, 0.13, 0.14, 0.16, 0.18]  # Increasing
trend = analyze_trend(error_rate_timeseries)

if trend == "DEGRADING":
    alert("Error rate trending upward - may require rollback soon")
```

### Composite Health Score

**Calculate overall deployment health**:

```python
def calculate_health_score(metrics, baseline, weights):
    """
    Calculate weighted health score (0-100)
    100 = Perfect (same as baseline)
    0 = Critical failure
    """
    scores = {}

    # Error rate score (inverse - lower is better)
    error_ratio = metrics.error_rate / baseline.error_rate
    scores['error_rate'] = max(0, 100 - (error_ratio - 1) * 200)  # -2 points per 1% increase

    # Response time score
    response_ratio = metrics.response_time_p95 / baseline.response_time_p95
    scores['response_time'] = max(0, 100 - (response_ratio - 1) * 150)

    # Resource utilization score
    scores['cpu'] = max(0, 100 - max(0, metrics.cpu - 70) * 2)  # -2 points per % above 70%
    scores['memory'] = max(0, 100 - max(0, metrics.memory - 70) * 2)

    # Weighted composite score
    health_score = (
        scores['error_rate'] * weights.error_rate +
        scores['response_time'] * weights.response_time +
        scores['cpu'] * weights.cpu +
        scores['memory'] * weights.memory
    )

    return {
        "overall_score": health_score,
        "component_scores": scores,
        "status": get_status_from_score(health_score)
    }

def get_status_from_score(score):
    if score >= 90:
        return "HEALTHY"
    elif score >= 70:
        return "WARNING"
    else:
        return "CRITICAL"

# Example
health = calculate_health_score(
    metrics=current_metrics,
    baseline=baseline_metrics,
    weights={"error_rate": 0.4, "response_time": 0.3, "cpu": 0.15, "memory": 0.15}
)

if health["status"] == "CRITICAL":
    trigger_rollback()
```

---

## Anomaly Detection

### Statistical Anomaly Detection

**Method**: Z-Score (Standard Deviations from Mean)

```python
import numpy as np

def detect_anomalies(metric_history, current_value, threshold=3):
    """
    Detect if current value is anomalous using z-score
    threshold: number of standard deviations (default 3 = 99.7% confidence)
    """
    mean = np.mean(metric_history)
    std_dev = np.std(metric_history)

    if std_dev == 0:
        return False  # No variance in history

    z_score = (current_value - mean) / std_dev

    if abs(z_score) > threshold:
        return True  # Anomaly detected
    else:
        return False

# Example
error_rate_history = [0.12, 0.11, 0.13, 0.12, 0.11, 0.12, 0.13]
current_error_rate = 0.35

is_anomaly = detect_anomalies(error_rate_history, current_error_rate, threshold=3)

if is_anomaly:
    alert("Error rate anomaly detected - significantly outside normal range")
```

### Machine Learning Anomaly Detection (Advanced)

**Method**: Isolation Forest (requires historical data)

```python
from sklearn.ensemble import IsolationForest

def train_anomaly_detector(historical_metrics):
    """
    Train ML model on historical "normal" metrics
    """
    model = IsolationForest(contamination=0.05)  # 5% expected anomalies
    model.fit(historical_metrics)
    return model

def predict_anomaly(model, current_metrics):
    """
    Predict if current metrics are anomalous
    Returns: -1 (anomaly) or 1 (normal)
    """
    prediction = model.predict([current_metrics])
    return prediction[0] == -1  # True if anomaly

# Example
historical_data = load_7_day_metrics()  # [[error_rate, p95, cpu, memory], ...]
model = train_anomaly_detector(historical_data)

current = [0.25, 300, 75, 80]  # [error_rate, p95, cpu, memory]
is_anomaly = predict_anomaly(model, current)

if is_anomaly:
    alert("ML model detected anomalous metrics")
```

---

## Monitoring Duration Recommendations

### Standard Monitoring Windows

**Staging Deployment**: 10 minutes
- Quick validation of deployment success
- Smoke tests complete within this window

**Production Deployment (Initial)**: 15 minutes
- Immediate post-deployment validation
- Detect critical issues early

**Production Deployment (Extended)**: 30-60 minutes
- When warning thresholds exceeded
- Gather more data before rollback decision

**Post-Release Monitoring**: 24 hours
- Detect delayed issues (memory leaks, cache degradation)
- Ensure stability before declaring success

### Canary Deployment Monitoring

**Per Canary Stage**:
- 5% traffic: 10 minutes
- 25% traffic: 10 minutes
- 50% traffic: 10 minutes
- 100% traffic: 15 minutes

**Total Canary Duration**: ~45 minutes

---

## Metrics Visualization Examples

### Dashboard Layout (Grafana/Datadog)

**Panel 1: Error Rate Comparison**
```
Graph: Line chart
Queries:
  - Current error rate (last 15 minutes)
  - Baseline error rate (dashed line)
Alert bands:
  - Yellow: 1.2x baseline
  - Red: 2.0x baseline
```

**Panel 2: Response Time Distribution**
```
Graph: Heatmap
Metrics:
  - p50, p75, p95, p99 response times
Comparison: Current vs. baseline
```

**Panel 3: Resource Utilization**
```
Graph: Gauge charts
Metrics:
  - CPU: 0-100% (critical at 80%)
  - Memory: 0-100% (critical at 85%)
  - DB Connections: 0-100% (critical at 80%)
```

**Panel 4: Traffic Patterns**
```
Graph: Area chart
Metrics:
  - Request rate (RPS)
  - Comparison to historical traffic
```

---

## Best Practices

1. **Establish Baselines Before Deployment**: Always capture baseline metrics 7-14 days before release
2. **Use Percentiles, Not Averages**: p95/p99 reveal worst-case performance
3. **Monitor Trends, Not Just Snapshots**: Upward trend may require rollback even if below threshold
4. **Account for Traffic Patterns**: Compare same time-of-day metrics (Monday 10am vs Monday 10am)
5. **Set Conservative Thresholds Initially**: Can relax after observing deployment behavior
6. **Automate Monitoring**: Use scripts/tools for consistent, repeatable monitoring
7. **Document Anomalies**: Track false positives to refine thresholds
8. **Alert Fatigue Prevention**: Too many alerts → ignored alerts. Use meaningful thresholds.

---

## Troubleshooting Common Issues

### Metric Collection Fails

**Symptom**: Cannot retrieve metrics from monitoring system

**Causes**:
- Authentication failure (expired credentials)
- Network connectivity issue
- Monitoring agent not running

**Resolution**:
```bash
# Test connectivity
curl -H "Authorization: Bearer $TOKEN" https://monitoring-api.example.com/health

# Check credentials
aws sts get-caller-identity  # AWS
az account show  # Azure

# Verify agent running
kubectl get pods -n monitoring
```

### Metrics Show No Data

**Symptom**: Metrics query returns empty result

**Causes**:
- Deployment not sending metrics yet
- Metric name/namespace incorrect
- Time window too narrow

**Resolution**:
```python
# Expand time window
start_time = now - timedelta(minutes=30)  # Instead of 15 minutes

# Verify metric name
available_metrics = cloudwatch.list_metrics(Namespace='AWS/ApplicationELB')
print(available_metrics)
```

### Baseline Comparison Invalid

**Symptom**: All metrics flagged as anomalies

**Causes**:
- Baseline from different traffic profile
- Baseline too old (application changed)
- Seasonal traffic not accounted for

**Resolution**:
```python
# Recalculate baseline from recent history
baseline = calculate_baseline(
    start_date=now - timedelta(days=7),
    end_date=now,
    same_time_of_week=True  # Match day of week and hour
)
```

---

**Use this reference during release workflow to ensure production deployments are properly monitored and validated against established baselines.**
