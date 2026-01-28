# Claude Code Terminal - Integration Patterns Reference

**Source:** Official docs from code.claude.com (updated 2025-12-09)

This reference consolidates all CI/CD integration, automation, and event-driven workflow patterns for Claude Code Terminal.

---

## Section 1: GitHub Actions Integration

> Learn about integrating Claude Code into your development workflow with Claude Code GitHub Actions

Claude Code GitHub Actions brings AI-powered automation to your GitHub workflow. With a simple `@claude` mention in any PR or issue, Claude can analyze your code, create pull requests, implement features, and fix bugs - all while following your project's standards.

<Note>
  Claude Code GitHub Actions is built on top of the [Claude Code
  SDK](/en/docs/claude-code/sdk), which enables programmatic integration of
  Claude Code into your applications. You can use the SDK to build custom
  automation workflows beyond GitHub Actions.
</Note>

### Why use Claude Code GitHub Actions?

* **Instant PR creation**: Describe what you need, and Claude creates a complete PR with all necessary changes
* **Automated code implementation**: Turn issues into working code with a single command
* **Follows your standards**: Claude respects your `CLAUDE.md` guidelines and existing code patterns
* **Simple setup**: Get started in minutes with our installer and API key
* **Secure by default**: Your code stays on Github's runners

### What can Claude do?

Claude Code provides a powerful GitHub Action that transforms how you work with code:

#### Claude Code Action

This GitHub Action allows you to run Claude Code within your GitHub Actions workflows. You can use this to build any custom workflow on top of Claude Code.

