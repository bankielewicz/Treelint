---
name: devforgeai-research
description: Capture, persist, and query research findings across sessions. Transforms web research, competitive analysis, technology evaluations, and market research into structured documents that survive session restarts.
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - WebSearch
  - WebFetch
  - AskUserQuestion
  - TodoWrite
  - Task
model: opus
---

# DevForgeAI Research Skill

Capture and persist research findings across sessions in structured, queryable documents.

---

## EXECUTION MODEL: This Skill Expands Inline

**After invocation, YOU (Claude) execute these instructions phase by phase.**

**When you invoke this skill:**
1. This SKILL.md content is now in your conversation
2. You execute each phase sequentially
3. You display results as you work through phases
4. You complete with success/failure report

**Do NOT:**
- Wait passively for skill to "return results"
- Assume skill is executing elsewhere
- Stop workflow after invocation

**Proceed to "Purpose" section below and begin execution.**

---

## Purpose

This skill serves as the **knowledge persistence layer** for DevForgeAI. It transforms ephemeral research (web searches, competitive analysis, technology evaluations) into persistent, structured documents that survive session restarts.

### Core Philosophy

**"Research Once, Reference Forever"**
- Capture research in structured documents
- Research survives context window clears
- Research is queryable and reusable

**"Evidence-Based Decision Making"**
- All recommendations cite research
- Research links to epics/stories/ADRs
- Bidirectional traceability

**"Knowledge Compounds Over Time"**
- Research index grows with each session
- Related research cross-referenced
- Staleness tracking ensures currency

---

## When to Use This Skill

### Trigger Scenarios

- Competitive analysis (AWS Kiro, Cursor, etc.)
- Technology evaluation (Treelint, new libraries)
- Market research (developer frustrations, trends)
- Integration planning (external tools, APIs)
- Architecture research (patterns, best practices)

### When NOT to Use

- Documenting code implementation (use stories)
- Recording decisions (use ADRs)
- Tracking bugs/issues (use RCA)

---

## Research Workflow (6 Phases)

**EXECUTION STARTS HERE - You are now executing the skill's workflow.**

### Phase 0: Initialization

**Reference:** `references/research-workflow.md` (Phase 0 section)

**Steps:**

1. **Detect Mode:**
   ```
   IF command includes --resume RESEARCH-NNN:
     mode = "resume"
     research_id = RESEARCH-NNN
   ELSE IF command includes --search query:
     mode = "search"
     GOTO Phase 0.5 (Search)
   ELSE IF command includes --list:
     mode = "list"
     GOTO Phase 0.6 (List)
   ELSE:
     mode = "new"
   ```

2. **Generate Research ID (if new):**
   ```
   existing = Glob("devforgeai/specs/research/RESEARCH-*.research.md")
   existing_ids = extract_ids(existing)
   next_id = max(existing_ids) + 1
   research_id = f"RESEARCH-{next_id:03d}"
   ```

3. **Load Research Index:**
   ```
   Read("devforgeai/specs/research/research-index.md")
   Parse index to understand existing research
   ```

4. **Create Directories if Needed:**
   ```
   IF NOT exists("devforgeai/specs/research/"):
     Create directory
   ```

**Output:** `research_id`, `mode`, index loaded

---

### Phase 0.5: Search Mode (Optional)

**Triggered by:** `--search query`

**Steps:**

1. **Grep Research Files:**
   ```
   results = Grep(
     pattern=query,
     path="devforgeai/specs/research/",
     glob="*.research.md",
     output_mode="files_with_matches"
   )
   ```

2. **Display Results:**
   ```
   FOR each result:
     Read frontmatter
     Display: ID, Title, Category, Status, Created

   Summary: Found {count} research documents matching "{query}"
   ```

3. **Exit Skill**

---

### Phase 0.6: List Mode (Optional)

**Triggered by:** `--list` or `--category {type}`

**Steps:**

1. **Load All Research:**
   ```
   all_research = Glob("devforgeai/specs/research/RESEARCH-*.research.md")
   ```

2. **Filter by Category (if specified):**
   ```
   IF --category provided:
     filtered = filter_by_category(all_research, category)
   ELSE:
     filtered = all_research
   ```

3. **Display Table:**
   ```
   | ID | Title | Category | Status | Created | Review By |
   |----|-------|----------|--------|---------|-----------|
   | RESEARCH-001 | ... | competitive | complete | 2026-01-18 | 2026-07-18 |
   ```

4. **Exit Skill**

---

### Phase 1: Topic Definition

**Reference:** `references/research-workflow.md` (Phase 1 section)

**Interactive Questions:**

