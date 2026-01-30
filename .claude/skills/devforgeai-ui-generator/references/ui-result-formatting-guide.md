# UI Result Formatting Guide

**Purpose:** Framework-aware guardrails for the `ui-spec-formatter` subagent

This reference document provides the subagent with DevForgeAI context to prevent autonomous decision-making and ensure UI specifications are validated and presented appropriately within the framework's governance model.

---

## DevForgeAI Context

### Story Workflow States

The UI generator operates on stories progressing through defined states:

```
Backlog → Architecture → Ready for Dev → In Development → Dev Complete →
QA In Progress → [QA Approved | QA Failed] → Releasing → Released
```

**Key State Transitions (where UI generator operates):**

- **Architecture → Ready for Dev (Optional):**
  - Invoked when story contains UI acceptance criteria
  - Generates visual specifications from requirements
  - Output becomes input for TDD implementation
  - Does NOT change story status
  - Quick feedback loop (2-5 minutes)

- **In Development → Dev Complete (During Implementation):**
  - Can be invoked during TDD cycles for reference
  - Helps developers understand specification requirements
  - Assists with test case creation and implementation

### Quality Gates

Three critical gates enforce progression:

**Gate 1: Context Validation** (Architecture → Ready for Dev)
- All 6 context files exist and are non-empty
- Enforced by: devforgeai-architecture skill

**Gate 2: Test Passing** (Dev Complete → QA In Progress)
- Build succeeds, all tests pass
- Enforced by: devforgeai-development skill (light QA)

**Gate 3: QA Approval** (QA In Progress → Releasing)
- UI implementation matches generated specification
- Accessibility features implemented correctly
- Responsive design working at all breakpoints

**Gate 4: Release Readiness** (Releasing → Released)
- Story approved and ready
- Enforced by: devforgeai-release skill

---

## Framework Constraints

### 1. Technology Stack Consistency (Strict, No Substitution)

All generated UI code **MUST** respect tech-stack.md:

```
Framework Layer:     Only use approved frameworks (React, Vue, Angular, etc.)
Styling Library:     Only use approved libraries (Tailwind, CSS Modules, etc.)
Component Library:   Only use approved UI component sets
Testing Framework:   Only use approved test runners and assertion libraries
State Management:    Only use approved state libraries
```

**Display guidance:**
- Always state which framework(s) will be used
- Alert if selection differs from tech-stack.md
- Ask for confirmation if change needed
- Never assume alternative frameworks are acceptable

**Validation rules:**
- Generate ONLY with frameworks in tech-stack.md
- Alert on warnings (e.g., "Tailwind not in dependencies.md yet")
- BLOCK generation if framework clearly violates tech-stack.md
- Ask user to verify before proceeding with mismatched framework

**Never say:** "Generated with Vue (even though React is specified)" or "Used CSS-in-JS (styling not defined)"
**Always enforce:** Use ONLY approved technologies; alert and ask before any deviation

### 2. File Structure Compliance (Source-Tree Mandatory)

Generated files **MUST** follow source-tree.md structure:

```
src/
├── components/           # UI components generated here
│   ├── forms/           # Form components
│   ├── displays/        # Data display components
│   ├── navigation/      # Navigation components
│   └── common/          # Reusable/shared components
├── styles/              # Styling files
│   ├── components/      # Component-specific styles
│   └── global/          # Global styles
└── tests/               # Test files
    └── components/      # Component tests
```

**Display guidance:**
- Always recommend file locations per source-tree.md
- Alert if generated files violate structure
- Suggest reorganization if structure mismatch
- Never place components outside recommended locations

**Validation rules:**
- Check EVERY generated file path against source-tree.md
- Alert if ANY file violates structure (MEDIUM severity)
- Mark as PARTIAL if structure issues exist
- Provide remediation (move files to correct locations)

**Never say:** "Components can go anywhere in src/" or "Styles don't need organization"
**Always enforce:** Structure follows source-tree.md; reorganize if needed

### 3. Accessibility Requirements (WCAG Mandatory)

All generated UI **MUST** meet accessibility standards:

```
Minimum: WCAG 2.1 Level A
Recommended: WCAG 2.1 Level AA (most projects)
Enhanced: WCAG 2.1 Level AAA (accessibility-critical projects)
```

**Requirements enforced:**

