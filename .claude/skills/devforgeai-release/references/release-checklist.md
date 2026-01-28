# Release Checklist Reference

This document provides comprehensive checklists for pre-deployment, deployment, and post-deployment activities to ensure safe, repeatable releases.

## Overview

**Purpose**: Ensure all release activities completed systematically with no missed steps.

**Usage**: Reference during all phases of release workflow. Check off items as completed for audit trail.

---

## Pre-Deployment Checklist

### Quality Validation (MANDATORY)

**Status Gate**: MUST pass before proceeding

- [ ] **QA Approval**: Story status = "QA Approved"
- [ ] **QA Report Exists**: `devforgeai/qa/reports/{story-id}-qa-report.md` present
- [ ] **QA Status**: Report shows PASS status
- [ ] **Zero Critical Violations**: No CRITICAL issues in QA report
- [ ] **Zero High Violations**: No HIGH issues (or documented exceptions approved)
- [ ] **Coverage Thresholds Met**:
  - [ ] Business logic: ≥ 95%
  - [ ] Application layer: ≥ 85%
  - [ ] Infrastructure: ≥ 80%
- [ ] **All Acceptance Criteria Validated**: QA report confirms all AC met
- [ ] **Test Execution**: 100% pass rate on all tests

**If ANY item fails**: BLOCK release, return to QA phase

---

### Dependency Verification

- [ ] **Blocking Dependencies Resolved**: All dependent stories deployed or included in release
- [ ] **Dependency Graph Validated**: No circular dependencies
- [ ] **Shared Library Versions**: Compatible versions across all services
- [ ] **API Contract Compatibility**: No breaking changes to published APIs
- [ ] **Database Schema Compatibility**: New code compatible with existing schema
- [ ] **Feature Flag Configuration**: Required feature flags configured in all environments

**Dependency Issues**:
- If unresolved → Use AskUserQuestion to decide: block, deploy together, or proceed with risk

---

### Environment Readiness

**Staging Environment**:
- [ ] **Staging Available**: Environment accessible and healthy
- [ ] **Staging Configuration**: Mirrors production configuration
- [ ] **Staging Data**: Representative test data loaded
- [ ] **Staging Credentials**: Valid and tested
- [ ] **Staging Infrastructure**: Sufficient capacity for deployment

**Production Environment**:
- [ ] **Production Available**: Environment accessible and healthy
- [ ] **Production Baseline Metrics**: Captured and stored
- [ ] **Production Credentials**: Valid and tested
- [ ] **Production Capacity**: Sufficient resources for deployment
- [ ] **No Conflicting Deployments**: No other releases in progress

**Infrastructure Validation**:
- [ ] **Kubernetes**: `kubectl get pods` succeeds in all namespaces
- [ ] **Azure**: `az webapp show` succeeds for staging and production apps
- [ ] **AWS**: `aws ecs describe-services` succeeds for staging and production
- [ ] **Load Balancer**: Health checks passing
- [ ] **Database**: Connection pools healthy, no locks

**If Environment Not Ready**: BLOCK until resolved

---

### Code & Artifact Preparation

- [ ] **Code Frozen**: No additional commits after QA approval
- [ ] **Branch Created**: Release branch created from main
- [ ] **Version Tag**: Git tag created with version number
- [ ] **Changelog Updated**: CHANGELOG.md includes this release
- [ ] **Release Notes Draft**: Initial release notes prepared
- [ ] **Build Successful**: Code builds without errors
- [ ] **Artifacts Created**:
  - [ ] Docker image built (if applicable)
  - [ ] .NET publish artifact (if applicable)
  - [ ] npm build output (if applicable)
  - [ ] Python wheel/package (if applicable)
- [ ] **Artifacts Pushed**: Published to registry/artifact store
- [ ] **Artifact Integrity**: Checksums verified

---

### Deployment Strategy Selection

- [ ] **Strategy Selected**: Blue-Green, Rolling, Canary, or Recreate
- [ ] **Strategy Rationale**: Documented reason for strategy choice
- [ ] **Resource Requirements**: Infrastructure supports chosen strategy
  - [ ] Blue-Green: 2x capacity available
  - [ ] Canary: Traffic routing configured
  - [ ] Rolling: Health checks configured
  - [ ] Recreate: Downtime window approved
- [ ] **Rollback Plan**: Strategy-specific rollback procedure documented
- [ ] **Deployment Configuration**: YAML/config files updated with new version

---

### Database Changes

**If deployment includes database migrations:**