1. **Research Topic:**
   ```
   AskUserQuestion(
     question: "What is the research topic?",
     header: "Topic",
     options: [
       "New topic (enter below)",
       "Expand existing research RESEARCH-XXX"
     ]
   )
   ```

2. **Research Category:**
   ```
   AskUserQuestion(
     question: "What category does this research fall under?",
     header: "Category",
     options: [
       {label: "Competitive Analysis", description: "Competitor features, pricing, positioning"},
       {label: "Technology Evaluation", description: "Libraries, tools, frameworks"},
       {label: "Market Research", description: "Trends, statistics, developer needs"},
       {label: "Integration Planning", description: "External tools, APIs, services"},
       {label: "Architecture Research", description: "Patterns, best practices, case studies"}
     ]
   )
   ```

3. **Research Questions:**
   ```
   AskUserQuestion(
     question: "What key questions should this research answer?",
     header: "Questions",
     options: [
       "Enter questions interactively (recommended)",
       "I'll provide them as I go"
     ]
   )

   IF interactive:
     LOOP until user says "done":
       question = AskUserQuestion("Enter a research question (or 'done'):")
       questions.append(question)
   ```

4. **Check for Duplicates:**
   ```
   similar = search_index(topic_keywords)
   IF similar exists:
     Display: "Similar research found: {similar.id} - {similar.title}"
     AskUserQuestion: "Continue with new research or resume {similar.id}?"
   ```

**Output:** `topic`, `category`, `questions[]`, checked for duplicates

---

### Phase 2: Research Execution (via internet-sleuth)

**Reference:** `references/research-workflow.md` (Phase 2 section)

This phase delegates research execution to the internet-sleuth subagent, which provides:
- Repository archaeology (clone & analyze GitHub repos)
- Context-validator integration (quality gates)
- Progressive methodology loading (65% token savings)
- Workflow state awareness

**Step 2.1: Map Category to Sleuth Research Mode**

```python
category_to_mode = {
    "competitive": "competitive-analysis",
    "technology": "repository-archaeology",
    "market": "market-intelligence",
    "integration": "investigation",
    "architecture": "discovery"
}

research_mode = category_to_mode[category]
```

**Step 2.2: Invoke internet-sleuth Subagent**

```
Task(
    subagent_type="internet-sleuth",
    description=f"{category} research: {topic}",
    prompt=f"""
Research Mode: {research_mode}
Topic: {topic}
Research ID: {research_id}

Research Questions:
{format_numbered_list(questions)}

Execute comprehensive research following your methodology.

Return structured findings with:
1. Executive summary (2-3 sentences max)
2. Key findings with evidence and citations
3. Recommendations ranked by priority (High/Medium/Low)
4. Sources with credibility assessment
5. Framework compliance check results (if brownfield)

Note: Results will be formatted into {research_id} research document.
"""
)
```

**Step 2.3: Handle Sleuth Results**

```python
sleuth_result = await_task_completion()

IF sleuth_result.success:
    findings = sleuth_result.findings
    recommendations = sleuth_result.recommendations
    sources = sleuth_result.sources
    compliance_check = sleuth_result.framework_compliance

    Display: "✓ Research execution complete"
    Display: f"  Findings: {len(findings)}"
    Display: f"  Recommendations: {len(recommendations)}"
    Display: f"  Sources: {len(sources)}"

    PROCEED to Phase 3 (Synthesis)

ELSE:
    # Fallback to direct web search
    Display: "⚠️ internet-sleuth returned error, falling back to direct search"

    # Use legacy search-strategies.md methodology
    Read(".claude/skills/devforgeai-research/references/search-strategies.md")
    Execute category-specific search strategy (see reference file)
```

**Output:** `findings[]`, `recommendations[]`, `sources[]`, `compliance_check`

---

### Phase 3: Findings Synthesis

**Reference:** `references/research-workflow.md` (Phase 3 section)

**Note:** When using internet-sleuth delegation, synthesis is simplified because sleuth already:
- Groups findings by theme
- Extracts key insights
- Generates prioritized recommendations
- Validates against context files

**Step 3.1: Validate Sleuth Output Structure**

```python
required_sections = ["findings", "recommendations", "sources"]
missing = [s for s in required_sections if s not in sleuth_result]

IF missing:
    Display: f"⚠️ Sleuth output missing: {missing}"
    # Request user input or use defaults
ELSE:
    Display: "✓ Sleuth output validated"
```

**Step 3.2: Enrich Recommendations (Optional)**

```python
# Add DevForgeAI-specific context if not present
FOR each recommendation in recommendations:
    IF not recommendation.has_devforgeai_context:
        recommendation.add_context(
            framework_impact=assess_impact(recommendation),
            implementation_effort=estimate_effort(recommendation)
        )
```