1. **Keyboard Navigation:** All interactive elements keyboard-accessible
2. **Screen Reader Support:** Proper semantic HTML and ARIA labels
3. **Color Contrast:** Text/background contrast ≥4.5:1 (AA) or ≥7:1 (AAA)
4. **Focus Management:** Clear focus indicators, logical tab order
5. **Alternative Text:** Images have alt text, icons have aria-labels
6. **Form Labels:** All form inputs have associated labels
7. **Error Messages:** Errors clearly communicated and linked to form fields

**Display guidance:**
- Always specify WCAG level achieved
- List accessibility features implemented
- Alert if accessibility features missing
- Recommend adding missing features before implementation

**Validation rules:**
- Spec generation MUST include accessibility checklist
- BLOCK generation if critical features missing (keyboard nav, ARIA)
- WARN if missing optional features (color contrast contrast tools)
- Provide specific accessibility test scenarios

**Never say:** "UI is accessible (without specifics)" or "Accessibility can be added later"
**Always enforce:** Accessibility built-in from specification; specific WCAG level verified

### 4. Component Categorization (Deterministic)

UI components always fit into consistent categories:

```
CATEGORY 1: Forms
  - Login forms, signup forms, data entry forms
  - Features: Input validation, error display, submission handling
  - Required tests: Valid/invalid inputs, edge cases, accessibility

CATEGORY 2: Data Display
  - Tables, dashboards, cards, lists
  - Features: Sorting, filtering, pagination, data transformation
  - Required tests: Different data volumes, empty states, error states

CATEGORY 3: Navigation
  - Menus, breadcrumbs, tabs, sidebars
  - Features: Active states, breadcrumb trails, responsive collapse
  - Required tests: Active state transitions, keyboard navigation

CATEGORY 4: Dialogs/Modals
  - Confirmation dialogs, alerts, modals
  - Features: Focus trap, escape key handling, backdrop
  - Required tests: Open/close scenarios, keyboard interaction

CATEGORY 5: Charts/Visualizations
  - Charts, graphs, visual data representations
  - Features: Legend, tooltips, responsive sizing
  - Required tests: Different data ranges, edge cases

CATEGORY 6: Common/Reusable
  - Buttons, inputs, dropdowns, badges
  - Features: Multiple states, sizes, variants
  - Required tests: All prop combinations, accessibility
```

**Display guidance:**
- Categorize each generated component correctly
- Show appropriate test scenarios for each category
- Recommend testing approach based on category
- List component interrelationships

### 5. Responsive Design Standards (Mobile-First)

Generated UI **MUST** support responsive design:

```
Mobile-First Approach:
  Mobile (< 640px)    - Default, optimized experience
  Tablet (640-1024px) - Enhanced layout, touch-optimized
  Desktop (> 1024px)  - Full feature set, optimized for mouse/keyboard

Breakpoints (standard):
  sm: 640px   (tablets)
  md: 768px   (tablets+)
  lg: 1024px  (desktops)
  xl: 1280px  (large desktops)
```

**Display guidance:**
- Always specify responsive breakpoints
- Show layout at each breakpoint
- Verify touch targets (min 48px for touch)
- Document responsive behavior

**Validation rules:**
- Generate specifications for all breakpoints
- Test component behavior at each size
- Ensure touch support on mobile
- Alert if responsive design incomplete
- Mark as PARTIAL if breakpoint coverage missing

**Never say:** "Responsive design added later" or "Only desktop required"
**Always enforce:** Mobile-first approach; all breakpoints specified

### 6. Component Dependencies (Minimal Coupling)

Generated components must minimize dependencies:

```
Ideal: Isolated components with clear props interface
Good: Components with <3 external dependencies
Acceptable: Components with <5 external dependencies
Problematic: Components with >5 dependencies (refactor needed)

External Dependencies:
  - Other components (acceptable: 1-3)
  - UI libraries (acceptable: 1-2)
  - Utilities/helpers (acceptable: any)
  - State management (acceptable: 1)
  - Styling (acceptable: 1)
```

**Display guidance:**
- Document component dependencies clearly
- List all required packages
- Show component composition hierarchy
- Alert if circular dependencies detected

**Validation rules:**
- Check for circular dependencies (CRITICAL if found)
- Alert if too many external dependencies (MEDIUM if >5)
- Recommend refactoring if coupling too tight
- Show component tree/hierarchy

**Never say:** "Component depends on everything" or "Dependencies can be added"
**Always enforce:** Clear, minimal dependency specification

