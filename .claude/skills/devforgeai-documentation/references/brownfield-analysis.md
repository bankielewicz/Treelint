# Brownfield Analysis Workflow

Workflow for analyzing existing codebases and generating/consolidating documentation (brownfield mode).

---

## Purpose

Guide the devforgeai-documentation skill through codebase analysis, existing documentation discovery, and gap identification for projects with existing code.

---

## When to Use

**Brownfield mode applies when:**
- No story files exist, but codebase exists
- `/document --mode=brownfield` specified
- Documentation analysis needed for existing project
- Gap identification before documentation sprint

---

## Workflow Phases

### Phase 1: Codebase Scanning

**Step 1.1: Invoke code-analyzer subagent**

```
Task(
    subagent_type="code-analyzer",
    description="Analyze codebase for documentation generation",
    prompt="Analyze the codebase at {project_path}.

    Extract:
    1. Architecture pattern (MVC, Clean, DDD, Layered, Custom)
    2. Layer structure (directories and responsibilities)
    3. Public APIs (all classes/functions/methods with public visibility)
    4. Entry points (main files, startup code)
    5. Dependencies (external packages from package.json/requirements.txt/etc)
    6. Internal dependencies (module import graphs)
    7. Key workflows (user flows, data flows from code)

    Return structured JSON with complete code metadata."
)
```

**Step 1.2: Parse code analysis**

```
analysis = parse_json(subagent_output)

Extract:
- architecture_pattern = analysis["architecture_pattern"]
- layers = analysis["layers"]
- public_apis = analysis["public_apis"]
- entry_points = analysis["entry_points"]
- dependencies = analysis["dependencies"]
- workflows = analysis["key_workflows"]

Display: "✓ Codebase analyzed"
Display: "  Architecture: {architecture_pattern}"
Display: "  Public APIs: {len(public_apis)}"
Display: "  Entry points: {len(entry_points)}"
```

**Performance:** <10 minutes for 500-file codebase

---

### Phase 2: Existing Documentation Discovery

**Step 2.1: Find documentation files**

```
# Standard documentation locations
patterns = [
    "README.md",
    "README.rst",
    "docs/**/*.md",
    "documentation/**/*.md",
    "CONTRIBUTING.md",
    "CHANGELOG.md",
    "ARCHITECTURE.md",
    "API.md"
]

existing_docs = []

FOR pattern in patterns:
    files = Glob(pattern=pattern)
    existing_docs.extend(files)
```

**Step 2.2: Categorize discovered docs**

```
categorized = {
    "readme": [],
    "api": [],
    "architecture": [],
    "contributing": [],
    "changelog": [],
    "other": []
}

FOR doc in existing_docs:
    filename = basename(doc)

    IF "readme" in filename.lower():
        category = "readme"
    ELIF "api" in filename.lower():
        category = "api"
    ELIF "architecture" in filename.lower() OR "design" in filename.lower():
        category = "architecture"
    ELIF "contributing" in filename.lower():
        category = "contributing"
    ELIF "changelog" in filename.lower() OR "history" in filename.lower():
        category = "changelog"
    ELSE:
        category = "other"

    categorized[category].append(doc)
```

**Step 2.3: Read and analyze existing docs**

```
existing_content = {}

FOR category, files in categorized.items():
    FOR file in files:
        content = Read(file_path=file)

        # Extract metadata
        word_count = count_words(content)
        sections = extract_headings(content)
        last_modified = git_last_modified(file)

        existing_content[file] = {
            "category": category,
            "word_count": word_count,
            "sections": sections,
            "last_modified": last_modified,
            "content": content
        }
```

---

### Phase 3: Gap Identification

**Step 3.1: Calculate documentation coverage**

```
total_public_apis = len(analysis["public_apis"])
documented_apis = 0

FOR api in analysis["public_apis"]:
    IF api["documented"]:  # Has docstring
        documented_apis += 1

coverage = (documented_apis / total_public_apis) * 100
```

**Step 3.2: Identify missing documentation files**

```
required_docs = {
    "readme": "README.md",
    "developer-guide": "docs/DEVELOPER.md",
    "api": "docs/API.md",
    "architecture": "docs/ARCHITECTURE.md",
    "troubleshooting": "docs/TROUBLESHOOTING.md",
    "contributing": "CONTRIBUTING.md",
    "changelog": "CHANGELOG.md"
}

missing_docs = []

FOR doc_type, expected_path in required_docs.items():
    IF expected_path not in existing_docs:
        missing_docs.append({
            "type": doc_type,
            "path": expected_path,
            "priority": calculate_priority(doc_type)
        })
```

**Priority calculation:**
```
def calculate_priority(doc_type):
    IF doc_type == "readme":
        return "CRITICAL"  # Every project needs README
    ELIF doc_type in ["api", "developer-guide"]:
        return "HIGH"
    ELIF doc_type in ["architecture", "contributing"]:
        return "MEDIUM"
    ELSE:
        return "LOW"
```

**Step 3.3: Identify outdated documentation**

```
outdated_docs = []

FOR file, metadata in existing_content.items():
    # Check last modified vs code changes
    file_age_days = (current_date - metadata["last_modified"]).days

    IF file_age_days > 90:  # 3 months
        outdated_docs.append({
            "file": file,
            "age_days": file_age_days,
            "recommendation": "Review and update"
        })
```

