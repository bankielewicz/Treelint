# Research Workflow Reference

Detailed implementation guide for all 6 research phases.

---

## Phase 0: Initialization (Detailed)

### Step 0.1: Mode Detection

**Check command arguments:**
```python
args = parse_command_args()

if "--resume" in args:
    mode = "resume"
    research_id = extract_id(args["--resume"])
elif "--search" in args:
    mode = "search"
    query = extract_query(args["--search"])
elif "--list" in args:
    mode = "list"
    category_filter = args.get("--category", None)
elif "--category" in args:
    mode = "list"
    category_filter = args["--category"]
else:
    mode = "new"
```

### Step 0.2: ID Generation (New Mode Only)

**Find next available ID:**
```bash
# Glob all research files
existing_files = Glob("devforgeai/specs/research/RESEARCH-*.research.md")

# Extract IDs
ids = []
for file in existing_files:
    match = re.match(r"RESEARCH-(\d{3})", basename(file))
    if match:
        ids.append(int(match.group(1)))

# Next ID
next_id = max(ids) + 1 if ids else 1
research_id = f"RESEARCH-{next_id:03d}"
```

### Step 0.3: Load Research Index

**Read and parse index:**
```
index_file = "devforgeai/specs/research/research-index.md"

IF exists(index_file):
    content = Read(index_file)
    parse_markdown_table(content)
ELSE:
    Display: "Research index not found. Creating..."
    create_initial_index()
```

### Step 0.4: Directory Creation

**Ensure directories exist:**
```bash
dirs = [
    "devforgeai/specs/research/",
    ".claude/skills/devforgeai-research/references/",
    ".claude/skills/devforgeai-research/assets/templates/"
]

for dir in dirs:
    if not exists(dir):
        Bash(command=f"mkdir -p {dir}")
```

---

## Phase 1: Topic Definition (Detailed)

### Step 1.1: Topic Input

**Interactive topic gathering:**
```
topic_response = AskUserQuestion(
    question: "What is the research topic?",
    header: "Topic",
    options: [
        {label: "New topic", description: "Start fresh research"},
        {label: "Expand RESEARCH-XXX", description: "Add to existing research"}
    ]
)

IF topic_response == "Expand":
    expand_id = AskUserQuestion("Which research ID?")
    Read(f"devforgeai/specs/research/{expand_id}-*.research.md")
    Display: "Loaded {expand_id}. New findings will be appended."
    mode = "expand"
ELSE:
    topic = AskUserQuestion("Enter research topic:")
```

### Step 1.2: Category Selection

**Category determines research strategy:**
```
category = AskUserQuestion(
    question: "Research category?",
    header: "Category",
    multiSelect: false,
    options: [
        {
            label: "Competitive Analysis",
            description: "Competitor features, pricing, market position, strengths/weaknesses"
        },
        {
            label: "Technology Evaluation",
            description: "Libraries, frameworks, tools - capabilities, performance, adoption"
        },
        {
            label: "Market Research",
            description: "Industry trends, developer needs, statistics, pain points"
        },
        {
            label: "Integration Planning",
            description: "External services, APIs, SDKs - integration requirements"
        },
        {
            label: "Architecture Research",
            description: "Design patterns, best practices, architectural styles"
        }
    ]
)

# Map to internal category code
category_map = {
    "Competitive Analysis": "competitive",
    "Technology Evaluation": "technology",
    "Market Research": "market",
    "Integration Planning": "integration",
    "Architecture Research": "architecture"
}

category_code = category_map[category]
```

### Step 1.3: Research Questions

**Define what questions research should answer:**
```
questions = []

approach = AskUserQuestion(
    question: "How would you like to define research questions?",
    header: "Questions",
    options: [
        {label: "Enter interactively (Recommended)", description: "Add questions one by one"},
        {label: "Provide later", description: "Define questions as research progresses"}
    ]
)

IF approach == "Enter interactively":
    while True:
        q = AskUserQuestion(
            question: "Enter a research question (or type 'done' when finished):",
            header: f"Question {len(questions) + 1}"
        )

        if q.lower() == "done":
            break

        questions.append(q)

    Display: f"Captured {len(questions)} research questions"
```

### Step 1.4: Duplicate Detection

