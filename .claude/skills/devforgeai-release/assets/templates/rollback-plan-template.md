# Rollback Report: {{VERSION}}

**Rollback Date**: {{ROLLBACK_TIMESTAMP}}
**Story ID**: {{STORY_ID}}
**Rolled Back By**: {{ROLLBACK_ENGINEER}}
**Previous Version Restored**: {{PREVIOUS_VERSION}}

---

## Rollback Summary

**Reason**: {{ROLLBACK_REASON}}

**Status**: {{ROLLBACK_STATUS}}

---

## Deployment Details

### Original Deployment

- **Deployment Started**: {{DEPLOYMENT_TIMESTAMP}}
- **Deployment Completed**: {{DEPLOYMENT_COMPLETE_TIMESTAMP}}
- **Deployment Duration**: {{DEPLOYMENT_DURATION}}
- **Deployment Strategy**: {{DEPLOYMENT_STRATEGY}}

### Rollback Execution

- **Rollback Initiated**: {{ROLLBACK_INITIATED_TIMESTAMP}}
- **Rollback Completed**: {{ROLLBACK_COMPLETED_TIMESTAMP}}
- **Rollback Duration**: {{ROLLBACK_DURATION}}
- **Rollback Method**: {{ROLLBACK_METHOD}}

---

## Root Cause

### Issue Description

{{ROOT_CAUSE_DESCRIPTION}}

### Trigger Event

{{TRIGGER_EVENT_DESCRIPTION}}

**Detection Time**: {{ISSUE_DETECTION_TIME}}
**Time to Rollback Decision**: {{TIME_TO_DECISION}}

---

## Impact Assessment

### User Impact

- **Affected Users**: {{AFFECTED_USERS_COUNT}} (estimated)
- **User-Facing Errors**: {{USER_FACING_ERRORS}}
- **Service Availability**: {{SERVICE_AVAILABILITY_PERCENTAGE}}%

### System Impact

- **Downtime Duration**: {{DOWNTIME_DURATION}}
- **Degraded Performance Period**: {{DEGRADED_PERIOD}}
- **Failed Requests**: {{FAILED_REQUESTS_COUNT}}
- **Error Rate Peak**: {{ERROR_RATE_PEAK}}%

### Business Impact

{{BUSINESS_IMPACT_DESCRIPTION}}

---

## Actions Taken

### Rollback Procedure

#### 1. Rollback Decision

**Time**: {{ROLLBACK_DECISION_TIME}}
**Decision Maker**: {{DECISION_MAKER}}
**Rationale**: {{ROLLBACK_DECISION_RATIONALE}}

#### 2. Application Rollback

**Platform**: {{DEPLOYMENT_PLATFORM}}

**Commands Executed**:
```bash
{{ROLLBACK_COMMANDS}}
```

**Rollback Status**: {{APPLICATION_ROLLBACK_STATUS}}

#### 3. Database Rollback

