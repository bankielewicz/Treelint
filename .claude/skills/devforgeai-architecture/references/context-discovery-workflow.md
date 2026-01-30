# Phase 1: Project Context Discovery

Gather project context through strategic questions to establish architectural foundation.

## Overview

Phase 1 collects essential information about the project through interactive discovery. This determines whether the project is greenfield (new) or brownfield (existing), and establishes the baseline understanding needed for creating context files.

---

## 1.1 Determine Project Type

Use AskUserQuestion to understand context:

```
Question: "Is this a new project or existing codebase?"
Header: "Project type"
Options:
  - "Greenfield - New project from scratch"
  - "Brownfield - Adding to existing codebase"
multiSelect: false
```

**Why this matters:**
- Greenfield: Start fresh with optimal architecture
- Brownfield: Must discover current state first, then decide migration strategy

---

## 1.2 Analyze Existing Project (Brownfield Only)

If brownfield, discover current state:

### Discover Project Structure

```
# Find solution/project files
Glob(pattern="**/*.sln")          # .NET solution files
Glob(pattern="**/*.csproj")       # C# project files
Glob(pattern="**/package.json")   # Node.js projects
Glob(pattern="**/requirements.txt") # Python projects
Glob(pattern="**/pom.xml")        # Java Maven projects
Glob(pattern="**/build.gradle")   # Java Gradle projects
```

### Check for Existing Context Files

```
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/source-tree.md")
Read(file_path="devforgeai/specs/context/dependencies.md")
Read(file_path="devforgeai/specs/context/coding-standards.md")
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
Read(file_path="devforgeai/specs/context/anti-patterns.md")
```

**If context files exist:**
- Load them as immutable constraints
- Validate spec against existing constraints
- Use AskUserQuestion if spec conflicts with constraints
- Proceed to Phase 5 (validation) if only validating

**If context files DON'T exist (technical debt risk!):**
- Create them from discovered project state
- Document current architecture as baseline
- Proceed with Phase 2 (create missing context files)

### Analyze Technology Stack

```
# .NET projects
Grep(pattern="<PackageReference", glob="**/*.csproj")  # NuGet packages

# Node.js projects
Read(file_path="package.json")                         # npm packages
Read(file_path="package-lock.json")                    # locked versions

# Python projects
Read(file_path="requirements.txt")                     # pip packages
Read(file_path="Pipfile")                              # pipenv

# Java projects
Read(file_path="pom.xml")                              # Maven dependencies
Read(file_path="build.gradle")                         # Gradle dependencies
```

### Understand Project Structure

```
# Source organization
Glob(pattern="src/**/*")        # Main source code
Glob(pattern="lib/**/*")        # Libraries
Glob(pattern="app/**/*")        # Application code

# Test organization
Glob(pattern="tests/**/*")      # Test files
Glob(pattern="test/**/*")       # Alternative test location
Glob(pattern="**/*.test.*")     # Test files by pattern
Glob(pattern="**/*.spec.*")     # Spec files
```

---

## 1.3 Discovery Documentation

After discovery phase completes, document findings:

### Greenfield Projects

**Minimal documentation needed:**
- Project name
- Target platforms (web, mobile, desktop)
- Primary programming language
- Basic architecture style preference (if any)

**Move directly to Phase 2** to create all 6 context files from scratch.

### Brownfield Projects

**Comprehensive documentation required:**

1. **Current State:**
   - Technologies in use (frameworks, libraries, databases)
   - Project structure (folders, namespaces, modules)
   - Existing patterns (architecture style, design patterns)
   - Technical debt identified (inconsistencies, violations)

2. **Gap Analysis:**
   - What exists vs what spec requires
   - Migration complexity assessment
   - Breaking changes identification
   - Timeline implications

3. **Migration Strategy Decision:**

Use AskUserQuestion for complex migrations:

```
Question: "Project currently uses [X], but [Y] is preferred. How should we proceed?"
Header: "Migration strategy"
Options:
  - "Gradual migration (new code uses [Y], legacy stays [X])"
  - "Full refactor (convert all to [Y])"
  - "Accept current state (continue with [X])"
  - "Reassess preference (maybe [X] is better for this project)"
multiSelect: false
```

---

## Output

Phase 1 produces:

1. **Project Type Classification:**
   - Greenfield or Brownfield
   - If brownfield: Existing context file status

2. **Technology Inventory** (brownfield only):
   - Current tech stack
   - Current dependencies
   - Current structure

3. **Gap Analysis** (brownfield only):
   - Differences between current state and desired state
   - Migration strategy (if needed)
   - Timeline considerations

**Next Phase:** Move to Phase 2 (Create Immutable Context Files)
