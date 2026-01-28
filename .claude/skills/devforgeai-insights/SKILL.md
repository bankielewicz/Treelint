---
name: devforgeai-insights
description: Orchestrates session data mining operations to deliver formatted, actionable insights from Claude Code history.jsonl files and workflow artifacts.
version: "1.0"
story: STORY-225
epic: EPIC-034
---

# DevForgeAI Insights Skill

Orchestrate session data mining operations through the session-miner subagent to deliver formatted, actionable insights from Claude Code session history.

**Story Reference:** STORY-225 - Implement devforgeai-insights Skill for Mining Orchestration
**Epic Reference:** EPIC-034 - Session Data Mining Infrastructure

---

## Purpose

This skill serves as the orchestration layer for EPIC-034 Session Data Mining. It:

1. **Coordinates the session-miner subagent** to extract patterns from history.jsonl files
2. **Aggregates and filters results** from raw session data into meaningful insights
3. **Formats output** as user-friendly markdown with tables, summaries, and recommendations
4. **Caches results** to optimize repeated queries (1-hour TTL)

**Integration Points:**
- Invoked by: `/insights` command (STORY-224)
- Invokes: `session-miner` subagent (STORY-221, STORY-222, STORY-223)
- Outputs to: User display via markdown formatting

---

## Query Types

The skill supports multiple query types, each generating specialized prompts for the session-miner subagent:

### Dashboard Query
**Purpose:** High-level overview of session activity and health metrics
**Prompt Generation:** Aggregates workflow counts, error rates, completion times

### Workflows Query
**Purpose:** Analyze workflow patterns and execution frequencies
**Prompt Generation:** Groups by workflow type, calculates success rates

### Errors Query
**Purpose:** Identify common error patterns and failure points
**Prompt Generation:** Extracts error messages, categorizes by type, ranks by frequency

### Decisions Query
**Purpose:** Surface architectural and implementation decisions made during development
**Prompt Generation:** Searches for decision patterns, ADR references, AskUserQuestion interactions

### Story-Specific Query
**Purpose:** Deep analysis of a specific story's development history
**Prompt Generation:** Filters by story ID, extracts timeline, identifies key events

### Command-Patterns Query (STORY-226)
**Purpose:** Identify high-frequency command sequences for workflow optimization
**Prompt Generation:** Routes to session-miner n-gram analysis, extracts 2-grams and 3-grams
**Output:** Top 10 command sequences ranked by frequency with success rates

---

## Workflow

The skill executes a 4-phase workflow for each query request:

### Phase 1: Cache Check

**Purpose:** Return cached results if available and fresh (1-hour TTL)

```
1. Generate cache key from query parameters
   cache_key = hash(query_type + filters + date_range)

2. Check cache file existence
   cache_path = devforgeai/cache/insights/{cache_key}.json

3. Validate cache freshness
   IF cache_file exists AND cache_age < 3600 seconds (1 hour):
       cache_hit = true
       RETURN cached_results (skip Phases 2-3)
   ELSE:
       cache_hit = false
       CONTINUE to Phase 2

4. Handle force refresh
   IF force_refresh flag:
       cache_hit = false
       Delete stale cache
       CONTINUE to Phase 2
```

**Cache Implementation:**
- Cache location: `devforgeai/cache/insights/`
- TTL: 1 hour (3600 seconds)
- Key format: `{query_type}_{filters_hash}_{date_range_hash}.json`
- Cache invalidation: On force_refresh flag or TTL expiry

### Phase 2: Subagent Orchestration

**Purpose:** Invoke session-miner subagent with query-specific prompts

**Base Invocation Pattern:**
```
Task(
  subagent_type="session-miner",
  description=QUERY_DESCRIPTIONS[query_type],
  prompt=QUERY_PROMPTS[query_type]
)
```

**Query Configuration Table:**

| Query Type | Description | Prompt Focus |
|------------|-------------|--------------|
| dashboard | Generate dashboard metrics from session data | Extract workflow counts, error rates, completion times |
| workflows | Analyze workflow patterns from sessions | Group by workflow type, calculate success rates |
| errors | Extract error patterns from sessions | Find error messages, categorize by type, rank by frequency |
| decisions | Surface development decisions from sessions | Extract ADR references, AskUserQuestion interactions |
| story | Deep analysis of story {story_id} | Filter by story ID, extract timeline, identify key events |
| command-patterns | Extract high-frequency command sequences | Route to session-miner n-gram analysis, top 10 by frequency |

**Common Parameters (all queries):**
- `filters`: Applied filter criteria
- `date_range`: Time bounds for query
- `story_id`: Required for story-specific queries only

**Output Requirements:**
Return JSON array of SessionEntry objects with: timestamp, entry_type, content, metadata

### Phase 3: Result Aggregation

**Purpose:** Aggregate, filter, and rank results by relevance

```
1. Parse SessionEntry objects from subagent response
   entries = parse_session_entries(raw_data)

2. Apply aggregation logic
   grouped = group_by(entries, aggregation_key)

3. Apply filtering criteria
   filtered = filter_by(grouped, filter_criteria)

4. Calculate metrics
   metrics = calculate_metrics(filtered)
   - count_by_type
   - success_rate
   - average_duration
   - error_frequency

5. Rank by relevance
   ranked = rank_by_relevance(filtered, ranking_criteria)
   - recency (newer = higher)
   - frequency (more common = higher)
   - severity (more severe = higher)

6. Transform for output
   transformed = transform_for_display(ranked)
```