{{#IF_DATABASE_ROLLBACK}}
**Database Changes**: {{DATABASE_CHANGES_DESCRIPTION}}

**Rollback Method**: {{DATABASE_ROLLBACK_METHOD}}

**Commands Executed**:
```bash
{{DATABASE_ROLLBACK_COMMANDS}}
```

**Database Rollback Status**: {{DATABASE_ROLLBACK_STATUS}}

**Data Loss**: {{DATA_LOSS_ASSESSMENT}}
{{/IF_DATABASE_ROLLBACK}}

{{#IF_NO_DATABASE_ROLLBACK}}
**No database rollback required** (no schema changes in failed deployment)
{{/IF_NO_DATABASE_ROLLBACK}}

#### 4. Configuration Rollback

{{#IF_CONFIG_ROLLBACK}}
**Configuration Changes Reverted**:
{{CONFIG_CHANGES_REVERTED_LIST}}
{{/IF_CONFIG_ROLLBACK}}

{{#IF_NO_CONFIG_ROLLBACK}}
**No configuration rollback required**
{{/IF_NO_CONFIG_ROLLBACK}}

#### 5. Validation

**Post-Rollback Validation**:
- [x] Health checks passing
- [x] Smoke tests passing
- [x] Error rate normalized
- [x] Response time normalized
- [x] User access restored

**Validation Timestamp**: {{VALIDATION_COMPLETE_TIMESTAMP}}

---

## Technical Details

### Error Logs

{{#IF_ERROR_LOGS}}
**Sample Error Logs**:
```
{{ERROR_LOG_SAMPLE}}
```

**Full Log Location**: {{ERROR_LOG_LOCATION}}
{{/IF_ERROR_LOGS}}

### Metrics During Failure

| Metric | Before Deployment | During Failure | After Rollback | Baseline |
|--------|-------------------|----------------|----------------|----------|
| **Error Rate** | {{ERROR_RATE_BEFORE}}% | {{ERROR_RATE_DURING}}% | {{ERROR_RATE_AFTER}}% | {{ERROR_RATE_BASELINE}}% |
| **Response Time (p95)** | {{RESPONSE_TIME_BEFORE}}ms | {{RESPONSE_TIME_DURING}}ms | {{RESPONSE_TIME_AFTER}}ms | {{RESPONSE_TIME_BASELINE}}ms |
| **CPU** | {{CPU_BEFORE}}% | {{CPU_DURING}}% | {{CPU_AFTER}}% | {{CPU_BASELINE}}% |
| **Memory** | {{MEMORY_BEFORE}}% | {{MEMORY_DURING}}% | {{MEMORY_AFTER}}% | {{MEMORY_BASELINE}}% |

### Failed Validations

{{FAILED_VALIDATIONS_LIST}}

---

## Timeline

| Time | Event |
|------|-------|
| {{DEPLOYMENT_START_TIME}} | Deployment to production initiated |
| {{DEPLOYMENT_COMPLETE_TIME}} | Deployment completed |
| {{ISSUE_DETECTED_TIME}} | Issue detected ({{DETECTION_METHOD}}) |
| {{ROLLBACK_DECISION_TIME}} | Rollback decision made |
| {{ROLLBACK_START_TIME}} | Rollback initiated |
| {{ROLLBACK_COMPLETE_TIME}} | Rollback completed |
| {{VALIDATION_COMPLETE_TIME}} | Post-rollback validation complete |
| {{SERVICE_RESTORED_TIME}} | Service fully restored |

**Total Incident Duration**: {{TOTAL_INCIDENT_DURATION}}

---

## Communication

### Stakeholders Notified

- [x] Tech Lead ({{TECH_LEAD_NAME}}) - Notified at {{TECH_LEAD_NOTIFY_TIME}}
- [x] Product Owner ({{PRODUCT_OWNER_NAME}}) - Notified at {{PRODUCT_OWNER_NOTIFY_TIME}}
- [x] QA Lead ({{QA_LEAD_NAME}}) - Notified at {{QA_LEAD_NOTIFY_TIME}}
- [x] Support Team - Notified at {{SUPPORT_NOTIFY_TIME}}
{{#IF_CUSTOMER_COMMUNICATION}}
- [x] Customers - Status page updated at {{CUSTOMER_NOTIFY_TIME}}
{{/IF_CUSTOMER_COMMUNICATION}}

### Communication Channels

{{COMMUNICATION_CHANNELS_USED}}

---

## Post-Rollback Actions

### Immediate Actions Completed

- [x] Application rolled back to {{PREVIOUS_VERSION}}
- [x] Database restored/migrated back (if applicable)
- [x] Health checks passing
- [x] Monitoring normalized
- [x] Stakeholders notified
- [x] Rollback report created

### Follow-Up Actions Required

- [ ] **Root Cause Analysis**: Scheduled for {{RCA_SCHEDULED_DATE}}
- [ ] **Fix Development**: {{#IF_HOTFIX_STORY}}Hotfix story {{HOTFIX_STORY_ID}} created{{/IF_HOTFIX_STORY}}{{#IF_NO_HOTFIX}}Address in next sprint{{/IF_NO_HOTFIX}}
- [ ] **Testing**: Additional tests to prevent recurrence
- [ ] **Process Improvement**: Review deployment checklist
- [ ] **Documentation**: Update runbooks with lessons learned

---

## Root Cause Analysis

### Investigation Status

**Status**: {{RCA_STATUS}}
**Lead Investigator**: {{RCA_LEAD}}
**Target Completion**: {{RCA_TARGET_DATE}}

### Preliminary Findings

{{PRELIMINARY_FINDINGS}}

### Contributing Factors

{{CONTRIBUTING_FACTORS_LIST}}

### Preventive Measures

{{PREVENTIVE_MEASURES_LIST}}

---

## Hotfix Plan

{{#IF_HOTFIX_REQUIRED}}
**Hotfix Required**: YES

**Hotfix Story**: {{HOTFIX_STORY_ID}}
**Hotfix Priority**: {{HOTFIX_PRIORITY}}
**Estimated Fix Duration**: {{HOTFIX_ESTIMATED_DURATION}}

### Hotfix Approach

{{HOTFIX_APPROACH_DESCRIPTION}}

### Hotfix Testing Plan

{{HOTFIX_TESTING_PLAN}}

### Re-Deployment Timeline

**Target Re-Deployment**: {{REDEPLOYMENT_TARGET_DATE}}

{{/IF_HOTFIX_REQUIRED}}

{{#IF_NO_HOTFIX}}
**Hotfix Required**: NO

**Reason**: {{NO_HOTFIX_REASON}}

**Alternative Plan**: {{ALTERNATIVE_PLAN}}
{{/IF_NO_HOTFIX}}

---

## Lessons Learned

### What Went Wrong

{{WHAT_WENT_WRONG_LIST}}

### What Went Right

{{WHAT_WENT_RIGHT_LIST}}

### Process Improvements

{{PROCESS_IMPROVEMENTS_LIST}}

### Technical Debt Identified

{{TECHNICAL_DEBT_LIST}}

---

## Preventive Actions

### Immediate Changes

{{IMMEDIATE_CHANGES_LIST}}

### Long-Term Improvements

{{LONG_TERM_IMPROVEMENTS_LIST}}

### Testing Enhancements

{{TESTING_ENHANCEMENTS_LIST}}

### Monitoring Improvements

{{MONITORING_IMPROVEMENTS_LIST}}

---

## References

- **Story Document**: `devforgeai/specs/Stories/{{STORY_ID}}.story.md`
- **Failed Release Notes**: `devforgeai/releases/release-{{VERSION}}.md`
- **QA Report**: `devforgeai/qa/reports/{{STORY_ID}}-qa-report.md`
- **Error Logs**: {{ERROR_LOG_LOCATION}}
- **Metrics Dashboard**: {{METRICS_DASHBOARD_URL}}

---

## Sign-Off

### Rollback Approval

**Rollback Engineer**: {{ROLLBACK_ENGINEER}} - {{ROLLBACK_TIMESTAMP}}
**Tech Lead Approval**: _____________ Date: _______
**Product Owner Acknowledgment**: _____________ Date: _______

### RCA Completion

**RCA Lead**: _____________ Date: _______
**Findings Reviewed By**: _____________ Date: _______

### Re-Deployment Approval

**QA Approval (for hotfix)**: _____________ Date: _______
**Tech Lead Approval**: _____________ Date: _______

---

**Status**: {{ROLLBACK_FINAL_STATUS}}

**Next Steps**:
1. Complete root cause analysis by {{RCA_TARGET_DATE}}
2. {{#IF_HOTFIX_REQUIRED}}Implement and deploy hotfix {{HOTFIX_STORY_ID}}{{/IF_HOTFIX_REQUIRED}}{{#IF_NO_HOTFIX}}Address in next sprint{{/IF_NO_HOTFIX}}
3. Implement preventive measures
4. Update deployment procedures to prevent recurrence

---

*Rollback report generated by devforgeai-release skill*
*Generated at: {{GENERATION_TIMESTAMP}}*