- [ ] **Migration Scripts**: Reviewed and tested in staging
- [ ] **Migration Direction**: Both up and down migrations exist
- [ ] **Data Backup Plan**: Backup strategy documented
- [ ] **Backup Tested**: Restore procedure verified in staging
- [ ] **Migration Duration**: Estimated time documented
- [ ] **Zero-Downtime Compatible**: Migration allows old code to run during rollout
- [ ] **Rollback Migration**: Down migration tested
- [ ] **Data Volume**: Migration performance tested with production-scale data

**If NO database changes**: Skip this section

---

### Monitoring & Alerting

- [ ] **Baseline Metrics Captured**: 7-14 day historical baseline saved
- [ ] **Monitoring Dashboard**: Updated with deployment markers
- [ ] **Alert Thresholds Configured**:
  - [ ] Error rate: > 2x baseline triggers alert
  - [ ] Response time (p95): > 1.5x baseline triggers alert
  - [ ] CPU: > 80% triggers alert
  - [ ] Memory: > 85% triggers alert
- [ ] **Monitoring Tools Available**: CloudWatch/Datadog/Prometheus accessible
- [ ] **On-Call Schedule**: Engineer available for 24 hours post-deployment
- [ ] **Escalation Path**: Contact list updated

---

### Communication & Documentation

- [ ] **Stakeholders Notified**: Product owner, tech lead, affected teams informed
- [ ] **Deployment Window**: Scheduled and communicated
- [ ] **Maintenance Banner**: Prepared (if downtime expected)
- [ ] **Deployment Runbook**: Step-by-step procedure documented
- [ ] **Rollback Procedure**: Documented and accessible
- [ ] **Post-Deployment Tasks**: Checklist for after deployment
- [ ] **User Documentation**: Updated (if user-facing changes)
- [ ] **API Documentation**: Updated (if API changes)

---

### Risk Assessment & Mitigation

- [ ] **Risk Analysis Complete**: Risks identified and documented
- [ ] **High-Risk Items**: Mitigation plans in place
- [ ] **Rollback Trigger Criteria**: Clear conditions for rollback
- [ ] **Emergency Contacts**: List of experts for troubleshooting
- [ ] **Business Impact**: Estimated user impact if deployment fails
- [ ] **Deployment Window**: Scheduled during low-traffic period (if high-risk)

---

### Pre-Deployment Sign-Off

**Required Approvals** (adapt to organization):

- [ ] **Tech Lead Approval**: Technical review complete
- [ ] **QA Lead Sign-Off**: Quality validation approved
- [ ] **Product Owner Acknowledgment**: Business stakeholder aware
- [ ] **DevOps Approval**: Infrastructure team ready (if required)
- [ ] **Security Review**: Security-sensitive changes reviewed (if applicable)
- [ ] **Compliance Check**: Regulatory requirements met (if applicable)

**Final Go/No-Go Decision**:
- [ ] **All Pre-Deployment Checks Passed**: Review checklist above
- [ ] **Deployment Team Ready**: All required personnel available
- [ ] **Proceed to Staging Deployment**: GREEN LIGHT

---

## Deployment Checklist

### Staging Deployment

**Phase 2 of Release Workflow**

- [ ] **Artifacts Deployed to Staging**: Version {version} deployed
- [ ] **Deployment Status**: Rollout completed successfully
- [ ] **Pods/Instances Healthy**: All replicas running and ready
- [ ] **Health Endpoint Check**: `GET /health` returns 200 OK
- [ ] **Logs Review**: No error logs during startup
- [ ] **Configuration Validation**: Environment variables correct

**Smoke Tests (Staging)**:
- [ ] **Health Check**: Application responding
- [ ] **Authentication**: Login/auth flow works
- [ ] **Critical Path Test**: Primary user workflow functional
- [ ] **API Endpoints**: Key APIs responding correctly
- [ ] **Database Connectivity**: Queries succeeding
- [ ] **External Integrations**: Third-party APIs reachable
- [ ] **Static Assets**: CSS/JS/images loading
- [ ] **Smoke Test Suite**: Automated tests pass

**Staging Validation**:
- [ ] **Functional Validation**: Smoke tests passed
- [ ] **Performance Check**: Response times acceptable
- [ ] **Error Logs**: No errors or exceptions
- [ ] **Resource Usage**: CPU/Memory within normal range
- [ ] **Manual Testing**: Optional manual validation complete (if required)

**Decision Point**:
- [ ] **Staging Validation Passed**: Ready to proceed to production
- [ ] **OR Staging Failed**: Rollback staging, fix issues, restart release

---

### Production Deployment