[View repository →](https://github.com/anthropics/claude-code-action)

### Setup

#### Quick setup

The easiest way to set up this action is through Claude Code in the terminal. Just open claude and run `/install-github-app`.

This command will guide you through setting up the GitHub app and required secrets.

<Note>
  * You must be a repository admin to install the GitHub app and add secrets
  * The GitHub app will request read & write permissions for Contents, Issues, and Pull requests
  * This quickstart method is only available for direct Claude API users. If
    you're using AWS Bedrock or Google Vertex AI, please see the [Using with AWS
    Bedrock & Google Vertex AI](#using-with-aws-bedrock-%26-google-vertex-ai)
    section.
</Note>

#### Manual setup

If the `/install-github-app` command fails or you prefer manual setup, please follow these manual setup instructions:

1. **Install the Claude GitHub app** to your repository: [https://github.com/apps/claude](https://github.com/apps/claude)

   The Claude GitHub app requires the following repository permissions:

   * **Contents**: Read & write (to modify repository files)
   * **Issues**: Read & write (to respond to issues)
   * **Pull requests**: Read & write (to create PRs and push changes)

   For more details on security and permissions, see the [security documentation](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).
2. **Add ANTHROPIC\_API\_KEY** to your repository secrets ([Learn how to use secrets in GitHub Actions](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions))
3. **Copy the workflow file** from [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) into your repository's `.github/workflows/`

<Tip>
  After completing either the quickstart or manual setup, test the action by
  tagging `@claude` in an issue or PR comment!
</Tip>

#### Upgrading from Beta

<Warning>
  Claude Code GitHub Actions v1.0 introduces breaking changes that require updating your workflow files in order to upgrade to v1.0 from the beta version.
</Warning>

If you're currently using the beta version of Claude Code GitHub Actions, we recommend that you update your workflows to use the GA version. The new version simplifies configuration while adding powerful new features like automatic mode detection.

##### Essential changes

All beta users must make these changes to their workflow files in order to upgrade:

1. **Update the action version**: Change `@beta` to `@v1`
2. **Remove mode configuration**: Delete `mode: "tag"` or `mode: "agent"` (now auto-detected)
3. **Update prompt inputs**: Replace `direct_prompt` with `prompt`
4. **Move CLI options**: Convert `max_turns`, `model`, `custom_instructions`, etc. to `claude_args`

##### Breaking Changes Reference

| Old Beta Input        | New v1.0 Input                   |
| --------------------- | -------------------------------- |
| `mode`                | *(Removed - auto-detected)*      |
| `direct_prompt`       | `prompt`                         |
| `override_prompt`     | `prompt` with GitHub variables   |
| `custom_instructions` | `claude_args: --system-prompt`   |
| `max_turns`           | `claude_args: --max-turns`       |
| `model`               | `claude_args: --model`           |
| `allowed_tools`       | `claude_args: --allowedTools`    |
| `disallowed_tools`    | `claude_args: --disallowedTools` |
| `claude_env`          | `settings` JSON format           |

##### Before and After Example

**Beta version:**

```yaml
- uses: anthropics/claude-code-action@beta
  with:
    mode: "tag"
    direct_prompt: "Review this PR for security issues"
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    custom_instructions: "Follow our coding standards"
    max_turns: "10"
    model: "sonnet"
```

**GA version (v1.0):**

```yaml
- uses: anthropics/claude-code-action@v1
  with:
    prompt: "Review this PR for security issues"
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    claude_args: |
      --system-prompt "Follow our coding standards"
      --max-turns 10
      --model claude-model: opus-4-5-20251001
```

<Tip>
  The action now automatically detects whether to run in interactive mode (responds to `@claude` mentions) or automation mode (runs immediately with a prompt) based on your configuration.
</Tip>

### Example use cases

Claude Code GitHub Actions can help you with a variety of tasks. The [examples directory](https://github.com/anthropics/claude-code-action/tree/main/examples) contains ready-to-use workflows for different scenarios.

#### Basic workflow

```yaml
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          # Responds to @claude mentions in comments
```

#### Using slash commands

```yaml
name: Code Review
on:
  pull_request:
    types: [opened, synchronize]
jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "/review"
          claude_args: "--max-turns 5"
```

#### Custom automation with prompts

```yaml
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: "--model claude-opus-4-1-20250805"
```

#### Common use cases

In issue or PR comments:

```
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude will automatically analyze the context and respond appropriately.

### Best practices

#### CLAUDE.md configuration

Create a `CLAUDE.md` file in your repository root to define code style guidelines, review criteria, project-specific rules, and preferred patterns. This file guides Claude's understanding of your project standards.

#### Security considerations

<Warning>Never commit API keys directly to your repository!</Warning>

For comprehensive security guidance including permissions, authentication, and best practices, see the [Claude Code Action security documentation](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md).

Always use GitHub Secrets for API keys:

* Add your API key as a repository secret named `ANTHROPIC_API_KEY`
* Reference it in workflows: `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`
* Limit action permissions to only what's necessary
* Review Claude's suggestions before merging

Always use GitHub Secrets (e.g., `${{ secrets.ANTHROPIC_API_KEY }}`) rather than hardcoding API keys directly in your workflow files.

#### Optimizing performance

Use issue templates to provide context, keep your `CLAUDE.md` concise and focused, and configure appropriate timeouts for your workflows.

#### CI costs

When using Claude Code GitHub Actions, be aware of the associated costs:

**GitHub Actions costs:**

* Claude Code runs on GitHub-hosted runners, which consume your GitHub Actions minutes
* See [GitHub's billing documentation](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions) for detailed pricing and minute limits

**API costs:**

* Each Claude interaction consumes API tokens based on the length of prompts and responses
* Token usage varies by task complexity and codebase size
* See [Claude's pricing page](https://claude.com/platform/api) for current token rates

**Cost optimization tips:**

* Use specific `@claude` commands to reduce unnecessary API calls
* Configure appropriate `--max-turns` in `claude_args` to prevent excessive iterations
* Set workflow-level timeouts to avoid runaway jobs
* Consider using GitHub's concurrency controls to limit parallel runs

### Configuration examples

The Claude Code Action v1 simplifies configuration with unified parameters:

```yaml
- uses: anthropics/claude-code-action@v1
  with:
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    prompt: "Your instructions here" # Optional
    claude_args: "--max-turns 5" # Optional CLI arguments
```

Key features:

* **Unified prompt interface** - Use `prompt` for all instructions
* **Slash commands** - Pre-built prompts like `/review` or `/fix`
* **CLI passthrough** - Any Claude Code CLI argument via `claude_args`
* **Flexible triggers** - Works with any GitHub event

Visit the [examples directory](https://github.com/anthropics/claude-code-action/tree/main/examples) for complete workflow files.

<Tip>
  When responding to issue or PR comments, Claude automatically responds to @claude mentions. For other events, use the `prompt` parameter to provide instructions.
</Tip>

### Using with AWS Bedrock & Google Vertex AI

For enterprise environments, you can use Claude Code GitHub Actions with your own cloud infrastructure. This approach gives you control over data residency and billing while maintaining the same functionality.

#### Prerequisites

Before setting up Claude Code GitHub Actions with cloud providers, you need:

##### For Google Cloud Vertex AI:

1. A Google Cloud Project with Vertex AI enabled
2. Workload Identity Federation configured for GitHub Actions
3. A service account with the required permissions
4. A GitHub App (recommended) or use the default GITHUB\_TOKEN

##### For AWS Bedrock:

1. An AWS account with Amazon Bedrock enabled
2. GitHub OIDC Identity Provider configured in AWS
3. An IAM role with Bedrock permissions
4. A GitHub App (recommended) or use the default GITHUB\_TOKEN

**Step 1: Create a custom GitHub App (Recommended for 3P Providers)**

For best control and security when using 3P providers like Vertex AI or Bedrock, we recommend creating your own GitHub App:

1. Go to [https://github.com/settings/apps/new](https://github.com/settings/apps/new)
2. Fill in the basic information:
   * **GitHub App name**: Choose a unique name (e.g., "YourOrg Claude Assistant")
   * **Homepage URL**: Your organization's website or the repository URL
3. Configure the app settings:
   * **Webhooks**: Uncheck "Active" (not needed for this integration)
4. Set the required permissions:
   * **Repository permissions**:
     * Contents: Read & Write
     * Issues: Read & Write
     * Pull requests: Read & Write
5. Click "Create GitHub App"
6. After creation, click "Generate a private key" and save the downloaded `.pem` file
7. Note your App ID from the app settings page
8. Install the app to your repository:
   * From your app's settings page, click "Install App" in the left sidebar
   * Select your account or organization
   * Choose "Only select repositories" and select the specific repository
   * Click "Install"
9. Add the private key as a secret to your repository:
   * Go to your repository's Settings → Secrets and variables → Actions
   * Create a new secret named `APP_PRIVATE_KEY` with the contents of the `.pem` file
10. Add the App ID as a secret:
    * Create a new secret named `APP_ID` with your GitHub App's ID

<Note>
  This app will be used with the [actions/create-github-app-token](https://github.com/actions/create-github-app-token) action to generate authentication tokens in your workflows.
</Note>

**Alternative for Claude API or if you don't want to setup your own Github app**: Use the official Anthropic app:

1. Install from: [https://github.com/apps/claude](https://github.com/apps/claude)
2. No additional configuration needed for authentication

**Step 2: Configure cloud provider authentication**

Choose your cloud provider and set up secure authentication:

##### AWS Bedrock

**Configure AWS to allow GitHub Actions to authenticate securely without storing credentials.**

> **Security Note**: Use repository-specific configurations and grant only the minimum required permissions.

**Required Setup**:

1. **Enable Amazon Bedrock**:
   * Request access to Claude models in Amazon Bedrock
   * For cross-region models, request access in all required regions

2. **Set up GitHub OIDC Identity Provider**:
   * Provider URL: `https://token.actions.githubusercontent.com`
   * Audience: `sts.amazonaws.com`

3. **Create IAM Role for GitHub Actions**:
   * Trusted entity type: Web identity
   * Identity provider: `token.actions.githubusercontent.com`
   * Permissions: `AmazonBedrockFullAccess` policy
   * Configure trust policy for your specific repository

**Required Values**:

After setup, you'll need:

* **AWS\_ROLE\_TO\_ASSUME**: The ARN of the IAM role you created

<Tip>
  OIDC is more secure than using static AWS access keys because credentials are temporary and automatically rotated.
</Tip>

See [AWS documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html) for detailed OIDC setup instructions.

##### Google Vertex AI

**Configure Google Cloud to allow GitHub Actions to authenticate securely without storing credentials.**

> **Security Note**: Use repository-specific configurations and grant only the minimum required permissions.

**Required Setup**:

1. **Enable APIs** in your Google Cloud project:
   * IAM Credentials API
   * Security Token Service (STS) API
   * Vertex AI API

2. **Create Workload Identity Federation resources**:
   * Create a Workload Identity Pool
   * Add a GitHub OIDC provider with:
     * Issuer: `https://token.actions.githubusercontent.com`
     * Attribute mappings for repository and owner
     * **Security recommendation**: Use repository-specific attribute conditions

3. **Create a Service Account**:
   * Grant only `Vertex AI User` role
   * **Security recommendation**: Create a dedicated service account per repository

4. **Configure IAM bindings**:
   * Allow the Workload Identity Pool to impersonate the service account
   * **Security recommendation**: Use repository-specific principal sets

**Required Values**:

After setup, you'll need:

* **GCP\_WORKLOAD\_IDENTITY\_PROVIDER**: The full provider resource name
* **GCP\_SERVICE\_ACCOUNT**: The service account email address

<Tip>
  Workload Identity Federation eliminates the need for downloadable service account keys, improving security.
</Tip>

For detailed setup instructions, consult the [Google Cloud Workload Identity Federation documentation](https://cloud.google.com/iam/docs/workload-identity-federation).

**Step 3: Add Required Secrets**

Add the following secrets to your repository (Settings → Secrets and variables → Actions):

##### For Claude API (Direct):

1. **For API Authentication**:
   * `ANTHROPIC_API_KEY`: Your Claude API key from [console.anthropic.com](https://console.anthropic.com)

2. **For GitHub App (if using your own app)**:
   * `APP_ID`: Your GitHub App's ID
   * `APP_PRIVATE_KEY`: The private key (.pem) content

##### For Google Cloud Vertex AI

1. **For GCP Authentication**:
   * `GCP_WORKLOAD_IDENTITY_PROVIDER`
   * `GCP_SERVICE_ACCOUNT`

2. **For GitHub App (if using your own app)**:
   * `APP_ID`: Your GitHub App's ID
   * `APP_PRIVATE_KEY`: The private key (.pem) content

##### For AWS Bedrock

1. **For AWS Authentication**:
   * `AWS_ROLE_TO_ASSUME`

2. **For GitHub App (if using your own app)**:
   * `APP_ID`: Your GitHub App's ID
   * `APP_PRIVATE_KEY`: The private key (.pem) content

**Step 4: Create workflow files**

Create GitHub Actions workflow files that integrate with your cloud provider. The examples below show complete configurations for both AWS Bedrock and Google Vertex AI:

##### AWS Bedrock workflow

**Prerequisites:**

* AWS Bedrock access enabled with Claude model permissions
* GitHub configured as an OIDC identity provider in AWS
* IAM role with Bedrock permissions that trusts GitHub Actions

**Required GitHub secrets:**

| Secret Name          | Description                                       |
| -------------------- | ------------------------------------------------- |
| `AWS_ROLE_TO_ASSUME` | ARN of the IAM role for Bedrock access            |
| `APP_ID`             | Your GitHub App ID (from app settings)            |
| `APP_PRIVATE_KEY`    | The private key you generated for your GitHub App |

```yaml
name: Claude PR Action

permissions:
  contents: write
  pull-requests: write
  issues: write
  id-token: write

on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
  issues:
    types: [opened, assigned]

jobs:
  claude-pr:
    if: |
      (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
      (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
      (github.event_name == 'issues' && contains(github.event.issue.body, '@claude'))
    runs-on: ubuntu-latest
    env:
      AWS_REGION: us-west-2
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Generate GitHub App token
        id: app-token
        uses: actions/create-github-app-token@v2
        with:
          app-id: ${{ secrets.APP_ID }}
          private-key: ${{ secrets.APP_PRIVATE_KEY }}

      - name: Configure AWS Credentials (OIDC)
        uses: aws-actions/configure-aws-credentials@v4
        with:
          role-to-assume: ${{ secrets.AWS_ROLE_TO_ASSUME }}
          aws-region: us-west-2

      - uses: anthropics/claude-code-action@v1
        with:
          github_token: ${{ steps.app-token.outputs.token }}
          use_bedrock: "true"
          claude_args: '--model us.anthropic.claude-model: opus-4-5-20251001-v1:0 --max-turns 10'
```

<Tip>
  The model ID format for Bedrock includes the region prefix (e.g., `us.anthropic.claude...`) and version suffix.
</Tip>

##### Google Vertex AI workflow

**Prerequisites:**

* Vertex AI API enabled in your GCP project
* Workload Identity Federation configured for GitHub
* Service account with Vertex AI permissions

**Required GitHub secrets:**

| Secret Name                      | Description                                       |
| -------------------------------- | ------------------------------------------------- |
| `GCP_WORKLOAD_IDENTITY_PROVIDER` | Workload identity provider resource name          |
| `GCP_SERVICE_ACCOUNT`            | Service account email with Vertex AI access       |
| `APP_ID`                         | Your GitHub App ID (from app settings)            |
| `APP_PRIVATE_KEY`                | The private key you generated for your GitHub App |

```yaml
name: Claude PR Action

permissions:
  contents: write
  pull-requests: write
  issues: write
  id-token: write

on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
  issues:
    types: [opened, assigned]

jobs:
  claude-pr:
    if: |
      (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
      (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
      (github.event_name == 'issues' && contains(github.event.issue.body, '@claude'))
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Generate GitHub App token
        id: app-token
        uses: actions/create-github-app-token@v2
        with:
          app-id: ${{ secrets.APP_ID }}
          private-key: ${{ secrets.APP_PRIVATE_KEY }}

      - name: Authenticate to Google Cloud
        id: auth
        uses: google-github-actions/auth@v2
        with:
          workload_identity_provider: ${{ secrets.GCP_WORKLOAD_IDENTITY_PROVIDER }}
          service_account: ${{ secrets.GCP_SERVICE_ACCOUNT }}

      - uses: anthropics/claude-code-action@v1
        with:
          github_token: ${{ steps.app-token.outputs.token }}
          trigger_phrase: "@claude"
          use_vertex: "true"
          claude_args: '--model claude-sonnet-4@20250514 --max-turns 10'
        env:
          ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
          CLOUD_ML_REGION: us-east5
          VERTEX_REGION_CLAUDE_3_7_SONNET: us-east5
```

<Tip>
  The project ID is automatically retrieved from the Google Cloud authentication step, so you don't need to hardcode it.
</Tip>

### Troubleshooting

#### Claude not responding to @claude commands

Verify the GitHub App is installed correctly, check that workflows are enabled, ensure API key is set in repository secrets, and confirm the comment contains `@claude` (not `/claude`).

#### CI not running on Claude's commits

Ensure you're using the GitHub App or custom app (not Actions user), check workflow triggers include the necessary events, and verify app permissions include CI triggers.

#### Authentication errors

Confirm API key is valid and has sufficient permissions. For Bedrock/Vertex, check credentials configuration and ensure secrets are named correctly in workflows.

### Advanced configuration

#### Action parameters

The Claude Code Action v1 uses a simplified configuration:

| Parameter           | Description                                     | Required |
| ------------------- | ----------------------------------------------- | -------- |
| `prompt`            | Instructions for Claude (text or slash command) | No\*     |
| `claude_args`       | CLI arguments passed to Claude Code             | No       |
| `anthropic_api_key` | Claude API key                                  | Yes\*\*  |
| `github_token`      | GitHub token for API access                     | No       |
| `trigger_phrase`    | Custom trigger phrase (default: "@claude")      | No       |
| `use_bedrock`       | Use AWS Bedrock instead of Claude API           | No       |
| `use_vertex`        | Use Google Vertex AI instead of Claude API      | No       |

\*Prompt is optional - when omitted for issue/PR comments, Claude responds to trigger phrase\
\*\*Required for direct Claude API, not for Bedrock/Vertex

##### Using claude\_args

The `claude_args` parameter accepts any Claude Code CLI arguments:

```yaml
claude_args: "--max-turns 5 --model claude-model: opus-4-5-20251001 --mcp-config /path/to/config.json"
```

Common arguments:

* `--max-turns`: Maximum conversation turns (default: 10)
* `--model`: Model to use (e.g., `claude-model: opus-4-5-20251001`)
* `--mcp-config`: Path to MCP configuration
* `--allowed-tools`: Comma-separated list of allowed tools
* `--debug`: Enable debug output

#### Alternative integration methods

While the `/install-github-app` command is the recommended approach, you can also:

* **Custom GitHub App**: For organizations needing branded usernames or custom authentication flows. Create your own GitHub App with required permissions (contents, issues, pull requests) and use the actions/create-github-app-token action to generate tokens in your workflows.
* **Manual GitHub Actions**: Direct workflow configuration for maximum flexibility
* **MCP Configuration**: Dynamic loading of Model Context Protocol servers

See the [Claude Code Action documentation](https://github.com/anthropics/claude-code-action/blob/main/docs) for detailed guides on authentication, security, and advanced configuration.

#### Customizing Claude's behavior

You can configure Claude's behavior in two ways:

1. **CLAUDE.md**: Define coding standards, review criteria, and project-specific rules in a `CLAUDE.md` file at the root of your repository. Claude will follow these guidelines when creating PRs and responding to requests. Check out our [Memory documentation](/en/docs/claude-code/memory) for more details.
2. **Custom prompts**: Use the `prompt` parameter in the workflow file to provide workflow-specific instructions. This allows you to customize Claude's behavior for different workflows or tasks.

Claude will follow these guidelines when creating PRs and responding to requests.

---

## Section 2: GitLab CI/CD Integration

> Learn about integrating Claude Code into your development workflow with GitLab CI/CD

<Info>
  Claude Code for GitLab CI/CD is currently in beta. Features and functionality may evolve as we refine the experience.

  This integration is maintained by GitLab. For support, see the following [GitLab issue](https://gitlab.com/gitlab-org/gitlab/-/issues/573776).
</Info>

<Note>
  This integration is built on top of the [Claude Code CLI and SDK](/en/docs/claude-code/sdk), enabling programmatic use of Claude in your CI/CD jobs and custom automation workflows.
</Note>

### Why use Claude Code with GitLab?

* **Instant MR creation**: Describe what you need, and Claude proposes a complete MR with changes and explanation
* **Automated implementation**: Turn issues into working code with a single command or mention
* **Project-aware**: Claude follows your `CLAUDE.md` guidelines and existing code patterns
* **Simple setup**: Add one job to `.gitlab-ci.yml` and a masked CI/CD variable
* **Enterprise-ready**: Choose Claude API, AWS Bedrock, or Google Vertex AI to meet data residency and procurement needs
* **Secure by default**: Runs in your GitLab runners with your branch protection and approvals

### How it works

Claude Code uses GitLab CI/CD to run AI tasks in isolated jobs and commit results back via MRs:

1. **Event-driven orchestration**: GitLab listens for your chosen triggers (for example, a comment that mentions `@claude` in an issue, MR, or review thread). The job collects context from the thread and repository, builds prompts from that input, and runs Claude Code.

2. **Provider abstraction**: Use the provider that fits your environment:
   * Claude API (SaaS)
   * AWS Bedrock (IAM-based access, cross-region options)
   * Google Vertex AI (GCP-native, Workload Identity Federation)

3. **Sandboxed execution**: Each interaction runs in a container with strict network and filesystem rules. Claude Code enforces workspace-scoped permissions to constrain writes. Every change flows through an MR so reviewers see the diff and approvals still apply.

Pick regional endpoints to reduce latency and meet data-sovereignty requirements while using existing cloud agreements.

### What can Claude do?

Claude Code enables powerful CI/CD workflows that transform how you work with code:

* Create and update MRs from issue descriptions or comments
* Analyze performance regressions and propose optimizations
* Implement features directly in a branch, then open an MR
* Fix bugs and regressions identified by tests or comments
* Respond to follow-up comments to iterate on requested changes

### Setup

#### Quick setup

The fastest way to get started is to add a minimal job to your `.gitlab-ci.yml` and set your API key as a masked variable.

1. **Add a masked CI/CD variable**
   * Go to **Settings** → **CI/CD** → **Variables**
   * Add `ANTHROPIC_API_KEY` (masked, protected as needed)

2. **Add a Claude job to `.gitlab-ci.yml`**

```yaml
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # Adjust rules to fit how you want to trigger the job:
  # - manual runs
  # - merge request events
  # - web/API triggers when a comment contains '@claude'
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - npm install -g @anthropic-ai/claude-code
  script:
    # Optional: start a GitLab MCP server if your setup provides one
    - /bin/gitlab-mcp-server || true
    # Use AI_FLOW_* variables when invoking via web/API triggers with context payloads
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash(*) Read(*) Edit(*) Write(*) mcp__gitlab"
      --debug
```

After adding the job and your `ANTHROPIC_API_KEY` variable, test by running the job manually from **CI/CD** → **Pipelines**, or trigger it from an MR to let Claude propose updates in a branch and open an MR if needed.

<Note>
  To run on AWS Bedrock or Google Vertex AI instead of the Claude API, see the [Using with AWS Bedrock & Google Vertex AI](#using-with-aws-bedrock--google-vertex-ai-1) section below for authentication and environment setup.
</Note>

#### Manual setup (recommended for production)

If you prefer a more controlled setup or need enterprise providers:

1. **Configure provider access**:
   * **Claude API**: Create and store `ANTHROPIC_API_KEY` as a masked CI/CD variable
   * **AWS Bedrock**: **Configure GitLab** → **AWS OIDC** and create an IAM role for Bedrock
   * **Google Vertex AI**: **Configure Workload Identity Federation for GitLab** → **GCP**

2. **Add project credentials for GitLab API operations**:
   * Use `CI_JOB_TOKEN` by default, or create a Project Access Token with `api` scope
   * Store as `GITLAB_ACCESS_TOKEN` (masked) if using a PAT

3. **Add the Claude job to `.gitlab-ci.yml`** (see examples below)

4. **(Optional) Enable mention-driven triggers**:
   * Add a project webhook for "Comments (notes)" to your event listener (if you use one)
   * Have the listener call the pipeline trigger API with variables like `AI_FLOW_INPUT` and `AI_FLOW_CONTEXT` when a comment contains `@claude`

### Example use cases

#### Turn issues into MRs

In an issue comment:

```
@claude implement this feature based on the issue description
```

Claude analyzes the issue and codebase, writes changes in a branch, and opens an MR for review.

#### Get implementation help

In an MR discussion:

```
@claude suggest a concrete approach to cache the results of this API call
```

Claude proposes changes, adds code with appropriate caching, and updates the MR.

#### Fix bugs quickly

In an issue or MR comment:

```
@claude fix the TypeError in the user dashboard component
```

Claude locates the bug, implements a fix, and updates the branch or opens a new MR.

### Using with AWS Bedrock & Google Vertex AI

For enterprise environments, you can run Claude Code entirely on your cloud infrastructure with the same developer experience.

#### AWS Bedrock

##### Prerequisites

Before setting up Claude Code with AWS Bedrock, you need:

1. An AWS account with Amazon Bedrock access to the desired Claude models
2. GitLab configured as an OIDC identity provider in AWS IAM
3. An IAM role with Bedrock permissions and a trust policy restricted to your GitLab project/refs
4. GitLab CI/CD variables for role assumption:
   * `AWS_ROLE_TO_ASSUME` (role ARN)
   * `AWS_REGION` (Bedrock region)

##### Setup instructions

Configure AWS to allow GitLab CI jobs to assume an IAM role via OIDC (no static keys).

**Required setup:**

1. Enable Amazon Bedrock and request access to your target Claude models
2. Create an IAM OIDC provider for GitLab if not already present
3. Create an IAM role trusted by the GitLab OIDC provider, restricted to your project and protected refs
4. Attach least-privilege permissions for Bedrock invoke APIs

**Required values to store in CI/CD variables:**

* `AWS_ROLE_TO_ASSUME`
* `AWS_REGION`

Add variables in Settings → CI/CD → Variables:

```yaml
# For AWS Bedrock:
- AWS_ROLE_TO_ASSUME
- AWS_REGION
```

Use the AWS Bedrock job example below to exchange the GitLab job token for temporary AWS credentials at runtime.

#### Google Vertex AI

##### Prerequisites

Before setting up Claude Code with Google Vertex AI, you need:

1. A Google Cloud project with:
   * Vertex AI API enabled
   * Workload Identity Federation configured to trust GitLab OIDC
2. A dedicated service account with only the required Vertex AI roles
3. GitLab CI/CD variables for WIF:
   * `GCP_WORKLOAD_IDENTITY_PROVIDER` (full resource name)
   * `GCP_SERVICE_ACCOUNT` (service account email)

##### Setup instructions

Configure Google Cloud to allow GitLab CI jobs to impersonate a service account via Workload Identity Federation.

**Required setup:**

1. Enable IAM Credentials API, STS API, and Vertex AI API
2. Create a Workload Identity Pool and provider for GitLab OIDC
3. Create a dedicated service account with Vertex AI roles
4. Grant the WIF principal permission to impersonate the service account

**Required values to store in CI/CD variables:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`
* `GCP_SERVICE_ACCOUNT`

Add variables in Settings → CI/CD → Variables:

```yaml
# For Google Vertex AI:
- GCP_WORKLOAD_IDENTITY_PROVIDER
- GCP_SERVICE_ACCOUNT
- CLOUD_ML_REGION (for example, us-east5)
```

Use the Google Vertex AI job example below to authenticate without storing keys.

### Configuration examples

Below are ready-to-use snippets you can adapt to your pipeline.

#### Basic .gitlab-ci.yml (Claude API)

```yaml
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - npm install -g @anthropic-ai/claude-code
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Summarize recent changes and suggest improvements'}"
      --permission-mode acceptEdits
      --allowedTools "Bash(*) Read(*) Edit(*) Write(*) mcp__gitlab"
      --debug
  # Claude Code will use ANTHROPIC_API_KEY from CI/CD variables
```

#### AWS Bedrock job example (OIDC)

**Prerequisites:**

* Amazon Bedrock enabled with access to your chosen Claude model(s)
* GitLab OIDC configured in AWS with a role that trusts your GitLab project and refs
* IAM role with Bedrock permissions (least privilege recommended)

**Required CI/CD variables:**

* `AWS_ROLE_TO_ASSUME`: ARN of the IAM role for Bedrock access
* `AWS_REGION`: Bedrock region (for example, `us-west-2`)

```yaml
claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apk add --no-cache bash curl jq git python3 py3-pip
    - pip install --no-cache-dir awscli
    - npm install -g @anthropic-ai/claude-code
    # Exchange GitLab OIDC token for AWS credentials
    - export AWS_WEB_IDENTITY_TOKEN_FILE="${CI_JOB_JWT_FILE:-/tmp/oidc_token}"
    - if [ -n "${CI_JOB_JWT_V2}" ]; then printf "%s" "$CI_JOB_JWT_V2" > "$AWS_WEB_IDENTITY_TOKEN_FILE"; fi
    - >
      aws sts assume-role-with-web-identity
      --role-arn "$AWS_ROLE_TO_ASSUME"
      --role-session-name "gitlab-claude-$(date +%s)"
      --web-identity-token "file://$AWS_WEB_IDENTITY_TOKEN_FILE"
      --duration-seconds 3600 > /tmp/aws_creds.json
    - export AWS_ACCESS_KEY_ID="$(jq -r .Credentials.AccessKeyId /tmp/aws_creds.json)"
    - export AWS_SECRET_ACCESS_KEY="$(jq -r .Credentials.SecretAccessKey /tmp/aws_creds.json)"
    - export AWS_SESSION_TOKEN="$(jq -r .Credentials.SessionToken /tmp/aws_creds.json)"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Implement the requested changes and open an MR'}"
      --permission-mode acceptEdits
      --allowedTools "Bash(*) Read(*) Edit(*) Write(*) mcp__gitlab"
      --debug
  variables:
    AWS_REGION: "us-west-2"
```

<Note>
  Model IDs for Bedrock include region-specific prefixes and version suffixes (for example, `us.anthropic.claude-3-7-sonnet-20250219-v1:0`). Pass the desired model via your job configuration or prompt if your workflow supports it.
</Note>

#### Google Vertex AI job example (Workload Identity Federation)

**Prerequisites:**

* Vertex AI API enabled in your GCP project
* Workload Identity Federation configured to trust GitLab OIDC
* A service account with Vertex AI permissions

**Required CI/CD variables:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: Full provider resource name
* `GCP_SERVICE_ACCOUNT`: Service account email
* `CLOUD_ML_REGION`: Vertex region (for example, `us-east5`)

```yaml
claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apt-get update && apt-get install -y git nodejs npm && apt-get clean
    - npm install -g @anthropic-ai/claude-code
    # Authenticate to Google Cloud via WIF (no downloaded keys)
    - >
      gcloud auth login --cred-file=<(cat <<EOF
      {
        "type": "external_account",
        "audience": "${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken",
        "token_url": "https://sts.googleapis.com/v1/token"
      }
      EOF
      )
    - gcloud config set project "$(gcloud projects list --format='value(projectId)' --filter="name:${CI_PROJECT_NAMESPACE}" | head -n1)" || true
  script:
    - /bin/gitlab-mcp-server || true
    - >
      CLOUD_ML_REGION="${CLOUD_ML_REGION:-us-east5}"
      claude
      -p "${AI_FLOW_INPUT:-'Review and update code as requested'}"
      --permission-mode acceptEdits
      --allowedTools "Bash(*) Read(*) Edit(*) Write(*) mcp__gitlab"
      --debug
  variables:
    CLOUD_ML_REGION: "us-east5"
```

<Note>
  With Workload Identity Federation, you do not need to store service account keys. Use repository-specific trust conditions and least-privilege service accounts.
</Note>

### Best practices

#### CLAUDE.md configuration

Create a `CLAUDE.md` file at the repository root to define coding standards, review criteria, and project-specific rules. Claude reads this file during runs and follows your conventions when proposing changes.

#### Security considerations

Never commit API keys or cloud credentials to your repository! Always use GitLab CI/CD variables:

* Add `ANTHROPIC_API_KEY` as a masked variable (and protect it if needed)
* Use provider-specific OIDC where possible (no long-lived keys)
* Limit job permissions and network egress
* Review Claude's MRs like any other contributor

#### Optimizing performance

* Keep `CLAUDE.md` focused and concise
* Provide clear issue/MR descriptions to reduce iterations
* Configure sensible job timeouts to avoid runaway runs
* Cache npm and package installs in runners where possible

#### CI costs

When using Claude Code with GitLab CI/CD, be aware of associated costs:

* **GitLab Runner time**:
  * Claude runs on your GitLab runners and consumes compute minutes
  * See your GitLab plan's runner billing for details

* **API costs**:
  * Each Claude interaction consumes tokens based on prompt and response size
  * Token usage varies by task complexity and codebase size
  * See [Anthropic pricing](/en/docs/about-claude/pricing) for details

* **Cost optimization tips**:
  * Use specific `@claude` commands to reduce unnecessary turns
  * Set appropriate `max_turns` and job timeout values
  * Limit concurrency to control parallel runs

### Security and governance

* Each job runs in an isolated container with restricted network access
* Claude's changes flow through MRs so reviewers see every diff
* Branch protection and approval rules apply to AI-generated code
* Claude Code uses workspace-scoped permissions to constrain writes
* Costs remain under your control because you bring your own provider credentials

### Troubleshooting

#### Claude not responding to @claude commands

* Verify your pipeline is being triggered (manually, MR event, or via a note event listener/webhook)
* Ensure CI/CD variables (`ANTHROPIC_API_KEY` or cloud provider settings) are present and unmasked
* Check that the comment contains `@claude` (not `/claude`) and that your mention trigger is configured

#### Job can't write comments or open MRs

* Ensure `CI_JOB_TOKEN` has sufficient permissions for the project, or use a Project Access Token with `api` scope
* Check the `mcp__gitlab` tool is enabled in `--allowedTools`
* Confirm the job runs in the context of the MR or has enough context via `AI_FLOW_*` variables

#### Authentication errors

* **For Claude API**: Confirm `ANTHROPIC_API_KEY` is valid and unexpired
* **For Bedrock/Vertex**: Verify OIDC/WIF configuration, role impersonation, and secret names; confirm region and model availability

### Advanced configuration

#### Common parameters and variables

Claude Code supports these commonly used inputs:

* `prompt` / `prompt_file`: Provide instructions inline (`-p`) or via a file
* `max_turns`: Limit the number of back-and-forth iterations
* `timeout_minutes`: Limit total execution time
* `ANTHROPIC_API_KEY`: Required for the Claude API (not used for Bedrock/Vertex)
* Provider-specific environment: `AWS_REGION`, project/region vars for Vertex

<Note>
  Exact flags and parameters may vary by version of `@anthropic-ai/claude-code`. Run `claude --help` in your job to see supported options.
</Note>

#### Customizing Claude's behavior

You can guide Claude in two primary ways:

1. **CLAUDE.md**: Define coding standards, security requirements, and project conventions. Claude reads this during runs and follows your rules.
2. **Custom prompts**: Pass task-specific instructions via `prompt`/`prompt_file` in the job. Use different prompts for different jobs (for example, review, implement, refactor).

---

## Section 3: Hooks - Event-Driven Automation

> Learn how to customize and extend Claude Code's behavior by registering shell commands

Claude Code hooks are user-defined shell commands that execute at various points
in Claude Code's lifecycle. Hooks provide deterministic control over Claude
Code's behavior, ensuring certain actions always happen rather than relying on
the LLM to choose to run them.

<Tip>
  For reference documentation on hooks, see [Hooks reference](#hooks-reference-documentation) below.
</Tip>

Example use cases for hooks include:

* **Notifications**: Customize how you get notified when Claude Code is awaiting
  your input or permission to run something.
* **Automatic formatting**: Run `prettier` on .ts files, `gofmt` on .go files,
  etc. after every file edit.
* **Logging**: Track and count all executed commands for compliance or
  debugging.
* **Feedback**: Provide automated feedback when Claude Code produces code that
  does not follow your codebase conventions.
* **Custom permissions**: Block modifications to production files or sensitive
  directories.

By encoding these rules as hooks rather than prompting instructions, you turn
suggestions into app-level code that executes every time it is expected to run.

<Warning>
  You must consider the security implication of hooks as you add them, because hooks run automatically during the agent loop with your current environment's credentials.
  For example, malicious hooks code can exfiltrate your data. Always review your hooks implementation before registering them.

  For full security best practices, see [Security Considerations](#security-considerations) in the hooks reference documentation.
</Warning>

### Hook Events Overview

Claude Code provides several hook events that run at different points in the
workflow:

* **PreToolUse**: Runs before tool calls (can block them)
* **PostToolUse**: Runs after tool calls complete
* **UserPromptSubmit**: Runs when the user submits a prompt, before Claude processes it
* **Notification**: Runs when Claude Code sends notifications
* **Stop**: Runs when Claude Code finishes responding
* **SubagentStop**: Runs when subagent tasks complete
* **PreCompact**: Runs before Claude Code is about to run a compact operation
* **SessionStart**: Runs when Claude Code starts a new session or resumes an existing session
* **SessionEnd**: Runs when Claude Code session ends
* **PermissionRequest**: Runs when user is shown a permission dialog (2025)

Each event receives different data and can control Claude's behavior in
different ways.

### New Hook Events (2025)

#### PermissionRequest

Runs when user is shown a permission dialog. Allows hooks to automatically approve or deny permission requests.

**Use cases:**
- Auto-approve specific operations (e.g., git status)
- Auto-deny dangerous operations
- Log permission requests for auditing

**Input format:**
```json
{
  "tool_name": "Bash",
  "tool_input": {
    "command": "git status",
    "description": "Check git status"
  }
}
```

**Supports same matchers as PreToolUse** - match on tool name, command patterns, etc.

#### MCP Tool Naming in Hooks

MCP tools follow the pattern: `mcp__<server>__<tool>`

```json
{
  "hooks": {
    "PreToolUse": [{
      "matcher": "mcp__memory__.*",
      "hooks": [{"type": "command", "command": "echo 'Memory operation' >> ~/mcp-ops.log"}]
    }]
  }
}
```

#### Prompt-Based Hooks

For `Stop` and `SubagentStop` hooks, you can use LLM evaluation instead of shell commands:

```json
{
  "hooks": {
    "Stop": [{
      "hooks": [{
        "type": "prompt",
        "prompt": "Analyze if Claude should stop. Input: $ARGUMENTS\n\nCheck if:\n1. All tasks complete\n2. Any errors need fixing\n\nRespond: {\"decision\": \"approve\" or \"block\", \"reason\": \"explanation\"}",
        "timeout": 30
      }]
    }]
  }
}
```

**Response schema for prompt hooks:**
```json
{
  "decision": "approve",  // or "block"
  "reason": "All tasks completed successfully",
  "continue": false,
  "stopReason": "Message shown to user (optional)",
  "systemMessage": "Warning or context (optional)"
}
```

### Quickstart

In this quickstart, you'll add a hook that logs the shell commands that Claude
Code runs.

#### Prerequisites

Install `jq` for JSON processing in the command line.

#### Step 1: Open hooks configuration

Run the `/hooks` [slash command](/en/docs/claude-code/slash-commands) and select
the `PreToolUse` hook event.

`PreToolUse` hooks run before tool calls and can block them while providing
Claude feedback on what to do differently.

#### Step 2: Add a matcher

Select `+ Add new matcher…` to run your hook only on Bash tool calls.

Type `Bash` for the matcher.

<Note>You can use `*` to match all tools.</Note>

#### Step 3: Add the hook

Select `+ Add new hook…` and enter this command:

```bash
jq -r '"\(.tool_input.command) - \(.tool_input.description // "No description")"' >> ~/.claude/bash-command-log.txt
```

#### Step 4: Save your configuration

For storage location, select `User settings` since you're logging to your home
directory. This hook will then apply to all projects, not just your current
project.

Then press Esc until you return to the REPL. Your hook is now registered!

#### Step 5: Verify your hook

Run `/hooks` again or check `~/.claude/settings.json` to see your configuration:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '\"\\(.tool_input.command) - \\(.tool_input.description // \"No description\")\"' >> ~/.claude/bash-command-log.txt"
          }
        ]
      }
    ]
  }
}
```

#### Step 6: Test your hook

Ask Claude to run a simple command like `ls` and check your log file:

```bash
cat ~/.claude/bash-command-log.txt
```

You should see entries like:

```
ls - Lists files and directories
```

### More Examples

<Note>
  For a complete example implementation, see the [bash command validator example](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py) in our public codebase.
</Note>

#### Code Formatting Hook

Automatically format TypeScript files after editing:

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | { read file_path; if echo \"$file_path\" | grep -q '\\.ts$'; then npx prettier --write \"$file_path\"; fi; }"
          }
        ]
      }
    ]
  }
}
```

#### Markdown Formatting Hook

Automatically fix missing language tags and formatting issues in markdown files:

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/markdown_formatter.py"
          }
        ]
      }
    ]
  }
}
```

Create `.claude/hooks/markdown_formatter.py` with this content:

```python
#!/usr/bin/env python3
"""
Markdown formatter for Claude Code output.
Fixes missing language tags and spacing issues while preserving code content.
"""
import json
import sys
import re
import os

def detect_language(code):
    """Best-effort language detection from code content."""
    s = code.strip()

    # JSON detection
    if re.search(r'^\s*[{\[]', s):
        try:
            json.loads(s)
            return 'json'
        except:
            pass

    # Python detection
    if re.search(r'^\s*def\s+\w+\s*\(', s, re.M) or \
       re.search(r'^\s*(import|from)\s+\w+', s, re.M):
        return 'python'

    # JavaScript detection
    if re.search(r'\b(function\s+\w+\s*\(|const\s+\w+\s*=)', s) or \
       re.search(r'=>|console\.(log|error)', s):
        return 'javascript'

    # Bash detection
    if re.search(r'^#!.*\b(bash|sh)\b', s, re.M) or \
       re.search(r'\b(if|then|fi|for|in|do|done)\b', s):
        return 'bash'

    # SQL detection
    if re.search(r'\b(SELECT|INSERT|UPDATE|DELETE|CREATE)\s+', s, re.I):
        return 'sql'

    return 'text'

def format_markdown(content):
    """Format markdown content with language detection."""
    # Fix unlabeled code fences
    def add_lang_to_fence(match):
        indent, info, body, closing = match.groups()
        if not info.strip():
            lang = detect_language(body)
            return f"{indent}```{lang}\n{body}{closing}\n"
        return match.group(0)

    fence_pattern = r'(?ms)^([ \t]{0,3})```([^\n]*)\n(.*?)(\n\1```)\s*$'
    content = re.sub(fence_pattern, add_lang_to_fence, content)

    # Fix excessive blank lines (only outside code fences)
    content = re.sub(r'\n{3,}', '\n\n', content)

    return content.rstrip() + '\n'

# Main execution
try:
    input_data = json.load(sys.stdin)
    file_path = input_data.get('tool_input', {}).get('file_path', '')

    if not file_path.endswith(('.md', '.mdx')):
        sys.exit(0)  # Not a markdown file

    if os.path.exists(file_path):
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        formatted = format_markdown(content)

        if formatted != content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(formatted)
            print(f"✓ Fixed markdown formatting in {file_path}")

except Exception as e:
    print(f"Error formatting markdown: {e}", file=sys.stderr)
    sys.exit(1)
```

Make the script executable:

```bash
chmod +x .claude/hooks/markdown_formatter.py
```

This hook automatically:

* Detects programming languages in unlabeled code blocks
* Adds appropriate language tags for syntax highlighting
* Fixes excessive blank lines while preserving code content
* Only processes markdown files (`.md`, `.mdx`)

#### Custom Notification Hook

Get desktop notifications when Claude needs input:

```json
{
  "hooks": {
    "Notification": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "notify-send 'Claude Code' 'Awaiting your input'"
          }
        ]
      }
    ]
  }
}
```

#### File Protection Hook

Block edits to sensitive files:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "python3 -c \"import json, sys; data=json.load(sys.stdin); path=data.get('tool_input',{}).get('file_path',''); sys.exit(2 if any(p in path for p in ['.env', 'package-lock.json', '.git/']) else 0)\""
          }
        ]
      }
    ]
  }
}
```

