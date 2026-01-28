# Registry Publishing Reference

Phase 0.5 of the devforgeai-release skill publishes packages to configured registries before deployment.

**Related:** `configuration-guide.md` (registry-config.yaml schema)

---

## Overview

Registry publishing executes between preflight validation and deployment phases. It ensures packages are available in configured registries before code is deployed to servers.

### RegistryPublisher Interface

```
RegistryPublisher.publish_all(config, dry_run=False) -> PublishResult[]

Parameters:
  - config: RegistryConfig from registry-config.yaml
  - dry_run: bool - If true, validate without publishing

Returns PublishResult[]:
  - registry: string        # Registry name (npm, pypi, nuget, docker, github, crates)
  - success: bool           # True if publish succeeded
  - package_name: string    # Package identifier
  - version: string         # Version published
  - url: string             # Published package URL (or null on failure)
  - error: string           # Error message (or null on success)
  - retries: int            # Number of retry attempts made
  - duration_ms: int        # Publish execution time
```

### Configuration Location

```yaml
# devforgeai/deployment/registry-config.yaml
registries:
  npm:
    enabled: true
    registry_url: "https://registry.npmjs.org"
    # ... registry-specific settings
```

---

## Registry Commands

### npm (Node Package Manager)

**Indicator Files:** `package.json`

**Publish Command:**
```bash
# Authenticate (uses NPM_TOKEN environment variable)
npm config set //registry.npmjs.org/:_authToken=${NPM_TOKEN}

# Publish package (default: public)
npm publish --access public

# Publish with specific tag
npm publish --tag beta

# Dry-run (validation only)
npm publish --dry-run
```

**Required Credentials:**
| Variable | Description | Required |
|----------|-------------|----------|
| `NPM_TOKEN` | npm authentication token | Yes |

**Getting NPM_TOKEN:**
```bash
# Login and generate token
npm login
npm token create --read-only=false

# Or use automation token (CI-friendly)
# Settings > Access Tokens > Generate New Token > Automation
```

**Version Extraction:**
```bash
# Read version from package.json
node -p "require('./package.json').version"
```

**Common Errors:**
| Error | Cause | Resolution |
|-------|-------|------------|
| `E401` | Invalid or expired token | Regenerate NPM_TOKEN |
| `E403` | Package name unavailable | Choose different name or request transfer |
| `E409` | Version already exists | Bump version in package.json |
| `E426` | HTTPS required | Ensure registry URL uses https:// |

**Retry Logic:**
- Max retries: 3
- Backoff: Exponential (1s, 2s, 4s)
- Retryable: E500, E502, E503, ETIMEDOUT, ECONNRESET

---

### PyPI (Python Package Index)

**Indicator Files:** `pyproject.toml`, `setup.py`

**Publish Command:**
```bash
# Build distribution packages
python -m build

# Upload to PyPI using twine
twine upload dist/*

# Upload to TestPyPI (for testing)
twine upload --repository testpypi dist/*

# Dry-run (validation only)
twine check dist/*
```

**Required Credentials:**
| Variable | Description | Required |
|----------|-------------|----------|
| `TWINE_USERNAME` | PyPI username | Yes (use `__token__` for API tokens) |
| `TWINE_PASSWORD` | PyPI password or API token | Yes |

**Getting PyPI Token:**
```bash
# Go to pypi.org > Account Settings > API tokens
# Create token with scope "Entire account" or specific project
# Use token as TWINE_PASSWORD with TWINE_USERNAME=__token__
```

**Version Extraction:**
```bash
# From pyproject.toml (using tomllib in Python 3.11+)
python -c "import tomllib; print(tomllib.load(open('pyproject.toml','rb'))['project']['version'])"

# From setup.py
python setup.py --version
```

