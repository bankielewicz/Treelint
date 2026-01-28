# External Project Setup Guide

**Purpose:** Guide for setting up test projects to validate DevForgeAI installer on Node.js and .NET projects.

**Audience:** Developers testing the installer, QA engineers, integration testers

**Last Updated:** 2025-11-20

---

## Quick Start

```bash
# 1. Create Node.js test project
mkdir /tmp/NodeJsTestProject && cd /tmp/NodeJsTestProject
npm init -y
# Create CLAUDE.md with your project instructions

# 2. Run installer
python3 /path/to/DevForgeAI/installer/install.py --target=$(pwd) --source=/path/to/DevForgeAI/src

# 3. Verify installation
ls -la .claude devforgeai
cat CLAUDE.md  # Should show merged content
```

---

## Node.js Test Project Setup

### Prerequisites

- Node.js 18+ installed
- npm available
- Write permissions in target directory

### Setup Steps

**1. Create Project Directory**

```bash
mkdir -p /tmp/NodeJsTestProject
cd /tmp/NodeJsTestProject
```

**2. Initialize package.json**

```bash
npm init -y
```

Or create manually:

```json
{
  "name": "NodeJsTestProject",
  "version": "1.0.0",
  "description": "Test project for DevForgeAI installation",
  "main": "index.js",
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  }
}
```

**3. Create Sample CLAUDE.md (Optional - tests merge logic)**

```markdown
# Node.js Project Instructions

## Project Setup
- Use npm for package management
- ESLint configuration in .eslintrc
- TypeScript strict mode enabled
- Node version: 18+

## API Documentation
- Express.js server on port 3000
- RESTful API endpoints in src/routes/
- Middleware configuration in src/middleware/

## Testing Guidelines
- Jest for unit tests
- Supertest for API tests
- Coverage threshold: 80%

## Deployment
- Docker container: Dockerfile in root
- Environment variables in .env
- Production builds: npm run build
```

**4. Run Installer**

```bash
# From DevForgeAI repository root
python3 installer/install.py --target=/tmp/NodeJsTestProject --source=src/
```

**5. Verify Installation**

```bash
# Check framework directories
ls -la .claude  # Should have ~370 files (skills, commands, agents)
ls -la devforgeai  # Should have ~80 files (config, protocols, specs)

# Check CLAUDE.md merge
cat CLAUDE.md | head -50  # Should show user content + DevForgeAI framework

# Check version tracking
cat devforgeai/.version.json  # Should show version 1.0.1, mode: fresh_install

# Check backups (if CLAUDE.md existed)
ls -la .backups  # Should have devforgeai-fresh-YYYYMMDD-HHMMSS/
```

---

## .NET Test Project Setup

### Prerequisites

- .NET SDK 8.0+ installed
- Write permissions in target directory

### Setup Steps

**1. Create Project Directory**

```bash
mkdir -p /tmp/DotNetTestProject
cd /tmp/DotNetTestProject
```

**2. Create .csproj File**

Create `TestProject.csproj`:

```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <RootNamespace>DotNetTestProject</RootNamespace>
  </PropertyGroup>
</Project>
```

**3. Create Sample Program.cs**

```csharp
using System;

namespace DotNetTestProject
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello, World!");
        }
    }
}
```

**4. Create Sample CLAUDE.md (Optional)**

```markdown
# .NET Project Instructions

## Configuration
- Target Framework: .NET 8.0
- Language: C# 12
- Nullable: enabled

## Architecture
- Clean Architecture pattern
- CQRS with MediatR
- Entity Framework Core for data access

## Testing
- xUnit for unit tests
- FluentAssertions for assertions
- Moq for mocking
```

**5. Run Installer**

```bash
# From DevForgeAI repository root
python3 installer/install.py --target=/tmp/DotNetTestProject --source=src/
```

**6. Verify Installation**