### Learn more

* For reference documentation on hooks, see [Hooks reference documentation](#hooks-reference-documentation) below.
* For comprehensive security best practices and safety guidelines, see [Security Considerations](#security-considerations) in the hooks reference documentation.
* For troubleshooting steps and debugging techniques, see [Debugging](#debugging) in the hooks reference
  documentation.

---

## Hooks Reference Documentation

> This section provides reference documentation for implementing hooks in Claude Code.

<Tip>
  For a quickstart guide with examples, see the [Hooks section](#section-3-hooks---event-driven-automation) above.
</Tip>

### Configuration

Claude Code hooks are configured in your [settings files](/en/docs/claude-code/settings):

* `~/.claude/settings.json` - User settings
* `.claude/settings.json` - Project settings
* `.claude/settings.local.json` - Local project settings (not committed)
* Enterprise managed policy settings

#### Structure

Hooks are organized by matchers, where each matcher can have multiple hooks:

```json
{
  "hooks": {
    "EventName": [
      {
        "matcher": "ToolPattern",
        "hooks": [
          {
            "type": "command",
            "command": "your-command-here"
          }
        ]
      }
    ]
  }
}
```

* **matcher**: Pattern to match tool names, case-sensitive (only applicable for
  `PreToolUse` and `PostToolUse`)
  * Simple strings match exactly: `Write` matches only the Write tool
  * Supports regex: `Edit|Write` or `Notebook.*`
  * Use `*` to match all tools. You can also use empty string (`""`) or leave
    `matcher` blank.
* **hooks**: Array of commands to execute when the pattern matches
  * `type`: Currently only `"command"` is supported
  * `command`: The bash command to execute (can use `$CLAUDE_PROJECT_DIR`
    environment variable)
  * `timeout`: (Optional) How long a command should run, in seconds, before
    canceling that specific command.

For events like `UserPromptSubmit`, `Notification`, `Stop`, and `SubagentStop`
that don't use matchers, you can omit the matcher field:

```json
{
  "hooks": {
    "UserPromptSubmit": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/prompt-validator.py"
          }
        ]
      }
    ]
  }
}
```

#### Project-Specific Hook Scripts

You can use the environment variable `CLAUDE_PROJECT_DIR` (only available when
Claude Code spawns the hook command) to reference scripts stored in your project,
ensuring they work regardless of Claude's current directory:

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/check-style.sh"
          }
        ]
      }
    ]
  }
}
```

#### Plugin hooks

[Plugins](/en/docs/claude-code/plugins) can provide hooks that integrate seamlessly with your user and project hooks. Plugin hooks are automatically merged with your configuration when plugins are enabled.

**How plugin hooks work**:

* Plugin hooks are defined in the plugin's `hooks/hooks.json` file or in a file given by a custom path to the `hooks` field.
* When a plugin is enabled, its hooks are merged with user and project hooks
* Multiple hooks from different sources can respond to the same event
* Plugin hooks use the `${CLAUDE_PLUGIN_ROOT}` environment variable to reference plugin files

**Example plugin hook configuration**:

```json
{
  "description": "Automatic code formatting",
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/scripts/format.sh",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

<Note>
  Plugin hooks use the same format as regular hooks with an optional `description` field to explain the hook's purpose.
</Note>

<Note>
  Plugin hooks run alongside your custom hooks. If multiple hooks match an event, they all execute in parallel.
</Note>

**Environment variables for plugins**:

* `${CLAUDE_PLUGIN_ROOT}`: Absolute path to the plugin directory
* `${CLAUDE_PROJECT_DIR}`: Project root directory (same as for project hooks)
* All standard environment variables are available

See the [plugin components reference](/en/docs/claude-code/plugins-reference#hooks) for details on creating plugin hooks.

### Hook Events

#### PreToolUse

Runs after Claude creates tool parameters and before processing the tool call.

**Common matchers:**

* `Task` - Subagent tasks (see [subagents documentation](/en/docs/claude-code/sub-agents))
* `Bash` - Shell commands
* `Glob` - File pattern matching
* `Grep` - Content search
* `Read` - File reading
* `Edit` - File editing
* `Write` - File writing
* `WebFetch`, `WebSearch` - Web operations

#### PostToolUse

Runs immediately after a tool completes successfully.

Recognizes the same matcher values as PreToolUse.

#### Notification

Runs when Claude Code sends notifications. Notifications are sent when:

1. Claude needs your permission to use a tool. Example: "Claude needs your
   permission to use Bash"
2. The prompt input has been idle for at least 60 seconds. "Claude is waiting
   for your input"

#### UserPromptSubmit

Runs when the user submits a prompt, before Claude processes it. This allows you
to add additional context based on the prompt/conversation, validate prompts, or
block certain types of prompts.

#### Stop

Runs when the main Claude Code agent has finished responding. Does not run if
the stoppage occurred due to a user interrupt.

#### SubagentStop

Runs when a Claude Code subagent (Task tool call) has finished responding.

#### PreCompact

Runs before Claude Code is about to run a compact operation.

**Matchers:**

* `manual` - Invoked from `/compact`
* `auto` - Invoked from auto-compact (due to full context window)

#### SessionStart

Runs when Claude Code starts a new session or resumes an existing session (which
currently does start a new session under the hood). Useful for loading in
development context like existing issues or recent changes to your codebase.

**Matchers:**

* `startup` - Invoked from startup
* `resume` - Invoked from `--resume`, `--continue`, or `/resume`
* `clear` - Invoked from `/clear`
* `compact` - Invoked from auto or manual compact.

#### SessionEnd

Runs when a Claude Code session ends. Useful for cleanup tasks, logging session
statistics, or saving session state.

The `reason` field in the hook input will be one of:

* `clear` - Session cleared with /clear command
* `logout` - User logged out
* `prompt_input_exit` - User exited while prompt input was visible
* `other` - Other exit reasons

### Hook Input

Hooks receive JSON data via stdin containing session information and
event-specific data:

```typescript
{
  // Common fields
  session_id: string
  transcript_path: string  // Path to conversation JSON
  cwd: string              // The current working directory when the hook is invoked

  // Event-specific fields
  hook_event_name: string
  ...
}
```

#### PreToolUse Input

The exact schema for `tool_input` depends on the tool.

```json
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PreToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "/path/to/file.txt",
    "content": "file content"
  }
}
```

#### PostToolUse Input

The exact schema for `tool_input` and `tool_response` depends on the tool.

```json
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "PostToolUse",
  "tool_name": "Write",
  "tool_input": {
    "file_path": "/path/to/file.txt",
    "content": "file content"
  },
  "tool_response": {
    "filePath": "/path/to/file.txt",
    "success": true
  }
}
```

#### Notification Input

```json
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "Notification",
  "message": "Task completed successfully"
}
```

#### UserPromptSubmit Input

```json
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "UserPromptSubmit",
  "prompt": "Write a function to calculate the factorial of a number"
}
```

#### Stop and SubagentStop Input

`stop_hook_active` is true when Claude Code is already continuing as a result of
a stop hook. Check this value or process the transcript to prevent Claude Code
from running indefinitely.

```json
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "hook_event_name": "Stop",
  "stop_hook_active": true
}
```

#### PreCompact Input

For `manual`, `custom_instructions` comes from what the user passes into
`/compact`. For `auto`, `custom_instructions` is empty.

```json
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "hook_event_name": "PreCompact",
  "trigger": "manual",
  "custom_instructions": ""
}
```

#### SessionStart Input

```json
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "hook_event_name": "SessionStart",
  "source": "startup"
}
```

#### SessionEnd Input

```json
{
  "session_id": "abc123",
  "transcript_path": "~/.claude/projects/.../00893aaf-19fa-41d2-8238-13269b9b3ca0.jsonl",
  "cwd": "/Users/...",
  "hook_event_name": "SessionEnd",
  "reason": "exit"
}
```

### Hook Output

There are two ways for hooks to return output back to Claude Code. The output
communicates whether to block and any feedback that should be shown to Claude
and the user.

#### Simple: Exit Code

Hooks communicate status through exit codes, stdout, and stderr:

* **Exit code 0**: Success. `stdout` is shown to the user in transcript mode
  (CTRL-R), except for `UserPromptSubmit` and `SessionStart`, where stdout is
  added to the context.
* **Exit code 2**: Blocking error. `stderr` is fed back to Claude to process
  automatically. See per-hook-event behavior below.
* **Other exit codes**: Non-blocking error. `stderr` is shown to the user and
  execution continues.

<Warning>
  Reminder: Claude Code does not see stdout if the exit code is 0, except for
  the `UserPromptSubmit` hook where stdout is injected as context.
</Warning>

##### Exit Code 2 Behavior

| Hook Event         | Behavior                                                           |
| ------------------ | ------------------------------------------------------------------ |
| `PreToolUse`       | Blocks the tool call, shows stderr to Claude                       |
| `PostToolUse`      | Shows stderr to Claude (tool already ran)                          |
| `Notification`     | N/A, shows stderr to user only                                     |
| `UserPromptSubmit` | Blocks prompt processing, erases prompt, shows stderr to user only |
| `Stop`             | Blocks stoppage, shows stderr to Claude                            |
| `SubagentStop`     | Blocks stoppage, shows stderr to Claude subagent                   |
| `PreCompact`       | N/A, shows stderr to user only                                     |
| `SessionStart`     | N/A, shows stderr to user only                                     |
| `SessionEnd`       | N/A, shows stderr to user only                                     |

#### Advanced: JSON Output

Hooks can return structured JSON in `stdout` for more sophisticated control:

##### Common JSON Fields

All hook types can include these optional fields:

```json
{
  "continue": true, // Whether Claude should continue after hook execution (default: true)
  "stopReason": "string", // Message shown when continue is false

  "suppressOutput": true, // Hide stdout from transcript mode (default: false)
  "systemMessage": "string" // Optional warning message shown to the user
}
```

If `continue` is false, Claude stops processing after the hooks run.

* For `PreToolUse`, this is different from `"permissionDecision": "deny"`, which
  only blocks a specific tool call and provides automatic feedback to Claude.
* For `PostToolUse`, this is different from `"decision": "block"`, which
  provides automated feedback to Claude.
* For `UserPromptSubmit`, this prevents the prompt from being processed.
* For `Stop` and `SubagentStop`, this takes precedence over any
  `"decision": "block"` output.
* In all cases, `"continue" = false` takes precedence over any
  `"decision": "block"` output.

`stopReason` accompanies `continue` with a reason shown to the user, not shown
to Claude.

##### `PreToolUse` Decision Control

`PreToolUse` hooks can control whether a tool call proceeds.

* `"allow"` bypasses the permission system. `permissionDecisionReason` is shown
  to the user but not to Claude.
* `"deny"` prevents the tool call from executing. `permissionDecisionReason` is
  shown to Claude.
* `"ask"` asks the user to confirm the tool call in the UI.
  `permissionDecisionReason` is shown to the user but not to Claude.

```json
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow" | "deny" | "ask",
    "permissionDecisionReason": "My reason here"
  }
}
```

<Note>
  The `decision` and `reason` fields are deprecated for PreToolUse hooks.
  Use `hookSpecificOutput.permissionDecision` and
  `hookSpecificOutput.permissionDecisionReason` instead. The deprecated fields
  `"approve"` and `"block"` map to `"allow"` and `"deny"` respectively.
</Note>

##### `PostToolUse` Decision Control

`PostToolUse` hooks can provide feedback to Claude after tool execution.

* `"block"` automatically prompts Claude with `reason`.
* `undefined` does nothing. `reason` is ignored.
* `"hookSpecificOutput.additionalContext"` adds context for Claude to consider.

```json
{
  "decision": "block" | undefined,
  "reason": "Explanation for decision",
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Additional information for Claude"
  }
}
```

##### `UserPromptSubmit` Decision Control

`UserPromptSubmit` hooks can control whether a user prompt is processed.

* `"block"` prevents the prompt from being processed. The submitted prompt is
  erased from context. `"reason"` is shown to the user but not added to context.
* `undefined` allows the prompt to proceed normally. `"reason"` is ignored.
* `"hookSpecificOutput.additionalContext"` adds the string to the context if not
  blocked.

```json
{
  "decision": "block" | undefined,
  "reason": "Explanation for decision",
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "My additional context here"
  }
}
```

##### `Stop`/`SubagentStop` Decision Control

`Stop` and `SubagentStop` hooks can control whether Claude must continue.

* `"block"` prevents Claude from stopping. You must populate `reason` for Claude
  to know how to proceed.
* `undefined` allows Claude to stop. `reason` is ignored.

```json
{
  "decision": "block" | undefined,
  "reason": "Must be provided when Claude is blocked from stopping"
}
```

##### `SessionStart` Decision Control

`SessionStart` hooks allow you to load in context at the start of a session.

* `"hookSpecificOutput.additionalContext"` adds the string to the context.
* Multiple hooks' `additionalContext` values are concatenated.

```json
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "My additional context here"
  }
}
```

##### `SessionEnd` Decision Control

`SessionEnd` hooks run when a session ends. They cannot block session termination
but can perform cleanup tasks.

##### Exit Code Example: Bash Command Validation

```python
#!/usr/bin/env python3
import json
import re
import sys

# Define validation rules as a list of (regex pattern, message) tuples
VALIDATION_RULES = [
    (
        r"\bgrep\b(?!.*\|)",
        "Use 'rg' (ripgrep) instead of 'grep' for better performance and features",
    ),
    (
        r"\bfind\s+\S+\s+-name\b",
        "Use 'rg --files | rg pattern' or 'rg --files -g pattern' instead of 'find -name' for better performance",
    ),
]


def validate_command(command: str) -> list[str]:
    issues = []
    for pattern, message in VALIDATION_RULES:
        if re.search(pattern, command):
            issues.append(message)
    return issues


try:
    input_data = json.load(sys.stdin)
except json.JSONDecodeError as e:
    print(f"Error: Invalid JSON input: {e}", file=sys.stderr)
    sys.exit(1)

tool_name = input_data.get("tool_name", "")
tool_input = input_data.get("tool_input", {})
command = tool_input.get("command", "")

if tool_name != "Bash" or not command:
    sys.exit(1)

# Validate the command
issues = validate_command(command)

if issues:
    for message in issues:
        print(f"• {message}", file=sys.stderr)
    # Exit code 2 blocks tool call and shows stderr to Claude
    sys.exit(2)
```

##### JSON Output Example: UserPromptSubmit to Add Context and Validation

<Note>
  For `UserPromptSubmit` hooks, you can inject context using either method:

  * Exit code 0 with stdout: Claude sees the context (special case for `UserPromptSubmit`)
  * JSON output: Provides more control over the behavior
</Note>

```python
#!/usr/bin/env python3
import json
import sys
import re
import datetime

# Load input from stdin
try:
    input_data = json.load(sys.stdin)
except json.JSONDecodeError as e:
    print(f"Error: Invalid JSON input: {e}", file=sys.stderr)
    sys.exit(1)

prompt = input_data.get("prompt", "")

# Check for sensitive patterns
sensitive_patterns = [
    (r"(?i)\b(password|secret|key|token)\s*[:=]", "Prompt contains potential secrets"),
]

for pattern, message in sensitive_patterns:
    if re.search(pattern, prompt):
        # Use JSON output to block with a specific reason
        output = {
            "decision": "block",
            "reason": f"Security policy violation: {message}. Please rephrase your request without sensitive information."
        }
        print(json.dumps(output))
        sys.exit(0)

# Add current time to context
context = f"Current time: {datetime.datetime.now()}"
print(context)

"""
The following is also equivalent:
print(json.dumps({
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": context,
  },
}))
"""

# Allow the prompt to proceed with the additional context
sys.exit(0)
```

##### JSON Output Example: PreToolUse with Approval

```python
#!/usr/bin/env python3
import json
import sys

# Load input from stdin
try:
    input_data = json.load(sys.stdin)
except json.JSONDecodeError as e:
    print(f"Error: Invalid JSON input: {e}", file=sys.stderr)
    sys.exit(1)

tool_name = input_data.get("tool_name", "")
tool_input = input_data.get("tool_input", {})

# Example: Auto-approve file reads for documentation files
if tool_name == "Read":
    file_path = tool_input.get("file_path", "")
    if file_path.endswith((".md", ".mdx", ".txt", ".json")):
        # Use JSON output to auto-approve the tool call
        output = {
            "decision": "approve",
            "reason": "Documentation file auto-approved",
            "suppressOutput": True  # Don't show in transcript mode
        }
        print(json.dumps(output))
        sys.exit(0)

# For other cases, let the normal permission flow proceed
sys.exit(0)
```

### Working with MCP Tools

Claude Code hooks work seamlessly with
[Model Context Protocol (MCP) tools](/en/docs/claude-code/mcp). When MCP servers
provide tools, they appear with a special naming pattern that you can match in
your hooks.

#### MCP Tool Naming

MCP tools follow the pattern `mcp__<server>__<tool>`, for example:

* `mcp__memory__create_entities` - Memory server's create entities tool
* `mcp__filesystem__read_file` - Filesystem server's read file tool
* `mcp__github__search_repositories` - GitHub server's search tool

#### Configuring Hooks for MCP Tools

You can target specific MCP tools or entire MCP servers:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "mcp__memory__.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Memory operation initiated' >> ~/mcp-operations.log"
          }
        ]
      },
      {
        "matcher": "mcp__.*__write.*",
        "hooks": [
          {
            "type": "command",
            "command": "/home/user/scripts/validate-mcp-write.py"
          }
        ]
      }
    ]
  }
}
```

### Security Considerations

#### Disclaimer

**USE AT YOUR OWN RISK**: Claude Code hooks execute arbitrary shell commands on
your system automatically. By using hooks, you acknowledge that:

* You are solely responsible for the commands you configure
* Hooks can modify, delete, or access any files your user account can access
* Malicious or poorly written hooks can cause data loss or system damage
* Anthropic provides no warranty and assumes no liability for any damages
  resulting from hook usage
* You should thoroughly test hooks in a safe environment before production use

Always review and understand any hook commands before adding them to your
configuration.

#### Security Best Practices

Here are some key practices for writing more secure hooks:

1. **Validate and sanitize inputs** - Never trust input data blindly
2. **Always quote shell variables** - Use `"$VAR"` not `$VAR`
3. **Block path traversal** - Check for `..` in file paths
4. **Use absolute paths** - Specify full paths for scripts (use
   "\$CLAUDE\_PROJECT\_DIR" for the project path)
5. **Skip sensitive files** - Avoid `.env`, `.git/`, keys, etc.

#### Configuration Safety

Direct edits to hooks in settings files don't take effect immediately. Claude
Code:

1. Captures a snapshot of hooks at startup
2. Uses this snapshot throughout the session
3. Warns if hooks are modified externally
4. Requires review in `/hooks` menu for changes to apply

This prevents malicious hook modifications from affecting your current session.

### Hook Execution Details

* **Timeout**: 60-second execution limit by default, configurable per command.
  * A timeout for an individual command does not affect the other commands.
* **Parallelization**: All matching hooks run in parallel
* **Deduplication**: Multiple identical hook commands are deduplicated automatically
* **Environment**: Runs in current directory with Claude Code's environment
  * The `CLAUDE_PROJECT_DIR` environment variable is available and contains the
    absolute path to the project root directory (where Claude Code was started)
* **Input**: JSON via stdin
* **Output**:
  * PreToolUse/PostToolUse/Stop/SubagentStop: Progress shown in transcript (Ctrl-R)
  * Notification/SessionEnd: Logged to debug only (`--debug`)
  * UserPromptSubmit/SessionStart: stdout added as context for Claude

### Debugging

#### Basic Troubleshooting

If your hooks aren't working:

1. **Check configuration** - Run `/hooks` to see if your hook is registered
2. **Verify syntax** - Ensure your JSON settings are valid
3. **Test commands** - Run hook commands manually first
4. **Check permissions** - Make sure scripts are executable
5. **Review logs** - Use `claude --debug` to see hook execution details

Common issues:

* **Quotes not escaped** - Use `\"` inside JSON strings
* **Wrong matcher** - Check tool names match exactly (case-sensitive)
* **Command not found** - Use full paths for scripts