**Check for similar existing research:**
```
# Extract keywords from topic
keywords = extract_keywords(topic)

# Search existing research
similar_research = []

for keyword in keywords:
    matches = Grep(
        pattern=keyword,
        path="devforgeai/specs/research/",
        glob="*.research.md",
        output_mode="files_with_matches",
        case_insensitive=true
    )
    similar_research.extend(matches)

# Deduplicate
similar_research = unique(similar_research)

IF len(similar_research) > 0:
    Display: "Similar research found:"
    for research in similar_research:
        Read(research, limit=10)  # Read frontmatter only
        Display: f"  - {id}: {title} ({category})"

    action = AskUserQuestion(
        question: "How would you like to proceed?",
        header: "Duplicate",
        options: [
            {label: "Continue with new research", description: "Topics are different enough"},
            {label: "Resume existing research", description: "Add to existing document"},
            {label: "Cancel", description: "Research not needed"}
        ]
    )

    IF action == "Resume existing":
        selected_id = AskUserQuestion("Which research ID to resume?")
        GOTO Phase 0 with --resume flag
    ELIF action == "Cancel":
        EXIT skill
```

---

## Phase 2: Research Execution (Detailed)

### Competitive Analysis Strategy

**Search sequence:**
```
# Step 1: Product overview
search_1 = WebSearch(
    query=f"{competitor_name} features pricing 2026"
)

# Step 2: Comparison
search_2 = WebSearch(
    query=f"{competitor_name} vs {our_product} comparison"
)

# Step 3: Reviews and sentiment
search_3 = WebSearch(
    query=f"{competitor_name} reviews developer experience 2026"
)

# Step 4: Official site
homepage = WebFetch(
    url=competitor_homepage,
    prompt="Extract: key features, pricing tiers, target audience, unique selling points"
)

# Step 5: Documentation quality
docs = WebFetch(
    url=competitor_docs,
    prompt="Assess: documentation quality, getting started experience, API design"
)
```

### Technology Evaluation Strategy

**Evaluation sequence:**
```
# Step 1: Repository health
repo = WebFetch(
    url=github_repo_url,
    prompt="Extract: star count, last commit date, issue count, contributor count, README quality"
)

# Step 2: Official documentation
docs = WebFetch(
    url=official_docs_url,
    prompt="Extract: API design, performance characteristics, limitations, browser/platform support"
)

# Step 3: Performance benchmarks
benchmarks = WebSearch(
    query=f"{technology_name} performance benchmarks 2026"
)

# Step 4: Adoption and ecosystem
adoption = WebSearch(
    query=f"{technology_name} npm downloads OR github stars OR market share 2026"
)

# Step 5: Comparison with alternatives
comparison = WebSearch(
    query=f"{technology_name} vs {alternative_1} vs {alternative_2} comparison"
)
```

### Market Research Strategy

**Search sequence:**
```
# Step 1: Industry statistics
stats = WebSearch(
    query=f"{market_topic} statistics market size 2026"
)

# Step 2: Developer surveys
surveys = WebSearch(
    query=f"developer survey {market_topic} 2025 2026 Stack Overflow JetBrains"
)

# Step 3: Trend analysis
trends = WebSearch(
    query=f"{market_topic} trends predictions 2026"
)

# Step 4: Pain points
pain_points = WebSearch(
    query=f"developer frustrations {market_topic} 2026"
)
```

### Integration Planning Strategy

**Research sequence:**
```
# Step 1: Internal codebase check
existing = Grep(
    pattern=integration_keywords,
    path=project_root,
    output_mode="files_with_matches"
)

# Step 2: Official integration docs
integration_docs = WebFetch(
    url=service_integration_docs,
    prompt="Extract: authentication methods, API endpoints, rate limits, error handling, SDKs available"
)

# Step 3: Community integrations
community = WebSearch(
    query=f"{service_name} integration examples github 2026"
)

# Step 4: Known issues
issues = WebSearch(
    query=f"{service_name} integration problems issues 2026"
)
```

### Architecture Research Strategy

**Research sequence:**
```
# Step 1: Pattern definition
pattern_def = WebSearch(
    query=f"{pattern_name} design pattern explanation 2026"
)

# Step 2: Best practices
best_practices = WebFetch(
    url=authoritative_source,
    prompt="Extract: when to use, when not to use, implementation guidelines, common pitfalls"
)

# Step 3: Real-world examples
examples = WebSearch(
    query=f"{pattern_name} implementation examples github 2026"
)

# Step 4: Trade-offs
tradeoffs = WebSearch(
    query=f"{pattern_name} advantages disadvantages trade-offs"
)
```

---

## Phase 3: Findings Synthesis (Detailed)

### Step 3.1: Theme Extraction

**Group findings by similarity:**
```python
def group_findings_by_theme(raw_findings):
    themes = {}

    for finding in raw_findings:
        # Extract key concepts
        concepts = extract_concepts(finding)

        # Find matching theme
        theme_match = None
        for theme_name, theme_findings in themes.items():
            if concepts_overlap(concepts, theme_findings.concepts):
                theme_match = theme_name
                break

        # Add to existing theme or create new
        if theme_match:
            themes[theme_match].append(finding)
        else:
            new_theme = generate_theme_name(concepts)
            themes[new_theme] = [finding]

    return themes
```

