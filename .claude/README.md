# ⚠️ DEPRECATED - This is the Deployed Framework Directory

**Status**: Deprecated as of v1.0.1 (2025-11-17)
**Support Until**: v2.0.0 (May 2026)

---

## What Changed?

This `.claude/` directory is now **deployed** by the installer from `src/` source directory.

### Old Approach (Deprecated ❌)

Previously, you edited files directly here:

```bash
# OLD (deprecated) - DON'T DO THIS
vim ~/.claude/skills/devforgeai-development/SKILL.md
# Direct edits to deployed files
```

### New Approach (Recommended ✅)

Now, edit source files in the repository and deploy with installer:

```bash
# NEW (recommended) - DO THIS
vim src/.claude/skills/devforgeai-development/SKILL.md

# Deploy your changes
python installer/install.py --mode=upgrade

# Restart Claude Code Terminal
```

---

## Why This Change?

| Aspect | Old Way | New Way |
|--------|---------|---------|
| **Source Control** | Changes in `.claude/` (deployed dir) | Changes in `src/` (version controlled) |
| **Reproducibility** | Manual edits, no version control | Git commits, full history |
| **Updates** | Manual merges, conflicts | Automated installer with rollback |
| **Backups** | Manual backups | Automatic backups with every upgrade |
| **Rollback** | Manual restoration | One-command rollback |

---

## Migration Instructions

See **[MIGRATION-GUIDE.md](../MIGRATION-GUIDE.md)** for complete 7-step migration process.

Quick start:

```bash
# 1. Go to repository
cd /path/to/DevForgeAI2

# 2. Run installer
python installer/install.py --mode=upgrade

# 3. Restart Claude Code Terminal

# 4. Now edit in src/ instead
vim src/.claude/skills/devforgeai-development/SKILL.md
python installer/install.py --mode=upgrade
```

---

## Directory Structure

This `.claude/` directory is now managed by the installer:

```
~/.claude/
├── skills/              # Deployed framework skills (DO NOT EDIT)
├── commands/            # Deployed slash commands (DO NOT EDIT)
├── agents/              # Deployed AI subagents (DO NOT EDIT)
├── memory/              # Reference documentation (DO NOT EDIT)
├── CLAUDE.md            # Framework instructions (DO NOT EDIT)
└── version.json         # Version metadata (AUTO-GENERATED)
```

**⚠️ Do not edit these files directly.** They are overwritten during installation.

---

## What to Do If You Have Custom Skills

If you created custom skills here, preserve them:

```bash
# 1. Backup your custom files
cp ~/.claude/skills/my-custom-skill ~/.claude/skills/my-custom-skill.backup

# 2. After running installer, restore them
python installer/install.py --mode=upgrade
cp ~/.claude/skills/my-custom-skill.backup ~/.claude/skills/my-custom-skill

# 3. Or better - add to src/ for version control
cp src/.claude/skills/my-custom-skill ~/.claude/skills/my-custom-skill
python installer/install.py --mode=upgrade
```

---

## Support Timeline

The manual copy approach from v1.0.0 will be supported for **at least 6 months** (through May 2026) to allow teams adequate time to migrate.

| Version | Status | Action | Deadline |
|---------|--------|--------|----------|
| **v1.0.0** | Deprecated | Use old approach ❌ (no longer recommended) | EOL |
| **v1.0.1** | **CURRENT** | Migrate to new approach ✅ | Now |
| **v1.0.2+** (Jan 2026) | Future | New approach standard | — |
| **v2.0.0** (May 2026) | Future | Old approach removed 🔴 | Hard deadline |

**Deprecation Date**: November 17, 2025 (v1.0.1 release)
**Support Ends**: May 17, 2026 (v2.0.0 release)
**Minimum Support Window**: 6 months

---

## See Also

- **[MIGRATION-GUIDE.md](../MIGRATION-GUIDE.md)** - 7-step migration process
- **[INSTALL.md](../installer/INSTALL.md)** - Complete installation guide
- **[README.md](../README.md)** - Framework overview

---

**Questions?** See MIGRATION-GUIDE.md for troubleshooting, or create a GitHub issue.

This directory is managed by the installer. Edit files in `src/.claude/` instead.