#### Advanced Debugging

For complex hook issues:

1. **Inspect hook execution** - Use `claude --debug` to see detailed hook
   execution
2. **Validate JSON schemas** - Test hook input/output with external tools
3. **Check environment variables** - Verify Claude Code's environment is correct
4. **Test edge cases** - Try hooks with unusual file paths or inputs
5. **Monitor system resources** - Check for resource exhaustion during hook
   execution
6. **Use structured logging** - Implement logging in your hook scripts

#### Debug Output Example

Use `claude --debug` to see hook execution details:

```
[DEBUG] Executing hooks for PostToolUse:Write
[DEBUG] Getting matching hook commands for PostToolUse with query: Write
[DEBUG] Found 1 hook matchers in settings
[DEBUG] Matched 1 hooks for query "Write"
[DEBUG] Found 1 hook commands to execute
[DEBUG] Executing hook command: <Your command> with timeout 60000ms
[DEBUG] Hook command completed with status 0: <Your stdout>
```

Progress messages appear in transcript mode (Ctrl-R) showing:

* Which hook is running
* Command being executed
* Success/failure status
* Output or error messages

---

## Section 4: Headless Mode Automation

> Run Claude Code programmatically without interactive UI

### Overview

The headless mode allows you to run Claude Code programmatically from command line scripts and automation tools without any interactive UI.

