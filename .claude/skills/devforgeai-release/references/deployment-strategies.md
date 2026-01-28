# Deployment Strategies Reference

Detailed guide for selecting and executing deployment strategies.

## Strategy Comparison Matrix

| Strategy | Downtime | Rollback Speed | Resource Cost | Complexity | Best For |
|----------|----------|----------------|---------------|------------|----------|
| **Blue-Green** | Zero | Instant (seconds) | High (2x resources) | Medium | Critical production services, zero-downtime requirement |
| **Rolling Update** | Zero | Moderate (minutes) | Low (minimal overhead) | Low | Standard deployments, resource-constrained environments |
| **Canary** | Zero | Fast (seconds-minutes) | Low (minimal overhead) | High | High-risk changes, gradual rollouts, A/B testing |
| **Recreate** | Yes (brief) | Slow (redeploy) | None | Very Low | Development/staging, maintenance windows acceptable |

## Blue-Green Deployment

### Overview
Two identical production environments (Blue = current, Green = new). Deploy to Green, validate, switch traffic.

### Infrastructure Requirements
- 2x production capacity during deployment
- Load balancer with traffic routing capability
- Identical environment configurations (Blue and Green)

### Execution Steps

1. **Deploy to Green**:
   ```bash
   helm upgrade myapp-green ./chart --set image.tag=v2.0.0 --namespace=prod-green
   ```

2. **Validate Green**:
   - Run smoke tests against Green environment
   - Monitor health endpoints
   - Verify database connectivity

3. **Switch Traffic**:
   ```bash
   # Kubernetes service selector switch
   kubectl patch service/myapp -p '{"spec":{"selector":{"version":"green"}}}'
   ```

4. **Monitor**:
   - Watch error rates for 5-10 minutes
   - Compare metrics against Blue baseline

5. **Cleanup** (after monitoring period):
   ```bash
   # Scale down Blue environment (keep for quick rollback)
   kubectl scale deployment/myapp-blue --replicas=1
   ```

### Rollback
```bash
# Instant rollback: switch traffic back to Blue
kubectl patch service/myapp -p '{"spec":{"selector":{"version":"blue"}}}'
```

### When to Use
- Zero downtime is critical
- Instant rollback required
- Infrastructure cost acceptable
- Database changes are backward-compatible

## Rolling Update Deployment

### Overview
Gradual replacement of old instances with new (e.g., 25% at a time).

### Infrastructure Requirements
- Single production environment
- Load balancer with health check support
- Kubernetes or equivalent orchestration

### Execution Steps

1. **Update Deployment**:
   ```bash
   kubectl set image deployment/myapp myapp=myapp:v2.0.0
   ```

2. **Monitor Rollout**:
   ```bash
   kubectl rollout status deployment/myapp --timeout=10m
   ```

3. **Automatic Progression**:
   - Kubernetes updates 25% of pods (default)
   - Waits for health checks to pass
   - Proceeds to next batch
   - Pauses if failures detected

### Rollback
```bash
kubectl rollout undo deployment/myapp
```

### When to Use
- Standard deployments
- Limited infrastructure resources
- Moderate risk changes
- Health checks can detect issues quickly

## Canary Deployment

### Overview
Progressive rollout: 5% → 25% → 50% → 100% with monitoring at each stage.

### Infrastructure Requirements
- Traffic splitting capability (Istio, AWS ALB, etc.)
- Metrics collection and alerting
- Baseline metrics for comparison

### Execution Steps

1. **Deploy Canary (5%)**:
   ```bash
   helm upgrade myapp-canary ./chart --set replicaCount=1 --set version=canary
   kubectl apply -f traffic-split-5.yaml  # 95% baseline, 5% canary
   ```

2. **Monitor Canary Metrics (10 min)**:
   - Error rate < baseline * 1.1
   - Response time < baseline * 1.3
   - No critical errors

3. **Increase to 25%** (if healthy):
   ```bash
   kubectl apply -f traffic-split-25.yaml
   ```
   Monitor for 10 minutes.

4. **Increase to 50%** (if healthy):
   ```bash
   kubectl apply -f traffic-split-50.yaml
   ```
   Monitor for 10 minutes.

5. **Complete Rollout (100%)**:
   ```bash
   kubectl scale deployment/myapp-baseline --replicas=0
   kubectl scale deployment/myapp-canary --replicas=5
   ```

### Rollback
```bash
# Remove canary, keep baseline at 100%
kubectl delete deployment/myapp-canary
kubectl apply -f traffic-split-100-baseline.yaml
```

### When to Use
- High-risk deployments
- Need to detect issues with real traffic
- Acceptable to roll out slowly
- Good monitoring infrastructure

