### Phase 6: Post-Release Monitoring & Closure

**Objective**: Monitor production stability and close release

#### Step 1: Configure Post-Release Monitoring

```
alert_config = {
    "error_rate_threshold": baseline.error_rate * 1.5,
    "response_time_threshold": baseline.p95 * 1.5,
    "duration": "24 hours",
    "story_id": story_id,
    "version": version
}

Write(file_path="devforgeai/monitoring/alerts/{story_id}-post-release.json", content=alert_config)
```

For monitoring guidance, see `references/monitoring-metrics.md`

#### Step 2: Schedule Post-Deployment Review

```
review_document = """# Post-Release Review: {story_id}

**Version:** {version}
**Released:** {timestamp}
**Review Date:** {timestamp + 24 hours}

