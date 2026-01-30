## Configuration

### Required Configuration Files

This skill expects deployment configurations and smoke test configs to be stored in standardized locations:

**Deployment Configurations:**
- **Location**: `devforgeai/deployment/`
- **Purpose**: Platform-specific deployment configurations (Kubernetes manifests, Helm charts, Docker Compose files, Terraform modules, etc.)
- **Examples**:
  - `devforgeai/deployment/kubernetes/` - K8s YAML manifests
  - `devforgeai/deployment/helm/` - Helm chart values
  - `devforgeai/deployment/terraform/` - Infrastructure as Code
  - `devforgeai/deployment/docker-compose.yml` - Docker Compose config

**Smoke Test Configuration:**
- **Location**: `devforgeai/smoke-tests/config.json`
- **Purpose**: Environment-specific smoke test configurations (base URLs, test users, API keys)
- **Schema**:
```json
{
  "environments": {
    "staging": {
      "base_url": "https://staging.example.com",
      "test_user": "staging_test@example.com",
      "api_key_env_var": "STAGING_API_KEY"
    },
    "production": {
      "base_url": "https://production.example.com",
      "test_user": "prod_test@example.com",
      "api_key_env_var": "PROD_API_KEY"
    }
  }
}
```

**Release Credentials:**
- **Location**: Environment variables (NEVER commit to repository)
- **Examples**: `KUBECONFIG`, `AWS_ACCESS_KEY_ID`, `AZURE_CLIENT_ID`, `DOCKER_REGISTRY_TOKEN`

**If configuration is missing:**
```
Release skill will HALT with:
"Missing deployment configuration. Expected files in devforgeai/deployment/"

Use AskUserQuestion to clarify:
- Which deployment platform? (Kubernetes, Azure App Service, AWS ECS, etc.)
- Create configuration files from templates in ./assets/templates/
```

---

