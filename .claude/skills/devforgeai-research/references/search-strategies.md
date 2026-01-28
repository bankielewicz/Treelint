# Search Strategies by Research Category

> **Note (2026-01-19):** This file is now a FALLBACK reference. Primary research execution
> is delegated to the internet-sleuth subagent which has its own methodology.
>
> This file is used when:
> - internet-sleuth is unavailable
> - Sleuth returns an error
> - Simple searches that don't need repository archaeology
>
> See: `.claude/agents/internet-sleuth.md` for primary research methodology

---

## Legacy Search Strategies (Fallback)

Optimized search approaches for each research category.

---

## Competitive Analysis

### Goal
Understand competitor features, pricing, positioning, strengths, and weaknesses.

### Search Sequence

1. **Product Overview**
   ```
   WebSearch: "{competitor} features pricing 2026"
   WebSearch: "{competitor} getting started tutorial"
   ```

2. **Direct Comparison**
   ```
   WebSearch: "{competitor} vs {our_product} comparison"
   WebSearch: "{competitor} alternative to {our_product}"
   ```

3. **User Sentiment**
   ```
   WebSearch: "{competitor} reviews reddit"
   WebSearch: "{competitor} developer experience 2026"
   WebSearch: "{competitor} frustrations issues 2026"
   ```

4. **Official Sources**
   ```
   WebFetch: {competitor_homepage}
     Prompt: "Extract key features, pricing tiers, target audience"

   WebFetch: {competitor_docs}
     Prompt: "Assess documentation quality, getting started experience"
   ```

5. **Market Position**
   ```
   WebSearch: "{competitor} market share adoption 2026"
   WebSearch: "{competitor} funding valuation news"
   ```

### Key Questions to Answer
- What are their core features?
- How do they price their product?
- Who is their target audience?
- What do users love/hate?
- What are we missing that they have?
- What do we have that they don't?

---

## Technology Evaluation

### Goal
Assess a library/framework/tool for potential adoption.

### Search Sequence

1. **Repository Health**
   ```
   WebFetch: https://github.com/{org}/{repo}
     Prompt: "Extract: stars, forks, last commit, open issues, contributors, README quality"
   ```

2. **Official Documentation**
   ```
   WebFetch: {official_docs_url}
     Prompt: "Extract: API design, getting started difficulty, examples quality, edge cases coverage"
   ```

3. **Performance**
   ```
   WebSearch: "{technology} performance benchmarks 2026"
   WebSearch: "{technology} vs {alternative} speed comparison"
   ```

4. **Adoption**
   ```
   WebSearch: "{technology} npm weekly downloads"
   WebSearch: "{technology} production usage examples 2026"
   WebSearch: "{technology} companies using"
   ```

5. **Comparison**
   ```
   WebSearch: "{technology} vs {alt1} vs {alt2} comparison 2026"
   WebFetch: {comparison_article}
     Prompt: "Extract pros/cons for each option"
   ```

6. **Issues/Limitations**
   ```
   WebSearch: "{technology} known issues limitations"
   WebSearch: "{technology} breaking changes migration"
   ```

### Key Questions to Answer
- Is it actively maintained?
- How mature is it?
- What's the performance profile?
- Who else is using it?
- What are the limitations?
- How does it compare to alternatives?
- What's the migration path if we adopt?

---

## Market Research

### Goal
Understand industry trends, developer needs, and market dynamics.

### Search Sequence

1. **Statistics**
   ```
   WebSearch: "{market_topic} statistics 2026"
   WebSearch: "{market_topic} market size growth 2026"
   ```

2. **Developer Surveys**
   ```
   WebSearch: "developer survey {topic} 2025 2026"
   WebSearch: "Stack Overflow survey {topic} 2026"
   WebSearch: "JetBrains developer survey {topic} 2026"
   ```

3. **Trends**
   ```
   WebSearch: "{topic} trends 2026 predictions"
   WebSearch: "{topic} adoption curve 2026"
   ```