**Common Errors:**
| Error | Cause | Resolution |
|-------|-------|------------|
| `HTTPError: 400` | Invalid package metadata | Fix pyproject.toml fields |
| `HTTPError: 401` | Invalid credentials | Verify TWINE_USERNAME/PASSWORD |
| `HTTPError: 403` | Permission denied | Check token scope |
| `HTTPError: 409` | Version exists | Bump version number |

**Retry Logic:**
- Max retries: 3
- Backoff: Exponential (2s, 4s, 8s)
- Retryable: 500, 502, 503, ConnectionError, Timeout

---

### NuGet (.NET Package Manager)

**Indicator Files:** `*.csproj`, `*.nuspec`

**Publish Command:**
```bash
# Pack the project
dotnet pack -c Release

# Push to NuGet.org
dotnet nuget push ./bin/Release/*.nupkg --api-key ${NUGET_API_KEY} --source https://api.nuget.org/v3/index.json

# Push with symbol packages
dotnet nuget push ./bin/Release/*.nupkg --api-key ${NUGET_API_KEY} --source https://api.nuget.org/v3/index.json --skip-duplicate

# Dry-run (verify package locally)
dotnet nuget push ./bin/Release/*.nupkg --source ./local-feed --skip-duplicate
```

**Required Credentials:**
| Variable | Description | Required |
|----------|-------------|----------|
| `NUGET_API_KEY` | NuGet.org API key | Yes |

**Getting NuGet API Key:**
```bash
# Go to nuget.org > API Keys > Create
# Glob pattern: * (or specific package pattern)
# Scopes: Push
```

**Version Extraction:**
```bash
# From .csproj
grep -oP '(?<=<Version>)[^<]+' *.csproj

# Or use dotnet CLI
dotnet msbuild -getProperty:Version
```

**Common Errors:**
| Error | Cause | Resolution |
|-------|-------|------------|
| `401 Unauthorized` | Invalid API key | Regenerate NUGET_API_KEY |
| `403 Forbidden` | Key lacks push scope | Create key with Push permission |
| `409 Conflict` | Version exists | Bump version in .csproj |
| `400 Bad Request` | Invalid package | Validate with `dotnet pack --no-build` |

**Retry Logic:**
- Max retries: 3
- Backoff: Exponential (1s, 2s, 4s)
- Retryable: 500, 502, 503, 429 (rate limit)

---

### Docker Hub / Container Registry

**Indicator Files:** `Dockerfile`, `docker-compose.yml`

**Publish Command:**
```bash
# Login to registry
echo ${DOCKER_PASSWORD} | docker login -u ${DOCKER_USERNAME} --password-stdin

# Build image with tag
docker build -t ${DOCKER_USERNAME}/${IMAGE_NAME}:${VERSION} .

# Push image
docker push ${DOCKER_USERNAME}/${IMAGE_NAME}:${VERSION}

# Push latest tag
docker tag ${DOCKER_USERNAME}/${IMAGE_NAME}:${VERSION} ${DOCKER_USERNAME}/${IMAGE_NAME}:latest
docker push ${DOCKER_USERNAME}/${IMAGE_NAME}:latest

# Dry-run (build only, no push)
docker build -t ${DOCKER_USERNAME}/${IMAGE_NAME}:${VERSION} .
```

**Required Credentials:**
| Variable | Description | Required |
|----------|-------------|----------|
| `DOCKER_USERNAME` | Docker Hub username | Yes |
| `DOCKER_PASSWORD` | Docker Hub password or access token | Yes |

**For Other Registries:**
| Registry | Login Command | Variables |
|----------|---------------|-----------|
| GitHub Container Registry | `echo ${GHCR_TOKEN} \| docker login ghcr.io -u ${GITHUB_ACTOR} --password-stdin` | `GHCR_TOKEN`, `GITHUB_ACTOR` |
| AWS ECR | `aws ecr get-login-password \| docker login --username AWS --password-stdin ${AWS_ACCOUNT}.dkr.ecr.${AWS_REGION}.amazonaws.com` | `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_REGION` |
| Azure ACR | `az acr login --name ${ACR_NAME}` | `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`, `AZURE_TENANT_ID` |
| Google GCR | `gcloud auth configure-docker` | `GOOGLE_APPLICATION_CREDENTIALS` |