### Basic usage

The primary command-line interface to Claude Code is the `claude` command. Use the `--print` (or `-p`) flag to run in non-interactive mode and print the final result:

```bash
claude -p "Stage my changes and write a set of commits for them" \
  --allowedTools "Bash,Read" \
  --permission-mode acceptEdits
```

### Configuration Options

Headless mode leverages all the CLI options available in Claude Code. Here are the key ones for automation and scripting:

| Flag                       | Description                                                                                            | Example                                                                                                                   |
| :------------------------- | :----------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `--print`, `-p`            | Run in non-interactive mode                                                                            | `claude -p "query"`                                                                                                       |
| `--output-format`          | Specify output format (`text`, `json`, `stream-json`)                                                  | `claude -p --output-format json`                                                                                          |
| `--resume`, `-r`           | Resume a conversation by session ID                                                                    | `claude --resume abc123`                                                                                                  |
| `--continue`, `-c`         | Continue the most recent conversation                                                                  | `claude --continue`                                                                                                       |
| `--verbose`                | Enable verbose logging                                                                                 | `claude --verbose`                                                                                                        |
| `--append-system-prompt`   | Append to system prompt (only with `--print`)                                                          | `claude --append-system-prompt "Custom instruction"`                                                                      |
| `--allowedTools`           | Space-separated list of allowed tools, or <br /><br /> string of comma-separated list of allowed tools | `claude --allowedTools mcp__slack mcp__filesystem`<br /><br />`claude --allowedTools "Bash(npm install),mcp__filesystem"` |
| `--disallowedTools`        | Space-separated list of denied tools, or <br /><br /> string of comma-separated list of denied tools   | `claude --disallowedTools mcp__splunk mcp__github`<br /><br />`claude --disallowedTools "Bash(git commit),mcp__github"`   |
| `--mcp-config`             | Load MCP servers from a JSON file                                                                      | `claude --mcp-config servers.json`                                                                                        |
| `--permission-prompt-tool` | MCP tool for handling permission prompts (only with `--print`)                                         | `claude --permission-prompt-tool mcp__auth__prompt`                                                                       |

