# Package Formats Reference

**Phase:** 0.3 (Package Creation)
**Implements:** STORY-241 - Language-Specific Package Creation Module
**Source:** `installer/package_creator.py`

---

## Purpose

Create distributable packages for detected technology stacks before registry publishing or deployment. This phase runs after build completion (Phase 0.2) and before registry publishing (Phase 0.5).

---

## Supported Package Formats

| Format | Command | Extensions | Indicator Files | Timeout |
|--------|---------|------------|-----------------|---------|
| **npm** | `npm pack` | `.tgz` | `package.json` | 60s |
| **pip** | `python -m build` | `.whl`, `.tar.gz` | `pyproject.toml`, `setup.py` | 60s |
| **nuget** | `dotnet pack -c Release` | `.nupkg` | `*.csproj` | 60s |
| **docker** | `docker build -t {name}:{version} .` | (image) | `Dockerfile` | 10min |
| **jar** | `mvn package` | `.jar` | `pom.xml` | 60s |
| **zip** | `zip -r {name}-{version}.zip .` | `.zip` | (any) | 60s |

---

## PackageResult Dataclass

Result model returned by package creation operations:

```python
@dataclass
class PackageResult:
    success: bool           # True if package created successfully
    format: str             # Package format (npm, pip, nuget, docker, jar, zip)
    package_path: str | None    # Path to created file (None for Docker)
    package_name: str       # Full name including version
    version: str            # Semver version string
    size_bytes: int | None  # File size in bytes (None for Docker)
    checksum: str | None    # SHA256 hash (None for Docker)
    docker_image: str | None    # Docker image name:tag (Docker only)
    command_executed: str   # Exact shell command executed
    duration_ms: int        # Package creation time in milliseconds
```

---

## Version Extraction

Version is extracted from metadata files in priority order:

1. **package.json** (npm) - `"version"` field
2. **pyproject.toml** (Python) - `version = "X.Y.Z"` field
3. **\*.csproj** (.NET) - `<Version>` XML element
4. **pom.xml** (Maven) - `<version>` XML element

**Fallback:** `"0.0.0"` if version not found (BR-002)

**Validation:** Semver pattern `^\d+\.\d+\.\d+(-[a-zA-Z0-9]+)?$`

---

## Name Extraction

Package name is extracted in priority order:

1. **package.json** - `"name"` field
2. **pyproject.toml** - `name = "..."` field
3. **\*.csproj** - `<PackageId>` element or filename
4. **pom.xml** - `<artifactId>` element

**Fallback:** `"package"` if name not found

---

## Package Discovery

After command execution, packages are located in:

| Format | Primary Location | Secondary Locations |
|--------|-----------------|---------------------|
| npm | Project root | (stdout from npm pack) |
| pip | `dist/` | Project root |
| nuget | `bin/Release/` | (recursive search) |
| jar | `target/` | Project root |
| zip | Project root | - |
| docker | (N/A - image) | - |

---

## Configuration Constants

```python
PACKAGE_COMMANDS = {
    "npm": "npm pack",
    "pip": "python -m build",
    "nuget": "dotnet pack -c Release",
    "docker": "docker build -t {name}:{version} .",
    "jar": "mvn package",
    "zip": "zip -r {name}-{version}.zip .",
}

PACKAGE_EXTENSIONS = {
    "npm": [".tgz"],
    "pip": [".whl", ".tar.gz"],
    "nuget": [".nupkg"],
    "docker": [],  # Image-based, no file
    "jar": [".jar"],
    "zip": [".zip"],
}

DEFAULT_TIMEOUT_MS = 60000   # 60 seconds (NFR-001)
DOCKER_TIMEOUT_MS = 600000   # 10 minutes (NFR-002)
DOCKER_ENABLED = True
```

---

## Business Rules

| ID | Rule | Behavior |
|----|------|----------|
| **BR-001** | Package failures must not halt workflow | Returns `success=False`, continues with other formats |
| **BR-002** | Version from canonical source | Falls back to `"0.0.0"` if not found |
| **BR-003** | Docker requires Dockerfile | Auto-generates for Node.js if missing |
| **BR-004** | Validation is advisory | Logs warning, doesn't fail result |

---

## Security: Command Injection Prevention (NFR-004)

1. **Allowlist Only:** Commands from `PACKAGE_COMMANDS` dictionary only
2. **Input Sanitization:** `[^a-zA-Z0-9._-]` removed from name/version
3. **Shell=False:** Uses `subprocess.run(shlex.split(cmd), shell=False)`

**Invalid format raises:** `ValueError: Unknown package format: {format}`

---

## Checksum Calculation

SHA256 checksum calculated for file-based packages:

```python
def _calculate_checksum(package_path: Path) -> str:
    sha256 = hashlib.sha256()
    with open(package_path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            sha256.update(chunk)
    return sha256.hexdigest()  # 64-character hex string
```

**NFR-003:** 100% checksum accuracy required.

---

## Multi-Format Creation

Create packages for multiple formats in sequence:

```python
creator = PackageCreator(project_dir="/path/to/project")
results = creator.create_multiple(["npm", "docker", "zip"])

# Each format creates independently (BR-001)
for result in results:
    if result.success:
        print(f"[{result.format}] Created: {result.package_name}")
    else:
        print(f"[{result.format}] Failed")
```

---

## Docker Auto-Generation

When Docker format requested but no `Dockerfile` exists:

1. Check for `package.json` (Node.js project)
2. If found, generate basic Dockerfile:
   ```dockerfile
   FROM node:18-alpine
   WORKDIR /app
   COPY . .
   RUN npm install
   CMD ["npm", "start"]
   ```
3. If not Node.js, return failure result

---

## Error Handling

All errors return `PackageResult` with `success=False`:

| Error | Handling |
|-------|----------|
| `FileNotFoundError` | Tool not installed |
| `TimeoutExpired` | Command exceeded timeout |
| Non-zero exit code | Command failed |
| Other exceptions | Logged and wrapped |

**No exceptions raised** - callers check `result.success`.

---

## Usage in Release Workflow

**Phase Position:** 0.3 (after build, before registry publish)

```
Phase 0.1 (Tech Detection)
    ↓
Phase 0.2 (Build/Compile)
    ↓
Phase 0.3 (Package Creation) ← THIS REFERENCE
    ↓
Phase 0.5 (Registry Publish)
    ↓
Phase 1+ (Deployment)
```

**Integration:**
- Receives: `TechStackInfo` from Phase 0.1
- Receives: `BuildResult` from Phase 0.2
- Outputs: `PackageResult[]` for Phase 0.5

---

## Related Stories

- **STORY-238:** TechStackDetector (Phase 0.1)
- **STORY-239:** BuildExecutor (Phase 0.2) - Prerequisite
- **STORY-241:** PackageCreator (Phase 0.3) - This story
- **STORY-246:** RegistryPublisher (Phase 0.5)