### Step 3.2: Finding Structuring

**Structure each finding:**
```markdown
### Finding {N}: {Title}

{Description paragraph synthesizing multiple sources}

**Evidence:**
- [{Source 1 Title}]({URL}) - "{Quote or key data point}"
- [{Source 2 Title}]({URL}) - "{Quote or key data point}"
- [{Source 3 Title}]({URL}) - "{Quote or key data point}"

**Implications for DevForgeAI:**
- {Implication 1}: {How this affects our framework}
- {Implication 2}: {How this affects our framework}

**Confidence Level:** High | Medium | Low
{Justification for confidence level}
```

### Step 3.3: Insight Extraction

**Identify patterns across findings:**
```
insights = []

# Cross-cutting patterns
for theme_A in themes:
    for theme_B in themes:
        if theme_A != theme_B:
            connection = find_connection(theme_A, theme_B)
            if connection:
                insights.append({
                    type: "cross-cutting",
                    themes: [theme_A, theme_B],
                    insight: connection
                })

# Contradictions
contradictions = find_contradictions(findings)
for contradiction in contradictions:
    insights.append({
        type: "contradiction",
        sources: contradiction.sources,
        insight: "Conflicting information requires deeper investigation"
    })

# Gaps
expected_findings = generate_expected_from_questions(questions)
actual_findings = extract_findings(findings)
gaps = expected_findings - actual_findings

for gap in gaps:
    insights.append({
        type: "gap",
        question: gap.question,
        insight: "Research question not fully answered - needs additional investigation"
    })
```

### Step 3.4: Recommendation Generation

**Generate actionable recommendations:**
```
recommendations = []

for finding in findings:
    if finding.has_actionable_insight:
        recommendation = {
            action: extract_action(finding),
            rationale: extract_rationale(finding),
            priority: calculate_priority(finding),
            effort: estimate_effort(finding),
            dependencies: extract_dependencies(finding)
        }
        recommendations.append(recommendation)

# Prioritize
recommendations = sort_by_priority(recommendations)
```

---

## Phase 4: Documentation (Detailed)

### Step 4.1: Template Loading

**Load and customize template:**
```
template_path = ".claude/skills/devforgeai-research/assets/templates/research-template.md"
template = Read(template_path)

# Calculate review date (6 months from now)
created_date = datetime.now()
review_date = created_date + timedelta(days=180)

# Fill frontmatter
frontmatter = {
    "id": research_id,
    "title": topic,
    "category": category_code,
    "status": "complete",
    "created": created_date.strftime("%Y-%m-%d"),
    "updated": created_date.strftime("%Y-%m-%d"),
    "review_by": review_date.strftime("%Y-%m-%d"),
    "sources_count": len(sources),
    "related_epics": [],
    "related_stories": [],
    "tags": extract_tags(topic, findings)
}
```

### Step 4.2: Content Population

**Fill all sections:**
```
document = template

# Executive Summary
executive_summary = generate_summary(findings, max_sentences=3)
document = replace(document, "{executive_summary}", executive_summary)

# Research Questions
questions_md = "\n".join([f"{i+1}. {q}" for i, q in enumerate(questions)])
document = replace(document, "{research_questions}", questions_md)

# Key Findings
findings_md = ""
for i, finding in enumerate(findings):
    findings_md += format_finding(finding, number=i+1)
document = replace(document, "{findings}", findings_md)

# Recommendations
recommendations_md = ""
for i, rec in enumerate(recommendations):
    recommendations_md += format_recommendation(rec, number=i+1)
document = replace(document, "{recommendations}", recommendations_md)

# Sources
sources_md = format_sources(sources)
document = replace(document, "{sources}", sources_md)
```

### Step 4.3: File Writing

**Write with proper naming:**
```
# Generate slug from title
slug = topic.lower()
slug = re.sub(r'[^a-z0-9]+', '-', slug)
slug = slug.strip('-')[:50]  # Max 50 chars

# Full filename
filename = f"{research_id}-{slug}.research.md"
filepath = f"devforgeai/specs/research/{filename}"

# Write file
Write(file_path=filepath, content=document)

Display: f"Research document created: {filepath}"
```

### Step 4.4: Assets Folder Creation

