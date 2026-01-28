# ADR-EXAMPLE-005: Kubernetes for Deployment Platform

**Date**: 2025-11-01
**Status**: Accepted
**Deciders**: DevOps Lead, Technical Architect, CTO
**Project**: E-Commerce Platform (SaaS)

---

## Context

### Project Background
Deploying a multi-tenant e-commerce SaaS platform with:
- **Traffic**: 500K requests/day (peak: 5K requests/minute)
- **Services**: 8 microservices (API, Auth, Payment, Inventory, Notifications, Analytics, Admin, Worker)
- **Databases**: PostgreSQL (primary), Redis (cache), ElasticSearch (search)
- **Scaling Requirement**: Auto-scale based on CPU/memory (horizontal scaling)
- **Availability Target**: 99.9% uptime (43 minutes downtime/month)
- **Geographic Distribution**: Multi-region (US-East, EU-West)

### Problem Statement
We need a deployment platform that balances:
1. **Scalability**: Auto-scale services based on demand (10x traffic spikes during sales)
2. **Availability**: Zero-downtime deployments, automatic failover
3. **Cost Efficiency**: Pay-per-use, optimize resource utilization
4. **Operational Complexity**: Team can manage with 1 DevOps engineer
5. **Flexibility**: Support custom configurations (environment variables, secrets, sidecars)

### Current Pain Points
Currently deploying on **Azure Virtual Machines** (IaaS):
- ❌ **Manual scaling**: Requires SSH, manual service restarts (30+ minutes)
- ❌ **No auto-healing**: Crashed services require manual intervention
- ❌ **Downtime during deploys**: Stop service → Deploy → Start (5-10 min downtime)
- ❌ **Resource waste**: VMs run at 30% utilization (over-provisioned for peak)
- ❌ **Complex configuration**: Each VM has custom scripts (snowflake servers)

### Requirements

**Functional Requirements**:
- **Horizontal scaling**: Auto-scale based on CPU (target: 70%), memory, or custom metrics
- **Zero-downtime deployments**: Rolling updates, canary releases
- **Multi-tenancy**: Isolate customer data (namespace per customer or shared infrastructure)
- **Health checks**: Liveness and readiness probes (restart unhealthy containers)
- **Secret management**: Secure storage for API keys, database credentials
- **Logging and monitoring**: Centralized logs, metrics dashboards

**Non-Functional Requirements**:
- **Availability**: 99.9% uptime (8.76 hours downtime/year)
- **Scalability**: Scale from 10 to 100+ pods within 2 minutes
- **Deployment speed**: <10 minutes from code commit to production
- **Cost**: <$5K/month infrastructure costs (current: $8K/month VMs)
- **Team capacity**: Manageable by 1 DevOps engineer + developers

### Constraints
- Must support .NET 8 containerized applications
- Must integrate with existing CI/CD (GitHub Actions)
- Must support PostgreSQL on Azure Database (managed service)
- Must comply with SOC 2 (audit logs, encryption at rest/transit)
- No vendor lock-in (portable to AWS, GCP, or on-prem if needed)

---

## Decision

**We will use Kubernetes (Azure Kubernetes Service) as the deployment platform.**

