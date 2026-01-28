# Design System

## Overview

This file documents UI/UX design system constraints for projects requiring consistent visual design patterns.

**Status:** Optional - Only required for projects with frontend components

## When to Create This File

Create this file during `/create-context` workflow when:
- Project includes web UI, mobile app, or desktop GUI
- Team requires consistent design language across components
- Brand guidelines mandate specific visual standards

## Structure

### Color Palette

```yaml
primary:
  main: "#1976d2"
  light: "#42a5f5"
  dark: "#1565c0"

secondary:
  main: "#dc004e"
  light: "#e33371"
  dark: "#9a0036"

neutral:
  gray-50: "#fafafa"
  gray-100: "#f5f5f5"
  # ... additional neutral tones
```

### Typography

```yaml
font_families:
  heading: "Inter, system-ui, sans-serif"
  body: "Inter, system-ui, sans-serif"
  monospace: "JetBrains Mono, monospace"

font_sizes:
  xs: "0.75rem"   # 12px
  sm: "0.875rem"  # 14px
  base: "1rem"    # 16px
  lg: "1.125rem"  # 18px
  xl: "1.25rem"   # 20px
  # ... additional sizes
```

### Spacing Scale

```yaml
spacing:
  xs: "0.25rem"  # 4px
  sm: "0.5rem"   # 8px
  md: "1rem"     # 16px
  lg: "1.5rem"   # 24px
  xl: "2rem"     # 32px
  # ... additional spacing units
```

### Component Patterns

Document reusable component specifications:
- Button variants (primary, secondary, ghost)
- Form input standards
- Card layouts
- Navigation patterns
- Modal/dialog behaviors

## Framework Integration

Specify how design system integrates with tech stack:

**Example (React + Tailwind):**
```typescript
// tailwind.config.js references this file
// All components use design tokens from theme
```

**Example (Angular Material):**
```typescript
// Custom Material theme configuration
// Following Angular Material theming guide
```

## Accessibility Requirements

- WCAG 2.1 Level AA compliance minimum
- Color contrast ratios: 4.5:1 for text, 3:1 for UI components
- Keyboard navigation support for all interactive elements
- Screen reader compatibility

## References

- Brand guidelines: [Link or file path]
- UI component library: [Storybook, Figma, etc.]
- Design tokens repository: [Link or file path]

---

**Note:** This is a template. Customize based on project requirements during context file creation.