**Create folder for attachments:**
```
assets_dir = f"devforgeai/specs/research/{research_id}/"

Bash(command=f"mkdir -p {assets_dir}")

Display: f"""
Assets folder created: {assets_dir}

You can now add:
  - Screenshots: {research_id}/screenshot-{name}.png
  - Diagrams: {research_id}/diagram-{name}.svg
  - PDFs: {research_id}/document-{name}.pdf

Reference in research doc:
  ![Screenshot]({research_id}/screenshot-{name}.png)
  [PDF]({research_id}/document-{name}.pdf)
"""
```

### Step 4.5: Index Update

**Add entry to research index:**
```
index_path = "devforgeai/specs/research/research-index.md"

# Read current index
index_content = Read(index_path)

# Create new row
new_row = f"| {research_id} | [{title}]({filename}) | {category_code} | complete | {created_date} | {review_date} |"

# Insert before marker
Edit(
    file_path=index_path,
    old_string="<!--- INSERT NEW RESEARCH HERE --->",
    new_string=f"{new_row}\n<!--- INSERT NEW RESEARCH HERE --->"
)

Display: "Research index updated"
```

---

## Phase 5: Cross-Reference (Detailed)

### Step 5.1: Epic/Story Linking

**Link research to epics/stories:**
```
link_choice = AskUserQuestion(
    question: "Should this research be linked to epics or stories?",
    header: "Cross-Reference",
    multiSelect: true,
    options: [
        {label: "Link to epics", description: "This research informs epic planning"},
        {label: "Link to stories", description: "This research supports story implementation"},
        {label: "Link to ADRs", description: "This research supports architecture decisions"},
        {label: "No linking needed", description: "Standalone research"}
    ]
)

IF "Link to epics" in link_choice:
    epic_ids = AskUserQuestion("Enter epic IDs (comma-separated, e.g., EPIC-001,EPIC-045):")

    for epic_id in parse_csv(epic_ids):
        epic_files = Glob(f"devforgeai/specs/Epics/{epic_id}*.epic.md")

        IF epic_files:
            epic_file = epic_files[0]

            # Add to References section
            Edit(
                file_path=epic_file,
                old_string="## References",
                new_string=f"""## References

- Research: [{research_id}: {title}](../research/{filename}) - {executive_summary[:100]}"""
            )

            # Update research doc with backlink
            Edit(
                file_path=research_filepath,
                old_string="related_epics: []",
                new_string=f"related_epics: [{epic_id}]"
            )
```

### Step 5.2: ADR Integration

**For technology research, check if ADR needed:**
```
IF category_code == "technology":
    # Check if research recommends new technology
    new_tech_recommended = check_recommendations_for_new_tech(recommendations)

    IF new_tech_recommended:
        Display: """
Technology Research Alert

This research recommends adopting new technology.
An Architecture Decision Record (ADR) should be created to document:
  - Decision to adopt (or not)
  - Alternatives considered
  - Rationale
  - Consequences

Create ADR now?
"""

        create_adr = AskUserQuestion(
            question: "Create ADR for this technology decision?",
            header: "ADR",
            options: [
                {label: "Yes, create ADR now", description: "Document decision immediately"},
                {label: "No, create ADR later", description: "I'll create it manually"},
                {label: "Not needed", description: "No technology decision being made"}
            ]
        )

        IF create_adr == "Yes, create ADR now":
            # Invoke ADR creation skill/workflow
            Skill(command="devforgeai-architecture", args=f"--create-adr {research_id}")
```

---

## Phase 6: Completion Summary (Detailed)

**Display comprehensive summary:**
```
summary = f"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Research Completed
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Research Details:
  ID: {research_id}
  Title: {title}
  Category: {category_code}
  Status: complete
  Created: {created_date}
  Review By: {review_date}

Content Summary:
  Research Questions: {len(questions)}
  Key Findings: {len(findings)}
  Recommendations: {len(recommendations)}
  Sources: {len(sources)}
  Tags: {", ".join(tags)}

Files Created:
  {research_filepath}
  {assets_dir} (for attachments)

{IF epic_links:
Related Epics:
  {", ".join(epic_links)}
}

{IF adr_created:
ADR Created: ADR-{adr_number} (linked to this research)
}

Next Steps:
  1. Review research document for accuracy
  2. {IF attachments_pending: Add screenshots/diagrams to assets folder}
  3. {IF new_tech: Ensure ADR created for technology decisions}
  4. {IF action_items: Create stories for actionable recommendations}
  5. Research will be reviewed on: {review_date}

Quick Access Commands:
  /research --resume {research_id}      # Update this research
  /research --search "{primary_tag}"     # Find related research
  /research --list --category {category_code}  # Browse category

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"""

Display(summary)
```

---

**Last Updated:** 2026-01-18
**Version:** 1.0
**Framework:** DevForgeAI Research
