# Citation Standards for Research

How to properly cite sources in DevForgeAI research documents.

---

## Citation Formats

### Web Articles
```markdown
- [Article Title](https://example.com/article) - Brief 1-sentence description
```

**Example:**
```markdown
- [AWS Kiro: Agentic IDE for Spec-Driven Development](https://www.infoq.com/news/2025/08/aws-kiro-spec-driven-agent/) - Kiro announcement with spec-driven features
```

### Documentation
```markdown
- [{Product} Documentation]({URL}) - Section: {specific section}
```

**Example:**
```markdown
- [Treelint Documentation](https://github.com/user/treelint#api) - Section: API Reference
```

### Research Papers
```markdown
- {Authors}. "{Paper Title}." {Venue}, {Year}. [{URL}]({URL})
```

**Example:**
```markdown
- Smith et al. "Memory in the Age of AI Agents." arXiv, 2026. [https://arxiv.org/abs/2512.13564](https://arxiv.org/abs/2512.13564)
```

### Statistics/Surveys
```markdown
- [{Organization} {Survey Name}]({URL}) - {Statistic description}
```

**Example:**
```markdown
- [Stack Overflow Developer Survey 2025](https://stackoverflow.com/survey/2025) - 29% trust AI-generated code accuracy
```

### GitHub Repositories
```markdown
- [{Repo Name}]({GitHub URL}) - {Brief description} (stars, last updated: {date})
```

**Example:**
```markdown
- [DevForgeAI](https://github.com/user/devforgeai) - Spec-driven AI framework (1.2k stars, last updated: 2026-01-15)
```

---

## Evidence Quoting

**Format:**
```markdown
**Evidence:**
- [{Source}]({URL}) - "{Exact quote or specific data point}"
- [{Source}]({URL}) - "{Exact quote or specific data point}"
```

**Rules:**
- Use exact quotes (in double quotes)
- Include specific data points (percentages, numbers)
- Always link to source URL
- Prefer primary sources over secondary

**Example:**
```markdown
**Evidence:**
- [IEEE Spectrum: AI Coding Degrades](https://spectrum.ieee.org/ai-coding-degrades) - "Only 29% of developers reported trusting the accuracy of AI-generated code in 2025, a sharp decline from 40% in prior years"
- [Cerbos: Productivity Paradox](https://www.cerbos.dev/blog/productivity-paradox-of-ai-coding-assistants) - "Developers using AI were on average 19% slower" (METR study, July 2025)
```

---

## Source Metadata

**Track for each source:**
- Title
- URL
- Access date
- Author/Organization (if available)
- Publication date (if available)
- Key excerpt

**In frontmatter:**
```yaml
sources_count: 15  # Total number of sources cited
```

---

## Avoiding Citation Issues

### Issue 1: Link Rot
**Problem:** URLs become invalid over time

**Solution:**
- Prefer stable domains (.gov, .edu, major publications)
- Include article title for searchability
- For critical sources, save to assets folder

### Issue 2: Paywall Content
**Problem:** Some sources require payment

**Solution:**
- Cite abstract/summary if available
- Note "(Paywall)" after link
- Include key finding in description

**Example:**
```markdown
- [Research Paper Title](https://journal.com/paper) (Paywall) - Abstract states: "Finding X was observed"
```

### Issue 3: Outdated Information
**Problem:** Research becomes stale

**Solution:**
- Include publication year in citation
- Flag research for review after 6 months
- Update sources when reviewed

---

## Confidence Levels

**Based on source quality:**

| Level | Criteria | Example Sources |
|-------|----------|-----------------|
| **High** | Primary sources, peer-reviewed, official docs | GitHub repos, official documentation, academic papers |
| **Medium** | Reputable secondary sources, industry reports | InfoQ, IEEE Spectrum, Stack Overflow surveys |
| **Low** | Opinion pieces, unverified claims, single-source | Blog posts, forum comments, social media |

**In findings:**
```markdown
**Confidence Level:** High

**Justification:** Based on official AWS announcement (primary source) and hands-on review from DevClass (secondary source). Multiple independent sources confirm the same findings.
```

---

**Use these standards consistently across all research documents.**