For a complete list of CLI options and features, see the [CLI reference](/en/docs/claude-code/cli-reference) documentation.

### Multi-turn conversations

For multi-turn conversations, you can resume conversations or continue from the most recent session:

```bash
# Continue the most recent conversation
claude --continue "Now refactor this for better performance"

# Resume a specific conversation by session ID
claude --resume 550e8400-e29b-41d4-a716-446655440000 "Update the tests"

# Resume in non-interactive mode
claude --resume 550e8400-e29b-41d4-a716-446655440000 "Fix all linting issues" --no-interactive
```

### Output Formats

#### Text Output (Default)

```bash
claude -p "Explain file src/components/Header.tsx"
# Output: This is a React component showing...
```

#### JSON Output

Returns structured data including metadata:

```bash
claude -p "How does the data layer work?" --output-format json
```

Response format:

```json
{
  "type": "result",
  "subtype": "success",
  "total_cost_usd": 0.003,
  "is_error": false,
  "duration_ms": 1234,
  "duration_api_ms": 800,
  "num_turns": 6,
  "result": "The response text here...",
  "session_id": "abc123"
}
```

#### Streaming JSON Output

Streams each message as it is received:

```bash
claude -p "Build an application" --output-format stream-json
```

Each conversation begins with an initial `init` system message, followed by a list of user and assistant messages, followed by a final `result` system message with stats. Each message is emitted as a separate JSON object.

