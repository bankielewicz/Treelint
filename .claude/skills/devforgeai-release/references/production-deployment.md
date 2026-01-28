### Phase 3: Production Deployment

**Objective**: Deploy to production using selected strategy

#### Step 1: Final Pre-Production Checks

```
AskUserQuestion:
Question: "Ready to deploy {story_id} to PRODUCTION using {deployment_strategy}?"
Options:
  - "Yes - Proceed with production deployment"
  - "No - Abort (need more testing)"
  - "Wait - Not ready yet (maintenance window pending)"

IF answer == "No" OR "Wait": BLOCK deployment
```

**Backup Current Production:**
```
current_production_version = get_current_production_version()

IF deployment includes database migrations:
    AskUserQuestion about creating database backup
    IF yes: Bash(command="./scripts/backup_database.sh production")
```

#### Step 2: Execute Deployment Strategy

**Strategy Selection:**
```
IF deployment_strategy == "Blue-Green":
    # Deploy to green, validate, switch traffic
    # See references/deployment-strategies.md for detailed procedure
    # See references/platform-deployment-commands.md for platform commands

ELIF deployment_strategy == "Rolling Update":
    # Gradual replacement with health checks
    # See references/deployment-strategies.md for detailed procedure

ELIF deployment_strategy == "Canary":
    # Progressive rollout 5%→25%→50%→100%
    # See references/deployment-strategies.md for detailed procedure

ELIF deployment_strategy == "Recreate":
    # Stop old, deploy new
    # See references/deployment-strategies.md for detailed procedure
```

**Example Blue-Green Workflow:**
```
1. Deploy to green environment
   Bash(command="helm upgrade {release}-green ./chart --set image.tag={version} --namespace=prod-green")

2. Run smoke tests against green
   Bash(command="python scripts/smoke_test_runner.py --environment production-green --url {green_url}")

3. Switch traffic to green
   Bash(command="kubectl patch service/{service} -p '{\"spec\":{\"selector\":{\"version\":\"green\"}}}'")

4. Monitor metrics for 5 minutes
   Bash(command="python scripts/metrics_collector.py --duration 300 --baseline-compare")

5. Declare success or rollback
```

For platform-specific deployment commands, see `references/platform-deployment-commands.md`

---