4. **Pain Points**
   ```
   WebSearch: "developer frustrations {topic} 2026"
   WebSearch: "{topic} problems issues challenges 2026"
   WebSearch: "{topic} reddit complaints 2026"
   ```

5. **Industry Reports**
   ```
   WebFetch: {gartner_or_forrester_report}
     Prompt: "Extract key findings, market leaders, emerging trends"
   ```

### Key Questions to Answer
- What is the market size?
- What are the growth trends?
- What do developers want?
- What are the current pain points?
- Who are the market leaders?
- What are emerging trends?

---

## Integration Planning

### Goal
Understand how to integrate an external service/API.

### Search Sequence

1. **Internal Analysis**
   ```
   Grep: "{service_keywords}"
     Path: project_root
     Find: Existing integration attempts
   ```

2. **Official Integration Docs**
   ```
   WebFetch: {service_integration_docs}
     Prompt: "Extract: auth methods, API endpoints, rate limits, error handling, SDKs"
   ```

3. **Community Integrations**
   ```
   WebSearch: "{service} integration example github 2026"
   WebSearch: "{service} {our_language} SDK example"
   ```

4. **Known Issues**
   ```
   WebSearch: "{service} integration problems 2026"
   WebSearch: "{service} API limitations gotchas"
   ```

5. **Cost Analysis**
   ```
   WebFetch: {service_pricing_page}
     Prompt: "Extract: pricing tiers, API call costs, free tier limits"
   ```

### Key Questions to Answer
- What auth method does it use?
- What are the API rate limits?
- Are there official SDKs for our stack?
- What are common integration pitfalls?
- What's the cost at our scale?
- How reliable is the service?

---

## Architecture Research

### Goal
Understand a design pattern, architectural style, or best practice.

### Search Sequence

1. **Pattern Definition**
   ```
   WebSearch: "{pattern} design pattern 2026"
   WebSearch: "{pattern} architecture explained"
   ```

2. **Authoritative Sources**
   ```
   WebFetch: {martin_fowler_or_similar}
     Prompt: "Extract: when to use, when not to use, trade-offs, implementation guidance"
   ```

3. **Real-World Examples**
   ```
   WebSearch: "{pattern} implementation example github"
   WebSearch: "{pattern} {our_language} example 2026"
   ```

4. **Trade-offs**
   ```
   WebSearch: "{pattern} advantages disadvantages"
   WebSearch: "{pattern} vs {alternative_pattern} comparison"
   ```

5. **Common Mistakes**
   ```
   WebSearch: "{pattern} anti-patterns mistakes"
   WebSearch: "{pattern} common pitfalls"
   ```

### Key Questions to Answer
- What problem does this pattern solve?
- When should it be used?
- When should it NOT be used?
- What are the trade-offs?
- How is it implemented?
- What are common mistakes?

---

## Search Optimization Tips

### Tip 1: Use Year Filters
**Why:** Get recent, relevant results

**Examples:**
- Bad: "AI coding assistant comparison"
- Good: "AI coding assistant comparison 2026"
- Good: "AI coding trends 2025 2026"

### Tip 2: Use Specific Sites
**Why:** Target high-quality sources

**Examples:**
```
WebSearch: "microservices site:martinfowler.com"
WebSearch: "React hooks site:github.com"
WebSearch: "developer survey site:stackoverflow.com"
```

### Tip 3: Use Quotes for Exact Phrases
**Why:** Get precise matches

**Examples:**
- Bad: AI hallucination problem
- Good: "AI hallucination" problem solutions

### Tip 4: Combine Keywords
**Why:** Narrow results

**Examples:**
```
"AWS Kiro" + "spec-driven" + features
"{technology}" + benchmarks + performance + "{our_language}"
```

### Tip 5: Check Multiple Sources
**Why:** Validate findings

**Pattern:**
1. Find claim in Source A
2. Verify in Source B (independent)
3. Check primary source if possible

---

**Use category-specific strategies for best results.**
