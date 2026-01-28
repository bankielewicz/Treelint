### Phase 1: Pre-Release Validation

**Objective**: Verify release readiness before deployment begins

#### Step 1: Load Story and QA Report

```
Read(file_path="devforgeai/specs/Stories/{story_id}.story.md")

HALT if story.status != "QA Approved":
    "Story must be QA Approved before deployment"

Read(file_path="devforgeai/qa/reports/{story_id}-qa-report.md")

HALT if qa_report NOT exists:
    "QA report not found. Run devforgeai-qa --mode=deep first"
```

#### Step 2: Validate Release Gates

**Gate 1: QA Approval**
- Story status == "QA Approved"
- QA report status == "PASS"
- Zero CRITICAL violations
- Zero HIGH violations (or documented exceptions)
- Coverage meets thresholds (95%/85%/80%)

**Gate 2: Dependency Check**
```
dependencies = extract_dependencies(story)

FOR each blocking_dependency:
    IF dependency_status != "Released":
        AskUserQuestion about proceeding or blocking
```

**Gate 3: Environment Readiness**
```
# Verify deployment environments available
Bash(command="kubectl get pods --namespace=staging")  # or platform-specific check
Bash(command="kubectl get pods --namespace=production")

HALT if environments unavailable:
    "Cannot connect to deployment environments"
```

#### Step 3: Select Deployment Strategy

```
AskUserQuestion:
Question: "Which deployment strategy for {story_id}?"
Header: "Deployment strategy"
Options:
  - "Blue-Green (Zero downtime, instant rollback, 2x resources)"
  - "Rolling Update (Gradual, minimal resources)"
  - "Canary (Progressive 5%→25%→50%→100%)"
  - "Recreate (Simple, brief downtime OK)"
multiSelect: false

deployment_strategy = answer
```

For deployment strategy details, see `references/deployment-strategies.md`

---

