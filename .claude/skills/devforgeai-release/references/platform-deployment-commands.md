# Platform Deployment Commands Reference

Comprehensive command templates for deploying applications across different platforms and infrastructures.

## Git Workflow Commands

### Release Branch and Tagging

```bash
# Ensure on correct branch
git status

# Create release branch from main
git checkout main && git pull origin main
git checkout -b release/STORY-001

# Generate version tag
# Format: v{major}.{minor}.{patch} or {story-id}-release
git tag -a v1.2.3 -m "Release STORY-001: User Authentication"
git push origin v1.2.3

# Alternative: Story-based tagging
git tag -a STORY-001-release -m "Release: User Authentication Feature"
git push origin STORY-001-release
```

### Commit and Push

```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m "$(cat <<'EOF'
Release: STORY-001 User Authentication

- Implemented OAuth2 authentication flow
- Added JWT token validation
- Created user session management
- Tests: 98% coverage
- QA: PASSED

Closes #STORY-001
EOF
)"

# Push to remote
git push origin release/STORY-001
```

---

## Build Commands

### .NET Applications

```bash
# Restore dependencies
dotnet restore

# Build application
dotnet build -c Release

# Publish for deployment
dotnet publish -c Release -o ./publish

# Create deployment package
cd publish && zip -r ../app.zip .
```

### Node.js Applications

```bash
# Install dependencies
npm install

# Run build (TypeScript compilation, bundling, etc.)
npm run build

# Create deployment package
tar -czf app.tar.gz dist/ node_modules/ package.json
```

### Python Applications

```bash
# Install dependencies
pip install -r requirements.txt

# Create virtual environment (optional)
python -m venv venv
source venv/bin/activate  # Linux/Mac
venv\Scripts\activate     # Windows

# Install in production mode
pip install --no-dev -r requirements.txt
```

### Docker Image Build

```bash
# Build Docker image
docker build -t myapp:v1.2.3 .

# Tag for registry
docker tag myapp:v1.2.3 registry.example.com/myapp:v1.2.3

# Push to registry
docker push registry.example.com/myapp:v1.2.3

# Multi-platform build (optional)
docker buildx build --platform linux/amd64,linux/arm64 -t myapp:v1.2.3 --push .
```

---

## Kubernetes Deployment

### Using kubectl Directly

```bash
# Set image for deployment
kubectl set image deployment/myapp \
  myapp=registry.example.com/myapp:v1.2.3 \
  --namespace=production

# Monitor rollout status
kubectl rollout status deployment/myapp \
  --namespace=production \
  --timeout=10m

# Check deployment
kubectl get deployments --namespace=production
kubectl get pods --namespace=production

# Describe deployment (for debugging)
kubectl describe deployment/myapp --namespace=production
```

### Using Helm

```bash
# Install or upgrade release
helm upgrade myapp ./chart \
  --set image.tag=v1.2.3 \
  --namespace=production \
  --install \
  --wait \
  --timeout=10m

# Check release status
helm status myapp --namespace=production

# List all releases
helm list --namespace=production

# Get release values
helm get values myapp --namespace=production
```

### Blue-Green Deployment (Kubernetes)

```bash
# Deploy to green environment
helm upgrade myapp-green ./chart \
  --set image.tag=v1.2.3 \
  --namespace=production-green \
  --install \
  --wait

# Check green deployment
kubectl rollout status deployment/myapp \
  --namespace=production-green \
  --timeout=5m

# Switch traffic from blue to green (patch service selector)
kubectl patch service/myapp \
  --namespace=production \
  -p '{"spec":{"selector":{"version":"green"}}}'

# Verify traffic switch
kubectl get service/myapp --namespace=production -o yaml | grep version

# Scale down blue environment (keep for rollback)
kubectl scale deployment/myapp-blue \
  --namespace=production-blue \
  --replicas=1
```

### Rolling Update (Kubernetes)