**Phase 3 of Release Workflow**

**Pre-Production Confirmation**:
- [ ] **Staging Validated**: All staging checks passed
- [ ] **Production Readiness**: Final confirmation received
- [ ] **Deployment Window**: Within approved maintenance window
- [ ] **Rollback Plan Ready**: Procedures documented and accessible
- [ ] **Monitoring Active**: Dashboards open and ready

**Deployment Execution** (strategy-specific):

**Blue-Green Deployment**:
- [ ] **Green Environment Deployed**: New version deployed to green
- [ ] **Green Health Checks**: All pods/instances healthy
- [ ] **Green Smoke Tests**: Tests passed in green environment
- [ ] **Traffic Switch**: Load balancer redirected to green
- [ ] **Blue Environment Retained**: Old version kept as rollback option

**Rolling Update Deployment**:
- [ ] **Rolling Update Started**: Kubernetes/ECS rolling update initiated
- [ ] **Rollout Progress**: Monitoring pod replacement progress
- [ ] **Health Checks**: New pods passing health checks
- [ ] **Rollout Complete**: All replicas updated successfully

**Canary Deployment**:
- [ ] **Canary 5% Deployed**: Small canary instance deployed
- [ ] **5% Traffic Routed**: Traffic split configured (95% baseline, 5% canary)
- [ ] **5% Monitoring (10 min)**: Metrics healthy at 5%
- [ ] **Canary 25% Increased**: Canary scaled, traffic increased to 25%
- [ ] **25% Monitoring (10 min)**: Metrics healthy at 25%
- [ ] **Canary 50% Increased**: Traffic increased to 50%
- [ ] **50% Monitoring (10 min)**: Metrics healthy at 50%
- [ ] **Canary 100% Complete**: All traffic routed to new version
- [ ] **Baseline Scaled Down**: Old version decommissioned

**Recreate Deployment**:
- [ ] **Old Version Stopped**: Previous deployment scaled to 0
- [ ] **New Version Deployed**: New deployment started
- [ ] **Pods/Instances Healthy**: All replicas running
- [ ] **Health Checks Passing**: Application responsive
- [ ] **Downtime Duration**: Recorded for post-deployment review

**Database Migration** (if applicable):
- [ ] **Backup Created**: Database backed up before migration
- [ ] **Migration Applied**: Schema changes executed
- [ ] **Migration Verified**: Schema version correct
- [ ] **Data Integrity**: Sample queries validated

---

### Post-Deployment Validation

**Phase 4 of Release Workflow**

**Production Smoke Tests**:
- [ ] **Health Endpoint**: `/health` returns 200 OK
- [ ] **Authentication**: Login flow works
- [ ] **Critical Path**: Primary user workflow functional
- [ ] **API Endpoints**: Key APIs responding
- [ ] **Database Queries**: Data access working
- [ ] **External Integrations**: Third-party services reachable
- [ ] **Smoke Test Suite**: Automated tests pass

**Metrics Monitoring (15 minutes)**:
- [ ] **Error Rate**: Within 1.2x baseline
- [ ] **Response Time (p95)**: Within 1.3x baseline
- [ ] **Request Rate**: Traffic levels normal
- [ ] **CPU Utilization**: < 80%
- [ ] **Memory Usage**: < 85%
- [ ] **Database Connections**: < 80% pool usage
- [ ] **Cache Hit Rate**: Within 0.8x baseline

**Logs Review**:
- [ ] **No Critical Errors**: No ERROR or CRITICAL log entries
- [ ] **No Exceptions**: No uncaught exceptions
- [ ] **Startup Logs Clean**: Application initialized correctly
- [ ] **No Warnings**: Or warnings understood and acceptable

**User Validation** (optional):
- [ ] **Manual UAT**: Optional user acceptance testing complete
- [ ] **User Feedback**: No critical issues reported
- [ ] **Support Tickets**: No spike in support requests

**Decision Point**:
- [ ] **Validation Passed**: Deployment successful
- [ ] **OR Validation Failed**: Execute rollback procedure

---

## Post-Deployment Checklist

### Immediate Post-Deployment (0-1 hour)

- [ ] **Deployment Status**: Confirmed successful
- [ ] **All Smoke Tests Passed**: Functional validation complete
- [ ] **Metrics Baseline**: Initial metrics within acceptable thresholds
- [ ] **No Critical Alerts**: Monitoring systems quiet
- [ ] **User Feedback Channels**: Monitored for immediate issues
- [ ] **Support Team Notified**: Aware of deployment, ready for questions

---

### Documentation Updates