Specifically:
- **Azure Kubernetes Service (AKS)** for managed Kubernetes
- **Helm** for package management and templating
- **Horizontal Pod Autoscaler (HPA)** for automatic scaling
- **Nginx Ingress Controller** for load balancing and TLS termination
- **Azure Container Registry (ACR)** for Docker image storage
- **Cert-Manager** for automatic TLS certificate renewal (Let's Encrypt)
- **Prometheus + Grafana** for monitoring and alerting

---

## Rationale

### Technical Rationale

#### 1. Scalability: Auto-Scaling and Resource Efficiency

**Problem with VMs**:
```
Current setup (VMs):
- 8 services × 3 VMs per service (high availability) = 24 VMs
- Each VM: 2 vCPU, 4GB RAM
- Average utilization: 30% CPU, 40% RAM (over-provisioned for peak)
- Cost: $8,000/month
- Scaling: Manual (30+ minutes to add VM, install dependencies, deploy)

Peak traffic (Black Friday):
- Need 10x capacity (240 VMs) for 6 hours
- Manual scaling is impossible
- Result: Service degradation, revenue loss
```

**Kubernetes Solution**:
```yaml
# Horizontal Pod Autoscaler (HPA)
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: api-service-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: api-service
  minReplicas: 3
  maxReplicas: 50
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
  behavior:
    scaleUp:
      stabilizationWindowSeconds: 60
      policies:
      - type: Percent
        value: 50
        periodSeconds: 60
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
      - type: Percent
        value: 10
        periodSeconds: 60
```

**Benefits**:
- ✅ **Auto-scaling**: Scales from 3 to 50 pods in <2 minutes (no manual intervention)
- ✅ **Resource efficiency**: Pods share nodes (bin-packing), 70-80% utilization
- ✅ **Cost savings**: Pay for actual usage (3 nodes baseline, scale to 20 nodes during peak)
- ✅ **Elasticity**: Scale down after peak (automatic cost reduction)

**Cost Comparison**:
```
Baseline (normal traffic):
- VMs: 24 VMs × $0.15/hour = $2,592/month
- Kubernetes: 3 nodes (8 vCPU, 32GB each) × $0.50/hour = $1,080/month
  Savings: $1,512/month (58% reduction)

Peak (Black Friday, 6 hours):
- VMs: Cannot scale (manual, too slow)
- Kubernetes: 20 nodes × $0.50/hour × 6 hours = $60 (one-time spike)
  Total cost: $1,080 + $60 = $1,140/month (with peak handled)
```

#### 2. Zero-Downtime Deployments: Rolling Updates

**Problem with VMs**:
```bash
# VM deployment process (downtime: 5-10 minutes)
1. Stop application (systemctl stop api-service) # ❌ Service unavailable
2. Pull new code (git pull)
3. Install dependencies (dotnet restore)
4. Build application (dotnet build)
5. Start application (systemctl start api-service)
6. Wait for health check (30 seconds)
7. Repeat for each VM (24 VMs × 10 min = 4 hours total)
```

**Kubernetes Rolling Update**:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-service
spec:
  replicas: 10
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 1 # Max 1 pod down at a time
      maxSurge: 2 # Max 2 extra pods during update
  template:
    spec:
      containers:
      - name: api
        image: acr.azurecr.io/api-service:v2.1.0
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 5
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 3
```

**Deployment Flow**:
```
1. kubectl apply -f deployment.yaml
2. Kubernetes creates 2 new pods (maxSurge: 2)
3. Waits for readiness probe (new pods healthy)
4. Terminates 1 old pod (maxUnavailable: 1)
5. Repeats until all 10 pods are v2.1.0
6. Total time: ~3 minutes (10 pods, 1-2 pods updated per cycle)
```

**Benefits**:
- ✅ **Zero downtime**: Always ≥9 pods available (10 - maxUnavailable 1)
- ✅ **Automatic rollback**: If readiness probe fails, rollback to previous version
- ✅ **Fast deployment**: 3 minutes (vs 4 hours with VMs)
- ✅ **Traffic routing**: Kubernetes removes unhealthy pods from load balancer

**Comparison**:
```
VMs:
- Downtime: 5-10 minutes per service
- Total downtime (8 services): 40-80 minutes
- Rollback: Manual (restore previous version, 30+ minutes)

Kubernetes:
- Downtime: 0 minutes
- Total deployment time: 3 minutes per service (parallel)
- Rollback: Automatic (kubectl rollout undo, <1 minute)
```

#### 3. High Availability: Self-Healing and Fault Tolerance

**VM Approach** (manual intervention required):
```
Scenario: API service crashes (out-of-memory error)

VM behavior:
1. Service crashes (process exits)
2. Systemd restarts service (if configured)
3. If systemd fails, requires manual SSH + restart
4. Monitoring alert (5 minutes to detect)
5. Engineer investigates (10-30 minutes)
6. Result: 15-35 minutes downtime

Scenario: Entire VM crashes (hardware failure)

VM behavior:
1. VM becomes unresponsive
2. Load balancer marks VM as unhealthy (2-5 minutes)
3. Traffic routed to remaining VMs (higher load)
4. Engineer must provision new VM (30-60 minutes)
5. Result: Reduced capacity for 30-60 minutes
```

**Kubernetes Self-Healing**:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-service
spec:
  replicas: 10
  template:
    spec:
      containers:
      - name: api
        image: api-service:latest
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          failureThreshold: 3
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          failureThreshold: 3
          periodSeconds: 5
```

**Self-Healing Flow**:
```
Scenario: API pod crashes (out-of-memory)

Kubernetes behavior:
1. Pod exceeds memory limit (512Mi)
2. Kubernetes kills pod (OOMKilled)
3. Deployment controller detects replica count < 10
4. Kubernetes creates new pod (replacement)
5. New pod passes readiness probe
6. Traffic routed to new pod
7. Total time: 30-60 seconds (automatic)

Scenario: Node crashes (hardware failure)

Kubernetes behavior:
1. Node becomes NotReady (30 seconds)
2. Kubernetes marks node as unhealthy
3. Pods on failed node rescheduled to healthy nodes
4. New pods created on available nodes
5. Total time: 2-3 minutes (automatic)
```

**Benefits**:
- ✅ **Automatic recovery**: No manual intervention (self-healing)
- ✅ **Fast recovery**: 30-60 seconds (vs 15-35 minutes with VMs)
- ✅ **Reduced downtime**: 99.9% uptime achievable
- ✅ **Resilience**: Node failures don't cause service outages

#### 4. Operational Simplicity: Declarative Configuration

**VM Configuration** (imperative, error-prone):
```bash
# setup-api-service.sh (run on each VM, 24 times)
#!/bin/bash

# Install .NET runtime
sudo apt-get update
sudo apt-get install -y dotnet-runtime-8.0

# Create service user
sudo useradd -r -s /bin/false api-service

# Clone repository
cd /opt
sudo git clone https://github.com/mycompany/api-service.git

# Configure environment variables
cat <<EOF | sudo tee /etc/api-service/config.env
DATABASE_URL=postgresql://user:pass@db.example.com:5432/ecommerce
REDIS_URL=redis://cache.example.com:6379
API_KEY=secret-key-123
EOF

# Create systemd service
cat <<EOF | sudo tee /etc/systemd/system/api-service.service
[Unit]
Description=API Service
After=network.target

[Service]
Type=simple
User=api-service
WorkingDirectory=/opt/api-service
EnvironmentFile=/etc/api-service/config.env
ExecStart=/usr/bin/dotnet /opt/api-service/Api.dll
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
sudo systemctl enable api-service
sudo systemctl start api-service

# Configure Nginx reverse proxy
sudo apt-get install -y nginx
cat <<EOF | sudo tee /etc/nginx/sites-available/api-service
server {
    listen 80;
    server_name api.example.com;
    location / {
        proxy_pass http://localhost:5000;
    }
}
EOF
sudo ln -s /etc/nginx/sites-available/api-service /etc/nginx/sites-enabled/
sudo systemctl restart nginx

# Setup monitoring
curl -sSL https://install.datadoghq.com/agent.sh | sudo bash
# ... more configuration ...
```

**Problems**:
- ❌ **Manual execution**: Run script on 24 VMs (error-prone)
- ❌ **Configuration drift**: Each VM has slightly different config (snowflakes)
- ❌ **No version control**: Config changes not tracked
- ❌ **Hard to replicate**: New VM requires manual setup (30-60 minutes)

**Kubernetes Configuration** (declarative, version-controlled):
```yaml
# api-service-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-service
  namespace: production
spec:
  replicas: 10
  selector:
    matchLabels:
      app: api-service
  template:
    metadata:
      labels:
        app: api-service
    spec:
      containers:
      - name: api
        image: acr.azurecr.io/api-service:v2.1.0
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: database-secret
              key: url
        - name: REDIS_URL
          valueFrom:
            configMapKeyRef:
              name: redis-config
              key: url
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: api-service
spec:
  selector:
    app: api-service
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: api-service-ingress
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  ingressClassName: nginx
  tls:
  - hosts:
    - api.example.com
    secretName: api-tls
  rules:
  - host: api.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: api-service
            port:
              number: 80
```

**Deployment**:
```bash
# Deploy to Kubernetes (declarative)
kubectl apply -f api-service-deployment.yaml

# Kubernetes ensures desired state:
# - 10 replicas running
# - Load balancer configured
# - TLS certificate issued
# - Health checks monitoring
# - Auto-scaling enabled
```

**Benefits**:
- ✅ **Declarative**: Define desired state, Kubernetes ensures it
- ✅ **Version control**: YAML in Git (audit trail, rollback)
- ✅ **Reproducible**: Same config creates identical environments
- ✅ **Self-documenting**: YAML is human-readable infrastructure

#### 5. Cost Efficiency: Resource Optimization

**Cost Comparison** (detailed breakdown):

```
Baseline Traffic (Normal): 500K requests/day

VM Setup:
- 8 services × 3 VMs (HA) = 24 VMs
- VM size: Standard_D2s_v3 (2 vCPU, 8GB RAM) = $96/month each
- Total: 24 VMs × $96 = $2,304/month
- Utilization: 30% CPU, 40% RAM (over-provisioned)
- Wasted capacity: 70% CPU, 60% RAM unused

Kubernetes Setup:
- 3 nodes: Standard_D8s_v3 (8 vCPU, 32GB RAM) = $360/month each
- Total: 3 nodes × $360 = $1,080/month
- Utilization: 75% CPU, 80% RAM (efficient bin-packing)
- Savings: $1,224/month (53% reduction)

Peak Traffic (Black Friday): 5M requests/day (10x spike, 6 hours)

VM Setup:
- Would need 240 VMs (10x capacity)
- Cannot scale in 6 hours (manual provisioning)
- Result: Service degradation, lost revenue

Kubernetes Setup:
- Scales to 20 nodes automatically (HPA)
- Additional cost: 17 nodes × $360/month × (6 hours / 720 hours) = $51
- Total peak month cost: $1,080 + $51 = $1,131/month
- Savings: $1,173/month during peak (vs VM baseline)
- No service degradation, no lost revenue

Annual Savings:
- Normal months: $1,224/month × 11 months = $13,464
- Peak month: $1,173 savings × 1 month = $1,173
- Total: $14,637/year (52% cost reduction)
```

**Additional Cost Savings**:
- **Developer time**: No manual VM management (saves 20 hours/month × $100/hour = $2,000/month)
- **Faster deployment**: 3 minutes vs 4 hours (12 deploys/month × 4 hours saved = 48 hours = $4,800/month)
- **Reduced downtime**: 99.9% uptime (43 min/month) vs 99% (7 hours/month) = less revenue loss

**Total Annual Savings**: $14,637 + $24,000 (dev time) + $57,600 (deploy time) = **$96,237/year**

---

## Consequences

### Positive Consequences

#### 1. Operational Excellence
- **Auto-scaling**: No manual intervention during traffic spikes
- **Self-healing**: Automatic recovery from failures (pods, nodes)
- **Zero-downtime deploys**: Rolling updates with automatic rollback
- **Simplified operations**: 1 DevOps engineer can manage (vs 2-3 for VMs)

#### 2. Cost Savings (Quantified)
- **52% infrastructure cost reduction**: $1,131/month (Kubernetes) vs $2,304/month (VMs)
- **Developer productivity**: $6,800/month saved (faster deploys, no manual VM management)
- **Revenue protection**: No lost sales during peak traffic (auto-scaling)

#### 3. Developer Experience
- **Fast feedback**: Deploy to staging in <3 minutes (vs 30 minutes with VMs)
- **Environment parity**: Dev, staging, production use same Kubernetes manifests
- **Easy debugging**: `kubectl logs`, `kubectl exec` for troubleshooting

#### 4. Future-Proofing
- **Multi-cloud portability**: Kubernetes runs on AWS (EKS), GCP (GKE), on-prem
- **Ecosystem**: Huge ecosystem (Helm charts, operators, monitoring tools)
- **No vendor lock-in**: Can migrate from Azure to AWS in 1-2 weeks

### Negative Consequences

#### 1. Learning Curve (Initial Training Required)
**Impact**: Medium
**Challenge**: Team must learn Kubernetes concepts (pods, deployments, services, ingress).

**Training Plan**:
- **Week 1**: Kubernetes fundamentals (pods, deployments, services)
- **Week 2**: Helm charts, ConfigMaps, Secrets
- **Week 3**: Monitoring (Prometheus, Grafana), troubleshooting
- **Week 4**: Hands-on (deploy sample app, practice rollbacks)

**Mitigation**:
- Use **managed Kubernetes (AKS)**: Azure handles control plane (easier)
- Create **runbooks**: Common tasks (deploy, rollback, scale, debug)
- Use **Helm charts**: Pre-packaged configurations (less YAML writing)

**Reality Check**: 4-week onboarding is acceptable for long-term benefits.

#### 2. Increased Complexity (More Moving Parts)
**Impact**: Medium
**Challenge**: Kubernetes adds components (control plane, etcd, kube-proxy, CoreDNS).

**Comparison**:
```
VMs (Simple):
- Components: VM, Nginx, systemd, application
- Failure points: 4

Kubernetes (Complex):
- Components: Control plane, nodes, pods, services, ingress, DNS
- Failure points: 12+

Debugging:
- VM: SSH to VM, check logs (journalctl)
- Kubernetes: kubectl logs, kubectl describe, kubectl get events
```

**Mitigation**:
- Use **Azure Kubernetes Service (AKS)**: Managed control plane (Azure handles upgrades, patches)
- **Monitoring**: Prometheus + Grafana dashboards (visibility into cluster health)
- **Runbooks**: Document common issues (pod CrashLoopBackOff, ImagePullBackOff, OOMKilled)

**Reality Check**: Complexity is **managed complexity** (controlled via YAML). Benefits outweigh costs.

#### 3. Networking Complexity (Service Mesh Optional)
**Impact**: Low (for now)
**Challenge**: Inter-service communication (8 microservices) requires service discovery.

**Kubernetes Built-In**:
```yaml
# Service discovery via DNS (automatic)
apiVersion: v1
kind: Service
metadata:
  name: payment-service
spec:
  selector:
    app: payment
  ports:
  - port: 80

# Other services call payment-service via DNS:
# http://payment-service.production.svc.cluster.local
```

**Future Consideration**: Service mesh (Istio, Linkerd) for advanced features:
- Mutual TLS (mTLS) for service-to-service encryption
- Traffic splitting (canary deployments, A/B testing)
- Distributed tracing (Jaeger)

**Current Decision**: **Not needed yet**. Use Kubernetes built-in service discovery. Revisit when we have 20+ services.

#### 4. Storage Management (StatefulSets Required)
**Impact**: Low (using managed databases)
**Challenge**: Kubernetes is best for stateless apps. StatefulSets (for databases) are complex.

**Our Approach**: **Use managed databases** (Azure Database for PostgreSQL, Redis Cache).
- Kubernetes runs stateless application pods
- Databases run outside Kubernetes (managed by Azure)

**Benefits**:
- ✅ No need for StatefulSets (complex volume management)
- ✅ Azure handles backups, high availability, patching
- ✅ Kubernetes focuses on application layer (simpler)

**Reality Check**: This is the **recommended pattern** for production Kubernetes.

---

## Alternatives Considered

### Alternative 1: Azure App Service (PaaS)

**Description**: Managed platform for deploying web apps (no Kubernetes, no VMs).

**Pros**:
- **Zero infrastructure management**: Azure handles everything
- **Auto-scaling**: Built-in (scale rules based on CPU, memory)
- **Deployment slots**: Blue-green deployments (swap slots)
- **Integrated monitoring**: Application Insights included

**Cons**:
- **Limited customization**: Cannot run custom containers or sidecars
- **Vendor lock-in**: Azure-specific (cannot migrate to AWS/GCP)
- **Cost**: $200/month per service (8 services × $200 = $1,600/month baseline)
- **No multi-service orchestration**: Each service deployed independently
- **Limited networking**: Cannot use service mesh or advanced routing

**Why Rejected**:
- **Cost**: $1,600/month (vs $1,080/month Kubernetes) = 48% more expensive
- **Vendor lock-in**: Cannot migrate to AWS or GCP (business risk)
- **Limited flexibility**: Cannot run custom configurations (sidecars, init containers)
- **No Kubernetes skills**: Team would learn App Service (less transferable)

**Conclusion**: App Service is excellent for **simple, single-service apps**. Our **8-service microservices architecture** benefits from Kubernetes orchestration.

---

### Alternative 2: AWS ECS (Elastic Container Service)

**Description**: AWS-managed container orchestration (alternative to Kubernetes).

**Pros**:
- **Simpler than Kubernetes**: Less learning curve (ECS tasks, services)
- **AWS integration**: Works well with AWS ecosystem (ALB, RDS, CloudWatch)
- **Fargate mode**: Serverless containers (no node management)
- **Cost-effective**: Fargate pricing is competitive

**Cons**:
- **AWS lock-in**: Cannot migrate to Azure or GCP
- **Less portable**: ECS concepts don't transfer to Kubernetes
- **Smaller ecosystem**: Fewer tools, Helm charts, operators
- **Limited community**: Kubernetes has 10x larger community

**Why Rejected**:
- **Current infrastructure**: Already on Azure (PostgreSQL, Redis, monitoring)
- **Migration cost**: Moving to AWS would require re-architecting database layer
- **Vendor lock-in**: ECS is AWS-specific (Kubernetes is portable)
- **Team preference**: Team wants to learn Kubernetes (industry standard)

**Conclusion**: ECS is a **valid choice for AWS-native teams**. We're on Azure, and Kubernetes is more portable.

---

### Alternative 3: Docker Swarm

**Description**: Docker's native clustering and orchestration tool.

**Pros**:
- **Simpler than Kubernetes**: Easier to learn (docker stack deploy)
- **Built into Docker**: No separate installation
- **Good for small clusters**: Works well for 3-10 nodes

**Cons**:
- **Limited features**: No auto-scaling (HPA), no advanced networking
- **Declining community**: Kubernetes has won the orchestration war
- **No managed service**: Must self-host (no Azure Swarm Service)
- **Limited ecosystem**: Few Helm-like tools, operators

**Why Rejected**:
- **Declining adoption**: Docker Swarm is deprecated in favor of Kubernetes
- **Limited features**: No HPA, no pod disruption budgets, no RBAC
- **No managed service**: Would require self-hosting (operational burden)
- **Future risk**: Docker may discontinue Swarm support

**Conclusion**: Docker Swarm was competitive in 2016-2018. Kubernetes has become the **industry standard** (2025).

---

### Alternative 4: Keep Current VMs (Improve Automation)

**Description**: Continue using VMs, but automate with Ansible/Terraform.

**Pros**:
- **No new technology**: Team already knows VMs
- **Automation possible**: Ansible can automate VM provisioning and configuration
- **Simple architecture**: VMs are well-understood

**Cons**:
- **Still manual scaling**: Ansible doesn't auto-scale (requires human trigger)
- **No self-healing**: Ansible doesn't restart crashed services automatically
- **Downtime during deploys**: Ansible cannot do zero-downtime rolling updates
- **Configuration drift**: Ansible scripts must be run manually (drift still occurs)
- **Cost**: Still $2,304/month (no bin-packing, no resource efficiency)

**Why Rejected**:
- **Does not solve core problems**: Auto-scaling, self-healing, zero-downtime deploys
- **Automation is not orchestration**: Ansible automates tasks, but doesn't manage runtime state
- **Cost**: No cost savings (VMs still over-provisioned)
- **Future**: Industry is moving to containers + Kubernetes (VMs are legacy)

**Conclusion**: Automation improves VMs, but **does not achieve** the operational excellence of Kubernetes.

---

## Implementation Plan

### Phase 1: Setup AKS Cluster (Week 1)

**1. Provision Azure Resources**:
```bash
# Create resource group
az group create --name ecommerce-prod-rg --location eastus

# Create AKS cluster
az aks create \
  --resource-group ecommerce-prod-rg \
  --name ecommerce-aks \
  --node-count 3 \
  --node-vm-size Standard_D8s_v3 \
  --enable-managed-identity \
  --enable-cluster-autoscaler \
  --min-count 3 \
  --max-count 20 \
  --network-plugin azure \
  --enable-addons monitoring

# Create Azure Container Registry
az acr create --resource-group ecommerce-prod-rg --name ecommerceacr --sku Standard

# Attach ACR to AKS
az aks update --name ecommerce-aks --resource-group ecommerce-prod-rg --attach-acr ecommerceacr
```

**2. Install Essential Components**:
```bash
# Install Helm
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash

# Install Nginx Ingress Controller
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm install nginx-ingress ingress-nginx/ingress-nginx \
  --set controller.service.type=LoadBalancer

# Install Cert-Manager (TLS certificates)
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.13.0/cert-manager.yaml

# Install Prometheus + Grafana (monitoring)
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm install prometheus prometheus-community/kube-prometheus-stack
```

### Phase 2: Containerize Applications (Week 2-3)

**3. Create Dockerfiles**:
```dockerfile
# api-service/Dockerfile
FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src
COPY ["Api/Api.csproj", "Api/"]
RUN dotnet restore "Api/Api.csproj"
COPY . .
WORKDIR "/src/Api"
RUN dotnet build "Api.csproj" -c Release -o /app/build
RUN dotnet publish "Api.csproj" -c Release -o /app/publish

FROM mcr.microsoft.com/dotnet/aspnet:8.0
WORKDIR /app
COPY --from=build /app/publish .
EXPOSE 8080
ENTRYPOINT ["dotnet", "Api.dll"]
```

**4. Build and Push Images**:
```bash
# Build images
docker build -t ecommerceacr.azurecr.io/api-service:v1.0.0 ./api-service
docker build -t ecommerceacr.azurecr.io/auth-service:v1.0.0 ./auth-service
# ... (repeat for 8 services)

# Push to ACR
az acr login --name ecommerceacr
docker push ecommerceacr.azurecr.io/api-service:v1.0.0
docker push ecommerceacr.azurecr.io/auth-service:v1.0.0
# ... (repeat for 8 services)
```

### Phase 3: Deploy to Kubernetes (Week 4-5)

**5. Create Kubernetes Manifests**:
```yaml
# api-service/k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-service
spec:
  replicas: 10
  selector:
    matchLabels:
      app: api-service
  template:
    metadata:
      labels:
        app: api-service
    spec:
      containers:
      - name: api
        image: ecommerceacr.azurecr.io/api-service:v1.0.0
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: database-secret
              key: url
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: api-service
spec:
  selector:
    app: api-service
  ports:
  - port: 80
    targetPort: 8080
---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: api-service-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: api-service
  minReplicas: 10
  maxReplicas: 50
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

**6. Deploy Services**:
```bash
# Create secrets
kubectl create secret generic database-secret --from-literal=url="postgresql://..."

# Deploy services
kubectl apply -f api-service/k8s/
kubectl apply -f auth-service/k8s/
# ... (repeat for 8 services)

# Verify deployments
kubectl get pods
kubectl get services
kubectl get ingress
```

### Phase 4: CI/CD Integration (Week 6)

**7. GitHub Actions Workflow**:
```yaml
# .github/workflows/deploy-api-service.yml
name: Deploy API Service
on:
  push:
    branches: [main]
    paths:
      - 'api-service/**'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3

    - name: Build Docker image
      run: docker build -t ecommerceacr.azurecr.io/api-service:${{ github.sha }} ./api-service

    - name: Login to ACR
      run: az acr login --name ecommerceacr

    - name: Push image
      run: docker push ecommerceacr.azurecr.io/api-service:${{ github.sha }}

    - name: Deploy to Kubernetes
      run: |
        kubectl set image deployment/api-service \
          api=ecommerceacr.azurecr.io/api-service:${{ github.sha }}

    - name: Wait for rollout
      run: kubectl rollout status deployment/api-service
```

### Phase 5: Monitoring and Alerting (Week 7)

**8. Configure Prometheus Alerts**:
```yaml
# prometheus-alerts.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: prometheus-alerts
data:
  alerts.yaml: |
    groups:
    - name: api-service
      rules:
      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.05
        for: 5m
        annotations:
          summary: "High error rate on API service"
      - alert: PodCrashLooping
        expr: rate(kube_pod_container_status_restarts_total[15m]) > 0
        for: 5m
        annotations:
          summary: "Pod is crash-looping"
```

**9. Create Grafana Dashboards**:
- Pod metrics (CPU, memory, restarts)
- HTTP request metrics (latency, error rate)
- HPA scaling events

### Phase 6: Migration from VMs (Week 8-10)

**10. Parallel Run** (Week 8):
- Run VMs and Kubernetes in parallel
- Route 10% traffic to Kubernetes (canary)
- Monitor for issues (latency, errors)

**11. Gradual Cutover** (Week 9):
- Route 50% traffic to Kubernetes
- Monitor stability
- Route 100% traffic to Kubernetes

**12. Decommission VMs** (Week 10):
- Backup VM configurations
- Shut down VMs
- Delete Azure VM resources
- Celebrate cost savings 🎉

---

## Monitoring and Metrics

### Key Performance Indicators (KPIs)

**Availability**:
- Uptime: ≥99.9% (target)
- Incident response time: <15 minutes

**Performance**:
- API latency P95: <200ms
- Deployment time: <10 minutes

**Cost**:
- Infrastructure cost: <$1,200/month
- Cost per request: <$0.0001

**Operational**:
- Auto-scaling events: Track scale-up/scale-down frequency
- Pod restarts: <5 restarts/day per service

---

## Review Schedule

### 3 Months (February 2026)
**Review Criteria**:
- Has uptime met 99.9% target?
- Has cost stayed under $1,200/month?
- Has auto-scaling handled traffic spikes?

### 6 Months (May 2026)
**Review Criteria**:
- Has team become proficient with Kubernetes?
- Have we avoided major incidents?
- Should we add service mesh (Istio)?

### 12 Months (November 2026)
**Full ADR Review**:
- Has Kubernetes delivered promised benefits?
- Should we expand to multi-region Kubernetes?

---

## Related Documents

- **ADR-001: Database Selection** (PostgreSQL on Azure)
- **ADR-004: Clean Architecture** (microservices design)
- **Tech Stack Documentation**: `devforgeai/specs/context/tech-stack.md`
- **Deployment Procedures**: `docs/runbooks/kubernetes-deployment.md`

---

## Approval and Sign-Off

**Approved By**:
- ✅ DevOps Lead
- ✅ Technical Architect
- ✅ CTO

**Date Approved**: 2025-11-01

---

## References

### Documentation
- [Azure Kubernetes Service (AKS)](https://learn.microsoft.com/en-us/azure/aks/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Helm Charts](https://helm.sh/docs/)

### Team Resources
- Kubernetes tutorial: `docs/tutorials/kubernetes-101.md`
- Helm guide: `docs/guides/helm-charts.md`
- Troubleshooting runbook: `docs/runbooks/kubernetes-troubleshooting.md`
