# Rollback Procedures Reference

Quick reference for platform-specific rollback commands and procedures.

## Automatic Rollback Triggers

IMMEDIATE rollback if:
- Health check fails (HTTP 500+)
- Smoke tests fail
- Error rate > 2x baseline
- Critical service unavailable
- Database migration fails with data loss risk

## Kubernetes Rollback

### Using kubectl
```bash
# View rollout history
kubectl rollout history deployment/myapp -n production

# Rollback to previous revision
kubectl rollout undo deployment/myapp -n production

# Rollback to specific revision
kubectl rollout undo deployment/myapp -n production --to-revision=3

# Monitor rollback
kubectl rollout status deployment/myapp -n production
```

### Using Helm
```bash
# View release history
helm history myapp -n production

# Rollback to previous release
helm rollback myapp -n production

# Rollback to specific revision
helm rollback myapp 3 -n production
```

## Azure App Service Rollback

### Deployment Slots
```bash
# Swap slots back (if blue-green)
az webapp deployment slot swap \
  --slot staging \
  --name myapp \
  --resource-group myResourceGroup \
  --target-slot production \
  --action swap

# Revert to previous deployment
az webapp deployment source config-zip \
  --resource-group myResourceGroup \
  --name myapp \
  --src previous-version.zip
```

## AWS ECS Rollback

```bash
# Update service to previous task definition
aws ecs update-service \
  --cluster myapp-production \
  --service myapp \
  --task-definition myapp:42

# Wait for service stability
aws ecs wait services-stable \
  --cluster myapp-production \
  --services myapp
```

## AWS Lambda Rollback

```bash
# Revert alias to previous version
aws lambda update-alias \
  --function-name myapp \
  --name production \
  --function-version 42
```

## Docker Rollback

```bash
# Stop current container
docker stop myapp

# Start previous version
docker run -d --name myapp \
  --restart always \
  -p 80:80 \
  myregistry/myapp:v1.9.0
```

## Database Rollback

### Migration Rollback
```bash
# .NET Entity Framework
dotnet ef database update PreviousMigration

# Python Alembic
alembic downgrade -1

# Node.js Sequelize
npx sequelize-cli db:migrate:undo
```

### Restore from Backup
```bash
# PostgreSQL
pg_restore -d mydb backup_20251030.dump

# MySQL
mysql -u root -p mydb < backup_20251030.sql

# MongoDB
mongorestore --db mydb backup_20251030/
```

## Post-Rollback Checklist

- [ ] Verify rollback completed successfully
- [ ] Run smoke tests on rolled-back version
- [ ] Check error rates returned to normal
- [ ] Update story status (Released → QA Approved)
- [ ] Document rollback reason
- [ ] Create hotfix story if needed
- [ ] Notify stakeholders
- [ ] Schedule post-incident review

## Rollback Time Estimates

| Strategy | Rollback Time | Notes |
|----------|---------------|-------|
| Blue-Green | < 30 seconds | Traffic switch only |
| Rolling Update | 2-5 minutes | Revert deployment |
| Canary | < 1 minute | Remove canary |
| Recreate | 3-10 minutes | Redeploy previous version |

## Emergency Rollback Script

```bash
#!/bin/bash
# scripts/emergency_rollback.sh

DEPLOYMENT_TARGET=$1
PREVIOUS_VERSION=$2

case $DEPLOYMENT_TARGET in
  kubernetes)
    kubectl rollout undo deployment/myapp -n production
    ;;
  azure)
    az webapp deployment slot swap --slot production --name myapp --target-slot staging
    ;;
  aws_ecs)
    aws ecs update-service --cluster myapp-prod --service myapp --task-definition myapp:$PREVIOUS_VERSION
    ;;
  *)
    echo "Unknown deployment target: $DEPLOYMENT_TARGET"
    exit 1
    ;;
esac

echo "Rollback initiated for $DEPLOYMENT_TARGET to version $PREVIOUS_VERSION"
```

## References

- [Kubernetes Rollback Documentation](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#rolling-back-a-deployment)
- [AWS ECS Deployments](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html)
- [Azure App Service Deployment](https://docs.microsoft.com/en-us/azure/app-service/deploy-staging-slots)