## Recreate Deployment

### Overview
Stop all old instances, deploy new instances. Brief downtime acceptable.

### Infrastructure Requirements
- Minimal - no special infrastructure needed
- Maintenance window for downtime

### Execution Steps

1. **Scale Down**:
   ```bash
   kubectl scale deployment/myapp --replicas=0
   ```

2. **Deploy New Version**:
   ```bash
   kubectl set image deployment/myapp myapp=myapp:v2.0.0
   ```

3. **Scale Up**:
   ```bash
   kubectl scale deployment/myapp --replicas=3
   ```

4. **Verify**:
   ```bash
   kubectl rollout status deployment/myapp
   ```

### Rollback
```bash
# Redeploy previous version
kubectl set image deployment/myapp myapp=myapp:v1.9.0
kubectl scale deployment/myapp --replicas=3
```

### When to Use
- Development/staging environments
- Maintenance windows available
- Simplicity prioritized over availability
- Database migrations require downtime

## Traffic Splitting Configuration

### Kubernetes with Istio

**5% Canary:**
```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: myapp
spec:
  hosts:
  - myapp
  http:
  - match:
    - headers:
        canary:
          exact: "true"
    route:
    - destination:
        host: myapp
        subset: canary
  - route:
    - destination:
        host: myapp
        subset: baseline
      weight: 95
    - destination:
        host: myapp
        subset: canary
      weight: 5
```

### AWS ALB Target Group Weighting

```bash
aws elbv2 modify-listener-rule \
  --rule-arn $RULE_ARN \
  --actions \
    Type=forward,ForwardConfig='{
      TargetGroups=[
        {TargetGroupArn=$BASELINE_TG,Weight=95},
        {TargetGroupArn=$CANARY_TG,Weight=5}
      ]
    }'
```

## Decision Matrix

Use this flowchart to select deployment strategy:

```
Q: Is zero downtime critical?
├─ NO → Use Recreate (simplest)
└─ YES →
    Q: Need instant rollback (< 10 sec)?
    ├─ YES → Use Blue-Green (if resources available)
    └─ NO →
        Q: High-risk deployment?
        ├─ YES → Use Canary (gradual with monitoring)
        └─ NO → Use Rolling Update (standard)
```

## Platform-Specific Commands

### Kubernetes/Helm
- Blue-Green: Helm releases + service selector switch
- Rolling: `kubectl set image` with rollout strategy
- Canary: Istio VirtualService or Flagger
- Recreate: Scale to 0, update, scale up

### Azure App Service
- Blue-Green: Deployment slots + swap
- Rolling: App Service auto-scaling with health checks
- Canary: Traffic Manager with weighted routing
- Recreate: Stop app, deploy, start app

### AWS ECS
- Blue-Green: CodeDeploy Blue/Green deployment
- Rolling: ECS rolling update with deployment configuration
- Canary: ALB target group weighting
- Recreate: Update service with force new deployment

### Traditional VPS/VM
- Blue-Green: Load balancer config switch
- Rolling: Ansible rolling updates
- Canary: Nginx upstream weighting
- Recreate: Stop service, deploy, start service

## Best Practices

1. **Always Test in Staging First** - Use same strategy in staging as production
2. **Monitor Continuously** - Watch metrics during deployment
3. **Have Rollback Plan Ready** - Document rollback commands before deploying
4. **Use Health Checks** - Ensure load balancers can detect unhealthy instances
5. **Gradual Rollouts for Risk** - Canary for high-risk changes
6. **Automate Rollback** - Define automatic rollback triggers (error rate, health checks)
7. **Document Everything** - Record deployment steps, issues, and resolutions

## Common Pitfalls

- **Blue-Green**: Forgetting to update database connection strings
- **Rolling Update**: Health checks not detecting application-level issues
- **Canary**: Insufficient monitoring data at low traffic percentages
- **Recreate**: Not communicating downtime window to users

## Metrics to Monitor

During any deployment strategy:
- **Error Rate**: Should remain < 1.2x baseline
- **Response Time (p95)**: Should remain < 1.3x baseline
- **CPU Utilization**: Should remain < 80%
- **Memory Usage**: Should remain < 85%
- **Request Success Rate**: Should remain > 99%
- **Database Connection Pool**: Should not saturate

## References

- [Kubernetes Deployment Strategies](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/)
- [AWS CodeDeploy Blue/Green](https://docs.aws.amazon.com/codedeploy/latest/userguide/welcome.html)
- [Azure Deployment Slots](https://docs.microsoft.com/en-us/azure/app-service/deploy-staging-slots)
- [Istio Traffic Management](https://istio.io/latest/docs/concepts/traffic-management/)
