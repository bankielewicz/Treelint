### Phase 2: Staging Deployment

**Objective**: Deploy to staging environment and validate before production

#### Step 1: Prepare Deployment Artifacts

**Git Workflow:**
```
Bash(command="git checkout main && git pull origin main")
Bash(command="git checkout -b release/{story_id}")
Bash(command="git tag -a {version} -m 'Release {story_id}: {story.title}'")
Bash(command="git push origin {version}")
```

**Build Artifacts:**
```
# Detect project type and build
IF .NET: Bash(command="dotnet publish -c Release -o ./publish")
IF Node.js: Bash(command="npm install && npm run build")
IF Python: Bash(command="pip install -r requirements.txt")
IF Docker: Bash(command="docker build -t {image}:{version} .")
```

For platform-specific build commands, see `references/platform-deployment-commands.md`

#### Step 2: Deploy to Staging

```
# Execute platform-specific deployment
IF Kubernetes:
    Bash(command="helm upgrade {release} ./chart --set image.tag={version} --namespace=staging --install")
    Bash(command="kubectl rollout status deployment/{name} --namespace=staging --timeout=5m")

IF Azure:
    Bash(command="az webapp deployment source config-zip --name {app}-staging --src ./publish.zip")

IF AWS ECS:
    Bash(command="aws ecs update-service --cluster {cluster}-staging --service {service} --task-definition {task_def}:{version}")
```

For complete platform commands, see `references/platform-deployment-commands.md`

#### Step 3: Smoke Test Staging

```
# Wait for application startup
sleep(30)

# Health check
Bash(command="python {SKILL_DIR}/scripts/health_check.py --url {staging_url}/health --retries 5")

HALT if health_check fails:
    "Staging health check failed - rollback staging"

# Run smoke tests
Bash(command="python {SKILL_DIR}/scripts/smoke_test_runner.py --environment staging --url {staging_url}")

HALT if smoke_tests fail:
    "Staging smoke tests failed - rollback staging"
```

For smoke test procedures, see `references/smoke-testing-guide.md`

**Optional Manual Validation:**
```
AskUserQuestion:
Question: "Staging deployed. Smoke tests passed. Perform manual validation?"
Options:
  - "Manual testing complete - Proceed to production"
  - "Issues found - Rollback staging"
  - "Skip manual testing - Proceed to production"
```

---