**Aggregation Functions:**
- `group_by`: Groups entries by specified key (type, story_id, date)
- `filter_by`: Applies inclusion/exclusion criteria
- `calculate_metrics`: Computes summary statistics
- `rank_by_relevance`: Orders results by relevance score

**SessionEntry Handling:**
Each SessionEntry contains:
- `timestamp`: ISO 8601 datetime
- `entry_type`: tool_call, user_message, assistant_response, error
- `content`: Raw content text
- `metadata`: Additional context (story_id, phase, etc.)

### Phase 4: Output Formatting

**Purpose:** Format aggregated results as user-friendly markdown

```
1. Select output template based on query_type

2. Generate markdown structure
   - Summary section
   - Data table(s)
   - Recommendations

3. Format tables using markdown syntax

4. Add metadata footer
```

**Output Template:**

```markdown
## {Query Type} Insights

**Generated:** {timestamp}
**Query:** {query_description}
**Cache:** {hit|miss}

### Summary

{summary_text}

### Results

| Column 1 | Column 2 | Column 3 |
|----------|----------|----------|
| data     | data     | data     |

### Recommendations

1. {recommendation_1}
2. {recommendation_2}
3. {recommendation_3}

---
*Source: session-miner subagent | Cached for: {remaining_ttl}*
```

---

## Output Templates

All templates follow a consistent structure with three sections:
1. **Header** - Query type and metadata
2. **Results Table** - Query-specific data columns
3. **Recommendations** - Actionable insights

### Common Template Structure

```markdown
## {Query Title}

**Generated:** {timestamp} | **Period:** {date_range} | **Cache:** {hit|miss}

### {Results Section Name}

| {Column Headers} |
|------------------|
| {Data Rows}      |

### Recommendations

1. {recommendation_1}
2. {recommendation_2}
```

### Query-Specific Table Schemas

| Query Type | Results Section | Column Headers |
|------------|-----------------|----------------|
| dashboard | Key Metrics | Metric, Value, Trend |
| workflows | Workflow Distribution | Workflow Type, Count, Success Rate |
| errors | Top Errors by Frequency | Error Type, Count, Last Seen |
| decisions | Key Decisions | Date, Story, Decision, Rationale |
| story | Timeline | Phase, Started, Completed, Duration |
| command-patterns | Top Command Sequences | Rank, Sequence, Frequency, Success Rate |

### Template Examples

**Dashboard Example:**
```markdown
## Dashboard Overview
| Metric | Value | Trend |
|--------|-------|-------|
| Total Sessions | {count} | {trend_indicator} |
| Error Rate | {percentage}% | {trend_indicator} |
```

**Story Deep Dive Example:**
```markdown
## Story Deep Dive: {story_id}
| Phase | Started | Completed | Duration |
|-------|---------|-----------|----------|
| Development | {time} | {time} | {duration} |
```

---

## Caching Mechanism

### Cache Architecture

```
devforgeai/cache/insights/
  ├── dashboard_{hash}.json
  ├── workflows_{hash}.json
  ├── errors_{hash}.json
  ├── decisions_{hash}.json
  └── story_{story_id}_{hash}.json
```

### Cache Entry Format

```json
{
  "cache_key": "{query_type}_{filters_hash}",
  "created_at": "2025-01-04T10:00:00Z",
  "ttl_seconds": 3600,
  "expires_at": "2025-01-04T11:00:00Z",
  "query": {
    "type": "{query_type}",
    "filters": {},
    "date_range": {}
  },
  "results": {
    "summary": {},
    "data": [],
    "recommendations": []
  }
}
```

### TTL Management

| Behavior | Condition | Action |
|----------|-----------|--------|
| Cache Hit | Cache exists AND age < 3600s | Return cached results immediately |
| Cache Miss | Cache missing OR expired | Invoke session-miner, cache new results |
| Force Refresh | `force_refresh=true` flag | Delete cache, execute workflow, return fresh |
| Invalidation | TTL expires (1 hour) | Automatic deletion on next query |

**Default TTL:** 1 hour (3600 seconds)

**Behavior Notes:** On cache miss, the skill invokes session-miner to perform mining operations.

---

## Success Criteria

This skill succeeds when:

- [ ] session-miner subagent invoked via Task() pattern
- [ ] Results aggregated, filtered, and ranked
- [ ] Output formatted as markdown with tables
- [ ] Cache mechanism operational (1-hour TTL)
- [ ] All 6 query types supported (dashboard, workflows, errors, decisions, story, command-patterns)
- [ ] Cached queries return in <10 seconds
- [ ] SKILL.md under 1000 lines

---

## Error Handling

### Subagent Errors

```
IF session-miner returns error:
    Log error details
    Return error template with troubleshooting steps
    Do NOT cache error responses
```

### Cache Errors

```
IF cache read fails:
    Proceed without cache (cache_hit=false)
    Log warning for debugging
```

### Empty Results

```
IF no matching sessions found:
    Return "no results" template
    Suggest query modifications
```

---

## Integration with /insights Command

The `/insights` command invokes this skill with:

```
Skill(command="devforgeai-insights", args="{query_type} {options}")
```

**Argument Parsing:**
- `query_type`: dashboard|workflows|errors|decisions|story
- `--story-id={id}`: Required for story-specific queries
- `--force`: Force cache refresh
- `--days={n}`: Limit to last N days

---

## Change Log

| Date | Change | Reference |
|------|--------|-----------|
| 2025-01-04 | Initial implementation | STORY-225 |