```bash
# Update deployment image
kubectl set image deployment/myapp \
  myapp=registry.example.com/myapp:v1.2.3 \
  --namespace=production

# Kubernetes automatically:
# - Updates 25% of pods at a time (configurable)
# - Waits for health checks to pass
# - Proceeds to next batch
# - Pauses if failures detected

# Monitor rolling update
kubectl rollout status deployment/myapp \
  --namespace=production \
  --timeout=10m

# Pause rollout (if issues detected)
kubectl rollout pause deployment/myapp --namespace=production

# Resume rollout
kubectl rollout resume deployment/myapp --namespace=production
```

### Canary Deployment (Kubernetes with Istio)

```bash
# Deploy canary version
helm upgrade myapp-canary ./chart \
  --set image.tag=v1.2.3 \
  --set replicaCount=1 \
  --namespace=production \
  --install

# Apply traffic split (5% canary)
kubectl apply -f - <<EOF
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: myapp
  namespace: production
spec:
  hosts:
  - myapp
  http:
  - route:
    - destination:
        host: myapp
        subset: baseline
      weight: 95
    - destination:
        host: myapp
        subset: canary
      weight: 5
EOF

# Gradually increase traffic (25%, 50%, 100%)
# Update weight values and reapply VirtualService

# Complete rollout (100% to canary)
kubectl scale deployment/myapp-baseline --replicas=0 --namespace=production
kubectl scale deployment/myapp-canary --replicas=5 --namespace=production
```

### Recreate Deployment (Kubernetes)

```bash
# Scale down current version
kubectl scale deployment/myapp --replicas=0 --namespace=production

# Wait for pods to terminate
sleep 30

# Update image
kubectl set image deployment/myapp \
  myapp=registry.example.com/myapp:v1.2.3 \
  --namespace=production

# Scale up new version
kubectl scale deployment/myapp --replicas=3 --namespace=production

# Wait for rollout
kubectl rollout status deployment/myapp \
  --namespace=production \
  --timeout=5m
```

---

## Azure App Service Deployment

### Using Azure CLI

```bash
# Deploy from zip file
az webapp deployment source config-zip \
  --resource-group myapp-rg \
  --name myapp-staging \
  --src ./publish.zip

# Check deployment status
az webapp show \
  --name myapp-staging \
  --resource-group myapp-rg \
  --query state \
  -o tsv

# Restart app (if needed)
az webapp restart \
  --name myapp-staging \
  --resource-group myapp-rg
```

### Blue-Green with Deployment Slots

```bash
# Deploy to staging slot
az webapp deployment source config-zip \
  --resource-group myapp-rg \
  --name myapp \
  --slot staging \
  --src ./publish.zip

# Verify staging slot
az webapp show \
  --name myapp \
  --resource-group myapp-rg \
  --slot staging \
  --query state

# Swap staging to production (Blue-Green switch)
az webapp deployment slot swap \
  --resource-group myapp-rg \
  --name myapp \
  --slot staging \
  --target-slot production

# Rollback (swap back)
az webapp deployment slot swap \
  --resource-group myapp-rg \
  --name myapp \
  --slot production \
  --target-slot staging
```

### Container Deployment (Azure App Service)

```bash
# Configure container
az webapp config container set \
  --resource-group myapp-rg \
  --name myapp \
  --docker-custom-image-name registry.example.com/myapp:v1.2.3

# Restart to pull new image
az webapp restart \
  --name myapp \
  --resource-group myapp-rg
```

---

## AWS ECS Deployment

### Update ECS Service

```bash
# Register new task definition
aws ecs register-task-definition \
  --cli-input-json file://task-definition.json

# Get new task definition revision
TASK_DEF_ARN=$(aws ecs describe-task-definition \
  --task-definition myapp \
  --query 'taskDefinition.taskDefinitionArn' \
  --output text)

# Update service with new task definition
aws ecs update-service \
  --cluster myapp-cluster-production \
  --service myapp-service \
  --task-definition $TASK_DEF_ARN \
  --force-new-deployment

# Wait for service to stabilize
aws ecs wait services-stable \
  --cluster myapp-cluster-production \
  --services myapp-service

# Check service status
aws ecs describe-services \
  --cluster myapp-cluster-production \
  --services myapp-service
```