**Release Documentation**:
- [ ] **Release Notes Generated**: `devforgeai/releases/release-{version}.md` created
- [ ] **Story Status Updated**: Status changed to "Released"
- [ ] **Story Metadata**: Version and release date added to frontmatter
- [ ] **Workflow History**: Release details appended to story
- [ ] **Changelog Updated**: `CHANGELOG.md` includes new version
- [ ] **Sprint Progress Updated**: Sprint document marked story complete
- [ ] **Git Tag Pushed**: Version tag available in repository

**Operational Documentation**:
- [ ] **Runbook Updated**: Deployment procedure refined (if improvements identified)
- [ ] **Configuration Changes**: Config management updated (Ansible, Terraform, etc.)
- [ ] **Known Issues**: Documented (if any discovered during deployment)
- [ ] **Workarounds**: Documented (if needed for known issues)

---

### Post-Release Monitoring Setup

**Monitoring Configuration**:
- [ ] **24-Hour Alerts Configured**: Post-release monitoring enabled
- [ ] **Alert Thresholds**: Set to 1.5x baseline for 24-hour window
- [ ] **On-Call Notification**: Engineer assigned for monitoring period
- [ ] **Dashboard Markers**: Deployment timestamp marked on dashboards
- [ ] **Monitoring Report Scheduled**: 24-hour post-release review task created

**Monitoring Focus Areas**:
- [ ] **Error Rate Tracking**: Continuous monitoring for spikes
- [ ] **Response Time Trends**: Watching for degradation
- [ ] **Resource Utilization**: Memory leaks or CPU creep
- [ ] **User Complaints**: Support ticket monitoring
- [ ] **External Dependencies**: Third-party service health

---

### Stakeholder Communication

- [ ] **Deployment Complete Notification**: Stakeholders informed of success
- [ ] **Release Notes Shared**: Distributed to relevant teams
- [ ] **Known Issues Communicated**: If any discovered
- [ ] **Next Steps Documented**: Follow-up tasks identified
- [ ] **Success Metrics**: Initial metrics shared (error rate, response time)

---

### Rollback Plan Confirmation

- [ ] **Rollback Procedure Validated**: Steps confirmed accessible
- [ ] **Rollback Artifact**: Previous version available in registry
- [ ] **Rollback Command**: Documented and tested (in staging if possible)
- [ ] **Database Rollback Plan**: Down migration or backup restore plan ready
- [ ] **Rollback Trigger Criteria**: Clear conditions documented

---

### 24-Hour Post-Deployment Review

**Scheduled**: {current_timestamp + 24 hours}

**Review Activities**:
- [ ] **Metrics Analysis**: 24-hour metrics compared to baseline
  - [ ] Error rate: < 1.2x baseline
  - [ ] Response time (p95): < 1.3x baseline
  - [ ] CPU utilization: < 70%
  - [ ] Memory usage: < 75%
- [ ] **User Feedback Review**: Support tickets analyzed
- [ ] **Log Analysis**: 24-hour logs reviewed for patterns
- [ ] **Incident Count**: Number of production incidents (target: 0)
- [ ] **Rollback Count**: Number of rollbacks required (target: 0)

**Lessons Learned**:
- [ ] **What Went Well**: Successful aspects documented
- [ ] **What Went Wrong**: Issues encountered documented
- [ ] **Improvements Identified**: Process improvements noted
- [ ] **Action Items**: Follow-up tasks created

**Release Sign-Off**:
- [ ] **Tech Lead Sign-Off**: Technical review complete
- [ ] **Product Owner Acknowledgment**: Business stakeholder notified
- [ ] **Release Declared Stable**: Monitoring period complete, release successful

---

## Rollback Checklist

**Use this if deployment fails and rollback required**

### Rollback Decision

**Triggers** (automatic rollback recommended if):
- [ ] Health check fails
- [ ] Smoke tests fail
- [ ] Error rate > 2x baseline
- [ ] Critical service unavailable
- [ ] Database migration fails with data loss risk

**Decision Point**:
- [ ] **Rollback Confirmed**: User or automatic trigger initiated rollback
- [ ] **Rollback Reason Documented**: Why rollback required

---

### Rollback Execution

**Platform-Specific Rollback**:

**Kubernetes**:
- [ ] **Rollback Command Executed**: `kubectl rollout undo deployment/{name}`
- [ ] **Rollback Status**: `kubectl rollout status` confirms success
- [ ] **Pods Healthy**: Previous version pods running

**Azure App Service**:
- [ ] **Slot Swap**: Swapped staging/production slots back
- [ ] **Previous Version Active**: Old version serving traffic