```bash
# Check framework directories
ls -la .claude
ls -la devforgeai

# Check CLAUDE.md
cat CLAUDE.md | grep ".NET"  # Should detect .NET in tech stack section

# Check version
cat devforgeai/.version.json
```

---

## Python Test Project Setup (Optional)

### Prerequisites

- Python 3.8+ installed
- pip available

### Setup Steps

**1. Create Project Directory**

```bash
mkdir -p /tmp/PythonTestProject
cd /tmp/PythonTestProject
```

**2. Create requirements.txt**

```
fastapi==0.104.1
uvicorn==0.24.0
pydantic==2.5.0
```

**3. Create setup.py (Optional)**

```python
from setuptools import setup, find_packages

setup(
    name="PythonTestProject",
    version="1.0.0",
    packages=find_packages(),
    install_requires=[
        "fastapi",
        "uvicorn",
        "pydantic",
    ],
)
```

**4. Run Installer**

```bash
python3 /path/to/DevForgeAI/installer/install.py --target=$(pwd) --source=/path/to/DevForgeAI/src
```

---

## Validation Checklist

After installation, verify these requirements:

### Framework Deployment

- [ ] `.claude/` directory exists with ~370 files
- [ ] `devforgeai/` directory exists with ~80 files
- [ ] Total files deployed: ~450 ±50

### CLAUDE.md Merge

- [ ] `CLAUDE.md` exists
- [ ] User content preserved (if original CLAUDE.md existed)
- [ ] Framework sections added (search for "DevForgeAI")
- [ ] Variables substituted:
  - `{{PROJECT_NAME}}` → actual project name
  - `{{TECH_STACK}}` → Node.js, .NET, or Python
  - `{{PYTHON_PATH}}` → detected Python path

### Version Tracking

- [ ] `devforgeai/.version.json` exists
- [ ] Version: 1.0.1
- [ ] Mode: fresh_install (or upgrade/rollback depending on scenario)
- [ ] Schema version: 1.0

### Backup Creation

- [ ] `.backups/` directory exists (if CLAUDE.md existed before install)
- [ ] Backup contains: .claude/, devforgeai/, CLAUDE.md
- [ ] Manifest.json with checksums present

### Command Infrastructure

- [ ] `.claude/commands/` has 14+ .md files
- [ ] `.claude/skills/` has skill directories
- [ ] `.claude/agents/` has agent definition files

---

## Rollback Testing

To test rollback functionality:

**1. Install DevForgeAI**

```bash
python3 installer/install.py --target=/tmp/TestProject --source=src/
```

**2. Verify Backup Created**

```bash
ls -la /tmp/TestProject/.backups
# Should show: devforgeai-fresh-YYYYMMDD-HHMMSS/
```

**3. Modify a Framework File**

```bash
echo "MODIFIED" >> /tmp/TestProject/.claude/skills/devforgeai-development/SKILL.md
```

**4. Run Rollback**

```bash
python3 installer/install.py --target=/tmp/TestProject --mode=rollback
```

**5. Verify Restoration**

```bash
# Check file restored
grep "MODIFIED" /tmp/TestProject/.claude/skills/devforgeai-development/SKILL.md
# Should return: nothing (file restored to original)

# Verify checksum match
# (Manual verification - compare file checksums before/after)
```

---

## Upgrade Testing

To test upgrade workflow:

**1. Install Version 1.0.1**

```bash
python3 installer/install.py --target=/tmp/TestProject --source=src/
cat /tmp/TestProject/devforgeai/.version.json  # Shows: 1.0.1
```

**2. Simulate Version 1.0.2**

```bash
# Modify src/devforgeai/version.json
echo '{"version": "1.0.2", ...}' > src/devforgeai/version.json

# Modify 5 framework files in src/
```

**3. Run Upgrade**

```bash
python3 installer/install.py --target=/tmp/TestProject --source=src/ --mode=upgrade
```

**4. Verify Selective Update**