### 7. Testing Strategy (Test Pyramid)

Generated specifications must include testing guidance:

```
Test Pyramid:
  70% Unit Tests       - Component in isolation
  20% Integration Tests - Components together, with services
  10% E2E Tests       - Full user workflows

Minimum Coverage:
  All components: 80% line coverage
  Forms: 90% coverage (critical for user input)
  Data displays: 85% coverage
  Navigation: 85% coverage

Test Scenarios per Component:
  Happy path (1-2 scenarios)
  Error cases (2-3 scenarios)
  Edge cases (1-2 scenarios)
  Accessibility (1-2 scenarios)
```

**Display guidance:**
- Show estimated test count per component
- Recommend testing approach by component type
- Provide test scenario templates
- Alert if testing coverage seems insufficient

**Validation rules:**
- Generate test scenario count for each component
- Estimate total test count
- Recommend test coverage targets
- Reference testing framework from tech-stack.md

**Never say:** "Tested during implementation" or "Testing is optional"
**Always enforce:** Testing guidance integral to specification

### 8. Generated Specification Quality (Completeness Checklist)

All specifications must include:

```
✅ Component Name and Type
✅ Framework and Styling Library
✅ File Locations (per source-tree.md)
✅ Accessibility Level (WCAG)
✅ Responsive Breakpoints
✅ Component Props/Inputs
✅ State Management Approach
✅ Key Features and Behaviors
✅ Test Scenarios (count and types)
✅ Dependencies (packages and internal)
✅ Implementation Order (if multiple components)
✅ Estimated Development Time
```

**Display guidance:**
- Verify all sections present before displaying
- Alert if any section missing
- Mark as PARTIAL if critical sections incomplete
- Mark as FAILED if specification fundamentally broken

**Validation rules:**
- Checklist validation before marking SUCCESS
- All 12 items required for SUCCESS status
- 9-11 items = PARTIAL
- <9 items = FAILED
- Provide list of missing items for PARTIAL/FAILED

---

## Display Template Guidelines

### Structure for All Templates

1. **Header** (2 lines)
   - Status emoji + Title: "✅ UI Component Specification Generated"
   - Story ID/Component name and title

2. **Summary** (3-5 lines)
   - Generation status
   - Component count
   - Framework and styling
   - Accessibility level

3. **Component Details** (varies by count)
   - Component type, purpose, features
   - Framework-specific details
   - Accessibility features
   - Responsive design support

4. **File Summary** (3-5 lines)
   - File count by type
   - Total lines of code
   - File locations (per source-tree.md)

5. **Implementation Guidance** (5-10 lines)
   - Recommended implementation order
   - Estimated time per component
   - Test scenarios needed
   - Dependencies to install

6. **Validation Status** (if issues)
   - Framework consistency
   - File structure compliance
   - Accessibility checklist status
   - Tech-stack alignment

7. **Next Steps** (3-5 lines)
   - Specific action for user
   - Command to run if applicable
   - Link to generated specification

### Emoji Usage

- ✅ Success, complete, compliant
- ⚠️ Warning, partial, needs review
- ❌ Failed, blocked, critical issue
- ℹ️ Information, context, helpful note
- → Arrow for actions/flow

### Tone Guidance

- **Success:** Enthusiastic, ready-to-go ("Complete and ready for implementation!")
- **Partial:** Constructive, actionable ("Almost ready; address warnings first")
- **Failed:** Clear, diagnostic ("Generation encountered issues; see below")

### Length Guidelines

- **Story mode success:** 50-70 lines
- **Standalone success:** 35-50 lines
- **Partial (warnings):** 40-60 lines (explanations)
- **Failed:** 30-50 lines (diagnostic info)

---

## Framework Integration Points

### 1. Context Files References

When validating or providing guidance, reference context files:

**tech-stack.md:**
- "Your approved framework is React (tech-stack.md)"
- "Styling library Tailwind is in your tech-stack.md"
- Alert: "{Framework} not in tech-stack.md; confirm selection?"

**source-tree.md:**
- "Component files should follow source-tree.md structure"
- "Recommend placing forms in src/components/forms/ (per source-tree.md)"
- Alert: "Generated files violate source-tree.md structure; reorganize?"

**dependencies.md:**
- "Required package: react-hook-form (verify it's in dependencies.md)"
- Alert: "Package {X} not in dependencies.md; add before using?"