**Version Extraction:**
```bash
# From package.json
node -p "require('./package.json').version"

# From VERSION file
cat VERSION

# From git tag
git describe --tags --abbrev=0
```

**Common Errors:**
| Error | Cause | Resolution |
|-------|-------|------------|
| `denied: access forbidden` | Invalid credentials or no push permission | Verify DOCKER_USERNAME/PASSWORD |
| `unauthorized: authentication required` | Not logged in | Run docker login first |
| `manifest unknown` | Image not built | Build image before push |
| `name invalid` | Invalid image name format | Use lowercase, no special chars |

**Retry Logic:**
- Max retries: 3
- Backoff: Exponential (2s, 4s, 8s)
- Retryable: 500, 502, 503, connection errors

---

### GitHub Packages

**Indicator Files:** Any package type (npm, Maven, NuGet, RubyGems, Docker)

**Publish Command (npm):**
```bash
# Configure npm for GitHub Packages
npm config set @OWNER:registry https://npm.pkg.github.com

# Authenticate
echo "//npm.pkg.github.com/:_authToken=${GITHUB_TOKEN}" >> ~/.npmrc

# Update package.json name to scoped
# "name": "@owner/package-name"

# Publish
npm publish
```

**Publish Command (Maven):**
```xml
<!-- pom.xml -->
<distributionManagement>
  <repository>
    <id>github</id>
    <url>https://maven.pkg.github.com/OWNER/REPOSITORY</url>
  </repository>
</distributionManagement>
```
```bash
mvn deploy -DaltDeploymentRepository=github::default::https://maven.pkg.github.com/OWNER/REPOSITORY
```

**Required Credentials:**
| Variable | Description | Required |
|----------|-------------|----------|
| `GITHUB_TOKEN` | GitHub personal access token or GITHUB_TOKEN | Yes |

**Token Scopes Required:**
- `read:packages` - Download packages
- `write:packages` - Publish packages
- `delete:packages` - Delete packages (optional)

**Version Extraction:**
```bash
# From package.json (npm)
node -p "require('./package.json').version"

# From pom.xml (Maven)
mvn help:evaluate -Dexpression=project.version -q -DforceStdout
```

**Common Errors:**
| Error | Cause | Resolution |
|-------|-------|------------|
| `E401` | Invalid or missing token | Verify GITHUB_TOKEN has write:packages |
| `E403` | Token lacks required scope | Add write:packages scope |
| `npm ERR! 404` | Package name not scoped | Use @owner/package-name format |
| `Package version already exists` | Version conflict | Bump version number |

**Retry Logic:**
- Max retries: 3
- Backoff: Exponential (1s, 2s, 4s)
- Retryable: 500, 502, 503

---

### crates.io (Rust Package Registry)

**Indicator Files:** `Cargo.toml`

**Publish Command:**
```bash
# Login with token
cargo login ${CRATES_IO_TOKEN}

# Publish crate
cargo publish

# Dry-run (validation only)
cargo publish --dry-run

# Publish with no verify (skip tests)
cargo publish --no-verify
```

**Required Credentials:**
| Variable | Description | Required |
|----------|-------------|----------|
| `CRATES_IO_TOKEN` | crates.io API token | Yes |

**Getting crates.io Token:**
```bash
# Go to crates.io > Account Settings > API Tokens
# Create new token with "publish-new" and "publish-update" scopes
```

**Version Extraction:**
```bash
# From Cargo.toml
grep -oP '(?<=^version = ")[^"]+' Cargo.toml

# Using cargo
cargo pkgid | cut -d# -f2
```