### Blue-Green Deployment (AWS ECS with CodeDeploy)

```bash
# Create CodeDeploy deployment
aws deploy create-deployment \
  --application-name myapp \
  --deployment-group-name myapp-dg \
  --revision revisionType=S3,s3Location={bucket=myapp-deployments,key=v1.2.3.zip,bundleType=zip}

# Monitor deployment
aws deploy get-deployment \
  --deployment-id d-XXXXXXXXX

# Automatic traffic shift handled by CodeDeploy
# Blue-Green with health checks and rollback on failure
```

---

## AWS Lambda Deployment

### Update Lambda Function

```bash
# Update function code
aws lambda update-function-code \
  --function-name myapp-function-staging \
  --zip-file fileb://function.zip

# Wait for update to complete
aws lambda wait function-updated \
  --function-name myapp-function-staging

# Update function configuration (if needed)
aws lambda update-function-configuration \
  --function-name myapp-function-staging \
  --environment Variables={ENV=staging,DB_HOST=staging-db}

# Publish new version
aws lambda publish-version \
  --function-name myapp-function-staging

# Update alias to point to new version
aws lambda update-alias \
  --function-name myapp-function \
  --name production \
  --function-version $LATEST
```

### Canary Deployment (Lambda)

```bash
# Create alias with traffic weighting
aws lambda create-alias \
  --function-name myapp-function \
  --name canary \
  --function-version 5 \
  --routing-config AdditionalVersionWeights={4=0.95,5=0.05}

# Gradually increase canary traffic
# Update routing config to shift more traffic

# Complete rollout
aws lambda update-alias \
  --function-name myapp-function \
  --name production \
  --function-version 5
```

---

## Traditional VPS Deployment

### Using Ansible

```bash
# Deploy to staging
ansible-playbook deploy-staging.yml \
  -i inventory/staging \
  --extra-vars "version=v1.2.3"

# Deploy to production
ansible-playbook deploy-production.yml \
  -i inventory/production \
  --extra-vars "version=v1.2.3"

# Rolling update (one server at a time)
ansible-playbook deploy-production.yml \
  -i inventory/production \
  --extra-vars "version=v1.2.3" \
  --serial=1

# Check deployment status
ansible all -i inventory/production -m shell -a "systemctl status myapp"
```

### Using Terraform

```bash
# Plan deployment
terraform plan \
  -var="app_version=v1.2.3" \
  -target=module.staging

# Apply to staging
terraform apply \
  -var="app_version=v1.2.3" \
  -target=module.staging \
  -auto-approve

# Apply to production
terraform apply \
  -var="app_version=v1.2.3" \
  -target=module.production \
  -auto-approve

# Check infrastructure state
terraform show
```

### Manual SSH Deployment

```bash
# Upload application package
scp app.tar.gz user@production-server:/tmp/

# SSH to server
ssh user@production-server

# On server:
# Stop application
sudo systemctl stop myapp

# Backup current version
sudo mv /opt/myapp /opt/myapp.backup

# Extract new version
sudo mkdir -p /opt/myapp
sudo tar -xzf /tmp/app.tar.gz -C /opt/myapp

# Start application
sudo systemctl start myapp

# Check status
sudo systemctl status myapp
```

---

## Health Check Commands

### HTTP Health Check

```bash
# Simple health check
curl -f https://api.example.com/health

# Health check with retry
for i in {1..5}; do
  curl -f https://api.example.com/health && break || sleep 10
done

# Health check with timeout
curl -f --max-time 10 https://api.example.com/health
```

### Kubernetes Health Check

```bash
# Check pod readiness
kubectl get pods \
  --namespace=production \
  --selector=app=myapp \
  -o jsonpath='{.items[*].status.conditions[?(@.type=="Ready")].status}'

# Check deployment health
kubectl get deployment/myapp \
  --namespace=production \
  -o jsonpath='{.status.conditions[?(@.type=="Available")].status}'
```