**AWS ECS**:
- [ ] **Task Definition Reverted**: Previous task definition deployed
- [ ] **Service Stable**: `aws ecs wait services-stable` succeeds

**Docker**:
- [ ] **Previous Image Deployed**: Old Docker image tag deployed
- [ ] **Containers Running**: Containers healthy

**Database Rollback** (if needed):
- [ ] **Migration Rolled Back**: Down migration executed
- [ ] **OR Backup Restored**: Database restored from backup (if migration failed)
- [ ] **Data Integrity Verified**: Sample queries validated

---

### Post-Rollback Validation

- [ ] **Health Check**: Application responding
- [ ] **Smoke Tests**: Critical paths functional
- [ ] **Metrics Stabilized**: Error rate and response time normal
- [ ] **User Impact**: Service availability confirmed
- [ ] **Downtime Duration**: Calculated and documented

---

### Post-Rollback Documentation

- [ ] **Rollback Report Created**: `devforgeai/releases/rollback-{version}.md`
- [ ] **Story Status Reverted**: Changed back to "QA Approved"
- [ ] **Root Cause Analysis Initiated**: Investigation started
- [ ] **Hotfix Story Created**: (Optional) New story for fix
- [ ] **Lessons Learned**: Documented for future releases
- [ ] **Monitoring Reset**: Alerts cleared, baseline restored

---

### Post-Rollback Communication

- [ ] **Stakeholders Notified**: Rollback announced
- [ ] **User Communication**: External communication (if user-facing)
- [ ] **Incident Report**: Post-mortem scheduled
- [ ] **Next Steps Communicated**: Plan for re-deployment

---

## Hotfix Release Checklist

**Expedited release process for critical production issues**

### Hotfix Pre-Deployment

- [ ] **Critical Issue Validated**: Production bug confirmed
- [ ] **Hotfix Story Created**: HOTFIX-XXX story created
- [ ] **Fix Implemented**: Code fix completed
- [ ] **QA Validation**: Even hotfixes require QA approval
- [ ] **Regression Tests**: Ensure fix doesn't break other functionality
- [ ] **Expedite Approval**: Fast-track approval from stakeholders

### Hotfix Deployment

- [ ] **Follow Standard Process**: Use standard deployment checklist (do NOT skip steps)
- [ ] **Accelerated Timeline**: Compress timelines, not skip validation
- [ ] **Extra Vigilance**: Monitor more closely due to urgency
- [ ] **Communication**: Extra transparency due to off-schedule release

---

## Checklist Usage Guidelines

### How to Use This Checklist

1. **Print or Open in Editor**: Keep checklist visible during release
2. **Check Off Items**: Mark completed items in real-time
3. **Document Exceptions**: Note why any item skipped (with justification)
4. **Store Completed Checklist**: Attach to release notes for audit trail
5. **Review After Release**: Identify process improvements

### Checklist Adaptation

**Adapt this checklist to your organization**:
- Add items specific to your infrastructure
- Remove items not applicable
- Adjust approval requirements
- Customize for regulatory compliance

### Checklist Versioning

**Update this checklist when**:
- New deployment platform added
- Process improvements identified
- Regulatory requirements change
- Post-mortems reveal gaps

---

## Sign-Off Sheet Template

**Use this for formal releases requiring approvals**

```
═══════════════════════════════════════════════════════════
Release Sign-Off: {story_id} - Version {version}
═══════════════════════════════════════════════════════════

Date: {current_date}
Release Engineer: __________________________

PRE-DEPLOYMENT APPROVALS:
□ Tech Lead:          _____________ Date: _______
□ QA Lead:            _____________ Date: _______
□ Product Owner:      _____________ Date: _______
□ DevOps Lead:        _____________ Date: _______ (if required)

DEPLOYMENT EXECUTION:
□ Staging Deployed:   _____________ Time: _______
□ Staging Validated:  _____________ Time: _______
□ Production Deployed: _____________ Time: _______
□ Production Validated: _____________ Time: _______

POST-DEPLOYMENT SIGN-OFF:
□ 24-Hour Review:     _____________ Date: _______
□ Release Stable:     _____________ Date: _______
□ Monitoring Period Complete: _____________ Date: _______

FINAL APPROVAL:
□ Tech Lead:          _____________ Date: _______
□ Product Owner:      _____________ Date: _______

═══════════════════════════════════════════════════════════
```

---

**This checklist ensures comprehensive, repeatable release processes with clear accountability and audit trail for all deployments.**
