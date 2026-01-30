---
name: validate-stories
description: Validate existing stories against constitutional context files
argument-hint: "[STORY-ID|all]"
---

# /validate-stories Command

Post-hoc validation of existing story files against constitutional context files.

## Purpose

Validates one or more stories against the 6 context files to identify compliance issues that need resolution before development.

## Usage

```
/validate-stories STORY-042        # Validate specific story
/validate-stories all              # Validate all stories in devforgeai/specs/Stories/
/validate-stories --since=STORY-100  # Validate stories STORY-100 and higher
```

## Workflow

### Phase 1: Input Resolution

```
1. Parse $ARGUMENTS:
   IF $ARGUMENTS matches "STORY-\d+":
     story_id = $ARGUMENTS
     mode = "single"
   ELIF $ARGUMENTS == "all" OR $ARGUMENTS == "":
     mode = "all"
   ELIF $ARGUMENTS starts with "--since=":
     since_id = extract_story_id($ARGUMENTS)
     mode = "since"
   ELSE:
     HALT: "Invalid argument. Use: STORY-XXX, 'all', or --since=STORY-XXX"
```

### Phase 2: Story Discovery

```
2. Find stories to validate:
   story_dir = "devforgeai/specs/Stories/"

   IF mode == "single":
     story_files = Glob(pattern=f"{story_dir}{story_id}-*.story.md")
     IF len(story_files) == 0:
       HALT: f"Story not found: {story_id}"
   ELIF mode == "all":
     story_files = Glob(pattern=f"{story_dir}STORY-*.story.md")
   ELIF mode == "since":
     all_stories = Glob(pattern=f"{story_dir}STORY-*.story.md")
     story_files = filter_since(all_stories, since_id)

   Display: f"Found {len(story_files)} stories to validate"
```

### Phase 3: Context File Loading

```
3. Load context files (if exist):
   context_dir = "devforgeai/specs/context/"

   # Check for greenfield mode
   context_files = Glob(pattern=f"{context_dir}*.md")

   IF len(context_files) == 0:
     Display: """
     ℹ️ Greenfield Mode: No context files found

     Context validation requires:
     - devforgeai/specs/context/tech-stack.md
     - devforgeai/specs/context/source-tree.md
     - devforgeai/specs/context/dependencies.md
     - devforgeai/specs/context/coding-standards.md
     - devforgeai/specs/context/architecture-constraints.md
     - devforgeai/specs/context/anti-patterns.md

     Run /create-context to generate context files.
     """
     RETURN early (no validation possible)

   # Load all context files in PARALLEL
   Read(file_path="devforgeai/specs/context/tech-stack.md")
   Read(file_path="devforgeai/specs/context/source-tree.md")
   Read(file_path="devforgeai/specs/context/dependencies.md")
   Read(file_path="devforgeai/specs/context/coding-standards.md")
   Read(file_path="devforgeai/specs/context/architecture-constraints.md")
   Read(file_path="devforgeai/specs/context/anti-patterns.md")

   # Note which files exist
   context_status = {
     "tech_stack": file_exists,
     "source_tree": file_exists,
     "dependencies": file_exists,
     "coding_standards": file_exists,
     "architecture": file_exists,
     "anti_patterns": file_exists
   }

   Display: f"Context files loaded: {sum(context_status.values())}/6"
```

### Phase 4: Story Validation

```
4. For each story, run validation:

   Reference: .claude/skills/devforgeai-story-creation/references/context-validation.md

   all_results = []

   FOR each story_file in story_files:
     # Read story content
     story_content = Read(file_path=story_file)

     # Extract sections
     tech_spec = extract_section(story_content, "Technical Specification")
     dependencies = extract_section(story_content, "Dependencies")
     dod = extract_section(story_content, "Definition of Done")
     file_paths = extract_file_paths(tech_spec)

     # Run validation functions
     violations = []

     IF context_status["tech_stack"]:
       violations.extend(validate_technologies(tech_spec))

     IF context_status["source_tree"]:
       violations.extend(validate_file_paths(tech_spec))

     IF context_status["dependencies"]:
       violations.extend(validate_dependencies(dependencies))

     IF context_status["coding_standards"]:
       violations.extend(validate_coverage_thresholds(dod, file_paths))

     IF context_status["architecture"]:
       violations.extend(validate_architecture(tech_spec))

     IF context_status["anti_patterns"]:
       violations.extend(validate_anti_patterns(tech_spec))

     # Store result
     story_result = {
       "story_id": extract_story_id(story_file),
       "file": story_file,
       "violations": violations,
       "critical": count_by_severity(violations, "CRITICAL"),
       "high": count_by_severity(violations, "HIGH"),
       "medium": count_by_severity(violations, "MEDIUM"),
       "low": count_by_severity(violations, "LOW"),
       "status": "COMPLIANT" if len(critical + high) == 0 else "FAILED"
     }

     all_results.append(story_result)

     # Progress indicator
     Display: f"  ✓ {story_result['story_id']}: {story_result['status']}"
```

