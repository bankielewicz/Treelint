# Build Commands Reference

Phase 0.2 of the devforgeai-release skill executes build commands for detected technology stacks.

**Related:** `tech-stack-detection.md` (Phase 0.1 detection logic)

---

## BuildExecutor Overview

The BuildExecutor service executes platform-specific build commands based on TechStackInfo from Phase 0.1 (Tech Stack Detection).

### BuildExecutor Interface

```
BuildExecutor.execute(tech_stack_info, config) -> BuildResult

Parameters:
  - tech_stack_info: TechStackInfo from Phase 0.1
  - config: Build configuration from build-config.yaml

Returns BuildResult:
  - success: bool           # True if build succeeded
  - stack_type: string      # e.g., "nodejs", "python", "dotnet"
  - output_path: string     # Build output directory (or null)
  - duration_ms: int        # Build execution time
  - stdout: string          # Standard output from build
  - stderr: string          # Standard error from build
  - exit_code: int          # Process exit code (0 = success)
```

### BuildExecutor Usage Example

```python
# Phase 0.2 execution flow
from release_skill import TechStackDetector, BuildExecutor

# Get detected stacks from Phase 0.1
stacks = TechStackDetector.detect(project_root)

# Execute builds for each stack
build_results = []
for stack_info in stacks:
    result = BuildExecutor.execute(stack_info, config)
    build_results.append(result)

# Pass results to Phase 1
phase_1_context["build_results"] = build_results
```

---

## Build Command Templates

### Node.js Projects

**Indicator File:** `package.json`

**Build Commands:**
```bash
# Clean install (reproducible builds)
npm ci

# Execute build script from package.json
npm run build
```

**Output Directory:** `dist/` or `build/` (varies by project)

**Notes:**
- Prefers `npm ci` over `npm install` for deterministic builds
- Respects `package-lock.json` for exact versions
- Falls back to `npm install` if lockfile missing

### Python Projects

**Indicator Files:** `pyproject.toml`, `setup.py`, `requirements.txt`

**Build Commands:**
```bash
# For pyproject.toml (modern Python packaging)
pip install build
python -m build

# For setup.py (legacy)
pip install -e .
python setup.py sdist bdist_wheel
```

**Output Directory:** `dist/`

**Notes:**
- Requires `build` package for pyproject.toml projects
- Creates wheel (.whl) and source distribution (.tar.gz)
- `requirements.txt` only = no build step (just install)

### .NET Projects

**Indicator Files:** `*.csproj`, `*.sln`

**Build Commands:**
```bash
# Restore NuGet packages
dotnet restore

# Build in Release configuration
dotnet build --configuration Release

# Publish self-contained executables
dotnet publish -c Release -o ./publish --self-contained
```

**Output Directory:** `publish/` or `bin/Release/`

**Cross-Platform Targets:**
```bash
# Windows x64
dotnet publish -c Release -r win-x64 --self-contained

# Linux x64
dotnet publish -c Release -r linux-x64 --self-contained

# macOS x64
dotnet publish -c Release -r osx-x64 --self-contained
```

### Go Projects

**Indicator File:** `go.mod`

**Build Commands:**
```bash
# Download dependencies
go mod download

# Build all packages
go build -o ./build/ ./...
```

**Output Directory:** `build/`

**Notes:**
- Go produces statically-linked binaries by default
- Cross-compilation via `GOOS` and `GOARCH` environment variables

### Rust Projects

**Indicator File:** `Cargo.toml`

**Build Commands:**
```bash
# Build release binary
cargo build --release
```

**Output Directory:** `target/release/`

**Notes:**
- Debug builds in `target/debug/`
- Release builds are optimized

### Java Projects (Maven)

**Indicator File:** `pom.xml`

**Build Commands:**
```bash
# Clean and package (skip tests for speed)
mvn clean package -DskipTests
```

**Output Directory:** `target/`

**Notes:**
- Tests run separately in QA phase
- Produces JAR or WAR files

### Java Projects (Gradle)

**Indicator Files:** `build.gradle`, `build.gradle.kts`

**Build Commands:**
```bash
# Build without tests
gradle build -x test

# Or using wrapper
./gradlew build -x test
```

**Output Directory:** `build/libs/`

**Notes:**
- Kotlin DSL uses `.kts` extension
- Wrapper script recommended for reproducibility

---

## Multi-Stack Build Example

For monorepo projects with multiple technology stacks:

```markdown
Project Structure:
  /frontend     (package.json - Node.js)
  /backend      (pyproject.toml - Python)
  /services     (*.csproj - .NET)

Phase 0.1 Detection:
  - TechStackInfo: nodejs, build_command="npm run build", output="frontend/dist/"
  - TechStackInfo: python, build_command="python -m build", output="backend/dist/"
  - TechStackInfo: dotnet, build_command="dotnet publish", output="services/publish/"

Phase 0.2 Execution:
  BuildResult[]:
    - success: true, stack_type: nodejs, duration_ms: 4200
    - success: true, stack_type: python, duration_ms: 2800
    - success: true, stack_type: dotnet, duration_ms: 12500
```

---

## Error Handling

### Build Failure

```markdown
BuildResult:
  success: false
  stack_type: nodejs
  exit_code: 1
  stderr: "error TS2307: Cannot find module 'react'"

Behavior depends on config:
  - fail_on_build_error=true  -> HALT workflow, display error
  - fail_on_build_error=false -> Log warning, continue to Phase 1
```

### Timeout Handling

```markdown
BuildResult:
  success: false
  exit_code: -1  # Timeout indicator
  stderr: "Build exceeded timeout_ms (600000ms)"

Always treated as failure. Workflow behavior per fail_on_build_error setting.
```

### Missing Build Command

```markdown
# Project detected but no build script found
TechStackInfo:
  stack_type: nodejs
  build_command: null  # No "build" script in package.json

Result: Skip build for this stack, log info message
```

---

## Configuration Integration

BuildExecutor respects settings from `devforgeai/deployment/build-config.yaml`:

| Setting | Effect on BuildExecutor |
|---------|------------------------|
| `build.enabled=false` | Skip Phase 0.1 and 0.2 entirely |
| `build.timeout_ms` | Maximum time for build command execution |
| `build.skip_stacks[]` | Skip specific tech stacks (e.g., `["python"]`) |
| `build.fail_on_build_error=true` | HALT workflow on any build failure |
| `build.fail_on_build_error=false` | Log warning and continue (default) |
| `build.cross_platform_targets[]` | .NET publish runtime targets |

---

## Best Practices

1. **Use Lockfiles:** Ensure reproducible builds with lockfiles (package-lock.json, Cargo.lock, etc.)

2. **Clean Builds:** Delete output directories before building to avoid stale artifacts

3. **Appropriate Timeouts:** Set `timeout_ms` based on project size (default 10 minutes)

4. **Cross-Platform Testing:** Test builds on all target platforms before release

5. **Error Logging:** Capture and preserve stderr for debugging failed builds

6. **Incremental Builds:** Consider incremental builds for development, full builds for release

---

## Related Files

- `tech-stack-detection.md` - Phase 0.1 detection logic and indicator files
- `devforgeai/deployment/build-config.yaml` - Build configuration schema
- `pre-release-validation.md` - Phase 1 uses BuildResult for validation