### Input Formats

#### Text Input (Default)

```bash
# Direct argument
claude -p "Explain this code"

# From stdin
echo "Explain this code" | claude -p
```

#### Streaming JSON Input

A stream of messages provided via `stdin` where each message represents a user turn. This allows multiple turns of a conversation without re-launching the `claude` binary and allows providing guidance to the model while it is processing a request.

Each message is a JSON 'User message' object, following the same format as the output message schema. Messages are formatted using the [jsonl](https://jsonlines.org/) format where each line of input is a complete JSON object. Streaming JSON input requires `-p` and `--output-format stream-json`.

```bash
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Explain this code"}]}}' | claude -p --output-format=stream-json --input-format=stream-json --verbose
```

### Agent Integration Examples

#### SRE Incident Response Bot

```bash
#!/bin/bash

# Automated incident response agent
investigate_incident() {
    local incident_description="$1"
    local severity="${2:-medium}"

    claude -p "Incident: $incident_description (Severity: $severity)" \
      --append-system-prompt "You are an SRE expert. Diagnose the issue, assess impact, and provide immediate action items." \
      --output-format json \
      --allowedTools "Bash,Read,WebSearch,mcp__datadog" \
      --mcp-config monitoring-tools.json
}

# Usage
investigate_incident "Payment API returning 500 errors" "high"
```

#### Automated Security Review

```bash
# Security audit agent for pull requests
audit_pr() {
    local pr_number="$1"

    gh pr diff "$pr_number" | claude -p \
      --append-system-prompt "You are a security engineer. Review this PR for vulnerabilities, insecure patterns, and compliance issues." \
      --output-format json \
      --allowedTools "Read,Grep,WebSearch"
}

# Usage and save to file
audit_pr 123 > security-report.json
```

#### Multi-turn Legal Assistant

```bash
# Legal document review with session persistence
session_id=$(claude -p "Start legal review session" --output-format json | jq -r '.session_id')

# Review contract in multiple steps
claude -p --resume "$session_id" "Review contract.pdf for liability clauses"
claude -p --resume "$session_id" "Check compliance with GDPR requirements"
claude -p --resume "$session_id" "Generate executive summary of risks"
```

### Best Practices

* **Use JSON output format** for programmatic parsing of responses:

  ```bash
  # Parse JSON response with jq
  result=$(claude -p "Generate code" --output-format json)
  code=$(echo "$result" | jq -r '.result')
  cost=$(echo "$result" | jq -r '.cost_usd')
  ```

* **Handle errors gracefully** - check exit codes and stderr:

  ```bash
  if ! claude -p "$prompt" 2>error.log; then
      echo "Error occurred:" >&2
      cat error.log >&2
      exit 1
  fi
  ```

* **Use session management** for maintaining context in multi-turn conversations

* **Consider timeouts** for long-running operations:

  ```bash
  timeout 300 claude -p "$complex_prompt" || echo "Timed out after 5 minutes"
  ```

* **Respect rate limits** when making multiple requests by adding delays between calls

### Related Resources

* [CLI usage and controls](/en/docs/claude-code/cli-reference) - Complete CLI documentation
* [Common workflows](/en/docs/claude-code/common-workflows) - Step-by-step guides for common use cases

---

## Integration Patterns Quick Reference

### Common Integration Scenarios

#### 1. Mention-Driven Workflow (@claude)

**Use Case:** Interactive code assistance in PRs/issues/MRs

**Platforms:** GitHub Actions, GitLab CI/CD

**Trigger:** Comment containing `@claude` mention

**Example Flow:**
1. Developer comments `@claude implement authentication` on issue
2. CI/CD pipeline triggered by webhook or event
3. Claude analyzes context, writes code, opens PR/MR
4. Team reviews changes through normal process

**Configuration:**
```yaml
# GitHub Actions
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]

# GitLab CI/CD
rules:
  - if: '$CI_PIPELINE_SOURCE == "web"'
  - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
```

#### 2. Scheduled Automation

**Use Case:** Daily/weekly code maintenance tasks

**Platforms:** GitHub Actions, GitLab CI/CD, Headless Mode

**Trigger:** Cron schedule

**Example Flow:**
1. Scheduled job runs at specified time
2. Claude generates reports, updates dependencies, fixes issues
3. Creates PR/MR with changes or posts summary

**Configuration:**
```yaml
# GitHub Actions
on:
  schedule:
    - cron: "0 9 * * *"  # Daily at 9 AM

# Headless mode via cron
0 9 * * * cd /project && claude -p "Update dependencies" --output-format json
```

#### 3. Event-Driven Hooks

**Use Case:** Automatic formatting, validation, notifications

**Platform:** Claude Code Terminal with hooks

**Trigger:** Tool usage (Read, Write, Edit, Bash, etc.)

**Example Flow:**
1. Claude uses Write tool to create file
2. PostToolUse hook triggers
3. Formatting script runs automatically
4. Claude sees formatted result

**Configuration:**
```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "prettier --write \"$(jq -r '.tool_input.file_path')\""
          }
        ]
      }
    ]
  }
}
```

#### 4. PR/MR Review Automation

**Use Case:** Automatic code review on new PRs/MRs

**Platforms:** GitHub Actions, GitLab CI/CD

**Trigger:** PR/MR opened or synchronized

**Example Flow:**
1. PR/MR opened with changes
2. CI/CD runs Claude Code with review prompt
3. Claude analyzes changes, posts review comments
4. Developers address feedback

**Configuration:**
```yaml
# GitHub Actions
on:
  pull_request:
    types: [opened, synchronize]

# GitLab CI/CD
on:
  merge_request_event
```

#### 5. Headless Agent Integration

**Use Case:** Custom automation scripts and tools

**Platform:** Headless Mode CLI

**Trigger:** External script or application

**Example Flow:**
1. Script detects condition (test failure, deployment issue)
2. Invokes `claude -p` with context
3. Parses JSON output
4. Takes action based on result

**Configuration:**
```bash
#!/bin/bash
result=$(claude -p "Analyze test failure" \
  --output-format json \
  --allowedTools "Bash,Read,Grep")

if [ "$(echo "$result" | jq -r '.is_error')" = "false" ]; then
  echo "$result" | jq -r '.result' | notify-send "Claude Analysis"
fi
```

### Security Best Practices Across All Integrations

**API Key Management:**
- ✅ Always use CI/CD secrets/variables
- ✅ Use OIDC/WIF for cloud providers (no long-lived keys)
- ❌ Never commit keys to repositories
- ❌ Never expose keys in logs or output

**Permissions:**
- ✅ Grant minimum required permissions
- ✅ Use repository-specific credentials
- ✅ Review all automated changes
- ✅ Apply branch protection rules

**Hooks Security:**
- ✅ Review hook commands before adding
- ✅ Validate and sanitize inputs
- ✅ Use absolute paths for scripts
- ✅ Test hooks in safe environment first
- ❌ Never trust user input blindly

**Cost Control:**
- ✅ Set appropriate `--max-turns` limits
- ✅ Configure workflow timeouts
- ✅ Use concurrency controls
- ✅ Monitor token usage

### Integration Decision Matrix

| Requirement | GitHub Actions | GitLab CI/CD | Hooks | Headless Mode |
|-------------|----------------|--------------|-------|---------------|
| Interactive (@mention) | ✅ | ✅ | ❌ | ❌ |
| Scheduled runs | ✅ | ✅ | ❌ | ✅ |
| Event-driven | ✅ | ✅ | ✅ | ⚠️ (via external) |
| Local development | ❌ | ❌ | ✅ | ✅ |
| Custom scripting | ⚠️ (limited) | ⚠️ (limited) | ⚠️ (limited) | ✅ |
| Real-time formatting | ❌ | ❌ | ✅ | ❌ |
| Multi-turn conversations | ✅ | ✅ | ❌ | ✅ |
| AWS Bedrock support | ✅ | ✅ | N/A | ✅ |
| Google Vertex AI support | ✅ | ✅ | N/A | ✅ |

### Common Workflow Patterns

#### Pattern 1: Review → Fix → Deploy

```
1. PR opened → GitHub Actions triggers review
2. Claude posts review comments
3. Developer addresses or @claude fix issues
4. Tests pass → Deploy
```

#### Pattern 2: Issue → Implementation → Review

```
1. Issue created with @claude mention
2. Claude analyzes, implements, opens PR
3. Team reviews PR
4. Merge when approved
```

#### Pattern 3: Local Development with Hooks

```
1. Developer writes code in terminal
2. PostToolUse hook auto-formats
3. PreToolUse hook validates before commits
4. Notification hook alerts when blocked
```

#### Pattern 4: Scheduled Maintenance

```
1. Daily cron job runs headless Claude
2. Updates dependencies, fixes linting
3. Opens PR with changes
4. Team reviews and merges
```

### Performance Optimization

**For GitHub Actions/GitLab CI/CD:**
- Cache npm/pip dependencies
- Use self-hosted runners for faster startup
- Limit `--max-turns` to prevent runaway jobs
- Use specific tool allowlists

**For Hooks:**
- Keep hook scripts fast (<100ms ideal)
- Run expensive operations async
- Use `suppressOutput: true` for silent hooks
- Deduplicate identical hooks

**For Headless Mode:**
- Reuse sessions with `--resume` for context
- Stream output with `stream-json` for responsiveness
- Use `--allowedTools` to reduce overhead
- Consider timeout values

### Troubleshooting Quick Reference

**Common Issues:**

| Issue | Solution |
|-------|----------|
| @claude not responding | Check webhook configuration, verify API key, ensure trigger phrase matches |
| Hooks not executing | Verify JSON syntax, check permissions, run `claude --debug` |
| Authentication errors | Confirm secrets are set, validate OIDC/WIF setup, check IAM roles |
| High costs | Set `--max-turns`, configure timeouts, use tool allowlists |
| CI/CD job timeouts | Reduce scope, optimize prompts, use faster runners |
| Hook timeouts | Set per-command timeout, optimize hook scripts |

---

## Section 5: IDE & Browser Integrations (December 2025)

### Claude in Chrome (Beta)

Control your browser directly from Claude Code using the Chrome extension:

**Installation:**
1. Install Claude Code Chrome extension from Chrome Web Store
2. Enable the extension in Chrome
3. Start Claude Code with browser control enabled

**Capabilities:**
- Navigate web pages
- Fill forms and interact with elements
- Capture screenshots for analysis
- Automated web testing and scraping
- Browser-based workflows

**Example Use Cases:**
```
> "Open GitHub and check my notifications"
> "Fill out the form on the current page with test data"
> "Take a screenshot of this error and analyze it"
```

**Note:** This is currently in Beta. Browser actions require explicit user approval.

### JetBrains IDE Integration

Claude Agent is now available in JetBrains IDEs (IntelliJ, PyCharm, WebStorm, etc.):

**Architecture:**
- Built on the Claude Agent SDK
- Uses the same context management as Claude Code terminal
- Integrates with JetBrains' editor and project structure

**Features:**
- In-editor code generation and refactoring
- Context-aware suggestions using project files
- Seamless handoff between IDE and terminal
- Access to JetBrains-specific tools and refactorings

**Setup:**
1. Install Claude Agent plugin from JetBrains Marketplace
2. Configure API credentials in IDE settings
3. Access via Tools menu or keyboard shortcuts

### GitHub Copilot Integration

Claude is available in GitHub Copilot for Chat, Ask, and Edit modes:

**Availability:**
- GitHub Copilot Enterprise
- GitHub Copilot Business
- GitHub Copilot Pro
- GitHub Copilot Pro+

**Modes:**
| Mode | Description |
|------|-------------|
| Chat | Conversational AI assistance |
| Ask | Quick questions about code |
| Edit | Inline code modifications |

**Note:** This uses GitHub's infrastructure and may have different capabilities than direct Claude Code access.

### VS Code Integration

Native VS Code extension available since September 2025:

**Features:**
- Chat panel integration
- Inline code suggestions
- File explorer integration
- Terminal embedding

**Installation:**
1. Search "Claude Code" in VS Code extensions
2. Install and configure API key
3. Access via activity bar icon

---

## Summary

This reference consolidates all integration and automation patterns for Claude Code Terminal across:

1. **GitHub Actions** - PR/issue automation with @mentions
2. **GitLab CI/CD** - MR/issue automation with flexible triggers
3. **Hooks** - Event-driven local automation and formatting
4. **Headless Mode** - Programmatic CLI integration for custom scripts
5. **IDE & Browser** - Chrome, JetBrains, VS Code, GitHub Copilot integrations

All patterns support:
- Direct Claude API access
- AWS Bedrock (OIDC authentication)
- Google Vertex AI (Workload Identity Federation)
- Custom prompts and system instructions
- CLAUDE.md project-specific configuration
- Security best practices and cost controls

**Key Takeaway:** Choose the integration method that fits your workflow:
- **CI/CD platforms** for team collaboration and PR/MR workflows
- **Hooks** for real-time local development automation
- **Headless mode** for custom scripting and agent integrations
- **IDE extensions** for embedded development experience
- **Combine multiple** for comprehensive automation coverage

---

**Document Version:** 2.0 (2025-12-20)
**Claude Code Version:** 2.0.74
**Sections:** 5 (GitHub Actions, GitLab CI/CD, Hooks, Headless Mode, IDE & Browser)