**Step 3.4: Identify undocumented APIs**

```
undocumented_apis = []

FOR api in analysis["public_apis"]:
    IF NOT api["documented"]:
        undocumented_apis.append({
            "api": api["endpoint"] OR api["signature"],
            "location": api["location"],
            "type": api["type"],  # function, class, method, endpoint
            "priority": "High" if "endpoint" in api else "Medium"
        })
```

---

### Phase 4: Coverage Report Generation

**Step 4.1: Build comprehensive report**

```
coverage_report = {
    "overall_coverage": coverage,
    "analysis_date": current_timestamp,

    "summary": {
        "total_public_apis": total_public_apis,
        "documented_apis": documented_apis,
        "undocumented_apis": len(undocumented_apis),
        "coverage_percentage": coverage
    },

    "existing_documentation": {
        "files_found": len(existing_docs),
        "by_category": categorized,
        "total_words": sum(word_counts),
        "outdated_files": len(outdated_docs)
    },

    "gaps": {
        "missing_files": missing_docs,
        "outdated_files": outdated_docs,
        "undocumented_apis": undocumented_apis
    },

    "recommendations": generate_recommendations(gaps)
}
```

**Step 4.2: Generate recommendations**

```
recommendations = []

# Missing critical docs
IF "readme" in missing_docs:
    recommendations.append({
        "priority": "CRITICAL",
        "action": "Create README.md",
        "reason": "Every project needs a README",
        "command": "/document --type=readme"
    })

# Low coverage
IF coverage < 50:
    recommendations.append({
        "priority": "HIGH",
        "action": f"Document {len(undocumented_apis)} APIs",
        "reason": f"Coverage {coverage}% is very low",
        "command": "/document --type=api"
    })

# Outdated docs
IF len(outdated_docs) > 0:
    recommendations.append({
        "priority": "MEDIUM",
        "action": f"Update {len(outdated_docs)} outdated files",
        "reason": "Documentation drift detected (>90 days old)",
        "files": [f["file"] for f in outdated_docs]
    })

# Missing architecture docs
IF architecture_pattern != "Custom" AND "architecture" in missing_docs:
    recommendations.append({
        "priority": "MEDIUM",
        "action": "Create architecture documentation",
        "reason": f"Clear {architecture_pattern} pattern but no docs",
        "command": "/document --type=architecture"
    })
```

---

### Phase 5: Documentation Consolidation

**Step 5.1: Consolidate scattered docs**

**If multiple READMEs found:**
```
IF len(categorized["readme"]) > 1:
    Display: "⚠️ Multiple README files found:"
    FOR file in categorized["readme"]:
        Display: "  - {file}"

    AskUserQuestion: "Which README should be primary?"
    Options: [list of README files] + "Merge all into one"

    IF "Merge" selected:
        combined_content = merge_readmes(categorized["readme"])
        primary_readme = "README.md"
    ELSE:
        primary_readme = selected_file
        # Archive others
```

**Step 5.2: Extract reusable content**

```
FOR file in existing_docs:
    content = Read(file)

    # Extract reusable sections
    IF contains installation instructions:
        Extract and store in installation_content

    IF contains API examples:
        Extract and store in api_examples

    IF contains troubleshooting:
        Extract and store in troubleshooting_content
```

**Step 5.3: Create consolidated structure**

```
# Propose new documentation structure
new_structure = {
    "README.md": {
        "source": "Consolidated from existing + generated",
        "sections": ["Overview", "Installation", "Quick Start"],
        "word_count": estimated
    },
    "docs/DEVELOPER.md": {
        "source": "Generated from code analysis + existing guides",
        "sections": ["Architecture", "Development Workflow", "Standards"],
        "word_count": estimated
    },
    "docs/API.md": {
        "source": "Generated from code + consolidated examples",
        "sections": ["Endpoints", "Models", "Authentication"],
        "word_count": estimated
    }
}

Display proposed structure
AskUserQuestion: "Proceed with consolidation?"
```

---

## Performance Optimization

### For Large Codebases (>500 files)

**Strategy 1: Sampling**
```
IF file_count > 500:
    Display: "Large codebase detected ({file_count} files)"
    Display: "Using sampling for performance"

    # Sample every Nth file
    sample_rate = 10
    sampled_files = files[::sample_rate]

    Analyze sample, extrapolate to full codebase
```

**Strategy 2: Progressive scanning**
```
# Scan directories in priority order
priority_dirs = ["src/", "lib/", "app/"]

FOR dir in priority_dirs:
    Scan dir first
    If sufficient APIs found (>20), may skip lower priority dirs
```

**Strategy 3: Parallel analysis** (if supported)
```
# Analyze different aspects concurrently
Concurrent:
- Thread 1: Extract public APIs
- Thread 2: Extract dependencies
- Thread 3: Discover existing docs

Merge results
```

**Target:** <10 minutes for 500-file project

---

## Integration with Greenfield

**Hybrid projects** (both stories and existing code):

1. Run greenfield workflow for documented features (from stories)
2. Run brownfield workflow for undocumented features (from code)
3. Merge documentation:
   - Story-based features get complete docs
   - Code-only features get basic reference docs
4. Flag code-only features for story creation

---

**Last Updated:** 2025-11-18
**Version:** 1.0.0
**Lines:** 520 (target met)
