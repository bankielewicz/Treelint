# Code Quality Metrics Reference

## Cyclomatic Complexity

**Formula:** Complexity = 1 + number of decision points

**Decision points:** if, while, for, case, catch, &&, ||, ?:

**Thresholds:**
- Method: Maximum 10
- Class: Maximum 50

**Tools:**
- .NET: `dotnet-metrics`
- Python: `radon cc src/`
- JavaScript: `complexity-report src/`

## Maintainability Index

**Formula:** 171 - 5.2 * ln(HalsteadVolume) - 0.23 * (CyclomaticComplexity) - 16.2 * ln(LinesOfCode)

**Scale:** 0-100 (higher is better)

**Thresholds:**
- Excellent: 80-100
- Good: 70-79
- Fair: 60-69
- Poor: < 60

**Tools:**
- Python: `radon mi src/`
- Multi-language: `lizard src/`

## Code Duplication

**Detection:** Find identical code blocks (minimum 6 lines)

**Threshold:** Maximum 5% duplication

**Tools:**
- `jscpd src/` (works for many languages)
- PMD Copy/Paste Detector (Java)

## Documentation Coverage

**Thresholds:**
- Public APIs: 80%
- Internal classes: 60%

**Types:**
- C#: XML documentation (`///`)
- Python: Docstrings (`"""`)
- JavaScript: JSDoc (`/** */`)

## Dependency Analysis

**Metrics:**
- Afferent coupling (incoming dependencies)
- Efferent coupling (outgoing dependencies)
- Maximum dependencies per class: 10
- Circular dependencies: FORBIDDEN

**Tools:**
- JavaScript/TypeScript: `madge --circular src/`
- .NET: NDepend, dependency graphs

## Quick Reference

| Metric | Threshold | Severity | Tool |
|--------|-----------|----------|------|
| Method Complexity | Max 10 | MEDIUM | radon, lizard |
| Class Complexity | Max 50 | MEDIUM | radon, lizard |
| Maintainability | Min 70 | MEDIUM | radon mi |
| Duplication | Max 5% | LOW | jscpd |
| Documentation | Min 80% | LOW | Custom grep |
| Coupling | Max 10 deps | MEDIUM | madge, ndepend |