**Step 3.3: Format for Research Template**

```python
# Convert sleuth output format to research template format
formatted_findings = format_findings_for_template(findings)
formatted_recommendations = format_recommendations_for_template(recommendations)
formatted_sources = format_sources_for_template(sources)
```

**Output:** `formatted_findings`, `formatted_recommendations`, `formatted_sources`

---

### Phase 4: Documentation

**Reference:** `references/research-workflow.md` (Phase 4 section) + `assets/templates/research-template.md`

**Steps:**

1. **Load Template:**
   ```
   template = Read(".claude/skills/devforgeai-research/assets/templates/research-template.md")
   ```

2. **Populate Template:**
   ```
   document = fill_template(
     template,
     research_id=research_id,
     title=topic,
     category=category,
     created=today,
     review_by=today + 6 months,
     questions=questions,
     findings=findings,
     recommendations=recommendations,
     sources=sources
   )
   ```

3. **Write Research Document:**
   ```
   Write(
     file_path=f"devforgeai/specs/research/{research_id}-{slug(topic)}.research.md",
     content=document
   )
   ```

4. **Create Assets Folder:**
   ```
   IF attachments_needed:
     Bash(command=f"mkdir -p devforgeai/specs/research/{research_id}/")
     Display: "Assets folder created at devforgeai/specs/research/{research_id}/"
     Display: "Add screenshots/PDFs there and reference in research doc"
   ```

5. **Update Research Index:**
   ```
   Read("devforgeai/specs/research/research-index.md")

   new_entry = f"| {research_id} | {title} | {category} | {status} | {created} | {review_by} |"

   Edit(
     file_path="devforgeai/specs/research/research-index.md",
     old_string="<!--- INSERT NEW RESEARCH HERE --->",
     new_string=f"{new_entry}\n<!--- INSERT NEW RESEARCH HERE --->"
   )
   ```

**Output:** Research document written, index updated

---

### Phase 5: Cross-Reference

**Reference:** `references/research-workflow.md` (Phase 5 section)

**Steps:**

1. **Identify Related Work:**
   ```
   AskUserQuestion(
     question: "Should this research be linked to existing epics or stories?",
     header: "Linking",
     options: [
       "Yes, link to specific items",
       "No, standalone research",
       "Link to ADRs (technology decisions)"
     ]
   )
   ```

2. **Link to Epics/Stories (if selected):**
   ```
   IF link_to_epics:
     epic_ids = AskUserQuestion("Enter epic IDs (comma-separated): ")

     FOR each epic_id:
       Read(f"devforgeai/specs/Epics/{epic_id}.epic.md")

       Edit(
         file_path=epic_file,
         old_string="## References",
         new_string=f"## References\n\n- Research: [{research_id}](../research/{research_file}) - {topic}"
       )
   ```

3. **Link to ADRs (if technology research):**
   ```
   IF category == "technology" AND link_to_adrs:
     adr_needed = check_if_new_technology(findings)

     IF adr_needed:
       Display: "This research suggests new technology adoption"
       Display: "Consider creating ADR to document decision"
       Display: f"Reference this research in ADR: {research_id}"
   ```

**Output:** Cross-references created

---

### Phase 6: Completion Summary

**Display:**

```
Research Completed

Research Details:
  ID: {research_id}
  Title: {title}
  Category: {category}
  Created: {created}
  Review By: {review_by}

Key Findings: {findings_count}
Recommendations: {recommendations_count}
Sources: {sources_count}

Files Created:
  devforgeai/specs/research/{research_file}
  {IF assets: devforgeai/specs/research/{research_id}/ (assets folder)}

Next Steps:
  1. Review research document: {research_file}
  2. {IF new_tech: Create ADR for technology decisions}
  3. {IF epic_related: Link to epics via Phase 5 or manually}
  4. Research will be reviewed on: {review_by}

Quick Access:
  View: /research --resume {research_id}
  Search: /research --search "{keyword}"
```

---

## Reference Files

**Load these on-demand during workflow execution:**

### Core Workflow
- `references/research-workflow.md` - Detailed phase execution (500 lines)
- `assets/templates/research-template.md` - Document template (120 lines)
- `references/citation-standards.md` - Source formatting (80 lines)
- `references/search-strategies.md` - Search tips by category (150 lines)

**Total reference content:** ~850 lines (loaded progressively)

---

## Common Issues

**Top 3 issues and quick solutions:**

1. **Duplicate research detected** - Offer to resume existing or create supplement
2. **No web results found** - Adjust search query, try alternative keywords
3. **Research index not found** - Auto-create on first use

---

**The research skill ensures knowledge persists across sessions and compounds over time.**