**coding-standards.md:**
- "Component names follow PascalCase (per coding-standards.md)"
- "Max 500 lines per component file (coding-standards.md)"

**architecture-constraints.md:**
- "UI components isolated in Presentation layer (architecture-constraints.md)"
- "Components import only from Application layer (architecture-constraints.md)"

**anti-patterns.md:**
- Alert: "Avoid God Objects (>500 lines) per anti-patterns.md"
- Alert: "Don't use direct instantiation (anti-patterns.md); use props instead"

### 2. Related Skills and Subagents

Reference when recommending next actions:

**For implementation:**
- "Begin implementation: `/dev STORY-XXX` (devforgeai-development skill will guide TDD)"

**For testing:**
- "Test scenarios (see above) can be written with test-automator subagent help"

**For quality validation:**
- "After implementation, run `/qa STORY-XXX` (QA will validate spec compliance)"

**For accessibility review:**
- "Security-auditor subagent can validate accessibility implementation"

### 3. Story Workflow Integration

Show understanding of where UI spec fits in workflow:

**Story Status Context:**
- "Story status: Ready for Dev (UI spec generation precedes TDD implementation)"
- "After spec generation, proceed to: `/dev STORY-XXX` for TDD"
- "When dev complete, run: `/qa STORY-XXX` to validate spec compliance"

**Next Step Recommendations:**
- Story mode: "Review spec, then implement: `/dev STORY-XXX`"
- Standalone: "Copy component to project, then integrate"

---

## Validation Rules and Severity Levels

### CRITICAL (Blocks Usage) - Status = FAILED

- Framework selection violates tech-stack.md AND user confirms it's wrong
- File structure fundamentally breaks source-tree.md requirements
- Accessibility missing critical features (keyboard nav, ARIA labels)
- Specification fundamentally broken or unparseable
- Component structure violates architecture-constraints.md layer boundaries

**Action:** Mark as FAILED, provide specific blockers and recovery steps

### HIGH (Needs Resolution) - Status = PARTIAL

- Non-critical accessibility features missing (color contrast, focus indicators)
- File structure deviations (minor reorganization needed)
- Framework/styling selection not in tech-stack.md (warning, not blocker)
- Too many component dependencies (>5, refactoring recommended)
- Missing test scenarios or coverage estimates

**Action:** Mark as PARTIAL, alert to warnings, provide remediation steps

### MEDIUM (Nice to Have) - Status = SUCCESS with notes

- Minor code structure inconsistencies
- Additional accessibility enhancements possible
- Optimization opportunities
- Documentation improvements

**Action:** Mark as SUCCESS, note opportunities for enhancement

### LOW (Informational) - Status = SUCCESS

- Styling preferences
- Component naming suggestions
- Optional features

**Action:** Mark as SUCCESS, include as FYI

---

## Error Scenarios and Handling

### Scenario 1: Framework Not in tech-stack.md

```
Detection: Generated framework {X} not found in tech-stack.md

Display:
"⚠️ Framework Consistency Check

Generated with: {Framework}
Tech-stack.md specifies: {Framework list}

Issue: Framework selection may not match project standards.

Action Required:
1. Verify your tech-stack.md includes {Framework}
2. If incorrect in spec: Ask me to regenerate with correct framework
3. If tech-stack.md needs update: Create ADR and update context files
4. If verified correct: Proceed with implementation

Status: PARTIAL (warning - not blocker)"
```

### Scenario 2: File Structure Violation

```
Detection: Generated files violate source-tree.md structure

Display:
"⚠️ File Structure Advisory

Generated files:
- Component: src/ui/LoginForm.jsx (violates source-tree.md)

Expected location (source-tree.md):
- Component: src/components/forms/LoginForm.jsx

Recommended Actions:
1. Move component files to correct location per source-tree.md
2. Update import statements if reorganizing
3. Verify imports point to correct paths after moving
4. Then proceed with implementation

Status: PARTIAL (needs organization)"
```

### Scenario 3: Accessibility Features Missing

```
Detection: Key accessibility features not in specification

Display:
"⚠️ Accessibility Check

Specification includes:
✓ Semantic HTML structure
✓ ARIA labels for inputs
✗ Keyboard navigation (missing)
✗ Focus indicators (missing)

WCAG Level: 2.1 A (missing features for AA)

Recommendation:
1. Request specification include keyboard navigation
2. Add focus indicator styling
3. Target WCAG 2.1 AA (minimum recommended)
4. Include accessibility test scenarios

Current Status: PARTIAL (accessibility incomplete)
Estimated impact: +1-2 hours development"
```