### Phase 5: Generate Report

```
5. Generate validation report:

   # Calculate summary
   total_stories = len(all_results)
   compliant = len([r for r in all_results if r["status"] == "COMPLIANT"])
   failed = len([r for r in all_results if r["status"] == "FAILED"])

   # Build report
   report = f"""
## Context Validation Report

**Generated:** {datetime.now().isoformat()}
**Stories Validated:** {total_stories}
**Context Files Used:** {sum(context_status.values())}/6

### Summary

| Status | Count |
|--------|-------|
| ✅ Compliant | {compliant} |
| ❌ Failed | {failed} |

**Compliance Rate:** {compliant}/{total_stories} ({compliant/total_stories*100:.1f}%)

### Results by Story

"""

   # Add per-story results
   FOR each result in all_results:
     IF result["status"] == "COMPLIANT":
       report += f"✅ **{result['story_id']}** - Compliant\n"
     ELSE:
       report += f"""
❌ **{result['story_id']}** - {result['critical']} CRITICAL, {result['high']} HIGH, {result['medium']} MEDIUM, {result['low']} LOW

"""
       FOR each violation in result["violations"]:
         IF violation.severity in ["CRITICAL", "HIGH"]:
           report += f"  - [{violation.severity}] {violation.type}: {violation.description}\n"

   # Add recommendations
   IF failed > 0:
     report += f"""
### Recommendations

{failed} stories have CRITICAL or HIGH violations that should be resolved:

1. Review violations listed above
2. Use AskUserQuestion resolution options:
   - "Fix in story" - Edit the story file
   - "Update context file" - Requires ADR
   - "Defer to manual review" - Flag for later

To fix a specific story:
  `/validate-stories STORY-XXX` then address each violation

"""

   Display: report
```

### Phase 6: Interactive Resolution (Optional)

```
6. If single story mode and violations found:

   IF mode == "single" AND failed == 1:
     violations = all_results[0]["violations"]
     critical_high = [v for v in violations if v.severity in ["CRITICAL", "HIGH"]]

     IF len(critical_high) > 0:
       AskUserQuestion:
         Question: f"Story has {len(critical_high)} blocking violations. Fix now?"
         Header: "Fix issues"
         Options:
           - "Fix all"
             Description: "Walk through each violation and resolve"
           - "Show details only"
             Description: "Just show details, don't fix"
           - "Skip"
             Description: "Exit without changes"

       IF user_choice == "Fix all":
         FOR each violation in critical_high:
           AskUserQuestion:
             Question: f"[{violation.severity}] {violation.type}: {violation.description}\n\nHow to resolve?"
             Header: "Resolution"
             Options:
               - "Fix in story"
                 Description: "I'll provide the correct value"
               - "Update context file"
                 Description: "Requires ADR - constraint should change"
               - "Defer to manual review"
                 Description: "Flag for later, mark as warning"

           # Apply resolution based on choice
           IF choice == "Fix in story":
             AskUserQuestion: "What is the correct value?"
             Apply fix to story file via Edit
           ELIF choice == "Update context file":
             Display: "Create ADR first, then update context file, then re-run /validate-stories"
           ELIF choice == "Defer":
             # Add deferral note to story
             Edit story to add: "<!-- DEFERRED: {violation.type} - pending manual review -->"

         # Re-validate after fixes
         Display: "Re-validating after fixes..."
         GOTO Phase 4 (for single story)
```

## Output

**Success (all compliant):**
```
✅ Context Validation Complete

Stories Validated: 15
Compliant: 15/15 (100%)

All stories are compliant with context files.
```

**Partial (some violations):**
```
⚠️ Context Validation Complete

Stories Validated: 15
Compliant: 12/15 (80%)
Failed: 3

See report above for violations and remediation steps.
```

## Integration

**Invokes validation logic from:**
- `.claude/skills/devforgeai-story-creation/references/context-validation.md`

**Can be called:**
- Standalone via `/validate-stories`
- From CI/CD pipelines (non-interactive mode via args)
- After bulk story creation
- Before sprint planning to ensure stories are ready

**Related commands:**
- `/create-context` - Generate context files (required for validation)
- `/create-story` - Create new story (includes context validation)
- `/qa` - Quality validation (includes context compliance)
