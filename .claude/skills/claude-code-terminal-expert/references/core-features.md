# Claude Code Terminal - Core Features Reference

**Source:** Official documentation from code.claude.com (updated 2025-12-09)

This comprehensive reference consolidates 5 core documentation files covering Claude Code Terminal's main extensibility features: Subagents, Skills, Slash Commands, Plugins, and MCP integration.

---

## Table of Contents

1. [Subagents - Specialized AI Workers](#section-1-subagents---specialized-ai-workers)
2. [Skills - Model-Invoked Capabilities](#section-2-skills---model-invoked-capabilities)
3. [Slash Commands - User-Invoked Workflows](#section-3-slash-commands---user-invoked-workflows)
4. [Plugins - Bundled Extensions](#section-4-plugins---bundled-extensions)
5. [MCP Servers - External Tool Integration](#section-5-mcp-servers---external-tool-integration)
6. [Cross-Reference Guide](#cross-reference-guide)

---

## Section 1: Subagents - Specialized AI Workers

> Create and use specialized AI subagents in Claude Code for task-specific workflows and improved context management.

Custom subagents in Claude Code are specialized AI assistants that can be invoked to handle specific types of tasks. They enable more efficient problem-solving by providing task-specific configurations with customized system prompts, tools and a separate context window.

### What are subagents?

Subagents are pre-configured AI personalities that Claude Code can delegate tasks to. Each subagent:

* Has a specific purpose and expertise area
* Uses its own context window separate from the main conversation
* Can be configured with specific tools it's allowed to use
* Includes a custom system prompt that guides its behavior

When Claude Code encounters a task that matches a subagent's expertise, it can delegate that task to the specialized subagent, which works independently and returns results.

### Key benefits

**Context preservation**: Each subagent operates in its own context, preventing pollution of the main conversation and keeping it focused on high-level objectives.

**Specialized expertise**: Subagents can be fine-tuned with detailed instructions for specific domains, leading to higher success rates on designated tasks.

**Reusability**: Once created, subagents can be used across different projects and shared with your team for consistent workflows.

**Flexible permissions**: Each subagent can have different tool access levels, allowing you to limit powerful tools to specific subagent types.

### Quick start

To create your first subagent:

**Step 1: Open the subagents interface**

Run the following command:

```
/agents
```

**Step 2: Select 'Create New Agent'**

Choose whether to create a project-level or user-level subagent

**Step 3: Define the subagent**

* **Recommended**: Generate with Claude first, then customize to make it yours
* Describe your subagent in detail and when it should be used
* Select the tools you want to grant access to (or leave blank to inherit all tools)
* The interface shows all available tools, making selection easy
* If you're generating with Claude, you can also edit the system prompt in your own editor by pressing `e`

**Step 4: Save and use**

Your subagent is now available! Claude will use it automatically when appropriate, or you can invoke it explicitly:

```
> Use the code-reviewer subagent to check my recent changes
```

### Subagent configuration

#### File locations

Subagents are stored as Markdown files with YAML frontmatter in two possible locations:

| Type                  | Location            | Scope                         | Priority |
| :-------------------- | :------------------ | :---------------------------- | :------- |
| **Project subagents** | `.claude/agents/`   | Available in current project  | Highest  |
| **User subagents**    | `~/.claude/agents/` | Available across all projects | Lower    |

When subagent names conflict, project-level subagents take precedence over user-level subagents.

#### Plugin agents

[Plugins](/en/docs/claude-code/plugins) can provide custom subagents that integrate seamlessly with Claude Code. Plugin agents work identically to user-defined agents and appear in the `/agents` interface.

**Plugin agent locations**: Plugins include agents in their `agents/` directory (or custom paths specified in the plugin manifest).

**Using plugin agents**:

* Plugin agents appear in `/agents` alongside your custom agents
* Can be invoked explicitly: "Use the code-reviewer agent from the security-plugin"
* Can be invoked automatically by Claude when appropriate
* Can be managed (viewed, inspected) through `/agents` interface

See the [plugin components reference](/en/docs/claude-code/plugins-reference#agents) for details on creating plugin agents.

#### CLI-based configuration

You can also define subagents dynamically using the `--agents` CLI flag, which accepts a JSON object:

```bash
claude --agents '{
  "code-reviewer": {
    "description": "Expert code reviewer. Use proactively after code changes.",
    "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
    "tools": ["Read", "Grep", "Glob", "Bash"],
    "model": "sonnet"
  }
}'
```

**Priority**: CLI-defined subagents have lower priority than project-level subagents but higher priority than user-level subagents.

**Use case**: This approach is useful for:

* Quick testing of subagent configurations
* Session-specific subagents that don't need to be saved
* Automation scripts that need custom subagents
* Sharing subagent definitions in documentation or scripts

For detailed information about the JSON format and all available options, see the [CLI reference documentation](/en/docs/claude-code/cli-reference#agents-flag-format).

#### File format

Each subagent is defined in a Markdown file with this structure:

```markdown
---
name: your-sub-agent-name
description: Description of when this subagent should be invoked
tools: tool1, tool2, tool3  # Optional - inherits all tools if omitted
model: opus  # Optional - specify model alias or 'inherit'
---

Your subagent's system prompt goes here. This can be multiple paragraphs
and should clearly define the subagent's role, capabilities, and approach
to solving problems.

Include specific instructions, best practices, and any constraints
the subagent should follow.
```

#### Configuration fields

| Field         | Required | Description                                                                                                                                                                                                                      |
| :------------ | :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Yes      | Unique identifier using lowercase letters and hyphens                                                                                                                                                                            |
| `description` | Yes      | Natural language description of the subagent's purpose                                                                                                                                                                           |
| `tools`       | No       | Comma-separated list of specific tools. If omitted, inherits all tools from the main thread                                                                                                                                      |
| `model`       | No       | Model to use for this subagent. Can be a model alias (`sonnet`, `opus`, `haiku`) or `'inherit'` to use the main conversation's model. If omitted, defaults to the [configured subagent model](/en/docs/claude-code/model-config) |
| `permissionMode` | No   | Permission handling mode: `default`, `acceptEdits`, `bypassPermissions`, `plan`, or `ignore` |
| `skills`      | No       | Comma-separated list of skills to auto-load when subagent is invoked |

#### Resumable Subagents (2025)

Subagents can be resumed to continue previous conversations:

```
> Use the code-analyzer agent to start reviewing the authentication module
[Returns agentId: "abc123"]

> Resume agent abc123 and now analyze the authorization logic as well
[Agent continues with full context from previous conversation]
```

**Storage:** Agent transcripts are stored as `agent-{agentId}.jsonl` in the project directory.

**Use cases:**
- Long-running research across multiple sessions
- Iterative refinement without losing context
- Multi-step workflows with sequential related tasks

#### Model selection

The `model` field allows you to control which [AI model](/en/docs/claude-code/model-config) the subagent uses:

* **Model alias**: Use one of the available aliases: `sonnet`, `opus`, or `haiku`
* **`'inherit'`**: Use the same model as the main conversation (useful for consistency)
* **Omitted**: If not specified, uses the default model configured for subagents (`sonnet`)

**Note**: Using `'inherit'` is particularly useful when you want your subagents to adapt to the model choice of the main conversation, ensuring consistent capabilities and response style throughout your session.

#### Available tools

Subagents can be granted access to any of Claude Code's internal tools. See the [tools documentation](/en/docs/claude-code/settings#tools-available-to-claude) for a complete list of available tools.

**Tip**: Use the `/agents` command to modify tool access - it provides an interactive interface that lists all available tools, including any connected MCP server tools, making it easier to select the ones you need.

You have two options for configuring tools:

* **Omit the `tools` field** to inherit all tools from the main thread (default), including MCP tools
* **Specify individual tools** as a comma-separated list for more granular control (can be edited manually or via `/agents`)

**MCP Tools**: Subagents can access MCP tools from configured MCP servers. When the `tools` field is omitted, subagents inherit all MCP tools available to the main thread.

### Managing subagents

#### Using the /agents command (Recommended)

The `/agents` command provides a comprehensive interface for subagent management:

```
/agents
```

This opens an interactive menu where you can:

* View all available subagents (built-in, user, and project)
* Create new subagents with guided setup
* Edit existing custom subagents, including their tool access
* Delete custom subagents
* See which subagents are active when duplicates exist
* **Easily manage tool permissions** with a complete list of available tools

#### Direct file management

You can also manage subagents by working directly with their files:

```bash
# Create a project subagent
mkdir -p .claude/agents
echo '---
name: test-runner
description: Use proactively to run tests and fix failures
---

You are a test automation expert. When you see code changes, proactively run the appropriate tests. If tests fail, analyze the failures and fix them while preserving the original test intent.' > .claude/agents/test-runner.md

# Create a user subagent
mkdir -p ~/.claude/agents
# ... create subagent file
```

### Using subagents effectively

#### Automatic delegation

Claude Code proactively delegates tasks based on:

* The task description in your request
* The `description` field in subagent configurations
* Current context and available tools

**Tip**: To encourage more proactive subagent use, include phrases like "use PROACTIVELY" or "MUST BE USED" in your `description` field.

#### Explicit invocation

Request a specific subagent by mentioning it in your command:

```
> Use the test-runner subagent to fix failing tests
> Have the code-reviewer subagent look at my recent changes
> Ask the debugger subagent to investigate this error
```

### Example subagents

#### Code reviewer

```markdown
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is simple and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

#### Debugger

```markdown
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not just symptoms.
```

#### Data scientist

```markdown
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: opus
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

### Best practices

* **Start with Claude-generated agents**: We highly recommend generating your initial subagent with Claude and then iterating on it to make it personally yours. This approach gives you the best results - a solid foundation that you can customize to your specific needs.

* **Design focused subagents**: Create subagents with single, clear responsibilities rather than trying to make one subagent do everything. This improves performance and makes subagents more predictable.

* **Write detailed prompts**: Include specific instructions, examples, and constraints in your system prompts. The more guidance you provide, the better the subagent will perform.

* **Limit tool access**: Only grant tools that are necessary for the subagent's purpose. This improves security and helps the subagent focus on relevant actions.

* **Version control**: Check project subagents into version control so your team can benefit from and improve them collaboratively.

### Advanced usage

#### Chaining subagents

For complex workflows, you can chain multiple subagents:

```
> First use the code-analyzer subagent to find performance issues, then use the optimizer subagent to fix them
```

#### Dynamic subagent selection

Claude Code intelligently selects subagents based on context. Make your `description` fields specific and action-oriented for best results.

### Performance considerations

* **Context efficiency**: Agents help preserve main context, enabling longer overall sessions
* **Latency**: Subagents start off with a clean slate each time they are invoked and may add latency as they gather context that they require to do their job effectively.

### Related documentation

* [Plugins](/en/docs/claude-code/plugins) - Extend Claude Code with custom agents through plugins
* [Slash commands](/en/docs/claude-code/slash-commands) - Learn about other built-in commands
* [Settings](/en/docs/claude-code/settings) - Configure Claude Code behavior
* [Hooks](/en/docs/claude-code/hooks) - Automate workflows with event handlers

---

## Section 2: Skills - Model-Invoked Capabilities

> Create, manage, and share Skills to extend Claude's capabilities in Claude Code.

This guide shows you how to create, use, and manage Agent Skills in Claude Code. Skills are modular capabilities that extend Claude's functionality through organized folders containing instructions, scripts, and resources.

### Prerequisites

* Claude Code version 1.0 or later
* Basic familiarity with [Claude Code](/en/docs/claude-code/quickstart)

### What are Agent Skills?

Agent Skills package expertise into discoverable capabilities. Each Skill consists of a `SKILL.md` file with instructions that Claude reads when relevant, plus optional supporting files like scripts and templates.

**How Skills are invoked**: Skills are **model-invoked**—Claude autonomously decides when to use them based on your request and the Skill's description. This is different from slash commands, which are **user-invoked** (you explicitly type `/command` to trigger them).

**Benefits**:

* Extend Claude's capabilities for your specific workflows
* Share expertise across your team via git
* Reduce repetitive prompting
* Compose multiple Skills for complex tasks

Learn more in the [Agent Skills overview](/en/docs/agents-and-tools/agent-skills/overview).

**Note**: For a deep dive into the architecture and real-world applications of Agent Skills, read our engineering blog: [Equipping agents for the real world with Agent Skills](https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills).

### Create a Skill

Skills are stored as directories containing a `SKILL.md` file.

#### Personal Skills

Personal Skills are available across all your projects. Store them in `~/.claude/skills/`:

```bash
mkdir -p ~/.claude/skills/my-skill-name
```

**Use personal Skills for**:

* Your individual workflows and preferences
* Experimental Skills you're developing
* Personal productivity tools

#### Project Skills

Project Skills are shared with your team. Store them in `.claude/skills/` within your project:

```bash
mkdir -p .claude/skills/my-skill-name
```

**Use project Skills for**:

* Team workflows and conventions
* Project-specific expertise
* Shared utilities and scripts

Project Skills are checked into git and automatically available to team members.

#### Plugin Skills

Skills can also come from [Claude Code plugins](/en/docs/claude-code/plugins). Plugins may bundle Skills that are automatically available when the plugin is installed. These Skills work the same way as personal and project Skills.

### Write SKILL.md

Create a `SKILL.md` file with YAML frontmatter and Markdown content:

```yaml
---
name: Your Skill Name
description: Brief description of what this Skill does and when to use it
---

# Your Skill Name

## Instructions
Provide clear, step-by-step guidance for Claude.

## Examples
Show concrete examples of using this Skill.
```

The `description` field is critical for Claude to discover when to use your Skill. It should include both what the Skill does and when Claude should use it.

See the [best practices guide](/en/docs/agents-and-tools/agent-skills/best-practices) for complete authoring guidance.

### Add supporting files

Create additional files alongside SKILL.md:

```
my-skill/
├── SKILL.md (required)
├── reference.md (optional documentation)
├── examples.md (optional examples)
├── scripts/
│   └── helper.py (optional utility)
└── templates/
    └── template.txt (optional template)
```

Reference these files from SKILL.md:

````markdown
For advanced usage, see [reference.md](reference.md).

Run the helper script:
```bash
python scripts/helper.py input.txt
```
````

Claude reads these files only when needed, using progressive disclosure to manage context efficiently.

### Restrict tool access with allowed-tools

Use the `allowed-tools` frontmatter field to limit which tools Claude can use when a Skill is active:

```yaml
---
name: Safe File Reader
description: Read files without making changes. Use when you need read-only file access.
allowed-tools: Read, Grep, Glob
---

# Safe File Reader

This Skill provides read-only file access.

## Instructions
1. Use Read to view file contents
2. Use Grep to search within files
3. Use Glob to find files by pattern
```

When this Skill is active, Claude can only use the specified tools (Read, Grep, Glob) without needing to ask for permission. This is useful for:

* Read-only Skills that shouldn't modify files
* Skills with limited scope (e.g., only data analysis, no file writing)
* Security-sensitive workflows where you want to restrict capabilities

If `allowed-tools` is not specified, Claude will ask for permission to use tools as normal, following the standard permission model.

**Note**: `allowed-tools` is only supported for Skills in Claude Code.

### Agent Skills Specification Compliance

Claude Code Skills follow the **Agent Skills specification** (agentskills.io), an open standard by Anthropic adopted by Microsoft, GitHub, Cursor, Spring AI, and VS Code.

#### YAML Frontmatter Schema

**Required fields:**
- `name`: 1-64 chars, lowercase-with-hyphens only
- `description`: 1-1024 chars, MUST include trigger context ("Use when...")

**Optional fields:**
- `license`: License identifier (e.g., `MIT`, `Apache-2.0`)
- `compatibility`: Version requirements (e.g., `"Claude Code v2.0+"`)
- `metadata`: Key-value map for author, version, category (NOT at top level)
- `allowed-tools`: Space-delimited tool whitelist
- `disable-model-invocation`: Boolean to prevent auto-triggering

**Example (fully compliant):**
```yaml
---
name: code-reviewer
description: |
  Expert code review assistant. Use when users ask for code review,
  want feedback on their code, or need security analysis.
license: MIT
compatibility: "Claude Code v2.0+"
metadata:
  author: TeamName
  version: "2.0.0"
  category: development-tools
allowed-tools: Read Grep Glob
---
```

**CRITICAL:** Fields like `version`, `author`, `category` MUST be nested under `metadata`, not at the top level.

#### Validation

Install and run the official validator:
```bash
pip install skills-ref
skills-ref validate .claude/skills/my-skill/
```

**For complete Agent Skills specification:** See `references/agent-skills-spec.md`

### View available Skills

Skills are automatically discovered by Claude from three sources:

* Personal Skills: `~/.claude/skills/`
* Project Skills: `.claude/skills/`
* Plugin Skills: bundled with installed plugins

**To view all available Skills**, ask Claude directly:

```
What Skills are available?
```

or

```
List all available Skills
```

This will show all Skills from all sources, including plugin Skills.

**To inspect a specific Skill**, you can also check the filesystem:

```bash
# List personal Skills
ls ~/.claude/skills/

# List project Skills (if in a project directory)
ls .claude/skills/

# View a specific Skill's content
cat ~/.claude/skills/my-skill/SKILL.md
```

### Test a Skill

After creating a Skill, test it by asking questions that match your description.

**Example**: If your description mentions "PDF files":

```
Can you help me extract text from this PDF?
```

Claude autonomously decides to use your Skill if it matches the request—you don't need to explicitly invoke it. The Skill activates automatically based on the context of your question.

### Debug a Skill

If Claude doesn't use your Skill, check these common issues:

#### Make description specific

**Too vague**:

```yaml
description: Helps with documents
```

**Specific**:

```yaml
description: Extract text and tables from PDF files, fill forms, merge documents. Use when working with PDF files or when the user mentions PDFs, forms, or document extraction.
```

Include both what the Skill does and when to use it in the description.

#### Verify file path

**Personal Skills**: `~/.claude/skills/skill-name/SKILL.md`
**Project Skills**: `.claude/skills/skill-name/SKILL.md`

Check the file exists:

```bash
# Personal
ls ~/.claude/skills/my-skill/SKILL.md

# Project
ls .claude/skills/my-skill/SKILL.md
```

#### Check YAML syntax

Invalid YAML prevents the Skill from loading. Verify the frontmatter:

```bash
cat SKILL.md | head -n 10
```

Ensure:

* Opening `---` on line 1
* Closing `---` before Markdown content
* Valid YAML syntax (no tabs, correct indentation)

#### View errors

Run Claude Code with debug mode to see Skill loading errors:

```bash
claude --debug
```

### Share Skills with your team

**Recommended approach**: Distribute Skills through [plugins](/en/docs/claude-code/plugins).

To share Skills via plugin:

1. Create a plugin with Skills in the `skills/` directory
2. Add the plugin to a marketplace
3. Team members install the plugin

For complete instructions, see [Add Skills to your plugin](/en/docs/claude-code/plugins#add-skills-to-your-plugin).

You can also share Skills directly through project repositories:

#### Step 1: Add Skill to your project

Create a project Skill:

```bash
mkdir -p .claude/skills/team-skill
# Create SKILL.md
```

#### Step 2: Commit to git

```bash
git add .claude/skills/
git commit -m "Add team Skill for PDF processing"
git push
```

#### Step 3: Team members get Skills automatically

When team members pull the latest changes, Skills are immediately available:

```bash
git pull
claude  # Skills are now available
```

### Update a Skill

Edit SKILL.md directly:

```bash
# Personal Skill
code ~/.claude/skills/my-skill/SKILL.md

# Project Skill
code .claude/skills/my-skill/SKILL.md
```

Changes take effect the next time you start Claude Code. If Claude Code is already running, restart it to load the updates.

### Remove a Skill

Delete the Skill directory:

```bash
# Personal
rm -rf ~/.claude/skills/my-skill

# Project
rm -rf .claude/skills/my-skill
git commit -m "Remove unused Skill"
```

### Best practices

#### Keep Skills focused

One Skill should address one capability:

**Focused**:

* "PDF form filling"
* "Excel data analysis"
* "Git commit messages"

**Too broad**:

* "Document processing" (split into separate Skills)
* "Data tools" (split by data type or operation)

#### Write clear descriptions

Help Claude discover when to use Skills by including specific triggers in your description:

**Clear**:

```yaml
description: Analyze Excel spreadsheets, create pivot tables, and generate charts. Use when working with Excel files, spreadsheets, or analyzing tabular data in .xlsx format.
```

**Vague**:

```yaml
description: For files
```

#### Test with your team

Have teammates use Skills and provide feedback:

* Does the Skill activate when expected?
* Are the instructions clear?
* Are there missing examples or edge cases?

#### Document Skill versions

You can document Skill versions in your SKILL.md content to track changes over time. Add a version history section:

```markdown
# My Skill

## Version History
- v2.0.0 (2025-10-01): Breaking changes to API
- v1.1.0 (2025-09-15): Added new features
- v1.0.0 (2025-09-01): Initial release
```

This helps team members understand what changed between versions.

### Troubleshooting

#### Claude doesn't use my Skill

**Symptom**: You ask a relevant question but Claude doesn't use your Skill.

**Check**: Is the description specific enough?

Vague descriptions make discovery difficult. Include both what the Skill does and when to use it, with key terms users would mention.

**Too generic**:

```yaml
description: Helps with data
```

**Specific**:

```yaml
description: Analyze Excel spreadsheets, generate pivot tables, create charts. Use when working with Excel files, spreadsheets, or .xlsx files.
```

**Check**: Is the YAML valid?

Run validation to check for syntax errors:

```bash
# View frontmatter
cat .claude/skills/my-skill/SKILL.md | head -n 15

# Check for common issues
# - Missing opening or closing ---
# - Tabs instead of spaces
# - Unquoted strings with special characters
```

**Check**: Is the Skill in the correct location?

```bash
# Personal Skills
ls ~/.claude/skills/*/SKILL.md

# Project Skills
ls .claude/skills/*/SKILL.md
```

#### Skill has errors

**Symptom**: The Skill loads but doesn't work correctly.

**Check**: Are dependencies available?

Claude will automatically install required dependencies (or ask for permission to install them) when it needs them.

**Check**: Do scripts have execute permissions?

```bash
chmod +x .claude/skills/my-skill/scripts/*.py
```

**Check**: Are file paths correct?

Use forward slashes (Unix style) in all paths:

**Correct**: `scripts/helper.py`
**Wrong**: `scripts\helper.py` (Windows style)

#### Multiple Skills conflict

**Symptom**: Claude uses the wrong Skill or seems confused between similar Skills.

**Be specific in descriptions**: Help Claude choose the right Skill by using distinct trigger terms in your descriptions.

Instead of:

```yaml
# Skill 1
description: For data analysis

# Skill 2
description: For analyzing data
```

Use:

```yaml
# Skill 1
description: Analyze sales data in Excel files and CRM exports. Use for sales reports, pipeline analysis, and revenue tracking.

# Skill 2
description: Analyze log files and system metrics data. Use for performance monitoring, debugging, and system diagnostics.
```

### Examples

#### Simple Skill (single file)

```
commit-helper/
└── SKILL.md
```

```yaml
---
name: Generating Commit Messages
description: Generates clear commit messages from git diffs. Use when writing commit messages or reviewing staged changes.
---

# Generating Commit Messages

## Instructions

1. Run `git diff --staged` to see changes
2. I'll suggest a commit message with:
   - Summary under 50 characters
   - Detailed description
   - Affected components

## Best practices

- Use present tense
- Explain what and why, not how
```

#### Skill with tool permissions

```
code-reviewer/
└── SKILL.md
```

```yaml
---
name: Code Reviewer
description: Review code for best practices and potential issues. Use when reviewing code, checking PRs, or analyzing code quality.
allowed-tools: Read, Grep, Glob
---

# Code Reviewer

## Review checklist

1. Code organization and structure
2. Error handling
3. Performance considerations
4. Security concerns
5. Test coverage

## Instructions

1. Read the target files using Read tool
2. Search for patterns using Grep
3. Find related files using Glob
4. Provide detailed feedback on code quality
```

#### Multi-file Skill

```
pdf-processing/
├── SKILL.md
├── FORMS.md
├── REFERENCE.md
└── scripts/
    ├── fill_form.py
    └── validate.py
```

**SKILL.md**:

````yaml
---
name: PDF Processing
description: Extract text, fill forms, merge PDFs. Use when working with PDF files, forms, or document extraction. Requires pypdf and pdfplumber packages.
---

# PDF Processing

## Quick start

Extract text:
```python
import pdfplumber
with pdfplumber.open("doc.pdf") as pdf:
    text = pdf.pages[0].extract_text()
```

For form filling, see [FORMS.md](FORMS.md).
For detailed API reference, see [REFERENCE.md](REFERENCE.md).

## Requirements

Packages must be installed in your environment:
```bash
pip install pypdf pdfplumber
```
````

**Note**: List required packages in the description. Packages must be installed in your environment before Claude can use them.

Claude loads additional files only when needed.

### Next steps

* **Authoring best practices**: Write Skills that Claude can use effectively - [Best practices guide](/en/docs/agents-and-tools/agent-skills/best-practices)
* **Agent Skills overview**: Learn how Skills work across Claude products - [Overview](/en/docs/agents-and-tools/agent-skills/overview)
* **Get started with Agent Skills**: Create your first Skill - [Quickstart](/en/docs/agents-and-tools/agent-skills/quickstart)

---

## Section 3: Slash Commands - User-Invoked Workflows

> Control Claude's behavior during an interactive session with slash commands.

### Built-in slash commands

| Command                   | Purpose                                                                                                                                      |
| :------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `/add-dir`                | Add additional working directories                                                                                                           |
| `/agents`                 | Manage custom AI subagents for specialized tasks                                                                                             |
| `/bug`                    | Report bugs (sends conversation to Anthropic)                                                                                                |
| `/clear`                  | Clear conversation history                                                                                                                   |
| `/compact [instructions]` | Compact conversation with optional focus instructions                                                                                        |
| `/config`                 | Open the Settings interface (Config tab)                                                                                                     |
| `/cost`                   | Show token usage statistics (see [cost tracking guide](/en/docs/claude-code/costs#using-the-cost-command) for subscription-specific details) |
| `/doctor`                 | Checks the health of your Claude Code installation                                                                                           |
| `/help`                   | Get usage help                                                                                                                               |
| `/init`                   | Initialize project with CLAUDE.md guide                                                                                                      |
| `/login`                  | Switch Anthropic accounts                                                                                                                    |
| `/logout`                 | Sign out from your Anthropic account                                                                                                         |
| `/mcp`                    | Manage MCP server connections and OAuth authentication                                                                                       |
| `/memory`                 | Edit CLAUDE.md memory files                                                                                                                  |
| `/model`                  | Select or change the AI model                                                                                                                |
| `/permissions`            | View or update [permissions](/en/docs/claude-code/iam#configuring-permissions)                                                               |
| `/pr_comments`            | View pull request comments                                                                                                                   |
| `/review`                 | Request code review                                                                                                                          |
| `/rewind`                 | Rewind the conversation and/or code                                                                                                          |
| `/status`                 | Open the Settings interface (Status tab) showing version, model, account, and connectivity                                                   |
| `/terminal-setup`         | Install Shift+Enter key binding for newlines (iTerm2 and VSCode only)                                                                        |
| `/usage`                  | Show plan usage limits and rate limit status (subscription plans only)                                                                       |
| `/vim`                    | Enter vim mode for alternating insert and command modes                                                                                      |

### Custom slash commands

Custom slash commands allow you to define frequently-used prompts as Markdown files that Claude Code can execute. Commands are organized by scope (project-specific or personal) and support namespacing through directory structures.

#### Syntax

```
/<command-name> [arguments]
```

**Parameters**:

| Parameter        | Description                                                       |
| :--------------- | :---------------------------------------------------------------- |
| `<command-name>` | Name derived from the Markdown filename (without `.md` extension) |
| `[arguments]`    | Optional arguments passed to the command                          |

#### Command types

**Project commands**

Commands stored in your repository and shared with your team. When listed in `/help`, these commands show "(project)" after their description.

**Location**: `.claude/commands/`

In the following example, we create the `/optimize` command:

```bash
# Create a project command
mkdir -p .claude/commands
echo "Analyze this code for performance issues and suggest optimizations:" > .claude/commands/optimize.md
```

**Personal commands**

Commands available across all your projects. When listed in `/help`, these commands show "(user)" after their description.

**Location**: `~/.claude/commands/`

In the following example, we create the `/security-review` command:

```bash
# Create a personal command
mkdir -p ~/.claude/commands
echo "Review this code for security vulnerabilities:" > ~/.claude/commands/security-review.md
```

#### Features

**Namespacing**

Organize commands in subdirectories. The subdirectories are used for organization and appear in the command description, but they do not affect the command name itself. The description will show whether the command comes from the project directory (`.claude/commands`) or the user-level directory (`~/.claude/commands`), along with the subdirectory name.

Conflicts between user and project level commands are not supported. Otherwise, multiple commands with the same base file name can coexist.

For example, a file at `.claude/commands/frontend/component.md` creates the command `/component` with description showing "(project:frontend)".
Meanwhile, a file at `~/.claude/commands/component.md` creates the command `/component` with description showing "(user)".

**Arguments**

Pass dynamic values to commands using argument placeholders:

**All arguments with `$ARGUMENTS`**

The `$ARGUMENTS` placeholder captures all arguments passed to the command:

```bash
# Command definition
echo 'Fix issue #$ARGUMENTS following our coding standards' > .claude/commands/fix-issue.md

# Usage
> /fix-issue 123 high-priority
# $ARGUMENTS becomes: "123 high-priority"
```

**Individual arguments with `$1`, `$2`, etc.**

Access specific arguments individually using positional parameters (similar to shell scripts):

```bash
# Command definition
echo 'Review PR #$1 with priority $2 and assign to $3' > .claude/commands/review-pr.md

# Usage
> /review-pr 456 high alice
# $1 becomes "456", $2 becomes "high", $3 becomes "alice"
```

Use positional arguments when you need to:

* Access arguments individually in different parts of your command
* Provide defaults for missing arguments
* Build more structured commands with specific parameter roles

**Bash command execution**

Execute bash commands before the slash command runs using the `!` prefix. The output is included in the command context. You *must* include `allowed-tools` with the `Bash` tool, but you can choose the specific bash commands to allow.

For example:

```markdown
---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git commit:*)
description: Create a git commit
---

## Context

- Current git status: !`git status`
- Current git diff (staged and unstaged changes): !`git diff HEAD`
- Current branch: !`git branch --show-current`
- Recent commits: !`git log --oneline -10`

## Your task

Based on the above changes, create a single git commit.
```

**File references**

Include file contents in commands using the `@` prefix to [reference files](/en/docs/claude-code/common-workflows#reference-files-and-directories).

For example:

```markdown
# Reference a specific file

Review the implementation in @src/utils/helpers.js

# Reference multiple files

Compare @src/old-version.js with @src/new-version.js
```

**Thinking mode**

Slash commands can trigger extended thinking by including [extended thinking keywords](/en/docs/claude-code/common-workflows#use-extended-thinking).

#### Frontmatter

Command files support frontmatter, useful for specifying metadata about the command:

| Frontmatter                | Purpose                                                                                                                                                                               | Default                             |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------- |
| `allowed-tools`            | List of tools the command can use                                                                                                                                                     | Inherits from the conversation      |
| `argument-hint`            | The arguments expected for the slash command. Example: `argument-hint: add [tagId] \| remove [tagId] \| list`. This hint is shown to the user when auto-completing the slash command. | None                                |
| `description`              | Brief description of the command                                                                                                                                                      | Uses the first line from the prompt |
| `model`                    | Specific model string (see [Models overview](/en/docs/about-claude/models/overview))                                                                                                  | Inherits from the conversation      |
| `disable-model-invocation` | Whether to prevent `SlashCommand` tool from calling this command                                                                                                                      | false                               |

For example:

```markdown
---
allowed-tools: Bash(git add:*), Bash(git status:*), Bash(git commit:*)
argument-hint: [message]
description: Create a git commit
model: claude-3-5-haiku-20241022
---

Create a git commit with message: $ARGUMENTS
```

Example using positional arguments:

```markdown
---
argument-hint: [pr-number] [priority] [assignee]
description: Review pull request
---

Review PR #$1 with priority $2 and assign to $3.
Focus on security, performance, and code style.
```

### Plugin commands

[Plugins](/en/docs/claude-code/plugins) can provide custom slash commands that integrate seamlessly with Claude Code. Plugin commands work exactly like user-defined commands but are distributed through [plugin marketplaces](/en/docs/claude-code/plugin-marketplaces).

#### How plugin commands work

Plugin commands are:

* **Namespaced**: Commands can use the format `/plugin-name:command-name` to avoid conflicts (plugin prefix is optional unless there are name collisions)
* **Automatically available**: Once a plugin is installed and enabled, its commands appear in `/help`
* **Fully integrated**: Support all command features (arguments, frontmatter, bash execution, file references)

#### Plugin command structure

**Location**: `commands/` directory in plugin root

**File format**: Markdown files with frontmatter

**Basic command structure**:

```markdown
---
description: Brief description of what the command does
---

# Command Name

Detailed instructions for Claude on how to execute this command.
Include specific guidance on parameters, expected outcomes, and any special considerations.
```

**Advanced command features**:

* **Arguments**: Use placeholders like `{arg1}` in command descriptions
* **Subdirectories**: Organize commands in subdirectories for namespacing
* **Bash integration**: Commands can execute shell scripts and programs
* **File references**: Commands can reference and modify project files

#### Invocation patterns

```shell
# Direct command (when no conflicts)
/command-name

# Plugin-prefixed (when needed for disambiguation)
/plugin-name:command-name

# With arguments (if command supports them)
/command-name arg1 arg2
```

### MCP slash commands

MCP servers can expose prompts as slash commands that become available in Claude Code. These commands are dynamically discovered from connected MCP servers.

#### Command format

MCP commands follow the pattern:

```
/mcp__<server-name>__<prompt-name> [arguments]
```

#### Features

**Dynamic discovery**

MCP commands are automatically available when:

* An MCP server is connected and active
* The server exposes prompts through the MCP protocol
* The prompts are successfully retrieved during connection

**Arguments**

MCP prompts can accept arguments defined by the server:

```
# Without arguments
> /mcp__github__list_prs

# With arguments
> /mcp__github__pr_review 456
> /mcp__jira__create_issue "Bug title" high
```

**Naming conventions**

* Server and prompt names are normalized
* Spaces and special characters become underscores
* Names are lowercased for consistency

#### Managing MCP connections

Use the `/mcp` command to:

* View all configured MCP servers
* Check connection status
* Authenticate with OAuth-enabled servers
* Clear authentication tokens
* View available tools and prompts from each server

#### MCP permissions and wildcards

When configuring [permissions for MCP tools](/en/docs/claude-code/iam#tool-specific-permission-rules), note that **wildcards are not supported**:

* ✅ **Correct**: `mcp__github` (approves ALL tools from the github server)
* ✅ **Correct**: `mcp__github__get_issue` (approves specific tool)
* ❌ **Incorrect**: `mcp__github__*` (wildcards not supported)

To approve all tools from an MCP server, use just the server name: `mcp__servername`. To approve specific tools only, list each tool individually.

### `SlashCommand` tool

The `SlashCommand` tool allows Claude to execute [custom slash commands](/en/docs/claude-code/slash-commands#custom-slash-commands) programmatically
during a conversation. This gives Claude the ability to invoke custom commands
on your behalf when appropriate.

To encourage Claude to trigger `SlashCommand` tool, your instructions (prompts,
CLAUDE.md, etc.) generally need to reference the command by name with its slash.

Example:

```
> Run /write-unit-test when you are about to start writing tests.
```

This tool puts each available custom slash command's metadata into context up to the
character budget limit. You can use `/context` to monitor token usage and follow
the operations below to manage context.

#### `SlashCommand` tool supported commands

`SlashCommand` tool only supports custom slash commands that:

* Are user-defined. Built-in commands like `/compact` and `/init` are *not* supported.
* Have the `description` frontmatter field populated. We use the `description` in the context.

For Claude Code versions >= 1.0.124, you can see which custom slash commands
`SlashCommand` tool can invoke by running `claude --debug` and triggering a query.

#### Disable `SlashCommand` tool

To prevent Claude from executing any slash commands via the tool:

```bash
/permissions
# Add to deny rules: SlashCommand
```

This will also remove SlashCommand tool (and the slash command descriptions) from context.

#### Disable specific commands only

To prevent a specific slash command from becoming available, add
`disable-model-invocation: true` to the slash command's frontmatter.

This will also remove the command's metadata from context.

#### `SlashCommand` permission rules

The permission rules support:

* **Exact match**: `SlashCommand:/commit` (allows only `/commit` with no arguments)
* **Prefix match**: `SlashCommand:/review-pr:*` (allows `/review-pr` with any arguments)

#### Character budget limit

The `SlashCommand` tool includes a character budget to limit the size of command
descriptions shown to Claude. This prevents token overflow when many commands
are available.

The budget includes each custom slash command's name, args, and description.

* **Default limit**: 15,000 characters
* **Custom limit**: Set via `SLASH_COMMAND_TOOL_CHAR_BUDGET` environment variable

When the character budget is exceeded, Claude will see only a subset of the
available commands. In `/context`, a warning will show with "M of N commands".

### Skills vs slash commands

**Slash commands** and **Agent Skills** serve different purposes in Claude Code:

#### Use slash commands for

**Quick, frequently-used prompts**:

* Simple prompt snippets you use often
* Quick reminders or templates
* Frequently-used instructions that fit in one file

**Examples**:

* `/review` → "Review this code for bugs and suggest improvements"
* `/explain` → "Explain this code in simple terms"
* `/optimize` → "Analyze this code for performance issues"

#### Use Skills for

**Comprehensive capabilities with structure**:

* Complex workflows with multiple steps
* Capabilities requiring scripts or utilities
* Knowledge organized across multiple files
* Team workflows you want to standardize

**Examples**:

* PDF processing Skill with form-filling scripts and validation
* Data analysis Skill with reference docs for different data types
* Documentation Skill with style guides and templates

#### Key differences

| Aspect         | Slash Commands                   | Agent Skills                        |
| -------------- | -------------------------------- | ----------------------------------- |
| **Complexity** | Simple prompts                   | Complex capabilities                |
| **Structure**  | Single .md file                  | Directory with SKILL.md + resources |
| **Discovery**  | Explicit invocation (`/command`) | Automatic (based on context)        |
| **Files**      | One file only                    | Multiple files, scripts, templates  |
| **Scope**      | Project or personal              | Project or personal                 |
| **Sharing**    | Via git                          | Via git                             |

#### Example comparison

**As a slash command**:

```markdown
# .claude/commands/review.md
Review this code for:
- Security vulnerabilities
- Performance issues
- Code style violations
```

Usage: `/review` (manual invocation)

**As a Skill**:

```
.claude/skills/code-review/
├── SKILL.md (overview and workflows)
├── SECURITY.md (security checklist)
├── PERFORMANCE.md (performance patterns)
├── STYLE.md (style guide reference)
└── scripts/
    └── run-linters.sh
```

Usage: "Can you review this code?" (automatic discovery)

The Skill provides richer context, validation scripts, and organized reference material.

#### When to use each

**Use slash commands**:

* You invoke the same prompt repeatedly
* The prompt fits in a single file
* You want explicit control over when it runs

**Use Skills**:

* Claude should discover the capability automatically
* Multiple files or scripts are needed
* Complex workflows with validation steps
* Team needs standardized, detailed guidance

Both slash commands and Skills can coexist. Use the approach that fits your needs.

Learn more about [Agent Skills](/en/docs/claude-code/skills).

### See also

* [Plugins](/en/docs/claude-code/plugins) - Extend Claude Code with custom commands through plugins
* [Identity and Access Management](/en/docs/claude-code/iam) - Complete guide to permissions, including MCP tool permissions
* [Interactive mode](/en/docs/claude-code/interactive-mode) - Shortcuts, input modes, and interactive features
* [CLI reference](/en/docs/claude-code/cli-reference) - Command-line flags and options
* [Settings](/en/docs/claude-code/settings) - Configuration options
* [Memory management](/en/docs/claude-code/memory) - Managing Claude's memory across sessions

---

## Section 4: Plugins - Bundled Extensions

> Extend Claude Code with custom commands, agents, hooks, and MCP servers through the plugin system.

**Tip**: For complete technical specifications and schemas, see [Plugins reference](/en/docs/claude-code/plugins-reference). For marketplace management, see [Plugin marketplaces](/en/docs/claude-code/plugin-marketplaces).

Plugins let you extend Claude Code with custom functionality that can be shared across projects and teams. Install plugins from [marketplaces](/en/docs/claude-code/plugin-marketplaces) to add pre-built commands, agents, hooks, and MCP servers, or create your own to automate your workflows.

### Quickstart

Let's create a simple greeting plugin to get you familiar with the plugin system. We'll build a working plugin that adds a custom command, test it locally, and understand the core concepts.

#### Prerequisites

* Claude Code installed on your machine
* Basic familiarity with command-line tools

#### Create your first plugin

**Step 1: Create the marketplace structure**

```bash
mkdir test-marketplace
cd test-marketplace
```

**Step 2: Create the plugin directory**

```bash
mkdir my-first-plugin
cd my-first-plugin
```

**Step 3: Create the plugin manifest**

```bash
mkdir .claude-plugin
cat > .claude-plugin/plugin.json << 'EOF'
{
  "name": "my-first-plugin",
  "description": "A simple greeting plugin to learn the basics",
  "version": "1.0.0",
  "author": {
    "name": "Your Name"
  }
}
EOF
```

**Step 4: Add a custom command**

```bash
mkdir commands
cat > commands/hello.md << 'EOF'
---
description: Greet the user with a personalized message
---

# Hello Command

Greet the user warmly and ask how you can help them today. Make the greeting personal and encouraging.
EOF
```

**Step 5: Create the marketplace manifest**

```bash
cd ..
mkdir .claude-plugin
cat > .claude-plugin/marketplace.json << 'EOF'
{
  "name": "test-marketplace",
  "owner": {
    "name": "Test User"
  },
  "plugins": [
    {
      "name": "my-first-plugin",
      "source": "./my-first-plugin",
      "description": "My first test plugin"
    }
  ]
}
EOF
```

**Step 6: Install and test your plugin**

```bash
# Start Claude Code from parent directory
cd ..
claude
```

```shell
# Add the test marketplace
/plugin marketplace add ./test-marketplace

# Install your plugin
/plugin install my-first-plugin@test-marketplace
```

Select "Install now". You'll then need to restart Claude Code in order to use the new plugin.

```shell
# Try your new command
/hello
```

You'll see Claude use your greeting command! Check `/help` to see your new command listed.

You've successfully created and tested a plugin with these key components:

* **Plugin manifest** (`.claude-plugin/plugin.json`) - Describes your plugin's metadata
* **Commands directory** (`commands/`) - Contains your custom slash commands
* **Test marketplace** - Allows you to test your plugin locally

#### Plugin structure overview

Your plugin follows this basic structure:

```
my-first-plugin/
├── .claude-plugin/
│   └── plugin.json          # Plugin metadata
├── commands/                 # Custom slash commands (optional)
│   └── hello.md
├── agents/                   # Custom agents (optional)
│   └── helper.md
├── skills/                   # Agent Skills (optional)
│   └── my-skill/
│       └── SKILL.md
└── hooks/                    # Event handlers (optional)
    └── hooks.json
```

**Additional components you can add:**

* **Commands**: Create markdown files in `commands/` directory
* **Agents**: Create agent definitions in `agents/` directory
* **Skills**: Create `SKILL.md` files in `skills/` directory
* **Hooks**: Create `hooks/hooks.json` for event handling
* **MCP servers**: Create `.mcp.json` for external tool integration

**Note**: Ready to add more features? Jump to [Develop more complex plugins](#develop-more-complex-plugins) to add agents, hooks, and MCP servers. For complete technical specifications of all plugin components, see [Plugins reference](/en/docs/claude-code/plugins-reference).

### Install and manage plugins

Learn how to discover, install, and manage plugins to extend your Claude Code capabilities.

#### Prerequisites

* Claude Code installed and running
* Basic familiarity with command-line interfaces

#### Add marketplaces

Marketplaces are catalogs of available plugins. Add them to discover and install plugins:

```shell
# Add a marketplace
/plugin marketplace add your-org/claude-plugins

# Browse available plugins
/plugin
```

For detailed marketplace management including Git repositories, local development, and team distribution, see [Plugin marketplaces](/en/docs/claude-code/plugin-marketplaces).

#### Install plugins

**Via interactive menu (recommended for discovery)**

```shell
# Open the plugin management interface
/plugin
```

Select "Browse Plugins" to see available options with descriptions, features, and installation options.

**Via direct commands (for quick installation)**

```shell
# Install a specific plugin
/plugin install formatter@your-org

# Enable a disabled plugin
/plugin enable plugin-name@marketplace-name

# Disable without uninstalling
/plugin disable plugin-name@marketplace-name

# Completely remove a plugin
/plugin uninstall plugin-name@marketplace-name
```

#### Verify installation

After installing a plugin:

1. **Check available commands**: Run `/help` to see new commands
2. **Test plugin features**: Try the plugin's commands and features
3. **Review plugin details**: Use `/plugin` → "Manage Plugins" to see what the plugin provides

### Set up team plugin workflows

Configure plugins at the repository level to ensure consistent tooling across your team. When team members trust your repository folder, Claude Code automatically installs specified marketplaces and plugins.

**To set up team plugins:**

1. Add marketplace and plugin configuration to your repository's `.claude/settings.json`
2. Team members trust the repository folder
3. Plugins install automatically for all team members

For complete instructions including configuration examples, marketplace setup, and rollout best practices, see [Configure team marketplaces](/en/docs/claude-code/plugin-marketplaces#how-to-configure-team-marketplaces).

### Develop more complex plugins

Once you're comfortable with basic plugins, you can create more sophisticated extensions.

#### Add Skills to your plugin

Plugins can include [Agent Skills](/en/docs/claude-code/skills) to extend Claude's capabilities. Skills are model-invoked—Claude autonomously uses them based on the task context.

To add Skills to your plugin, create a `skills/` directory at your plugin root and add Skill folders with `SKILL.md` files. Plugin Skills are automatically available when the plugin is installed.

For complete Skill authoring guidance, see [Agent Skills](/en/docs/claude-code/skills).

#### Organize complex plugins

For plugins with many components, organize your directory structure by functionality. For complete directory layouts and organization patterns, see [Plugin directory structure](/en/docs/claude-code/plugins-reference#plugin-directory-structure).

#### Test your plugins locally

When developing plugins, use a local marketplace to test changes iteratively. This workflow builds on the quickstart pattern and works for plugins of any complexity.

**Step 1: Set up your development structure**

Organize your plugin and marketplace for testing:

```bash
mkdir dev-marketplace
cd dev-marketplace
mkdir my-plugin
```

This creates:

```
dev-marketplace/
├── .claude-plugin/marketplace.json  (you'll create this)
└── my-plugin/                        (your plugin under development)
    ├── .claude-plugin/plugin.json
    ├── commands/
    ├── agents/
    └── hooks/
```

**Step 2: Create the marketplace manifest**

```bash
mkdir .claude-plugin
cat > .claude-plugin/marketplace.json << 'EOF'
{
  "name": "dev-marketplace",
  "owner": {
    "name": "Developer"
  },
  "plugins": [
    {
      "name": "my-plugin",
      "source": "./my-plugin",
      "description": "Plugin under development"
    }
  ]
}
EOF
```

**Step 3: Install and test**

```bash
# Start Claude Code from parent directory
cd ..
claude
```

```shell
# Add your development marketplace
/plugin marketplace add ./dev-marketplace

# Install your plugin
/plugin install my-plugin@dev-marketplace
```

Test your plugin components:

* Try your commands with `/command-name`
* Check that agents appear in `/agents`
* Verify hooks work as expected

**Step 4: Iterate on your plugin**

After making changes to your plugin code:

```shell
# Uninstall the current version
/plugin uninstall my-plugin@dev-marketplace

# Reinstall to test changes
/plugin install my-plugin@dev-marketplace
```

Repeat this cycle as you develop and refine your plugin.

**Note**: For multiple plugins, organize plugins in subdirectories like `./plugins/plugin-name` and update your marketplace.json accordingly. See [Plugin sources](/en/docs/claude-code/plugin-marketplaces#plugin-sources) for organization patterns.

#### Debug plugin issues

If your plugin isn't working as expected:

1. **Check the structure**: Ensure your directories are at the plugin root, not inside `.claude-plugin/`
2. **Test components individually**: Check each command, agent, and hook separately
3. **Use validation and debugging tools**: See [Debugging and development tools](/en/docs/claude-code/plugins-reference#debugging-and-development-tools) for CLI commands and troubleshooting techniques

#### Share your plugins

When your plugin is ready to share:

1. **Add documentation**: Include a README.md with installation and usage instructions
2. **Version your plugin**: Use semantic versioning in your `plugin.json`
3. **Create or use a marketplace**: Distribute through plugin marketplaces for easy installation
4. **Test with others**: Have team members test the plugin before wider distribution

**Note**: For complete technical specifications, debugging techniques, and distribution strategies, see [Plugins reference](/en/docs/claude-code/plugins-reference).

### Next steps

Now that you understand Claude Code's plugin system, here are suggested paths for different goals:

#### For plugin users

* **Discover plugins**: Browse community marketplaces for useful tools
* **Team adoption**: Set up repository-level plugins for your projects
* **Marketplace management**: Learn to manage multiple plugin sources
* **Advanced usage**: Explore plugin combinations and workflows

#### For plugin developers

* **Create your first marketplace**: [Plugin marketplaces guide](/en/docs/claude-code/plugin-marketplaces)
* **Advanced components**: Dive deeper into specific plugin components:
  * [Slash commands](/en/docs/claude-code/slash-commands) - Command development details
  * [Subagents](/en/docs/claude-code/sub-agents) - Agent configuration and capabilities
  * [Agent Skills](/en/docs/claude-code/skills) - Extend Claude's capabilities
  * [Hooks](/en/docs/claude-code/hooks) - Event handling and automation
  * [MCP](/en/docs/claude-code/mcp) - External tool integration
* **Distribution strategies**: Package and share your plugins effectively
* **Community contribution**: Consider contributing to community plugin collections

#### For team leads and administrators

* **Repository configuration**: Set up automatic plugin installation for team projects
* **Plugin governance**: Establish guidelines for plugin approval and security review
* **Marketplace maintenance**: Create and maintain organization-specific plugin catalogs
* **Training and documentation**: Help team members adopt plugin workflows effectively

### See also

* [Plugin marketplaces](/en/docs/claude-code/plugin-marketplaces) - Creating and managing plugin catalogs
* [Slash commands](/en/docs/claude-code/slash-commands) - Understanding custom commands
* [Subagents](/en/docs/claude-code/sub-agents) - Creating and using specialized agents
* [Agent Skills](/en/docs/claude-code/skills) - Extend Claude's capabilities
* [Hooks](/en/docs/claude-code/hooks) - Automating workflows with event handlers
* [MCP](/en/docs/claude-code/mcp) - Connecting to external tools and services
* [Settings](/en/docs/claude-code/settings) - Configuration options for plugins

---

## Section 5: MCP Servers - External Tool Integration

> Learn how to connect Claude Code to your tools with the Model Context Protocol.

Claude Code can connect to hundreds of external tools and data sources through the [Model Context Protocol (MCP)](https://modelcontextprotocol.io/introduction), an open-source standard for AI-tool integrations. MCP servers give Claude Code access to your tools, databases, and APIs.

### What you can do with MCP

With MCP servers connected, you can ask Claude Code to:

* **Implement features from issue trackers**: "Add the feature described in JIRA issue ENG-4521 and create a PR on GitHub."
* **Analyze monitoring data**: "Check Sentry and Statsig to check the usage of the feature described in ENG-4521."
* **Query databases**: "Find emails of 10 random users who used feature ENG-4521, based on our Postgres database."
* **Integrate designs**: "Update our standard email template based on the new Figma designs that were posted in Slack"
* **Automate workflows**: "Create Gmail drafts inviting these 10 users to a feedback session about the new feature."

### Popular MCP servers

Here are some commonly used MCP servers you can connect to Claude Code:

**Warning**: Use third party MCP servers at your own risk - Anthropic has not verified the correctness or security of all these servers. Make sure you trust MCP servers you are installing. Be especially careful when using MCP servers that could fetch untrusted content, as these can expose you to prompt injection risk.

*Note: Due to length constraints, only a representative sample of MCP servers is shown below. For the complete list including all Development Tools, Project Management, Databases, Payments, Design, Infrastructure, and Automation categories, see the full documentation at code.claude.com/mcp.*

**Development & Testing Tools:**

* **Sentry**: Monitor errors, debug production issues
* **Socket**: Security analysis for dependencies
* **Jam**: Debug with recordings, console logs, network requests
* **Hugging Face**: Access Hub information and Gradio apps

**Project Management & Documentation:**

* **Asana**: Interact with workspace, track projects
* **Linear**: Issue tracking and project management
* **Notion**: Read docs, update pages, manage tasks
* **Atlassian**: Manage Jira tickets and Confluence docs

**Find hundreds more**: [MCP servers on GitHub](https://github.com/modelcontextprotocol/servers), or build your own using the [MCP SDK](https://modelcontextprotocol.io/quickstart/server).

### Installing MCP servers

MCP servers can be configured in three different ways depending on your needs:

#### Option 1: Add a remote HTTP server

HTTP servers are the recommended option for connecting to remote MCP servers. This is the most widely supported transport for cloud-based services.

```bash
# Basic syntax
claude mcp add --transport http <name> <url>

# Real example: Connect to Notion
claude mcp add --transport http notion https://mcp.notion.com/mcp

# Example with Bearer token
claude mcp add --transport http secure-api https://api.example.com/mcp \
  --header "Authorization: Bearer your-token"
```

#### Option 2: Add a remote SSE server

**Warning**: The SSE (Server-Sent Events) transport is deprecated. Use HTTP servers instead, where available.

```bash
# Basic syntax
claude mcp add --transport sse <name> <url>

# Real example: Connect to Asana
claude mcp add --transport sse asana https://mcp.asana.com/sse

# Example with authentication header
claude mcp add --transport sse private-api https://api.company.com/sse \
  --header "X-API-Key: your-key-here"
```

#### Option 3: Add a local stdio server

Stdio servers run as local processes on your machine. They're ideal for tools that need direct system access or custom scripts.

```bash
# Basic syntax
claude mcp add --transport stdio <name> <command> [args...]

# Real example: Add Airtable server
claude mcp add --transport stdio airtable --env AIRTABLE_API_KEY=YOUR_KEY \
  -- npx -y airtable-mcp-server
```

**Note**: Understanding the "--" parameter - The `--` (double dash) separates Claude's own CLI flags from the command and arguments that get passed to the MCP server. Everything before `--` are options for Claude (like `--env`, `--scope`), and everything after `--` is the actual command to run the MCP server.

For example:

* `claude mcp add --transport stdio myserver -- npx server` → runs `npx server`
* `claude mcp add --transport stdio myserver --env KEY=value -- python server.py --port 8080` → runs `python server.py --port 8080` with `KEY=value` in environment

This prevents conflicts between Claude's flags and the server's flags.

#### Managing your servers

Once configured, you can manage your MCP servers with these commands:

```bash
# List all configured servers
claude mcp list

# Get details for a specific server
claude mcp get github

# Remove a server
claude mcp remove github

# (within Claude Code) Check server status
/mcp
```

**Tips:**

* Use the `--scope` flag to specify where the configuration is stored:
  * `local` (default): Available only to you in the current project
  * `project`: Shared with everyone in the project via `.mcp.json` file
  * `user`: Available to you across all projects
* Set environment variables with `--env` flags (e.g., `--env KEY=value`)
* Configure MCP server startup timeout using the MCP_TIMEOUT environment variable (e.g., `MCP_TIMEOUT=10000 claude` sets a 10-second timeout)
* Claude Code will display a warning when MCP tool output exceeds 10,000 tokens. To increase this limit, set the `MAX_MCP_OUTPUT_TOKENS` environment variable (e.g., `MAX_MCP_OUTPUT_TOKENS=50000`)
* Use `/mcp` to authenticate with remote servers that require OAuth 2.0 authentication

#### OAuth Authentication (December 2025)

Remote MCP servers often require OAuth 2.0 authentication. Use the `/mcp` command:

```bash
# In Claude Code terminal
> /mcp
# Select "Authenticate" for the server
# Follow the browser prompts to login
```

**OAuth Features:**
- Tokens stored securely and refreshed automatically
- Use "Clear authentication" in `/mcp` menu to revoke access
- If browser doesn't open, copy the provided URL manually
- Works with HTTP servers only

**Practical Example - Sentry Integration:**
```bash
# 1. Add the Sentry MCP server
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp

# 2. Authenticate within Claude Code
> /mcp
# Select "Authenticate" for Sentry

# 3. Use authenticated queries
> "What are the most common errors in the last 24 hours?"
```

#### MCP Scope Terminology (Updated December 2025)

| Scope | Description | Storage |
|-------|-------------|---------|
| `local` | Personal only, current project | `~/.claude.json` under project path |
| `project` | Team-shared via version control | `.mcp.json` in project root |
| `user` | Available across all projects | `~/.claude.json` |

**Scope Precedence:** local > project > user (personal configs override shared)

```bash
# Explicitly specify scope
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
claude mcp add --transport http paypal --scope project https://mcp.paypal.com/mcp
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

#### Environment Variable Expansion in .mcp.json

Share team configurations with dynamic values:

```json
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

**Syntax:**
- `${VAR}` - Expands to environment variable value
- `${VAR:-default}` - Uses `VAR` if set, otherwise `default`

**Warning - Windows Users**: On native Windows (not WSL), local MCP servers that use `npx` require the `cmd /c` wrapper to ensure proper execution.

```bash
# This creates command="cmd" which Windows can execute
claude mcp add --transport stdio my-server -- cmd /c npx -y @some/package
```

Without the `cmd /c` wrapper, you'll encounter "Connection closed" errors because Windows cannot directly execute `npx`.

#### Plugin-provided MCP servers

[Plugins](/en/docs/claude-code/plugins) can bundle MCP servers, automatically providing tools and integrations when the plugin is enabled. Plugin MCP servers work identically to user-configured servers.

**How plugin MCP servers work**:

* Plugins define MCP servers in `.mcp.json` at the plugin root or inline in `plugin.json`
* When a plugin is enabled, its MCP servers start automatically
* Plugin MCP tools appear alongside manually configured MCP tools
* Plugin servers are managed through plugin installation (not `/mcp` commands)

**Example plugin MCP configuration**:

In `.mcp.json` at plugin root:

```json
{
  "database-tools": {
    "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
    "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
    "env": {
      "DB_URL": "${DB_URL}"
    }
  }
}
```

Or inline in `plugin.json`:

```json
{
  "name": "my-plugin",
  "mcpServers": {
    "plugin-api": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/api-server",
      "args": ["--port", "8080"]
    }
  }
}
```

**Plugin MCP features**:

* **Automatic lifecycle**: Servers start when plugin enables, but you must restart Claude Code to apply MCP server changes
* **Environment variables**: Use `${CLAUDE_PLUGIN_ROOT}` for plugin-relative paths
* **User environment access**: Access to same environment variables as manually configured servers
* **Multiple transport types**: Support stdio, SSE, and HTTP transports

**Viewing plugin MCP servers**:

```bash
# Within Claude Code, see all MCP servers including plugin ones
/mcp
```

Plugin servers appear in the list with indicators showing they come from plugins.

**Benefits of plugin MCP servers**:

* **Bundled distribution**: Tools and servers packaged together
* **Automatic setup**: No manual MCP configuration needed
* **Team consistency**: Everyone gets the same tools when plugin is installed

See the [plugin components reference](/en/docs/claude-code/plugins-reference#mcp-servers) for details on bundling MCP servers with plugins.

### Environment Variable Expansion in .mcp.json (2025)

Claude Code supports variable expansion in `.mcp.json` files:

```json
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

**Syntax:**
- `${VAR}` - Expands to environment variable value
- `${VAR:-default}` - Uses default value if variable not set

### MCP installation scopes

MCP servers can be configured at three different scope levels, each serving distinct purposes for managing server accessibility and sharing.

#### Local scope

Local-scoped servers represent the default configuration level and are stored in your project-specific user settings. These servers remain private to you and are only accessible when working within the current project directory.

```bash
# Add a local-scoped server (default)
claude mcp add --transport http stripe https://mcp.stripe.com

# Explicitly specify local scope
claude mcp add --transport http stripe --scope local https://mcp.stripe.com
```

#### Project scope

Project-scoped servers enable team collaboration by storing configurations in a `.mcp.json` file at your project's root directory. This file is designed to be checked into version control, ensuring all team members have access to the same MCP tools and services.

```bash
# Add a project-scoped server
claude mcp add --transport http paypal --scope project https://mcp.paypal.com/mcp
```

The resulting `.mcp.json` file follows a standardized format:

```json
{
  "mcpServers": {
    "shared-server": {
      "command": "/path/to/server",
      "args": [],
      "env": {}
    }
  }
}
```

For security reasons, Claude Code prompts for approval before using project-scoped servers from `.mcp.json` files.

#### User scope

User-scoped servers provide cross-project accessibility, making them available across all projects on your machine while remaining private to your user account.

```bash
# Add a user server
claude mcp add --transport http hubspot --scope user https://mcp.hubspot.com/anthropic
```

#### Choosing the right scope

* **Local scope**: Personal servers, experimental configurations, or sensitive credentials specific to one project
* **Project scope**: Team-shared servers, project-specific tools, or services required for collaboration
* **User scope**: Personal utilities needed across multiple projects, development tools, or frequently-used services

#### Environment variable expansion

Claude Code supports environment variable expansion in `.mcp.json` files:

**Supported syntax:**

* `${VAR}` - Expands to the value of environment variable `VAR`
* `${VAR:-default}` - Expands to `VAR` if set, otherwise uses `default`

**Expansion locations:**

* `command` - The server executable path
* `args` - Command-line arguments
* `env` - Environment variables passed to the server
* `url` - For HTTP server types
* `headers` - For HTTP server authentication

**Example with variable expansion:**

```json
{
  "mcpServers": {
    "api-server": {
      "type": "http",
      "url": "${API_BASE_URL:-https://api.example.com}/mcp",
      "headers": {
        "Authorization": "Bearer ${API_KEY}"
      }
    }
  }
}
```

### Practical examples

#### Example: Monitor errors with Sentry

```bash
# 1. Add the Sentry MCP server
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp

# 2. Use /mcp to authenticate with your Sentry account
> /mcp

# 3. Debug production issues
> "What are the most common errors in the last 24 hours?"
> "Show me the stack trace for error ID abc123"
> "Which deployment introduced these new errors?"
```

#### Example: Connect to GitHub for code reviews

```bash
# 1. Add the GitHub MCP server
claude mcp add --transport http github https://api.githubcopilot.com/mcp/

# 2. In Claude Code, authenticate if needed
> /mcp
# Select "Authenticate" for GitHub

# 3. Now you can ask Claude to work with GitHub
> "Review PR #456 and suggest improvements"
> "Create a new issue for the bug we just found"
> "Show me all open PRs assigned to me"
```

#### Example: Query your PostgreSQL database

```bash
# 1. Add the database server with your connection string
claude mcp add --transport stdio db -- npx -y @bytebase/dbhub \
  --dsn "postgresql://readonly:pass@prod.db.com:5432/analytics"

# 2. Query your database naturally
> "What's our total revenue this month?"
> "Show me the schema for the orders table"
> "Find customers who haven't made a purchase in 90 days"
```

### Authenticate with remote MCP servers

Many cloud-based MCP servers require authentication. Claude Code supports OAuth 2.0 for secure connections.

**Step 1: Add the server that requires authentication**

```bash
claude mcp add --transport http sentry https://mcp.sentry.dev/mcp
```

**Step 2: Use the /mcp command within Claude Code**

```
> /mcp
```

Then follow the steps in your browser to login.

**Tips:**

* Authentication tokens are stored securely and refreshed automatically
* Use "Clear authentication" in the `/mcp` menu to revoke access
* If your browser doesn't open automatically, copy the provided URL
* OAuth authentication works with HTTP servers

### Additional MCP operations

**Add MCP servers from JSON configuration:**

```bash
# Add an HTTP server
claude mcp add-json weather-api '{"type":"http","url":"https://api.weather.com/mcp"}'

# Add a stdio server
claude mcp add-json local-weather '{"type":"stdio","command":"/path/to/weather-cli","args":["--api-key","abc123"]}'
```

**Import MCP servers from Claude Desktop:**

```bash
# Import servers (macOS and WSL only)
claude mcp add-from-claude-desktop

# Select which servers to import in the interactive dialog
```

**Use Claude Code as an MCP server:**

```bash
# Start Claude as a stdio MCP server
claude mcp serve
```

Add to Claude Desktop:

```json
{
  "mcpServers": {
    "claude-code": {
      "type": "stdio",
      "command": "claude",
      "args": ["mcp", "serve"],
      "env": {}
    }
  }
}
```

### MCP output limits

Claude Code displays a warning when MCP tool output exceeds 10,000 tokens. You can adjust the maximum:

```bash
export MAX_MCP_OUTPUT_TOKENS=50000
claude
```

This is useful for tools that query large datasets, generate detailed reports, or process extensive logs.

### Use MCP resources

MCP servers can expose resources that you can reference using @ mentions:

**List available resources:**

Type `@` in your prompt to see resources from all connected MCP servers.

**Reference a specific resource:**

```
> Can you analyze @github:issue://123 and suggest a fix?
> Please review @docs:file://api/authentication
> Compare @postgres:schema://users with @docs:file://database/user-model
```

**Tips:**

* Resources are automatically fetched as attachments
* Resource paths are fuzzy-searchable
* Resources can contain text, JSON, or structured data

### Use MCP prompts as slash commands

MCP servers can expose prompts that become available as slash commands.

**Discover available prompts:**

Type `/` to see all commands, including MCP prompts with format `/mcp__servername__promptname`.

**Execute prompts:**

```
# Without arguments
> /mcp__github__list_prs

# With arguments
> /mcp__github__pr_review 456
> /mcp__jira__create_issue "Bug in login flow" high
```

**Tips:**

* MCP prompts are dynamically discovered
* Arguments are parsed based on prompt parameters
* Server and prompt names are normalized (spaces → underscores)

### Enterprise MCP configuration

For organizations needing centralized control, Claude Code supports enterprise-managed MCP configurations through system-level files:

* **macOS**: `/Library/Application Support/ClaudeCode/managed-mcp.json`
* **Windows**: `C:\ProgramData\ClaudeCode\managed-mcp.json`
* **Linux**: `/etc/claude-code/managed-mcp.json`

This allows IT administrators to:

* Deploy standardized approved MCP servers
* Prevent unauthorized MCP servers
* Disable MCP entirely if needed

The enterprise configuration has the highest precedence and cannot be overridden when `useEnterpriseMcpConfigOnly` is enabled.

---

## Section 6: Background Tasks & Agents

> Run long-running commands and agents in the background while continuing work.

### Overview

Claude Code supports running commands and agents asynchronously, allowing you to continue working while long-running processes execute in the background.

### Background Bash Commands

**Method 1: Keyboard Shortcut**

```
Ctrl+B  # Move any Bash command to background
        # (Tmux users: press Ctrl+B twice due to tmux prefix key)
```

**Method 2: Prompt Claude**

Ask Claude to run a command in the background:
```
> Run the test suite in the background
```

**Method 3: Bash Mode with `!` Prefix**

Run commands directly without Claude interpretation:
```bash
! npm test              # Run directly, real-time output
! webpack --watch       # Supports Ctrl+B backgrounding
! git status            # No Claude approval needed
```

### Key Characteristics

- **Unique Task IDs**: Each background task gets an ID for tracking
- **Output Buffering**: Results are captured and retrievable
- **Auto-Cleanup**: Tasks cleaned up when Claude Code exits
- **Concurrent Execution**: Run multiple tasks simultaneously

### Common Use Cases

| Task Type | Example |
|-----------|---------|
| Build tools | `webpack`, `vite`, `make` |
| Package managers | `npm`, `yarn`, `pnpm` |
| Test runners | `jest`, `pytest`, `cargo test` |
| Dev servers | `npm run dev`, `python -m http.server` |
| Long processes | `docker`, `terraform` |

### Background Agents

Use the `run_in_background` parameter when launching agents:

```
Task(
  subagent_type="code-reviewer",
  prompt="Review these files",
  run_in_background=true
)
```

Retrieve results later:
```
TaskOutput(task_id="<task-id>")
```

### Best Practices

1. **Use for long-running processes**: Dev servers, builds, test suites
2. **Monitor output**: Check results periodically with TaskOutput
3. **Clean up**: Background tasks auto-cleanup, but long sessions may accumulate
4. **Tmux awareness**: Remember to press Ctrl+B twice in tmux

---

## Section 7: Checkpoints & Rewind System

> Automatic code state tracking with instant restoration for risk-free experimentation.

### Overview

Claude Code automatically creates checkpoints before each edit, providing a safety net that lets you pursue ambitious changes knowing you can always restore to previous states.

### How Checkpointing Works

**Automatic Tracking:**
- Every user prompt creates a new checkpoint
- All changes made by Claude's file editing tools are captured
- Checkpoints persist across sessions (30-day retention by default)
- Auto-cleanup removes old checkpoints with session cleanup

### Accessing Checkpoints

**Method 1: Keyboard Shortcut (Recommended)**

```
Esc + Esc  → Opens rewind menu directly
```

**Method 2: Slash Command**

```
/rewind
```

### Rewind Options

Once you open the rewind menu, choose what to restore:

| Option | Effect |
|--------|--------|
| **Conversation only** | Rewind to a user message, keep code changes |
| **Code only** | Revert file changes, keep conversation |
| **Both** | Restore code AND conversation to prior point |

### Common Use Cases

1. **Exploring alternatives**: Try different implementations without losing your starting point
2. **Recovering from mistakes**: Quickly undo changes that broke functionality
3. **Iterating on features**: Experiment with variations, revert to working states
4. **Risk-free refactoring**: Make bold changes with confidence

### Important Limitations

**NOT Tracked:**
- Bash command changes (`rm`, `mv`, `cp`, shell scripts)
- External file modifications (manual edits, other tools)
- Changes from concurrent sessions (usually)

**Example of untrackable changes:**
```bash
rm important-file.txt     # NOT undoable via checkpoint
mv old.txt new.txt        # NOT undoable via checkpoint
```

### Relationship with Git

| Feature | Checkpoints | Git |
|---------|------------|-----|
| **Scope** | Session-level | Permanent history |
| **Purpose** | Quick local undo | Version control |
| **Coverage** | Claude edits only | All tracked files |
| **Persistence** | 30 days | Forever |

**Best Practice**: Use checkpoints for quick experimentation, Git for permanent history. They complement each other.

### Configuration

Checkpoint retention is tied to session cleanup (default: 30 days). Configure via:
```json
{
  "sessionRetentionDays": 30
}
```

---

## Section 6: New Features in Claude Code 2.1.x (2.1.12-2.1.23)

This section documents features added between Claude Code 2.1.12 and 2.1.23.

### 6.1 Task Management System (2.1.16+)

**Availability:** Claude Code 2.1.16+
**Category:** Core

New task management system with dependency tracking. Tasks can have dependencies on other tasks, enabling complex workflow orchestration.

**Key Tools:**
- `TaskCreate` - Create new tasks with subject, description, activeForm
- `TaskUpdate` - Update task status, add dependencies, or delete tasks (delete added in 2.1.20)
- `TaskList` - View all tasks with dependency information
- `TaskGet` - Get detailed task information by ID

**Task Properties:**
```
- id: Unique task identifier
- subject: Brief task title
- description: Detailed requirements
- status: pending → in_progress → completed
- activeForm: Present continuous form for spinner (e.g., "Running tests")
- blockedBy: Array of task IDs that must complete first
- blocks: Array of task IDs waiting on this task
```

**Example Usage:**
```javascript
// Create a task
TaskCreate({
  subject: "Implement user authentication",
  description: "Add JWT-based auth with login/logout",
  activeForm: "Implementing authentication"
})

// Set dependencies
TaskUpdate({
  taskId: "2",
  addBlockedBy: ["1"]  // Task 2 waits for Task 1
})

// Mark complete
TaskUpdate({
  taskId: "1",
  status: "completed"
})

// Delete a task (2.1.20+)
TaskUpdate({
  taskId: "3",
  status: "deleted"
})
```

**Environment Variable:**
- `CLAUDE_CODE_ENABLE_TASKS=false` - Temporarily disable new task system (2.1.19+)

**Reference:** https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md#2116

---

### 6.2 Customizable Keyboard Shortcuts (2.1.18+)

**Availability:** Claude Code 2.1.18+
**Category:** Configuration

Customize keyboard shortcuts via the `/keybindings` command.

**Command:**
```bash
/keybindings
```

**Configuration File:** `~/.claude/keybindings.json`

**Documentation:** https://code.claude.com/docs/en/keybindings

**Key Points:**
- Remap any default shortcuts
- Add chord bindings (multi-key sequences)
- Configure per-project or globally
- Export/import keybinding configurations

---

### 6.3 History-Based Bash Autocomplete (2.1.14+)

**Availability:** Claude Code 2.1.14+
**Category:** Input

In bash mode (`!`), type a partial command and press Tab to autocomplete from your bash history.

**Usage:**
```bash
# In bash mode:
!git pu<Tab>  # Completes to previous git push command
!npm ru<Tab>  # Completes to previous npm run command
```

**Key Points:**
- Works only in bash mode (prefix with `!`)
- Searches through command history
- Tab key triggers autocomplete
- Shell completion cache stored locally

---

### 6.4 PR Review Status Indicator (2.1.20+)

**Availability:** Claude Code 2.1.20+
**Category:** UI

The prompt footer now shows the PR state for your current branch.

**Status Colors:**
- Purple: Merged PR
- Other colors indicate open, draft, or closed states

**Key Points:**
- Automatically detects current branch
- Shows PR status without running git commands
- Helps track PR workflow progress

---

### 6.5 Additional CLAUDE.md Directories (2.1.20+)

**Availability:** Claude Code 2.1.20+
**Category:** Configuration

Load `CLAUDE.md` files from additional directories using the `--add-dir` flag.

**Environment Variable (Required):**
```bash
export CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1
```

**Usage:**
```bash
claude --add-dir /path/to/shared/project
claude --add-dir ../common-configs --add-dir ../team-standards
```

**Key Points:**
- Must enable environment variable first
- Supports multiple `--add-dir` flags
- Merges CLAUDE.md content from all directories
- Useful for monorepos and shared team configurations

---

### 6.6 Customizable Spinner Verbs (2.1.23+)

**Availability:** Claude Code 2.1.23+
**Category:** Configuration

Customize the spinner text shown during operations.

**Setting:** `spinnerVerbs`

**Configuration:**
```json
{
  "spinnerVerbs": ["Processing", "Working on", "Handling"]
}
```

---

### 6.7 Breaking Changes in 2.1.x

#### Indexed Argument Syntax (2.1.19)

**Previous (deprecated):**
```
$ARGUMENTS.0
$ARGUMENTS.1
```

**Current (2.1.19+):**
```
$ARGUMENTS[0]
$ARGUMENTS[1]

# Or shorthand:
$0, $1, $2, etc.
```

**Migration:** Update custom commands using old `.N` syntax to `[N]` or shorthand.

#### npm Installation Deprecated (2.1.15)

**Previous:**
```bash
npm install -g @anthropic/claude-code
```

**Current:**
```bash
claude install
# Or see: https://docs.anthropic.com/en/docs/claude-code/getting-started
```

---

## Cross-Reference Guide

This section provides navigation between related topics across the five core features.

### Feature Comparison

| Feature | Invocation | Scope | Use Case | File Location |
|---------|-----------|-------|----------|---------------|
| **Subagents** | Model-invoked or explicit | Task-specific context | Specialized AI workers for domain expertise | `.claude/agents/` (project)<br>`~/.claude/agents/` (user) |
| **Skills** | Model-invoked | Capability extension | Package expertise into discoverable capabilities | `.claude/skills/` (project)<br>`~/.claude/skills/` (user) |
| **Slash Commands** | User-invoked | Quick prompts | Frequently-used instructions and workflows | `.claude/commands/` (project)<br>`~/.claude/commands/` (user) |
| **Background Tasks** | `Ctrl+B` or explicit | Async execution | Long-running processes, dev servers, parallel work | N/A (session-based) |
| **Checkpoints** | `Esc Esc` or `/rewind` | Code state | Risk-free experimentation, undo changes | N/A (auto-managed) |
| **Plugins** | Mixed (contains commands, agents, skills) | Bundled distribution | Package and distribute multiple features together | Via marketplace installation |
| **MCP Servers** | Tool integration | External systems | Connect Claude to external tools and data sources | Configured via CLI or `.mcp.json` |

### When to Use Each Feature

**Use Subagents when:**
- You need specialized AI workers with domain expertise
- Task requires separate context to preserve main conversation
- Different tool permissions needed for specific tasks
- Complex workflows benefit from focused AI personalities

**Use Skills when:**
- Claude should automatically discover capabilities based on context
- Capability requires multiple files, scripts, or templates
- Team needs standardized, detailed guidance for workflows
- Complex multi-step processes need documentation

**Use Slash Commands when:**
- You frequently invoke the same prompt
- Simple instruction fits in a single file
- You want explicit control over when it runs
- Quick reminders or templates are needed

**Use Plugins when:**
- Distributing multiple features together (commands + agents + skills)
- Team adoption requires bundled installation
- You want to share via marketplace
- Multiple components work together as a package

**Use MCP Servers when:**
- Connecting to external tools and APIs
- Accessing databases and data sources
- Integrating with third-party services
- Extending Claude's capabilities beyond the codebase

### Integration Patterns

**Plugins can bundle:**
- Custom slash commands (`commands/` directory)
- Specialized subagents (`agents/` directory)
- Agent Skills (`skills/` directory)
- MCP server configurations (`.mcp.json` or inline in `plugin.json`)
- Event hooks (`hooks/hooks.json`)

**Skills can reference:**
- Scripts and utilities (in skill directory)
- MCP tools (when available)
- Other Skills (via composition)

**Slash Commands can:**
- Execute bash commands (with `!` prefix)
- Reference files (with `@` prefix)
- Invoke Skills (via SlashCommand tool)
- Access MCP resources

**Subagents can:**
- Use MCP tools (when `tools` field omits MCP tools or explicitly includes them)
- Inherit tools from main conversation
- Be invoked by Skills or Commands
- Chain with other subagents

**MCP Servers can:**
- Provide tools accessible by all features
- Expose resources (referenced with `@` mentions)
- Offer prompts as slash commands (`/mcp__server__prompt`)
- Be bundled with plugins

### Related Documentation

**Core Documentation:**
- [CLI Reference](/en/docs/claude-code/cli-reference) - Command-line flags and options
- [Settings](/en/docs/claude-code/settings) - Configuration options
- [IAM](/en/docs/claude-code/iam) - Permissions and security
- [Hooks](/en/docs/claude-code/hooks) - Event handling and automation

**Advanced Topics:**
- [Plugin Marketplaces](/en/docs/claude-code/plugin-marketplaces) - Creating and managing catalogs
- [Plugins Reference](/en/docs/claude-code/plugins-reference) - Technical specifications
- [Agent Skills Best Practices](/en/docs/agents-and-tools/agent-skills/best-practices) - Skill authoring guidance
- [MCP SDK](https://modelcontextprotocol.io/quickstart/server) - Building custom MCP servers

**Community Resources:**
- [MCP Servers on GitHub](https://github.com/modelcontextprotocol/servers) - Hundreds of integrations
- [Agent Skills Overview](/en/docs/agents-and-tools/agent-skills/overview) - Cross-product Skills architecture
- [Anthropic Engineering Blog](https://www.anthropic.com/engineering) - Technical deep dives

---

**Document Version:** 2.0 (2025-12-20)
**Total Sections:** 7 (Subagents, Skills, Slash Commands, Plugins, MCP Servers, Background Tasks, Checkpoints)
**Source Files:** 7 official documentation files from code.claude.com
**Claude Code Version:** 2.0.74