```bash
# Check version updated
cat /tmp/TestProject/devforgeai/.version.json  # Should show: 1.0.2

# Verify only 5 files changed (not all 450)
# Check installer output: "Files updated: 5, Files unchanged: 445"
```

---

## Isolation Testing

To test project isolation (no cross-contamination):

**1. Install in Project A**

```bash
python3 installer/install.py --target=/tmp/ProjectA --source=src/
```

**2. Install in Project B**

```bash
python3 installer/install.py --target=/tmp/ProjectB --source=src/
```

**3. Create Story in Project A**

```bash
cd /tmp/ProjectA
# (Requires Claude Code Terminal session)
# /create-story "Test feature"
```

**4. Verify Isolation**

```bash
# Check Project B has no Project A artifacts
grep -r "ProjectA" /tmp/ProjectB  # Should return: 0 results
ls /tmp/ProjectB/devforgeai/specs/Stories/  # Should be empty (or only ProjectB stories)

# Verify version tracking
cat /tmp/ProjectA/devforgeai/.version.json  # Shows: ProjectA install path
cat /tmp/ProjectB/devforgeai/.version.json  # Shows: ProjectB install path (different)
```

---

## Troubleshooting

### Installation Fails

**Symptom:** Installer exits with error
**Check:**
```bash
# Verify source directories exist
ls src/claude src/devforgeai

# Check permissions
touch /tmp/TestProject/test-write && rm /tmp/TestProject/test-write

# Check Python version
python3 --version  # Should be 3.8+
```

### CLAUDE.md Not Merged

**Symptom:** CLAUDE.md doesn't contain framework sections
**Check:**
```bash
# Verify source template exists
ls src/CLAUDE.md

# Check merge log
# (Installer should output "✓ CLAUDE.md merged..." message)
```

### Commands Not Working

**Symptom:** `/create-context` command not found
**Check:**
```bash
# Verify commands deployed
ls .claude/commands/*.md | wc -l  # Should be 14+

# Check Claude Code Terminal
# Restart terminal to reload commands
```

### Rollback Fails

**Symptom:** Rollback doesn't restore files
**Check:**
```bash
# Verify backup exists
ls .backups/

# Check manifest
cat .backups/devforgeai-*/manifest.json

# Verify integrity
# (Installer validates checksums automatically)
```

---

## Test Automation

For automated testing (CI/CD), use the test suite:

```bash
# Run full integration test suite
pytest tests/external/test_install_integration.py -v

# Run specific test category
pytest tests/external/test_install_integration.py -k "ac1" -v  # AC1 tests only
pytest tests/external/test_install_integration.py -k "nodejs" -v  # Node.js tests
pytest tests/external/test_install_integration.py -k "dotnet" -v  # .NET tests

# Run with coverage
pytest tests/external/test_install_integration.py --cov=installer --cov-report=term
```

---

## Performance Benchmarks

Expected performance targets:

| Operation | Target | Measured |
|-----------|--------|----------|
| Fresh install (Node.js) | <3 minutes | TBD (run tests) |
| Fresh install (.NET) | <3 minutes | TBD (run tests) |
| Upgrade (5 files) | <30 seconds | TBD (run tests) |
| Rollback (450 files) | <45 seconds | TBD (run tests) |

To measure:

```bash
# Fresh install timing
time python3 installer/install.py --target=/tmp/TestProject --source=src/

# Rollback timing
time python3 installer/install.py --target=/tmp/TestProject --mode=rollback
```

---

## Summary

This guide provides setup instructions for:
- ✅ Node.js test projects (package.json + CLAUDE.md)
- ✅ .NET test projects (*.csproj + Program.cs)
- ✅ Python test projects (requirements.txt + setup.py)
- ✅ Installation verification steps
- ✅ Rollback testing procedures
- ✅ Upgrade workflow testing
- ✅ Isolation validation
- ✅ Troubleshooting common issues
- ✅ Test automation examples
- ✅ Performance benchmarking

**Status:** Ready for use in external testing scenarios.