### Azure App Service Health Check

```bash
# Get app service status
az webapp show \
  --name myapp \
  --resource-group myapp-rg \
  --query state \
  -o tsv

# Check health endpoint
curl -f https://myapp.azurewebsites.net/health
```

### AWS ECS Health Check

```bash
# Check service health
aws ecs describe-services \
  --cluster myapp-cluster \
  --services myapp-service \
  --query 'services[0].runningCount'

# Check task health
aws ecs describe-tasks \
  --cluster myapp-cluster \
  --tasks $(aws ecs list-tasks --cluster myapp-cluster --service myapp-service --query 'taskArns[0]' --output text) \
  --query 'tasks[0].healthStatus'
```

---

## Rollout Monitoring

### Kubernetes Rollout Status

```bash
# Watch rollout progress
kubectl rollout status deployment/myapp \
  --namespace=production \
  --watch

# Check rollout history
kubectl rollout history deployment/myapp --namespace=production

# View specific revision
kubectl rollout history deployment/myapp \
  --namespace=production \
  --revision=3
```

### Azure App Service Deployment Logs

```bash
# Stream deployment logs
az webapp log tail \
  --name myapp \
  --resource-group myapp-rg

# Download logs
az webapp log download \
  --name myapp \
  --resource-group myapp-rg \
  --log-file logs.zip
```

### AWS ECS Deployment Progress

```bash
# Watch service events
aws ecs describe-services \
  --cluster myapp-cluster \
  --services myapp-service \
  --query 'services[0].events[:5]'

# Check task status
aws ecs list-tasks \
  --cluster myapp-cluster \
  --service myapp-service

# View task details
aws ecs describe-tasks \
  --cluster myapp-cluster \
  --tasks TASK_ARN
```

---

## Common Patterns

### Sequential Deployment (Staging → Production)

```bash
# 1. Deploy to staging
<platform-specific-deploy-command> --environment=staging

# 2. Run smoke tests
pytest tests/smoke/ --environment=staging

# 3. Deploy to production (if tests pass)
<platform-specific-deploy-command> --environment=production

# 4. Run smoke tests
pytest tests/smoke/ --environment=production
```

### Parallel Multi-Region Deployment

```bash
# Deploy to all regions simultaneously
for region in us-east-1 eu-west-1 ap-southeast-1; do
  <platform-specific-deploy-command> --region=$region &
done

# Wait for all deployments
wait

# Verify all regions
for region in us-east-1 eu-west-1 ap-southeast-1; do
  <health-check-command> --region=$region
done
```

### Staged Rollout (10% → 50% → 100%)

```bash
# Deploy to 10% of infrastructure
<platform-specific-deploy-command> --percentage=10

# Monitor for 30 minutes
sleep 1800

# If healthy, increase to 50%
<platform-specific-deploy-command> --percentage=50

# Monitor for 30 minutes
sleep 1800

# If healthy, complete rollout
<platform-specific-deploy-command> --percentage=100
```

---

## Platform Selection Guide

| Platform | Best For | Deployment Speed | Complexity | Cost |
|----------|----------|------------------|------------|------|
| **Kubernetes** | Microservices, containerized apps | Fast | High | Medium-High |
| **Azure App Service** | .NET, Node.js, Python web apps | Fast | Low | Medium |
| **AWS ECS** | Containerized apps, AWS ecosystem | Medium | Medium | Medium |
| **AWS Lambda** | Serverless, event-driven | Very Fast | Low | Low |
| **Traditional VPS** | Legacy apps, full control | Slow | Medium | Low-Medium |

---

## References

- [Kubernetes Deployment Documentation](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/)
- [Helm Documentation](https://helm.sh/docs/)
- [Azure CLI Reference](https://docs.microsoft.com/en-us/cli/azure/)
- [AWS CLI Reference](https://docs.aws.amazon.com/cli/)
- [Ansible Documentation](https://docs.ansible.com/)
- [Terraform Documentation](https://www.terraform.io/docs/)