**Common Errors:**
| Error | Cause | Resolution |
|-------|-------|------------|
| `the remote server responded with an error: unauthorized` | Invalid token | Regenerate CRATES_IO_TOKEN |
| `crate version already uploaded` | Version exists | Bump version in Cargo.toml |
| `failed to verify package tarball` | Build fails | Fix compilation errors |
| `crate ... does not exist` | New crate needs name reservation | Crate name available, proceed |

**Retry Logic:**
- Max retries: 3
- Backoff: Exponential (2s, 4s, 8s)
- Retryable: 500, 502, 503, connection errors

---

## Credential Requirements Summary

| Registry | Required Variables | Optional Variables |
|----------|-------------------|-------------------|
| npm | `NPM_TOKEN` | `NPM_REGISTRY` |
| PyPI | `TWINE_USERNAME`, `TWINE_PASSWORD` | `TWINE_REPOSITORY_URL` |
| NuGet | `NUGET_API_KEY` | `NUGET_SOURCE` |
| Docker | `DOCKER_USERNAME`, `DOCKER_PASSWORD` | `DOCKER_REGISTRY` |
| GitHub | `GITHUB_TOKEN` | `GITHUB_OWNER` |
| crates.io | `CRATES_IO_TOKEN` | - |

### Credential Validation

Before publishing, Phase 0.5 checks each registry's required environment variables. Missing credentials are reported before any publish attempts, allowing early failure detection.

---

## Error Handling

### Error Categories

| Category | Examples | Action |
|----------|----------|--------|
| **Authentication** | 401, 403, invalid token | Log error, skip registry, prompt user |
| **Conflict** | 409, version exists | Log warning, skip registry (version published) |
| **Transient** | 500, 502, 503, timeout | Retry with backoff |
| **Validation** | 400, invalid metadata | Log error, skip registry, prompt user |
| **Network** | ECONNRESET, ETIMEDOUT | Retry with backoff |

### Error Recovery Flow

```
1. Attempt publish
   ├─ Success → Record result, continue to next registry
   └─ Failure → Check error category
       ├─ Transient → Retry (up to max_retries)
       │   ├─ Success → Record result, continue
       │   └─ Exhaust retries → Mark failed, continue
       ├─ Authentication → Mark failed, log credentials issue
       ├─ Conflict → Mark skipped (already published)
       └─ Validation → Mark failed, log validation error

2. After all registries attempted:
   ├─ All success → Continue to deployment
   ├─ Some failed → Prompt user (Continue/Abort)
   └─ All failed → Prompt user (Continue/Abort)
```

### User Prompt on Failure

```
Registry publishing completed with errors:

  [npm]    ✓ Published @scope/package@1.2.0
  [pypi]   ✓ Published package-1.2.0
  [docker] ✗ Failed: authentication error (retried 3 times)
  [nuget]  ⊘ Skipped: disabled in config

Failed registries: docker

How would you like to proceed?
  [C] Continue to deployment (packages may be unavailable)
  [A] Abort release workflow
  [R] Retry failed registries only

>
```

---

## Retry Logic

**Algorithm:** Exponential backoff with `delay = base_delay_ms * 2^(attempt-1)`
**Default:** 3 attempts with 1s, 2s, 4s delays
**Retryable:** HTTP 500/502/503/429, ETIMEDOUT, ECONNRESET, ECONNREFUSED

**Configuration:** Set in `devforgeai/deployment/registry-config.yaml` under `retry:` section.

---

## Dry-Run Mode

When `--dry-run` flag is provided, Phase 0.5:
1. Validates credentials (checks environment variables)
2. Validates package metadata (version, name, dependencies)
3. Tests registry connectivity (no actual publish)
4. Reports what would be published

No packages are published in dry-run mode.

---

## Related Files

- `configuration-guide.md` - Registry config YAML schema
- `pre-release-validation.md` - Phase 1 uses registry results
- `devforgeai/deployment/registry-config.yaml` - Registry configuration
- `STORY-244-registry-publishing-commands.story.md` - RegistryPublisher implementation
- `STORY-245-registry-configuration.story.md` - RegistryConfigLoader implementation
