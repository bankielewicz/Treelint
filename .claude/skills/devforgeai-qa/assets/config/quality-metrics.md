# Code Quality Thresholds

## Cyclomatic Complexity

### Thresholds
- **Method:** Maximum 10
- **Class:** Maximum 50

### Severity
- Exceeds threshold: MEDIUM
- Exceeds threshold by 50%: HIGH

### Calculation
Complexity = 1 + number of decision points (if, while, for, case, catch, &&, ||, ?:)

### Tools
- .NET: `dotnet-metrics`, Visual Studio Code Metrics
- Python: `radon cc src/ -a`
- JavaScript: `complexity-report src/`
- Multi-language: `lizard src/`

## Maintainability Index

### Scale
0-100 (higher is better)

### Thresholds
- **Excellent:** 80-100
- **Good:** 70-79 (minimum acceptable)
- **Fair:** 60-69 (warning)
- **Poor:** < 60 (fail)

### Severity
- Below 70: MEDIUM
- Below 50: HIGH

### Formula
171 - 5.2 * ln(Halstead Volume) - 0.23 * (Cyclomatic Complexity) - 16.2 * ln(Lines of Code)

### Factors
- Cyclomatic complexity
- Lines of code
- Halstead volume (program vocabulary and length)
- Comment ratio

### Tools
- Python: `radon mi src/ -j`
- Multi-language: `lizard src/ -l [language]`

## Code Duplication

### Threshold
- **Maximum:** 5% duplication

### Detection
- Minimum duplicate block: 6 lines
- Token-based comparison (ignores whitespace, comments)

### Severity
- 5-10% duplication: LOW
- 10-15% duplication: MEDIUM
- > 15% duplication: HIGH

### Tools
- Multi-language: `jscpd src/ --format json --min-lines 6`
- Java: PMD Copy/Paste Detector
- .NET: Resharper, SonarQube

## Documentation Coverage

### Thresholds
- **Public APIs:** 80% minimum
- **Internal classes:** 60% minimum
- **Private methods:** Optional

### Severity
- Below threshold: LOW

### Types
- C#: XML documentation (`///`)
- Python: Docstrings (`"""`)
- JavaScript/TypeScript: JSDoc (`/** */`)
- Java: Javadoc (`/** */`)

### Requirements
- Method summary
- Parameter descriptions
- Return value description
- Exception documentation

## Method/Class Size

### Thresholds
- **Method:** Maximum 100 lines
- **Class:** Maximum 500 lines

### Severity
- Exceeds threshold: MEDIUM
- Exceeds threshold by 100%: HIGH

### Exceptions
- Generated code
- Data classes/DTOs
- Test classes (may be larger)

## Dependency Metrics

### Coupling

**Thresholds:**
- **Maximum dependencies per class:** 10
- **Afferent coupling (incoming):** Monitor, no hard limit
- **Efferent coupling (outgoing):** < 10

**Severity:**
- > 10 dependencies: MEDIUM
- > 20 dependencies: HIGH

### Circular Dependencies

**Threshold:** ZERO (forbidden)

**Severity:** HIGH

**Detection:**
- JavaScript/TypeScript: `madge --circular src/`
- .NET: Dependency graphs, NDepend
- Python: `pydeps src/`

## Lines of Code (LOC)

### Thresholds (per file)
- **Small:** < 100 lines (ideal)
- **Medium:** 100-300 lines (acceptable)
- **Large:** 300-500 lines (warning)
- **Very Large:** > 500 lines (refactor recommended)

### Considerations
- Exclude comments and blank lines
- Count logical lines (statements)
- Consider complexity, not just size

## Cohesion Metrics

### Lack of Cohesion of Methods (LCOM)

**Threshold:** LCOM < 0.8 (higher cohesion is better)

**Interpretation:**
- LCOM = 0: Perfect cohesion (all methods use all fields)
- LCOM = 1: No cohesion (methods share no fields)

**Severity:**
- LCOM > 0.8: MEDIUM (low cohesion, consider splitting class)

## Enforcement Levels

### CRITICAL (Blocks Release)
- None (quality metrics are advisory)

### HIGH (Should Fix)
- Maintainability index < 50
- Circular dependencies
- Duplication > 15%
- Coupling > 20 dependencies

### MEDIUM (Technical Debt)
- Complexity exceeds thresholds
- Maintainability 50-70
- Duplication 10-15%
- Class/method size violations
- Coupling 10-20 dependencies

### LOW (Nice to Have)
- Documentation below 80%
- Duplication 5-10%
- Maintainability 70-80

## Quality Score Calculation

**Formula:**
```
Quality Score = (
    (100 - avg_complexity_violation%) * 0.25 +
    (maintainability_index) * 0.25 +
    (100 - duplication%) * 0.20 +
    (documentation_coverage%) * 0.15 +
    (100 - coupling_violation%) * 0.15
)
```

**Thresholds:**
- **Excellent:** 90-100
- **Good:** 80-89
- **Fair:** 70-79
- **Poor:** < 70

## References

- [Cyclomatic Complexity](https://en.wikipedia.org/wiki/Cyclomatic_complexity)
- [Maintainability Index](https://docs.microsoft.com/en-us/visualstudio/code-quality/code-metrics-values)
- [Code Metrics Best Practices](https://martinfowler.com/articles/useOfMetrics.html)
