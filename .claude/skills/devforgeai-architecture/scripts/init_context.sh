#!/bin/bash
# Initialize DevForgeAI context files for a new project
#
# This script creates all 6 critical context files that prevent technical debt:
#   1. tech-stack.md - Locks technology choices
#   2. source-tree.md - Enforces project structure
#   3. dependencies.md - Locks approved packages
#   4. coding-standards.md - Project-specific code patterns
#   5. architecture-constraints.md - Layer boundaries and design rules
#   6. anti-patterns.md - Explicitly forbidden patterns

set -e

PROJECT_ROOT="${1:-.}"
CONTEXT_DIR="${PROJECT_ROOT}/devforgeai/context"
SKILL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "🏗️  Initializing DevForgeAI context files..."
echo ""

# Create context directory
mkdir -p "${CONTEXT_DIR}"

# Create docs directory for ADRs
mkdir -p "${PROJECT_ROOT}/docs/architecture/decisions"

# Copy all 6 context file templates
echo "📄 Copying context file templates..."

cp "${SKILL_DIR}/assets/context-templates/tech-stack.md" "${CONTEXT_DIR}/" && \
  echo "  ✓ tech-stack.md (locks technology choices)"

cp "${SKILL_DIR}/assets/context-templates/source-tree.md" "${CONTEXT_DIR}/" && \
  echo "  ✓ source-tree.md (enforces project structure)"

cp "${SKILL_DIR}/assets/context-templates/dependencies.md" "${CONTEXT_DIR}/" && \
  echo "  ✓ dependencies.md (locks approved packages)"

cp "${SKILL_DIR}/assets/context-templates/coding-standards.md" "${CONTEXT_DIR}/" && \
  echo "  ✓ coding-standards.md (project-specific code patterns)"

cp "${SKILL_DIR}/assets/context-templates/architecture-constraints.md" "${CONTEXT_DIR}/" && \
  echo "  ✓ architecture-constraints.md (layer boundaries and design rules)"

cp "${SKILL_DIR}/assets/context-templates/anti-patterns.md" "${CONTEXT_DIR}/" && \
  echo "  ✓ anti-patterns.md (explicitly forbidden patterns)"

# Add timestamps and placeholders
echo ""
echo "🔧 Configuring templates..."

DATE=$(date +%Y-%m-%d)
PROJECT_NAME=$(basename "$(realpath "${PROJECT_ROOT}")")

# Replace placeholders in all files
for file in "${CONTEXT_DIR}"/*.md; do
  if [[ -f "$file" ]]; then
    sed -i.bak "s/\[DATE\]/${DATE}/g" "$file"
    sed -i.bak "s/\[PROJECT_NAME\]/${PROJECT_NAME}/g" "$file"
    rm "${file}.bak" 2>/dev/null || true
  fi
done

echo "  ✓ Timestamps and project name set"

# Create initial README in docs/architecture
cat > "${PROJECT_ROOT}/docs/architecture/README.md" << 'EOF'
# Architecture Documentation

This directory contains architecture documentation for the project.

## Structure

- `decisions/` - Architecture Decision Records (ADRs)
- Context files are in `devforgeai/specs/context/`

## Context Files

The 6 critical context files that prevent technical debt:

1. **tech-stack.md** - Locks technology choices
2. **source-tree.md** - Enforces project structure
3. **dependencies.md** - Locks approved packages
4. **coding-standards.md** - Project-specific code patterns
5. **architecture-constraints.md** - Layer boundaries and design rules
6. **anti-patterns.md** - Explicitly forbidden patterns

## Architecture Decision Records (ADRs)

ADRs document significant technology and architecture decisions.

Format: `ADR-XXX-short-title.md`

Example:
- `ADR-001-use-dapper-for-data-access.md`
- `ADR-002-use-zustand-for-state-management.md`
- `ADR-003-implement-clean-architecture.md`
EOF

echo "  ✓ Created docs/architecture/README.md"

echo ""
echo "✅ Context files initialized successfully!"
echo ""
echo "📝 Next steps:"
echo ""
echo "  1. Customize context files in devforgeai/specs/context/:"
echo "     - tech-stack.md - Fill in technology choices"
echo "     - source-tree.md - Customize project structure"
echo "     - dependencies.md - Add approved packages and versions"
echo "     - coding-standards.md - Add technology-specific patterns"
echo "     - architecture-constraints.md - Define layer dependency rules"
echo "     - anti-patterns.md - Customize forbidden patterns for your stack"
echo ""
echo "  2. Create ADRs for major decisions:"
echo "     - Database choice"
echo "     - ORM selection"
echo "     - State management library"
echo "     - Architecture pattern"
echo ""
echo "  3. Use devforgeai-architecture skill to complete setup:"
echo "     - Load skill in Claude Code"
echo "     - Answer AskUserQuestion prompts for technology choices"
echo "     - Skill will customize all templates based on your answers"
echo ""
echo "  4. Validate your setup:"
echo "     - Run: ${SKILL_DIR}/scripts/validate_all_context.py"
echo "     - Ensures all context files are properly configured"
echo ""
echo "⚠️  CRITICAL: These files prevent technical debt by locking technology decisions."
echo "    AI agents will enforce these constraints during development."
echo ""
echo "    Example: If tech-stack.md specifies Dapper as ORM, AI agents will:"
echo "      ✓ Use Dapper for all data access"
echo "      ✗ NEVER substitute Entity Framework (even if encountering issues)"
echo "      ✗ NEVER add unauthorized packages"
echo "      ? ALWAYS use AskUserQuestion for ambiguous decisions"
echo ""
