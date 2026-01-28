# Tech Stack Detection Reference

## Overview

The Tech Stack Detection module (`installer/tech_stack_detector.py`) identifies project technology stacks from indicator files and returns build configuration information.

## Detection Matrix

| Indicator       | stack_type   | build_command                    | output_directory  |
|-----------------|--------------|----------------------------------|-------------------|
| package.json    | nodejs       | npm run build                    | dist/             |
| pyproject.toml  | python       | python -m build                  | dist/             |
| requirements.txt| python       | pip install -r requirements.txt  | None              |
| *.csproj        | dotnet       | dotnet publish -c Release        | publish/          |
| *.sln           | dotnet       | dotnet build -c Release          | bin/Release/      |
| pom.xml         | java_maven   | mvn package                      | target/           |
| build.gradle    | java_gradle  | gradle build                     | build/            |
| go.mod          | go           | go build                         | ./                |
| Cargo.toml      | rust         | cargo build --release            | target/release/   |

## Usage Examples

### Basic Detection

```python
from installer.tech_stack_detector import TechStackDetector, TechStackInfo
from pathlib import Path

# Initialize detector
detector = TechStackDetector()

# Detect tech stack(s) in a project
results = detector.detect(Path("/path/to/project"))

for info in results:
    print(f"Stack: {info.stack_type}")
    print(f"Indicator: {info.indicator_file}")
    print(f"Build command: {info.build_command}")
    print(f"Output dir: {info.output_directory}")
```

### Multi-Stack Project

```python
# Detect multiple stacks (e.g., Node.js frontend + Python backend)
results = detector.detect(Path("/path/to/fullstack-project"))

# Results ordered by priority: nodejs, python, dotnet, java_maven, java_gradle, go, rust
for info in results:
    print(f"{info.stack_type}: {info.build_command}")
```

### Recursive Detection (Monorepo)

```python
# Scan nested directories (e.g., packages/*, apps/*)
results = detector.detect(Path("/path/to/monorepo"), recursive=True)
```

### Build Command Lookup

```python
# Fast lookup of build command by stack type
cmd = detector.get_build_command("python", "pyproject.toml")
# Returns: "python -m build"
```

## Business Rules

| Rule | Description |
|------|-------------|
| BR-001 | pyproject.toml takes precedence over requirements.txt for Python |
| BR-002 | .csproj takes precedence over .sln for .NET |
| BR-003 | Default scan is root-level only (recursive=False) |
| BR-004 | Detection is read-only (no filesystem modifications) |

## TechStackInfo Dataclass

```python
@dataclass
class TechStackInfo:
    stack_type: str           # e.g., "nodejs", "python", "dotnet"
    indicator_file: str       # Relative path to detected file
    build_command: str | None # Primary build command
    output_directory: str | None  # Expected output directory
    version_file: str | None  # File containing version info
    detection_confidence: float  # 1.0 = definitive, 0.7 = partial
```

## Non-Functional Requirements

| NFR | Requirement | Measured |
|-----|-------------|----------|
| NFR-001 | Detection < 5 seconds | ~0.4 seconds typical |
| NFR-002 | Build command lookup < 100ms | ~1ms typical |
| NFR-003 | Graceful degradation on unreadable files | Logs warning, continues |
| NFR-004 | Path traversal prevention | ValueError on ".." patterns |

## Error Handling

```python
# FileNotFoundError if path doesn't exist
try:
    results = detector.detect(Path("/nonexistent"))
except FileNotFoundError:
    print("Path not found")

# NotADirectoryError if path is a file
try:
    results = detector.detect(Path("/path/to/file.txt"))
except NotADirectoryError:
    print("Not a directory")

# ValueError for path traversal attempts
try:
    results = detector.detect(Path("../../../etc"))
except ValueError:
    print("Path traversal detected")
```

## Integration with Release Skill

The TechStackDetector is used by the devforgeai-release skill to:

1. **Auto-detect build commands** for projects without explicit configuration
2. **Determine output directories** for artifact collection
3. **Support multi-stack deployments** (e.g., frontend + backend)

```python
# Release skill usage example
from installer.tech_stack_detector import TechStackDetector

detector = TechStackDetector()
stacks = detector.detect(project_path)

for stack in stacks:
    # Execute build command for each detected stack
    run_build(stack.build_command)
    # Collect artifacts from output directory
    collect_artifacts(stack.output_directory)
```

## References

- **Story:** STORY-238 (Tech Stack Detection Module)
- **Epic:** EPIC-036 (Release Skill Build Phase Enhancement)
- **Implementation:** installer/tech_stack_detector.py
- **Tests:** tests/STORY-238/test_tech_stack_detector.py