### Scenario 4: Spec File Missing

```
Status: ERROR
Issue: Specification file not found at expected path

Display:
"❌ Specification File Not Found

Expected: devforgeai/specs/ui/STORY-XXX-ui-spec.md
Status: Not found

This may indicate:
1. Skill generation was interrupted
2. File directory wasn't created
3. File system permissions issue

Recovery:
1. Regenerate specification: `/create-ui STORY-XXX`
2. Check directory exists: mkdir -p devforgeai/specs/ui/
3. Review skill output above for errors

Try regenerating the UI specification."
```

---

## Testing Checklist for Subagent

Use this when testing ui-spec-formatter:

- [ ] Parse story mode success spec (React)
- [ ] Parse story mode success spec (Vue)
- [ ] Parse story mode success spec (Blazor)
- [ ] Parse standalone mode success spec
- [ ] Parse spec with framework consistency warning
- [ ] Parse spec with file structure violation
- [ ] Parse spec with accessibility features missing
- [ ] Parse spec with too many dependencies
- [ ] Parse spec with correct WCAG level
- [ ] Parse spec with test scenarios included
- [ ] Generate success template (story mode)
- [ ] Generate success template (standalone mode)
- [ ] Generate partial template (warnings)
- [ ] Generate failed template (critical issues)
- [ ] Recommend correct next steps for each result type
- [ ] Validate framework consistency with tech-stack.md
- [ ] Validate file structure with source-tree.md
- [ ] Validate accessibility completeness (WCAG level)
- [ ] Handle missing/malformed spec gracefully
- [ ] Extract component details correctly
- [ ] Generate accurate component categorization
- [ ] Create appropriate implementation guidance
- [ ] Reference context files appropriately
- [ ] Show component dependencies clearly

---

## Framework Compliance Checklist

Before marking specification as SUCCESS:

### Technology Stack
- [ ] Framework is in tech-stack.md
- [ ] Styling library is in tech-stack.md (or warned)
- [ ] Testing framework is in tech-stack.md
- [ ] State management library is in tech-stack.md (if used)
- [ ] All external dependencies are in dependencies.md (or warned)

### File Structure
- [ ] Components placed in src/components/ (or per source-tree.md)
- [ ] No files in root directory (violates source-tree.md)
- [ ] Styles organized per source-tree.md
- [ ] Tests in dedicated tests/ directory
- [ ] Spec documentation in devforgeai/specs/ui/

### Accessibility
- [ ] WCAG level specified (A, AA, or AAA)
- [ ] Keyboard navigation confirmed
- [ ] Screen reader support (ARIA labels)
- [ ] Color contrast considerations noted
- [ ] Focus management specified
- [ ] Test scenarios include accessibility

### Architecture
- [ ] Components in Presentation layer (per architecture-constraints.md)
- [ ] No cross-layer imports from components
- [ ] Dependency injection for services (no direct instantiation)
- [ ] No forbidden patterns (per anti-patterns.md)

### Quality
- [ ] Component dependencies minimal (<5)
- [ ] Test scenarios count reasonable
- [ ] Implementation time estimate provided
- [ ] File count and structure complete
- [ ] All required sections present (12-item checklist)

---

## Remember

This reference file is **NOT generating specifications** — the skill does that.

This reference file is **validating and presenting** the results within framework constraints.

Your job (ui-spec-formatter subagent):
1. Read the generated specification ✅
2. Understand DevForgeAI context (story states, gates, constraints) ✅
3. Validate against context files (tech-stack, source-tree, dependencies) ✅
4. Display results appropriately for the situation ✅
5. Recommend clear next steps ✅

**DO:**
- Reference context files when validating
- Alert to framework violations (stop and ask)
- Check accessibility completeness
- Verify file structure compliance
- Provide specific, actionable guidance
- Show understanding of where spec fits in story workflow

**DON'T:**
- Re-generate specification (skill already did that)
- Make decisions about tech-stack changes (ask user)
- Downplay accessibility requirements (WCAG is mandatory)
- Assume frameworks outside tech-stack.md are acceptable (validate)
- Place files outside source-tree.md without warning (alert)
- Create silos (understand DevForgeAI context)
