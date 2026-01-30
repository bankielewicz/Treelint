# Claude Code Terminal - Advanced Features Reference

**Source:** Official documentation from code.claude.com (updated 2025-12-09)

**Purpose:** Comprehensive reference for advanced Claude Code features including sandboxing, security, network configuration, monitoring, data privacy, and enterprise deployment.

---

## Table of Contents

1. [Sandboxing & Isolation](#section-1-sandboxing--isolation)
2. [Security Features](#section-2-security-features)
3. [Network Configuration](#section-3-network-configuration)
4. [Usage Monitoring & Analytics](#section-4-usage-monitoring--analytics)
5. [Data Privacy & Compliance](#section-5-data-privacy--compliance)
6. [Enterprise Features](#section-6-enterprise-features)
7. [Advanced Configuration Examples](#section-7-advanced-configuration-examples)
8. [Headless Mode & Automation](#section-8-headless-mode--automation)
9. [CLI Reference](#section-9-cli-reference)

---

## Section 1: Sandboxing & Isolation

### Overview

Claude Code provides multiple layers of sandboxing to protect your system:

**Sandbox Levels:**
1. **Filesystem Isolation** - Controls which directories Claude can access
2. **Network Isolation** - Controls network connectivity and external access
3. **Tool Restrictions** - Controls which tools and commands Claude can execute
4. **Permission System** - Fine-grained control over all operations

### Filesystem Sandboxing

**Default Behavior:**
- Claude can only access the current working directory
- Hidden files and system directories are restricted by default
- Environment-specific paths (.env files) are blocked

**Configuration:**

```json
{
  "permissions": {
    "additionalDirectories": [
      "../shared-lib",
      "../docs",
      "/usr/local/config"
    ],
    "deny": [
      "Read(.env*)",
      "Read(secrets/*)",
      "Read(*.key)",
      "Read(*.pem)",
      "Write(/etc/*)",
      "Write(/usr/*)",
      "Write(/sys/*)"
    ]
  }
}
```

**Filesystem Access Patterns:**

```json
{
  "permissions": {
    "allow": [
      "Read(src/**/*)",           // All files in src/
      "Read(tests/**/*.test.ts)", // Test files only
      "Write(build/**/*)",        // Build output directory
      "Edit(*.{ts,tsx,js,jsx})"   // Source code files
    ],
    "deny": [
      "Read(node_modules/**/*)",  // Block large directories
      "Write(src/**/config.json)", // Protect configuration
      "Edit(.git/**/*)"            // Prevent git corruption
    ]
  }
}
```

### Network Sandboxing

**Network Access Control:**

Claude Code can be configured to restrict network access:

```json
{
  "network": {
    "allowedDomains": [
      "api.anthropic.com",
      "cdn.jsdelivr.net",
      "registry.npmjs.org"
    ],
    "blockedDomains": [
      "malicious-site.com"
    ],
    "requireProxy": true,
    "proxyUrl": "http://corporate-proxy:8080"
  }
}
```

**Network Isolation Modes:**

1. **Full Access** (default) - Unrestricted network access
2. **Proxy Required** - All traffic through corporate proxy
3. **Allowlist Only** - Only specified domains accessible
4. **No Network** - Complete network isolation

### Tool Sandboxing

**Tool-Level Restrictions:**

```json
{
  "permissions": {
    "allow": [
      "Bash(git status)",
      "Bash(git diff*)",
      "Bash(npm test*)",
      "Bash(node scripts/*)"
    ],
    "ask": [
      "Bash(git push*)",
      "Bash(npm publish*)",
      "Bash(docker *)"
    ],
    "deny": [
      "Bash(rm -rf *)",
      "Bash(sudo *)",
      "Bash(chmod 777 *)",
      "Bash(curl * | bash)",
      "Bash(wget * && *)",
      "Bash(*shutdown*)",
      "Bash(*reboot*)"
    ]
  }
}
```

**Tool Categories:**

1. **Native Tools** - Read, Write, Edit, Grep, Glob (safer)
2. **Bash Tool** - Requires explicit command approval
3. **MCP Tools** - Controlled by MCP server configuration
4. **Custom Tools** - Defined via plugins/hooks

### Sandbox Bypass Prevention

**Disable Bypass Mode:**

```json
{
  "permissions": {
    "disableBypassPermissionsMode": "disable",
    "enforceStrictSandbox": true
  }
}
```

**Enterprise Enforcement:**

In enterprise deployments, managed policies can enforce sandboxing at the system level, preventing users from disabling protections.

### Security Best Practices

1. **Principle of Least Privilege** - Grant minimal necessary permissions
2. **Deny by Default** - Use allowlists rather than denylists
3. **Protect Secrets** - Always deny access to credential files
4. **Audit Tool Usage** - Monitor and log all tool invocations
5. **Layer Defenses** - Use multiple sandbox layers (filesystem + network + tool restrictions)

**Example Secure Configuration:**

```json
{
  "permissions": {
    "allow": [
      "Read(src/**/*)",
      "Read(tests/**/*)",
      "Grep(*)",
      "Glob(*)"
    ],
    "deny": [
      "Read(.env*)",
      "Read(secrets/*)",
      "Read(*.key)",
      "Bash(rm *)",
      "Bash(sudo *)",
      "Write(/etc/*)"
    ],
    "disableBypassPermissionsMode": "disable",
    "defaultMode": "readOnly"
  }
}
```

---

## Section 2: Security Features

### Permission Architecture

**Three-Tier Permission System:**

1. **Allow List** - Pre-approved tools and commands (no prompts)
2. **Ask List** - Requires user confirmation before execution
3. **Deny List** - Blocked operations (cannot be overridden)

**Permission Modes:**

| Mode | Description | Use Case |
|------|-------------|----------|
| `readOnly` | Only read operations allowed | Safe exploration, code review |
| `acceptEdits` | Read + Edit allowed, prompts for Write/Bash | Development with safety |
| `acceptAll` | All operations pre-approved | Trusted automation |
| `plan` | Show plan before execution | Review-first workflows |

**Configuring Permission Modes:**

```json
{
  "permissions": {
    "defaultMode": "acceptEdits",
    "allow": ["Read(*)", "Edit(src/**/*)", "Grep(*)", "Glob(*)"],
    "ask": ["Write(*)", "Bash(git *)"],
    "deny": ["Bash(rm -rf *)", "Bash(sudo *)"]
  }
}
```

**CLI Override:**

```bash
# Start in read-only mode
claude --permission-mode readOnly

# Start with all operations pre-approved (use with caution)
claude --permission-mode acceptAll
```

### Prompt Injection Defense

**Built-in Protections:**

Claude Code includes multiple layers of prompt injection defense:

1. **System Prompt Isolation** - User content cannot override system instructions
2. **Tool Call Validation** - All tool calls validated against permission rules
3. **Path Traversal Prevention** - File paths sanitized and validated
4. **Command Injection Prevention** - Bash commands validated and escaped
5. **Content Filtering** - Suspicious patterns detected and blocked

**Additional Safeguards:**

```json
{
  "security": {
    "enablePromptInjectionDefense": true,
    "validateAllToolCalls": true,
    "sanitizeFilePaths": true,
    "blockSuspiciousPatterns": true,
    "maxFileSize": "10MB",
    "maxBashCommandLength": 1000
  }
}
```

### Enterprise Security Controls

**Managed Policies:**

Organizations can enforce security policies at the system level:

```json
{
  "managedPolicies": {
    "enforcePermissions": true,
    "disableBypassMode": true,
    "requireMFA": true,
    "allowedModels": ["claude-sonnet-4-5"],
    "blockedTools": ["Bash(sudo *)", "Bash(rm -rf *)"],
    "auditLogging": true,
    "dataResidency": "us-east-1"
  }
}
```

**Role-Based Access Control:**

```json
{
  "roles": {
    "junior-developer": {
      "permissions": {
        "allow": ["Read(*)", "Edit(src/**/*.ts)", "Bash(npm test*)"],
        "deny": ["Bash(git push*)", "Write(config/*)"]
      }
    },
    "senior-developer": {
      "permissions": {
        "allow": ["*"],
        "deny": ["Bash(rm -rf *)", "Bash(sudo *)"]
      }
    },
    "security-auditor": {
      "permissions": {
        "allow": ["Read(*)", "Grep(*)", "Glob(*)"],
        "deny": ["Write(*)", "Edit(*)", "Bash(*)"]
      }
    }
  }
}
```

### Secret Detection & Prevention

**Automatic Secret Scanning:**

Claude Code can detect and prevent exposure of secrets:

```json
{
  "security": {
    "secretDetection": {
      "enabled": true,
      "blockCommits": true,
      "patterns": [
        "AKIA[0-9A-Z]{16}",              // AWS Access Keys
        "sk-[a-zA-Z0-9]{20,}",           // API Keys
        "ghp_[a-zA-Z0-9]{36}",           // GitHub Tokens
        "xox[baprs]-[0-9a-zA-Z]{10,48}", // Slack Tokens
        "-----BEGIN (RSA |)PRIVATE KEY-----" // Private Keys
      ]
    }
  }
}
```

**Pre-Commit Hook:**

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Write(*)",
        "hooks": [
          {
            "type": "command",
            "command": "git secrets --scan \"$FILE_PATH\"",
            "blockOnFailure": true
          }
        ]
      }
    ]
  }
}
```

### Authentication & Authorization

**API Key Management:**

```json
{
  "authentication": {
    "apiKeySource": "environment",  // or "file", "vault"
    "environmentVariable": "ANTHROPIC_API_KEY",
    "requireAuthentication": true,
    "sessionTimeout": 3600,
    "mfaRequired": false
  }
}
```

**SSO Integration:**

For enterprise deployments:

```json
{
  "authentication": {
    "provider": "okta",
    "domain": "company.okta.com",
    "clientId": "${OKTA_CLIENT_ID}",
    "scopes": ["openid", "profile", "email"]
  }
}
```

### Security Audit Logging

**Enable Comprehensive Logging:**

```json
{
  "security": {
    "auditLogging": {
      "enabled": true,
      "logLevel": "INFO",
      "logDestination": "/var/log/claude-code/audit.log",
      "logEvents": [
        "tool_use",
        "file_access",
        "bash_execution",
        "permission_denial",
        "authentication",
        "configuration_change"
      ],
      "includeContent": false,  // Don't log file contents
      "rotation": {
        "maxSize": "100MB",
        "maxAge": "30d",
        "compress": true
      }
    }
  }
}
```

**Audit Log Format:**

```json
{
  "timestamp": "2025-11-06T10:30:45.123Z",
  "event_type": "tool_use",
  "tool": "Bash",
  "command": "npm test",
  "user": "developer@company.com",
  "session_id": "abc123",
  "result": "success",
  "permission_mode": "acceptEdits",
  "working_directory": "/home/user/project"
}
```

### Vulnerability Scanning

**Integration with Security Tools:**

```json
{
  "security": {
    "vulnerabilityScanning": {
      "enabled": true,
      "scanOnWrite": true,
      "scanDependencies": true,
      "tools": [
        {
          "name": "snyk",
          "command": "snyk test --file=\"$FILE_PATH\"",
          "blockOnHigh": true
        },
        {
          "name": "semgrep",
          "command": "semgrep --config=auto \"$FILE_PATH\"",
          "blockOnError": true
        }
      ]
    }
  }
}
```

### Data Loss Prevention (DLP)

**Prevent Sensitive Data Exposure:**

```json
{
  "security": {
    "dlp": {
      "enabled": true,
      "blockPII": true,
      "blockCreditCards": true,
      "blockSSN": true,
      "customPatterns": [
        {
          "name": "employee-id",
          "pattern": "EMP-[0-9]{6}",
          "action": "block"
        },
        {
          "name": "internal-url",
          "pattern": "https?://internal\\.company\\.com",
          "action": "warn"
        }
      ]
    }
  }
}
```

### Security Best Practices

1. **Use Managed Policies** - Enforce security at organization level
2. **Enable Audit Logging** - Track all tool usage and file access
3. **Scan for Secrets** - Prevent credential leaks with pre-commit hooks
4. **Limit Bash Access** - Use native tools (Read, Edit) over Bash when possible
5. **Regular Security Reviews** - Audit permissions and access patterns
6. **Principle of Least Privilege** - Grant minimal necessary permissions
7. **Defense in Depth** - Layer multiple security controls
8. **Monitor and Alert** - Set up monitoring for suspicious activity

**Example Secure Enterprise Configuration:**

```json
{
  "permissions": {
    "defaultMode": "acceptEdits",
    "disableBypassPermissionsMode": "disable",
    "allow": [
      "Read(src/**/*)",
      "Read(tests/**/*)",
      "Edit(src/**/*.{ts,tsx,js,jsx})",
      "Grep(*)",
      "Glob(*)",
      "Bash(npm test*)",
      "Bash(git status)",
      "Bash(git diff*)"
    ],
    "deny": [
      "Read(.env*)",
      "Read(secrets/*)",
      "Read(*.key)",
      "Bash(rm -rf *)",
      "Bash(sudo *)",
      "Bash(curl * | bash)"
    ]
  },
  "security": {
    "auditLogging": {
      "enabled": true,
      "logDestination": "syslog"
    },
    "secretDetection": {
      "enabled": true,
      "blockCommits": true
    },
    "vulnerabilityScanning": {
      "enabled": true,
      "scanOnWrite": true
    },
    "dlp": {
      "enabled": true,
      "blockPII": true
    }
  },
  "authentication": {
    "provider": "okta",
    "mfaRequired": true
  }
}
```

---

## Section 3: Network Configuration

### Proxy Configuration

**HTTP/HTTPS Proxy:**

```bash
# Environment variables (recommended)
export HTTP_PROXY=http://proxy.company.com:8080
export HTTPS_PROXY=http://proxy.company.com:8080
export NO_PROXY=localhost,127.0.0.1,.local

# Start Claude Code
claude
```

**Configuration File:**

```json
{
  "network": {
    "proxy": {
      "http": "http://proxy.company.com:8080",
      "https": "http://proxy.company.com:8080",
      "noProxy": ["localhost", "127.0.0.1", "*.local", "internal.company.com"]
    },
    "timeout": 30000,
    "retries": 3
  }
}
```

**SOCKS Proxy:**

```json
{
  "network": {
    "proxy": {
      "protocol": "socks5",
      "host": "socks-proxy.company.com",
      "port": 1080,
      "auth": {
        "username": "${PROXY_USER}",
        "password": "${PROXY_PASS}"
      }
    }
  }
}
```

### Custom Certificates

**Self-Signed Certificate Support:**

```bash
# Set certificate path
export NODE_EXTRA_CA_CERTS=/path/to/corporate-ca-bundle.pem

# Or in settings.json
```

```json
{
  "network": {
    "tls": {
      "ca": "/path/to/corporate-ca-bundle.pem",
      "rejectUnauthorized": true
    }
  }
}
```

**Certificate Pinning:**

```json
{
  "network": {
    "tls": {
      "certificatePinning": {
        "enabled": true,
        "pins": {
          "api.anthropic.com": [
            "sha256/ABC123...",
            "sha256/DEF456..."
          ]
        }
      }
    }
  }
}
```

### Mutual TLS (mTLS)

**Client Certificate Authentication:**

```json
{
  "network": {
    "tls": {
      "clientCertificate": {
        "cert": "/path/to/client-cert.pem",
        "key": "/path/to/client-key.pem",
        "passphrase": "${CLIENT_CERT_PASSPHRASE}"
      }
    }
  }
}
```

### Required Network Endpoints

**Anthropic API Endpoints:**

Claude Code requires connectivity to these endpoints:

| Endpoint | Purpose | Port | Protocol |
|----------|---------|------|----------|
| `api.anthropic.com` | API requests | 443 | HTTPS |
| `claude.ai` | Authentication (optional) | 443 | HTTPS |
| `cdn.jsdelivr.net` | Asset delivery (optional) | 443 | HTTPS |

**Firewall Rules:**

```bash
# Allow outbound HTTPS to Anthropic API
iptables -A OUTPUT -p tcp --dport 443 -d api.anthropic.com -j ACCEPT

# Allow outbound to CDN (optional)
iptables -A OUTPUT -p tcp --dport 443 -d cdn.jsdelivr.net -j ACCEPT

# Block all other outbound (if desired)
iptables -A OUTPUT -p tcp --dport 443 -j DROP
```

**DNS Configuration:**

```json
{
  "network": {
    "dns": {
      "servers": ["8.8.8.8", "8.8.4.4"],
      "timeout": 5000,
      "retries": 3
    }
  }
}
```

### Network Isolation

**Air-Gapped Deployment:**

For completely offline deployments:

```json
{
  "network": {
    "mode": "offline",
    "cacheDirectory": "/var/cache/claude-code",
    "allowLocalhost": true,
    "allowPrivateNetworks": true,
    "blockInternet": true
  }
}
```

**Requirements for Offline Mode:**
- Pre-downloaded model cache
- Local MCP servers only
- No external API calls
- Local documentation and resources

### Rate Limiting

**Client-Side Rate Limiting:**

```json
{
  "network": {
    "rateLimiting": {
      "enabled": true,
      "maxRequestsPerMinute": 50,
      "maxConcurrentRequests": 5,
      "backoffStrategy": "exponential",
      "maxRetries": 3
    }
  }
}
```

### Network Monitoring

**Connection Monitoring:**

```json
{
  "network": {
    "monitoring": {
      "enabled": true,
      "logConnections": true,
      "logFailures": true,
      "alertOnFailure": true,
      "healthCheck": {
        "enabled": true,
        "interval": 60000,
        "endpoint": "https://api.anthropic.com/v1/health"
      }
    }
  }
}
```

### Troubleshooting Network Issues

**Diagnostic Commands:**

```bash
# Test connectivity
curl -v https://api.anthropic.com/v1/health

# Test with proxy
curl -x http://proxy.company.com:8080 https://api.anthropic.com/v1/health

# Test DNS resolution
nslookup api.anthropic.com

# Test certificate validation
openssl s_client -connect api.anthropic.com:443 -showcerts
```

**Common Issues:**

1. **Proxy Authentication Failures**
   - Solution: Verify credentials, check proxy logs
   - Use `--verbose` flag to see detailed error messages

2. **Certificate Validation Errors**
   - Solution: Install corporate CA bundle
   - Set `NODE_EXTRA_CA_CERTS` environment variable

3. **Timeout Errors**
   - Solution: Increase timeout values
   - Check firewall rules and network latency

4. **DNS Resolution Failures**
   - Solution: Configure custom DNS servers
   - Add entries to /etc/hosts if needed

### Network Best Practices

1. **Use Corporate Proxy** - Route all traffic through approved proxy
2. **Enable Certificate Validation** - Never disable certificate checks in production
3. **Monitor Network Usage** - Track API calls and bandwidth
4. **Configure Timeouts** - Set appropriate timeout values for your network
5. **Test Connectivity** - Verify network configuration before deployment
6. **Document Requirements** - Maintain list of required endpoints for firewall teams
7. **Plan for Failures** - Implement retry logic and graceful degradation

**Example Enterprise Network Configuration:**

```json
{
  "network": {
    "proxy": {
      "http": "http://proxy.company.com:8080",
      "https": "http://proxy.company.com:8080",
      "noProxy": ["localhost", "*.local"]
    },
    "tls": {
      "ca": "/etc/ssl/certs/corporate-ca-bundle.pem",
      "rejectUnauthorized": true,
      "clientCertificate": {
        "cert": "/etc/claude/client-cert.pem",
        "key": "/etc/claude/client-key.pem"
      }
    },
    "rateLimiting": {
      "enabled": true,
      "maxRequestsPerMinute": 50
    },
    "monitoring": {
      "enabled": true,
      "logConnections": true,
      "healthCheck": {
        "enabled": true,
        "interval": 60000
      }
    },
    "timeout": 30000,
    "retries": 3
  }
}
```

---

## Section 4: Usage Monitoring & Analytics

### OpenTelemetry Integration

**Enable Telemetry:**

```json
{
  "telemetry": {
    "enabled": true,
    "provider": "opentelemetry",
    "exporters": ["otlp", "console"],
    "samplingRate": 1.0,
    "includeSpans": true,
    "includeMetrics": true,
    "includeEvents": true
  }
}
```

**OpenTelemetry Configuration:**

```json
{
  "telemetry": {
    "opentelemetry": {
      "endpoint": "http://otel-collector:4318",
      "protocol": "http/protobuf",
      "headers": {
        "X-API-Key": "${OTEL_API_KEY}"
      },
      "compression": "gzip",
      "timeout": 10000,
      "retry": {
        "enabled": true,
        "maxAttempts": 3
      }
    }
  }
}
```

### Metrics Collection

**Available Metrics:**

```json
{
  "telemetry": {
    "metrics": {
      "enabled": true,
      "interval": 60000,
      "collect": [
        "api_requests_total",
        "api_request_duration_ms",
        "api_tokens_input",
        "api_tokens_output",
        "api_cost_usd",
        "tool_invocations_total",
        "tool_duration_ms",
        "file_operations_total",
        "bash_commands_total",
        "session_duration_ms",
        "errors_total",
        "permission_denials_total"
      ]
    }
  }
}
```

**Metric Labels:**

- `user` - User identifier
- `session_id` - Session identifier
- `model` - Model used (sonnet, opus, haiku)
- `tool` - Tool name (Bash, Read, Edit, etc.)
- `status` - Success/failure status
- `permission_mode` - Permission mode (readOnly, acceptEdits, etc.)

**Example Prometheus Scrape Configuration:**

```yaml
scrape_configs:
  - job_name: 'claude-code'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: '/metrics'
```

### Event Tracking

**Enable Event Tracking:**

```json
{
  "telemetry": {
    "events": {
      "enabled": true,
      "buffer": 100,
      "flushInterval": 30000,
      "track": [
        "session_start",
        "session_end",
        "tool_use",
        "file_write",
        "file_edit",
        "bash_execution",
        "error",
        "permission_denial",
        "user_feedback"
      ]
    }
  }
}
```

**Event Schema:**

```json
{
  "event_id": "uuid",
  "timestamp": "2025-11-06T10:30:45.123Z",
  "event_type": "tool_use",
  "user_id": "developer@company.com",
  "session_id": "abc123",
  "properties": {
    "tool": "Bash",
    "command": "npm test",
    "duration_ms": 1234,
    "success": true,
    "model": "sonnet"
  },
  "context": {
    "working_directory": "/home/user/project",
    "permission_mode": "acceptEdits"
  }
}
```

### Cost Tracking

**Monitor API Costs:**

```json
{
  "telemetry": {
    "costTracking": {
      "enabled": true,
      "currency": "USD",
      "alertThresholds": {
        "session": 1.0,
        "daily": 50.0,
        "monthly": 1000.0
      },
      "notificationChannels": ["email", "slack"]
    }
  }
}
```

**Cost Breakdown:**

```json
{
  "period": "2025-11",
  "total_cost_usd": 234.56,
  "breakdown": {
    "by_model": {
      "sonnet": 180.00,
      "opus": 54.56
    },
    "by_user": {
      "developer1@company.com": 100.00,
      "developer2@company.com": 134.56
    },
    "by_project": {
      "frontend": 150.00,
      "backend": 84.56
    }
  }
}
```

### Usage Reports

**Generate Usage Reports:**

```bash
# Daily usage summary
claude usage --period daily --format json > usage-daily.json

# Monthly cost report
claude usage --period monthly --breakdown user,model --format csv > usage-monthly.csv

# Real-time usage monitoring
claude usage --watch --interval 60
```

**Report Configuration:**

```json
{
  "reporting": {
    "enabled": true,
    "schedule": "0 9 * * *",  // Daily at 9 AM
    "format": "json",
    "destination": "/var/log/claude-code/reports/",
    "email": {
      "enabled": true,
      "recipients": ["team@company.com"],
      "subject": "Claude Code Daily Usage Report"
    },
    "slack": {
      "enabled": true,
      "webhook": "${SLACK_WEBHOOK_URL}",
      "channel": "#claude-usage"
    }
  }
}
```

### Enterprise Monitoring Dashboards

**Grafana Dashboard Configuration:**

```json
{
  "monitoring": {
    "dashboards": {
      "grafana": {
        "enabled": true,
        "url": "http://grafana.company.com",
        "apiKey": "${GRAFANA_API_KEY}",
        "dashboards": [
          {
            "name": "Claude Code Overview",
            "panels": [
              "api_requests_rate",
              "token_usage",
              "cost_trends",
              "error_rate",
              "session_duration"
            ]
          },
          {
            "name": "Security & Compliance",
            "panels": [
              "permission_denials",
              "secret_detection_events",
              "security_violations"
            ]
          }
        ]
      }
    }
  }
}
```

**DataDog Integration:**

```json
{
  "telemetry": {
    "datadog": {
      "enabled": true,
      "apiKey": "${DATADOG_API_KEY}",
      "site": "datadoghq.com",
      "service": "claude-code",
      "env": "production",
      "tags": [
        "team:engineering",
        "project:ai-tools"
      ]
    }
  }
}
```

**Splunk Integration:**

```json
{
  "telemetry": {
    "splunk": {
      "enabled": true,
      "hec": {
        "url": "https://splunk.company.com:8088/services/collector",
        "token": "${SPLUNK_HEC_TOKEN}",
        "index": "claude_code",
        "sourcetype": "claude:usage"
      }
    }
  }
}
```

### Real-Time Alerts

**Alert Configuration:**

```json
{
  "alerts": {
    "enabled": true,
    "channels": ["email", "slack", "pagerduty"],
    "rules": [
      {
        "name": "high_cost",
        "condition": "session_cost_usd > 5.0",
        "severity": "warning",
        "message": "Session cost exceeded $5.00"
      },
      {
        "name": "permission_denials",
        "condition": "permission_denials > 10 in 5m",
        "severity": "critical",
        "message": "Unusual number of permission denials"
      },
      {
        "name": "api_errors",
        "condition": "error_rate > 0.1",
        "severity": "critical",
        "message": "API error rate above 10%"
      }
    ]
  }
}
```

### Performance Monitoring

**Track Performance Metrics:**

```json
{
  "telemetry": {
    "performance": {
      "enabled": true,
      "trackSlowOperations": true,
      "slowThreshold": 1000,  // ms
      "metrics": [
        "request_latency",
        "tool_execution_time",
        "file_operation_time",
        "total_session_time"
      ]
    }
  }
}
```

**Performance Benchmarks:**

| Operation | Target | Warning | Critical |
|-----------|--------|---------|----------|
| API Request | <500ms | >1000ms | >3000ms |
| File Read | <50ms | >200ms | >500ms |
| File Write | <100ms | >500ms | >1000ms |
| Bash Command | <1000ms | >5000ms | >10000ms |

### Usage Quotas

**Enforce Usage Limits:**

```json
{
  "quotas": {
    "enabled": true,
    "enforcement": "hard",  // or "soft"
    "limits": {
      "session": {
        "max_turns": 100,
        "max_tokens": 100000,
        "max_cost_usd": 10.0,
        "max_duration_ms": 3600000
      },
      "daily": {
        "max_sessions": 50,
        "max_tokens": 1000000,
        "max_cost_usd": 100.0
      },
      "monthly": {
        "max_cost_usd": 2000.0
      }
    },
    "onExceeded": {
      "action": "block",  // or "warn"
      "notifyUser": true,
      "notifyAdmin": true
    }
  }
}
```

### Monitoring Best Practices

1. **Enable Comprehensive Telemetry** - Track all relevant metrics and events
2. **Set Up Alerts** - Monitor for cost, errors, and security issues
3. **Regular Reporting** - Generate and review usage reports
4. **Cost Optimization** - Track and optimize API costs per team/project
5. **Performance Monitoring** - Identify and address bottlenecks
6. **Security Monitoring** - Alert on permission denials and security events
7. **Capacity Planning** - Use historical data for forecasting
8. **Dashboard Visibility** - Make metrics accessible to stakeholders

**Example Enterprise Monitoring Configuration:**

```json
{
  "telemetry": {
    "enabled": true,
    "provider": "opentelemetry",
    "exporters": ["otlp"],
    "opentelemetry": {
      "endpoint": "http://otel-collector:4318"
    },
    "metrics": {
      "enabled": true,
      "interval": 60000
    },
    "events": {
      "enabled": true,
      "track": [
        "session_start",
        "session_end",
        "tool_use",
        "error",
        "permission_denial"
      ]
    },
    "costTracking": {
      "enabled": true,
      "alertThresholds": {
        "daily": 100.0,
        "monthly": 2000.0
      }
    },
    "datadog": {
      "enabled": true,
      "apiKey": "${DATADOG_API_KEY}",
      "service": "claude-code"
    }
  },
  "alerts": {
    "enabled": true,
    "channels": ["email", "slack"],
    "rules": [
      {
        "name": "high_cost",
        "condition": "session_cost_usd > 5.0",
        "severity": "warning"
      },
      {
        "name": "api_errors",
        "condition": "error_rate > 0.1",
        "severity": "critical"
      }
    ]
  },
  "quotas": {
    "enabled": true,
    "limits": {
      "daily": {
        "max_cost_usd": 100.0
      }
    }
  }
}
```

---

## Section 5: Data Privacy & Compliance

### Data Retention

**Claude Code Data Handling:**

- **Session Data** - Stored locally in `~/.claude/sessions/`
- **Conversation History** - Encrypted at rest, user-controlled retention
- **Telemetry Data** - Configurable retention period
- **Audit Logs** - Configurable storage location and retention

**Retention Configuration:**

```json
{
  "dataRetention": {
    "sessions": {
      "enabled": true,
      "retentionPeriod": "90d",
      "autoDelete": true,
      "location": "~/.claude/sessions/"
    },
    "auditLogs": {
      "enabled": true,
      "retentionPeriod": "365d",
      "location": "/var/log/claude-code/audit/",
      "encryption": {
        "enabled": true,
        "algorithm": "AES-256-GCM"
      }
    },
    "telemetry": {
      "retentionPeriod": "30d",
      "anonymize": true
    }
  }
}
```

### Training Opt-In/Out

**Control Data Usage:**

```json
{
  "telemetry": {
    "enabled": false,  // Completely disable telemetry
    "trainingOptOut": true,  // Opt out of model training
    "anonymize": true,  // Anonymize collected data
    "sendCrashReports": false,  // Disable crash reporting
    "sendUsageStats": false  // Disable usage statistics
  }
}
```

**CLI Option:**

```bash
# Disable telemetry for session
claude --no-telemetry

# Global opt-out
claude config set telemetry.enabled false
```

**Verify Opt-Out Status:**

```bash
# Check telemetry status
claude config get telemetry

# Output:
# {
#   "enabled": false,
#   "trainingOptOut": true
# }
```

### Data Anonymization

**Anonymize Telemetry Data:**

```json
{
  "telemetry": {
    "anonymization": {
      "enabled": true,
      "removeUserIdentifiers": true,
      "removeFilePaths": true,
      "removeCommands": false,
      "hashIdentifiers": true,
      "saltValue": "${ANONYMIZATION_SALT}"
    }
  }
}
```

**Anonymization Rules:**

| Data Type | Action | Example |
|-----------|--------|---------|
| User ID | Hash with salt | `user_abc123` → `hash_xyz789` |
| File Paths | Remove or generalize | `/home/user/project/src/app.ts` → `src/app.ts` |
| Bash Commands | Keep or redact | Configurable |
| Session IDs | Hash | `session_123` → `hash_456` |

### Compliance Features

**GDPR Compliance:**

```json
{
  "compliance": {
    "gdpr": {
      "enabled": true,
      "dataProcessingAgreement": true,
      "rightToErasure": {
        "enabled": true,
        "endpoint": "/api/gdpr/erase",
        "method": "POST"
      },
      "rightToAccess": {
        "enabled": true,
        "endpoint": "/api/gdpr/export",
        "method": "GET"
      },
      "consentManagement": {
        "required": true,
        "granular": true
      }
    }
  }
}
```

**HIPAA Compliance:**

```json
{
  "compliance": {
    "hipaa": {
      "enabled": true,
      "dataEncryption": {
        "atRest": true,
        "inTransit": true,
        "algorithm": "AES-256-GCM"
      },
      "auditLogging": {
        "enabled": true,
        "tamperProof": true
      },
      "accessControls": {
        "mfaRequired": true,
        "sessionTimeout": 900  // 15 minutes
      }
    }
  }
}
```

**SOC 2 Compliance:**

```json
{
  "compliance": {
    "soc2": {
      "enabled": true,
      "controls": {
        "accessControl": true,
        "changeManagement": true,
        "systemMonitoring": true,
        "dataProtection": true,
        "incidentResponse": true
      },
      "auditTrail": {
        "enabled": true,
        "immutable": true,
        "location": "/var/log/claude-code/audit/"
      }
    }
  }
}
```

### Data Residency

**Control Data Location:**

```json
{
  "dataResidency": {
    "region": "us-east-1",  // or "eu-west-1", "ap-southeast-1"
    "enforceRegion": true,
    "allowedRegions": ["us-east-1", "us-west-2"],
    "apiEndpoint": "https://api.us-east-1.anthropic.com"
  }
}
```

**Region Options:**

| Region | Location | Endpoint |
|--------|----------|----------|
| `us-east-1` | US East | `https://api.us-east-1.anthropic.com` |
| `us-west-2` | US West | `https://api.us-west-2.anthropic.com` |
| `eu-west-1` | EU (Ireland) | `https://api.eu-west-1.anthropic.com` |
| `ap-southeast-1` | Asia Pacific (Singapore) | `https://api.ap-southeast-1.anthropic.com` |

### Encryption

**Data Encryption Configuration:**

```json
{
  "encryption": {
    "atRest": {
      "enabled": true,
      "algorithm": "AES-256-GCM",
      "keyManagement": "kms",
      "keyId": "${KMS_KEY_ID}",
      "rotationPeriod": "90d"
    },
    "inTransit": {
      "enabled": true,
      "tlsVersion": "1.3",
      "cipherSuites": [
        "TLS_AES_256_GCM_SHA384",
        "TLS_CHACHA20_POLY1305_SHA256"
      ]
    }
  }
}
```

**Key Management:**

```json
{
  "keyManagement": {
    "provider": "aws-kms",  // or "azure-keyvault", "gcp-kms", "vault"
    "keyId": "${KMS_KEY_ID}",
    "region": "us-east-1",
    "rotationEnabled": true,
    "rotationPeriod": "90d"
  }
}
```

### Privacy-Focused Configuration

**Minimal Data Collection:**

```json
{
  "privacy": {
    "telemetry": {
      "enabled": false
    },
    "crashReports": {
      "enabled": false
    },
    "usageStats": {
      "enabled": false
    },
    "sessions": {
      "autoDelete": true,
      "retentionPeriod": "7d"
    },
    "auditLogs": {
      "includeContent": false,
      "anonymize": true
    }
  }
}
```

### Data Subject Rights

**GDPR Rights Implementation:**

```bash
# Right to Access - Export all user data
claude gdpr export --user user@company.com --format json

# Right to Erasure - Delete all user data
claude gdpr erase --user user@company.com --confirm

# Right to Rectification - Update user data
claude gdpr update --user user@company.com --field email --value new@company.com

# Right to Data Portability - Export in standard format
claude gdpr export --user user@company.com --format json --include-all
```

### Compliance Certifications

**Anthropic Compliance Status:**

- **SOC 2 Type II** - Certified
- **GDPR** - Compliant
- **HIPAA** - BAA available for eligible customers
- **ISO 27001** - Certified
- **CCPA** - Compliant

**Verify Compliance:**

```bash
# Check compliance status
claude compliance status

# Output:
# SOC 2 Type II: ✓ Certified
# GDPR: ✓ Compliant
# HIPAA: ✓ Available (BAA required)
# ISO 27001: ✓ Certified
```

### Audit Trail Requirements

**Compliance Audit Logging:**

```json
{
  "auditLogging": {
    "enabled": true,
    "format": "json",
    "destination": "syslog",
    "tamperProof": true,
    "includeFields": [
      "timestamp",
      "user_id",
      "action",
      "resource",
      "result",
      "ip_address",
      "user_agent"
    ],
    "excludeFields": [
      "file_content",
      "command_output"
    ],
    "retention": "7y",  // 7 years for compliance
    "encryption": {
      "enabled": true,
      "algorithm": "AES-256-GCM"
    },
    "integrity": {
      "enabled": true,
      "method": "blockchain-hash"  // Tamper-proof
    }
  }
}
```

### Privacy Best Practices

1. **Minimize Data Collection** - Only collect necessary data
2. **Opt-Out by Default** - Require explicit consent for telemetry
3. **Encrypt Everything** - Encrypt data at rest and in transit
4. **Regular Data Purging** - Automatically delete old data
5. **Access Controls** - Restrict who can access user data
6. **Audit Everything** - Maintain comprehensive audit logs
7. **Compliance Documentation** - Document all privacy controls
8. **Regular Reviews** - Audit privacy practices quarterly
9. **User Transparency** - Clear documentation of data practices
10. **Data Minimization** - Don't store what you don't need

**Example Privacy-Focused Enterprise Configuration:**

```json
{
  "telemetry": {
    "enabled": false,
    "trainingOptOut": true
  },
  "dataRetention": {
    "sessions": {
      "retentionPeriod": "30d",
      "autoDelete": true
    },
    "auditLogs": {
      "retentionPeriod": "365d",
      "encryption": {
        "enabled": true
      }
    }
  },
  "encryption": {
    "atRest": {
      "enabled": true,
      "algorithm": "AES-256-GCM"
    },
    "inTransit": {
      "enabled": true,
      "tlsVersion": "1.3"
    }
  },
  "compliance": {
    "gdpr": {
      "enabled": true,
      "rightToErasure": {
        "enabled": true
      }
    },
    "hipaa": {
      "enabled": true,
      "dataEncryption": {
        "atRest": true,
        "inTransit": true
      }
    }
  },
  "auditLogging": {
    "enabled": true,
    "tamperProof": true,
    "includeContent": false
  },
  "dataResidency": {
    "region": "eu-west-1",
    "enforceRegion": true
  }
}
```

---

## Section 6: Enterprise Features

### Managed Policies

**Organization-Wide Policy Enforcement:**

Managed policies allow organizations to enforce security and compliance policies across all users, overriding individual user settings.

**Policy Hierarchy:**

1. **Enterprise Managed Policies** (highest priority - cannot be overridden)
2. Command Line Arguments
3. Local Project Settings (`.claude/settings.local.json`)
4. Shared Project Settings (`.claude/settings.json`)
5. User Settings (`~/.claude/settings.json`) (lowest priority)

**Managed Policy Configuration:**

```json
{
  "managedPolicies": {
    "version": "1.0",
    "enforced": true,
    "allowOverrides": false,
    "policies": {
      "security": {
        "disableBypassMode": true,
        "requireMFA": true,
        "sessionTimeout": 3600,
        "maxConcurrentSessions": 3
      },
      "permissions": {
        "defaultMode": "acceptEdits",
        "globalDeny": [
          "Bash(rm -rf *)",
          "Bash(sudo *)",
          "Bash(curl * | bash)",
          "Read(.env*)",
          "Read(secrets/*)"
        ],
        "requireApproval": [
          "Bash(git push*)",
          "Bash(npm publish*)",
          "Write(/etc/*)"
        ]
      },
      "models": {
        "allowed": ["claude-sonnet-4-5", "claude-opus-4"],
        "default": "claude-sonnet-4-5",
        "costLimits": {
          "daily": 100.0,
          "monthly": 2000.0
        }
      },
      "monitoring": {
        "telemetryRequired": true,
        "auditLoggingRequired": true,
        "logDestination": "syslog://logs.company.com:514"
      },
      "compliance": {
        "gdpr": true,
        "hipaa": true,
        "soc2": true,
        "dataResidency": "us-east-1"
      }
    }
  }
}
```

**Deploy Managed Policies:**

```bash
# macOS
sudo cp managed-policies.json /Library/Application\ Support/Claude/policies.json

# Linux
sudo cp managed-policies.json /etc/claude/policies.json

# Windows
copy managed-policies.json "C:\ProgramData\Claude\policies.json"
```

### Organization Controls

**Centralized User Management:**

```json
{
  "organization": {
    "id": "org-abc123",
    "name": "Acme Corporation",
    "domain": "acme.com",
    "sso": {
      "enabled": true,
      "provider": "okta",
      "enforceSSO": true
    },
    "users": {
      "autoProvision": true,
      "defaultRole": "developer",
      "requireApproval": true
    },
    "teams": [
      {
        "id": "team-frontend",
        "name": "Frontend Team",
        "permissions": {
          "allow": ["Read(*)", "Edit(src/**/*.{ts,tsx})"]
        }
      },
      {
        "id": "team-backend",
        "name": "Backend Team",
        "permissions": {
          "allow": ["Read(*)", "Edit(api/**/*.ts)"]
        }
      }
    ]
  }
}
```

**Role-Based Access Control (RBAC):**

```json
{
  "rbac": {
    "enabled": true,
    "roles": {
      "junior-developer": {
        "permissions": {
          "allow": [
            "Read(*)",
            "Edit(src/**/*.ts)",
            "Grep(*)",
            "Glob(*)"
          ],
          "deny": [
            "Bash(git push*)",
            "Write(config/*)",
            "Bash(npm publish*)"
          ]
        },
        "modelAccess": ["claude-sonnet-4-5"],
        "costLimit": {
          "daily": 10.0,
          "monthly": 200.0
        }
      },
      "senior-developer": {
        "permissions": {
          "allow": ["*"],
          "deny": [
            "Bash(rm -rf *)",
            "Bash(sudo *)"
          ]
        },
        "modelAccess": ["claude-sonnet-4-5", "claude-opus-4"],
        "costLimit": {
          "daily": 50.0,
          "monthly": 1000.0
        }
      },
      "security-auditor": {
        "permissions": {
          "allow": [
            "Read(*)",
            "Grep(*)",
            "Glob(*)"
          ],
          "deny": [
            "Write(*)",
            "Edit(*)",
            "Bash(*)"
          ]
        },
        "modelAccess": ["claude-sonnet-4-5"],
        "costLimit": {
          "daily": 20.0,
          "monthly": 400.0
        },
        "auditAccess": true
      },
      "admin": {
        "permissions": {
          "allow": ["*"]
        },
        "modelAccess": ["claude-sonnet-4-5", "claude-opus-4"],
        "costLimit": {
          "daily": 200.0,
          "monthly": 5000.0
        },
        "manageUsers": true,
        "managePolicies": true,
        "auditAccess": true
      }
    },
    "inheritance": {
      "senior-developer": ["junior-developer"],
      "admin": ["senior-developer"]
    }
  }
}
```

### Centralized Configuration Management

**Configuration Distribution:**

```json
{
  "configManagement": {
    "enabled": true,
    "method": "pull",  // or "push"
    "source": {
      "type": "s3",
      "bucket": "acme-claude-configs",
      "path": "/policies/",
      "region": "us-east-1"
    },
    "sync": {
      "interval": 3600,  // 1 hour
      "autoUpdate": true,
      "notifyOnUpdate": true
    },
    "versioning": {
      "enabled": true,
      "rollback": true,
      "maxVersions": 10
    }
  }
}
```

**Alternative Configuration Sources:**

```json
{
  "configManagement": {
    "source": {
      "type": "git",
      "repository": "https://github.com/acme/claude-configs",
      "branch": "main",
      "path": "/policies/",
      "auth": {
        "method": "ssh-key",
        "keyPath": "/etc/claude/deploy-key"
      }
    }
  }
}
```

### License Management

**Enterprise Licensing:**

```json
{
  "licensing": {
    "type": "enterprise",
    "licenseKey": "${CLAUDE_ENTERPRISE_LICENSE}",
    "seats": 100,
    "expirationDate": "2026-01-01",
    "features": [
      "managed-policies",
      "sso",
      "audit-logging",
      "advanced-monitoring",
      "custom-models",
      "dedicated-support"
    ],
    "enforcement": {
      "blockOnExpiry": true,
      "warningPeriod": 30  // days
    }
  }
}
```

**License Server:**

```json
{
  "licensing": {
    "server": {
      "url": "https://licenses.acme.com/api/v1",
      "checkInterval": 86400,  // 24 hours
      "cacheValidity": 604800,  // 7 days
      "offlineGrace": 604800  // 7 days
    }
  }
}
```

### SSO Integration

**SAML Configuration:**

```json
{
  "authentication": {
    "provider": "saml",
    "saml": {
      "entryPoint": "https://sso.acme.com/saml/sso",
      "issuer": "claude-code",
      "cert": "/etc/claude/saml-cert.pem",
      "identifierFormat": "urn:oasis:names:tc:SAML:1.1:nameid-format:emailAddress",
      "signatureAlgorithm": "sha256",
      "authnContext": [
        "urn:oasis:names:tc:SAML:2.0:ac:classes:PasswordProtectedTransport"
      ]
    },
    "enforceSSO": true,
    "localAuthFallback": false
  }
}
```

**Okta Configuration:**

```json
{
  "authentication": {
    "provider": "okta",
    "okta": {
      "domain": "acme.okta.com",
      "clientId": "${OKTA_CLIENT_ID}",
      "clientSecret": "${OKTA_CLIENT_SECRET}",
      "authorizationServerId": "default",
      "redirectUri": "http://localhost:8080/callback",
      "scopes": ["openid", "profile", "email", "groups"],
      "pkce": true
    },
    "sessionManagement": {
      "timeout": 3600,
      "refreshTokenRotation": true,
      "maxSessionDuration": 28800  // 8 hours
    }
  }
}
```

**Azure AD Configuration:**

```json
{
  "authentication": {
    "provider": "azure-ad",
    "azureAd": {
      "tenantId": "${AZURE_TENANT_ID}",
      "clientId": "${AZURE_CLIENT_ID}",
      "clientSecret": "${AZURE_CLIENT_SECRET}",
      "authority": "https://login.microsoftonline.com/${AZURE_TENANT_ID}",
      "redirectUri": "http://localhost:8080/callback",
      "scopes": ["User.Read", "Group.Read.All"]
    }
  }
}
```

### Cost Allocation

**Departmental Cost Tracking:**

```json
{
  "costAllocation": {
    "enabled": true,
    "dimensions": [
      "department",
      "project",
      "environment"
    ],
    "tags": {
      "department": "${USER_DEPARTMENT}",
      "project": "${PROJECT_NAME}",
      "environment": "production",
      "costCenter": "${COST_CENTER}"
    },
    "reporting": {
      "enabled": true,
      "schedule": "0 9 * * *",  // Daily at 9 AM
      "recipients": [
        "finance@acme.com",
        "engineering-leads@acme.com"
      ],
      "format": "csv",
      "breakdown": ["department", "project", "user"]
    },
    "budgets": {
      "engineering": {
        "monthly": 10000.0,
        "alertThresholds": [0.5, 0.8, 0.9, 1.0]
      },
      "product": {
        "monthly": 5000.0,
        "alertThresholds": [0.5, 0.8, 0.9, 1.0]
      }
    }
  }
}
```

### Centralized Logging

**Enterprise Log Aggregation:**

```json
{
  "logging": {
    "enabled": true,
    "level": "INFO",
    "destinations": [
      {
        "type": "syslog",
        "host": "logs.acme.com",
        "port": 514,
        "protocol": "tcp",
        "facility": "user",
        "format": "rfc5424"
      },
      {
        "type": "file",
        "path": "/var/log/claude-code/app.log",
        "rotation": {
          "maxSize": "100MB",
          "maxAge": "30d",
          "maxFiles": 10,
          "compress": true
        }
      },
      {
        "type": "elasticsearch",
        "hosts": ["https://es.acme.com:9200"],
        "index": "claude-code-logs",
        "auth": {
          "username": "${ES_USER}",
          "password": "${ES_PASS}"
        }
      }
    ],
    "includeFields": [
      "timestamp",
      "level",
      "user_id",
      "session_id",
      "message",
      "tool",
      "command"
    ],
    "sensitiveFields": [
      "api_key",
      "password",
      "token"
    ],
    "redaction": {
      "enabled": true,
      "patterns": [
        "api[_-]?key",
        "password",
        "token",
        "secret"
      ]
    }
  }
}
```

### Update Management

**Controlled Updates:**

```json
{
  "updates": {
    "autoUpdate": false,
    "channel": "stable",  // or "beta", "canary"
    "schedule": "0 2 * * 0",  // Sundays at 2 AM
    "staging": {
      "enabled": true,
      "testGroups": ["qa-team", "early-adopters"],
      "rolloutDelay": 604800  // 7 days
    },
    "rollback": {
      "autoRollback": true,
      "errorThreshold": 0.1,  // 10% error rate
      "monitoringPeriod": 86400  // 24 hours
    },
    "approvals": {
      "required": true,
      "approvers": ["admin@acme.com"],
      "notificationChannels": ["email", "slack"]
    }
  }
}
```

### Support & SLA

**Enterprise Support Configuration:**

```json
{
  "support": {
    "tier": "enterprise",
    "sla": {
      "responseTime": {
        "critical": 60,  // minutes
        "high": 240,     // 4 hours
        "medium": 480,   // 8 hours
        "low": 1440      // 24 hours
      },
      "availability": "24/7/365"
    },
    "channels": {
      "email": "support@anthropic.com",
      "slack": "#anthropic-support",
      "phone": "+1-555-0100",
      "portal": "https://support.anthropic.com"
    },
    "accountManager": {
      "name": "Jane Smith",
      "email": "jane.smith@anthropic.com",
      "phone": "+1-555-0101"
    }
  }
}
```

### Enterprise Best Practices

1. **Centralized Policy Management** - Deploy managed policies across organization
2. **SSO Integration** - Enforce single sign-on for all users
3. **Role-Based Access** - Define roles with appropriate permissions
4. **Cost Allocation** - Track and allocate costs by department/project
5. **Comprehensive Monitoring** - Centralized logging and metrics
6. **Regular Audits** - Review usage, costs, and compliance quarterly
7. **Staged Rollouts** - Test updates with pilot groups before full deployment
8. **Dedicated Support** - Leverage enterprise support for critical issues
9. **Documentation** - Maintain internal documentation for policies and procedures
10. **Training** - Regular training for users on security and best practices

**Example Complete Enterprise Configuration:**

```json
{
  "managedPolicies": {
    "version": "1.0",
    "enforced": true,
    "policies": {
      "security": {
        "disableBypassMode": true,
        "requireMFA": true,
        "sessionTimeout": 3600
      },
      "permissions": {
        "defaultMode": "acceptEdits",
        "globalDeny": [
          "Bash(rm -rf *)",
          "Bash(sudo *)",
          "Read(.env*)"
        ]
      },
      "monitoring": {
        "telemetryRequired": true,
        "auditLoggingRequired": true
      }
    }
  },
  "authentication": {
    "provider": "okta",
    "okta": {
      "domain": "acme.okta.com",
      "clientId": "${OKTA_CLIENT_ID}"
    },
    "enforceSSO": true
  },
  "rbac": {
    "enabled": true,
    "roles": {
      "junior-developer": {
        "permissions": {
          "allow": ["Read(*)", "Edit(src/**/*.ts)"]
        },
        "costLimit": {
          "daily": 10.0
        }
      },
      "senior-developer": {
        "permissions": {
          "allow": ["*"],
          "deny": ["Bash(rm -rf *)", "Bash(sudo *)"]
        },
        "costLimit": {
          "daily": 50.0
        }
      }
    }
  },
  "costAllocation": {
    "enabled": true,
    "dimensions": ["department", "project"],
    "reporting": {
      "enabled": true,
      "schedule": "0 9 * * *",
      "recipients": ["finance@acme.com"]
    }
  },
  "logging": {
    "enabled": true,
    "destinations": [
      {
        "type": "syslog",
        "host": "logs.acme.com",
        "port": 514
      }
    ]
  },
  "network": {
    "proxy": {
      "http": "http://proxy.acme.com:8080",
      "https": "http://proxy.acme.com:8080"
    },
    "tls": {
      "ca": "/etc/ssl/certs/acme-ca-bundle.pem"
    }
  },
  "compliance": {
    "gdpr": {
      "enabled": true
    },
    "soc2": {
      "enabled": true
    }
  }
}
```

---

## Section 7: Advanced Configuration Examples

### Example 1: High-Security Development Environment

**Use Case:** Financial services company with strict security requirements

```json
{
  "permissions": {
    "defaultMode": "readOnly",
    "disableBypassPermissionsMode": "disable",
    "allow": [
      "Read(src/**/*)",
      "Read(tests/**/*)",
      "Grep(*)",
      "Glob(*)"
    ],
    "ask": [
      "Edit(src/**/*.{ts,tsx,js,jsx})",
      "Write(tests/**/*.test.ts)",
      "Bash(npm test*)",
      "Bash(git status)",
      "Bash(git diff*)"
    ],
    "deny": [
      "Read(.env*)",
      "Read(secrets/*)",
      "Read(*.key)",
      "Read(*.pem)",
      "Write(.env*)",
      "Bash(rm -rf *)",
      "Bash(sudo *)",
      "Bash(curl * | bash)",
      "Bash(git push*)"
    ]
  },
  "security": {
    "auditLogging": {
      "enabled": true,
      "logDestination": "syslog://logs.finserv.com:514",
      "includeContent": false
    },
    "secretDetection": {
      "enabled": true,
      "blockCommits": true
    },
    "vulnerabilityScanning": {
      "enabled": true,
      "scanOnWrite": true
    },
    "dlp": {
      "enabled": true,
      "blockPII": true,
      "blockCreditCards": true,
      "blockSSN": true
    }
  },
  "network": {
    "proxy": {
      "http": "http://proxy.finserv.com:8080",
      "https": "http://proxy.finserv.com:8080"
    },
    "tls": {
      "ca": "/etc/ssl/certs/finserv-ca-bundle.pem",
      "rejectUnauthorized": true
    }
  },
  "authentication": {
    "provider": "saml",
    "mfaRequired": true,
    "sessionTimeout": 1800  // 30 minutes
  },
  "compliance": {
    "soc2": {
      "enabled": true
    },
    "pciDss": {
      "enabled": true
    }
  },
  "telemetry": {
    "enabled": false,
    "trainingOptOut": true
  }
}
```

### Example 2: AI-Assisted SRE Operations

**Use Case:** SRE team with 24/7 incident response

```json
{
  "permissions": {
    "defaultMode": "acceptEdits",
    "allow": [
      "Read(*)",
      "Grep(*)",
      "Glob(*)",
      "Bash(kubectl *)",
      "Bash(docker *)",
      "Bash(aws *)",
      "Bash(git status)",
      "Bash(git diff*)",
      "Bash(curl *)",
      "Bash(wget *)",
      "Bash(ping *)",
      "Bash(netstat *)",
      "Bash(ps *)",
      "Bash(top)",
      "Bash(htop)"
    ],
    "ask": [
      "Bash(kubectl delete*)",
      "Bash(kubectl scale*)",
      "Bash(docker rm*)",
      "Bash(aws ec2 terminate*)",
      "Edit(*.yaml)",
      "Edit(*.json)",
      "Write(manifests/*)"
    ],
    "deny": [
      "Bash(rm -rf *)",
      "Bash(sudo rm*)",
      "Read(.env*)"
    ],
    "additionalDirectories": [
      "/var/log",
      "/etc/kubernetes",
      "/home/sre/runbooks"
    ]
  },
  "mcpServers": {
    "datadog": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-datadog"],
      "env": {
        "DATADOG_API_KEY": "${DATADOG_API_KEY}",
        "DATADOG_APP_KEY": "${DATADOG_APP_KEY}"
      }
    },
    "pagerduty": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-pagerduty"],
      "env": {
        "PAGERDUTY_API_KEY": "${PAGERDUTY_API_KEY}"
      }
    },
    "kubernetes": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-kubernetes"],
      "env": {
        "KUBECONFIG": "${KUBECONFIG}"
      }
    }
  },
  "agents": {
    "incident-responder": {
      "description": "SRE expert for incident response. Use proactively when errors or incidents occur.",
      "prompt": "You are an SRE expert specializing in incident response. Diagnose issues, assess impact, provide immediate action items, and suggest preventive measures.",
      "tools": ["Read", "Bash", "WebSearch", "mcp__datadog", "mcp__pagerduty", "mcp__kubernetes"],
      "model": "opus"
    }
  },
  "telemetry": {
    "enabled": true,
    "datadog": {
      "enabled": true,
      "apiKey": "${DATADOG_API_KEY}",
      "service": "claude-code-sre"
    }
  },
  "alerts": {
    "enabled": true,
    "channels": ["pagerduty", "slack"],
    "rules": [
      {
        "name": "api_errors",
        "condition": "error_rate > 0.1",
        "severity": "critical"
      }
    ]
  }
}
```

### Example 3: AI-Powered Code Review

**Use Case:** Automated code review for pull requests

```json
{
  "permissions": {
    "defaultMode": "readOnly",
    "allow": [
      "Read(*)",
      "Grep(*)",
      "Glob(*)",
      "Bash(git log*)",
      "Bash(git diff*)",
      "Bash(git show*)",
      "Bash(npm test*)",
      "Bash(npm run lint*)"
    ],
    "deny": [
      "Write(*)",
      "Edit(*)",
      "Bash(git push*)",
      "Bash(git commit*)"
    ]
  },
  "agents": {
    "security-reviewer": {
      "description": "Security-focused code reviewer. Use proactively after analyzing code changes.",
      "prompt": "You are a security expert. Review code for OWASP Top 10 vulnerabilities, insecure patterns, credential leaks, and security anti-patterns. Provide actionable recommendations.",
      "tools": ["Read", "Grep", "Glob"],
      "model": "opus"
    },
    "performance-reviewer": {
      "description": "Performance optimization expert. Use when analyzing code efficiency.",
      "prompt": "You are a performance optimization expert. Review code for performance bottlenecks, inefficient algorithms, memory leaks, and suggest optimizations.",
      "tools": ["Read", "Grep", "Glob"],
      "model": "sonnet"
    },
    "accessibility-reviewer": {
      "description": "Accessibility specialist. Use when reviewing UI code.",
      "prompt": "You are an accessibility expert. Review code for WCAG 2.1 AA compliance, semantic HTML, ARIA attributes, keyboard navigation, and screen reader compatibility.",
      "tools": ["Read", "Grep", "Glob"],
      "model": "sonnet"
    }
  },
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Read(src/**/*.{ts,tsx,js,jsx})",
        "hooks": [
          {
            "type": "agent",
            "agent": "security-reviewer",
            "autoInvoke": true
          }
        ]
      }
    ]
  },
  "telemetry": {
    "enabled": true,
    "events": {
      "track": ["session_start", "session_end", "tool_use", "agent_invocation"]
    }
  }
}
```

### Example 4: AI-Assisted Documentation

**Use Case:** Automated documentation generation and maintenance

```json
{
  "permissions": {
    "defaultMode": "acceptEdits",
    "allow": [
      "Read(*)",
      "Edit(docs/**/*.md)",
      "Edit(README.md)",
      "Edit(*.md)",
      "Write(docs/**/*.md)",
      "Grep(*)",
      "Glob(*)",
      "Bash(git status)",
      "Bash(git diff*)"
    ],
    "ask": [
      "Bash(git add*)",
      "Bash(git commit*)"
    ],
    "deny": [
      "Edit(src/**/*)",
      "Bash(rm -rf *)"
    ]
  },
  "agents": {
    "documentation-writer": {
      "description": "Technical documentation specialist. Use for creating or updating documentation.",
      "prompt": "You are a technical writer specializing in developer documentation. Write clear, concise, well-structured documentation following best practices. Include code examples, API references, and tutorials.",
      "tools": ["Read", "Edit", "Write", "Grep", "Glob"],
      "model": "sonnet"
    },
    "api-documenter": {
      "description": "API documentation expert. Use when documenting APIs or SDKs.",
      "prompt": "You are an API documentation expert. Generate comprehensive API documentation including endpoints, parameters, request/response examples, error codes, and authentication details.",
      "tools": ["Read", "Edit", "Write", "Grep", "Glob"],
      "model": "sonnet"
    }
  },
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write(docs/**/*.md)",
        "hooks": [
          {
            "type": "command",
            "command": "npx markdownlint \"$FILE_PATH\" --fix"
          }
        ]
      }
    ]
  },
  "outputStyle": "explanatory"
}
```

### Example 5: Compliance-Focused Healthcare Environment

**Use Case:** Healthcare application with HIPAA requirements

```json
{
  "permissions": {
    "defaultMode": "acceptEdits",
    "disableBypassPermissionsMode": "disable",
    "allow": [
      "Read(src/**/*)",
      "Read(tests/**/*)",
      "Edit(src/**/*.{ts,tsx,js,jsx})",
      "Grep(*)",
      "Glob(*)",
      "Bash(npm test*)",
      "Bash(git status)",
      "Bash(git diff*)"
    ],
    "deny": [
      "Read(.env*)",
      "Read(patient-data/*)",
      "Read(phi/*)",
      "Read(*.pem)",
      "Read(*.key)",
      "Bash(rm -rf *)",
      "Bash(sudo *)"
    ]
  },
  "security": {
    "auditLogging": {
      "enabled": true,
      "logDestination": "/var/log/claude-code/audit.log",
      "tamperProof": true,
      "includeContent": false,
      "encryption": {
        "enabled": true,
        "algorithm": "AES-256-GCM"
      }
    },
    "dlp": {
      "enabled": true,
      "blockPII": true,
      "blockSSN": true,
      "blockHealthData": true,
      "customPatterns": [
        {
          "name": "patient-id",
          "pattern": "P-[0-9]{8}",
          "action": "block"
        },
        {
          "name": "medical-record-number",
          "pattern": "MRN-[0-9]{10}",
          "action": "block"
        }
      ]
    },
    "secretDetection": {
      "enabled": true,
      "blockCommits": true
    }
  },
  "authentication": {
    "provider": "okta",
    "mfaRequired": true,
    "sessionTimeout": 900  // 15 minutes (HIPAA requirement)
  },
  "compliance": {
    "hipaa": {
      "enabled": true,
      "dataEncryption": {
        "atRest": true,
        "inTransit": true
      },
      "auditLogging": {
        "enabled": true,
        "tamperProof": true
      },
      "accessControls": {
        "mfaRequired": true,
        "sessionTimeout": 900
      }
    }
  },
  "telemetry": {
    "enabled": false,
    "trainingOptOut": true
  },
  "dataRetention": {
    "sessions": {
      "retentionPeriod": "7d",
      "autoDelete": true
    },
    "auditLogs": {
      "retentionPeriod": "2555d",  // 7 years (HIPAA requirement)
      "encryption": {
        "enabled": true
      }
    }
  },
  "dataResidency": {
    "region": "us-east-1",
    "enforceRegion": true
  }
}
```

### Example 6: Multi-Project Development with MCP

**Use Case:** Full-stack developer working across multiple projects with various integrations

```json
{
  "workingDirectories": [
    "/home/dev/frontend-app",
    "/home/dev/backend-api",
    "/home/dev/mobile-app",
    "/home/dev/shared-lib"
  ],
  "permissions": {
    "defaultMode": "acceptEdits",
    "allow": [
      "Read(*)",
      "Edit(src/**/*)",
      "Edit(tests/**/*)",
      "Write(src/**/*)",
      "Write(tests/**/*)",
      "Grep(*)",
      "Glob(*)",
      "Bash(npm *)",
      "Bash(git *)",
      "Bash(docker *)"
    ],
    "deny": [
      "Read(.env*)",
      "Bash(rm -rf *)",
      "Bash(sudo *)"
    ]
  },
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "${GITHUB_TOKEN}"
      }
    },
    "jira": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-jira"],
      "env": {
        "JIRA_URL": "https://company.atlassian.net",
        "JIRA_EMAIL": "${JIRA_EMAIL}",
        "JIRA_API_TOKEN": "${JIRA_API_TOKEN}"
      }
    },
    "slack": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-slack"],
      "env": {
        "SLACK_BOT_TOKEN": "${SLACK_BOT_TOKEN}",
        "SLACK_TEAM_ID": "${SLACK_TEAM_ID}"
      }
    },
    "postgres": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres"],
      "env": {
        "DATABASE_URL": "${DATABASE_URL}"
      }
    }
  },
  "agents": {
    "full-stack-developer": {
      "description": "Full-stack development expert. Use for implementing features across frontend, backend, and database.",
      "prompt": "You are a full-stack developer expert in React, Node.js, TypeScript, and PostgreSQL. Implement features following best practices, write tests, and ensure proper error handling.",
      "tools": ["Read", "Edit", "Write", "Bash", "mcp__github", "mcp__jira", "mcp__postgres"],
      "model": "sonnet"
    }
  },
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit(*.{ts,tsx,js,jsx})",
        "hooks": [
          {
            "type": "command",
            "command": "npx prettier --write \"$FILE_PATH\""
          },
          {
            "type": "command",
            "command": "npx eslint --fix \"$FILE_PATH\""
          }
        ]
      },
      {
        "matcher": "Write(tests/**/*.test.ts)",
        "hooks": [
          {
            "type": "command",
            "command": "npm test \"$FILE_PATH\""
          }
        ]
      }
    ]
  }
}
```

---

## Section 8: Headless Mode & Automation

### Basic Headless Usage

**Non-Interactive Mode:**

```bash
# Single query
claude -p "Analyze this codebase and suggest improvements"

# With JSON output
claude -p "Generate tests for src/app.ts" --output-format json

# From stdin
echo "Explain this function" | claude -p

# With file input
cat analysis.txt | claude -p --output-format json > results.json
```

### Output Formats

**Text Output (Default):**

```bash
claude -p "Explain file src/components/Header.tsx"
# Output: This is a React component showing...
```

**JSON Output:**

```bash
claude -p "How does the data layer work?" --output-format json

# Response format:
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

**Streaming JSON Output:**

```bash
claude -p "Build an application" --output-format stream-json

# Each message emitted as separate JSON object
{"type":"init","model":"sonnet"}
{"type":"user","message":{"role":"user","content":"Build an application"}}
{"type":"assistant","message":{"role":"assistant","content":"..."}}
{"type":"result","total_cost_usd":0.05,"session_id":"abc123"}
```

### Multi-Turn Conversations

**Resume Conversations:**

```bash
# Continue most recent conversation
claude --continue "Now refactor this for better performance"

# Resume specific session
claude --resume 550e8400-e29b-41d4-a716-446655440000 "Update the tests"

# Resume in non-interactive mode
claude --resume abc123 "Fix all linting issues" --print
```

### Tool Control in Headless Mode

**Allow/Deny Specific Tools:**

```bash
# Allow only specific tools
claude -p "Stage my changes" \
  --allowedTools "Bash(git add),Bash(git status),Read"

# Deny specific tools
claude -p "Analyze code" \
  --disallowedTools "Write,Bash(rm*)"

# Complex tool patterns
claude -p "Review and test" \
  --allowedTools "Read(*),Bash(npm test*),mcp__github" \
  --disallowedTools "Bash(git push*)"
```

**Permission Modes:**

```bash
# Read-only mode
claude -p "Analyze codebase" --permission-mode readOnly

# Accept edits (prompts for Write/Bash)
claude -p "Refactor code" --permission-mode acceptEdits

# Accept all (dangerous - use carefully)
claude -p "Deploy changes" --permission-mode acceptAll \
  --dangerously-skip-permissions
```

### Agent Integration Examples

**SRE Incident Response Bot:**

```bash
#!/bin/bash

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

**Automated Security Review:**

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

**Multi-Turn Legal Assistant:**

```bash
# Legal document review with session persistence
session_id=$(claude -p "Start legal review session" --output-format json | jq -r '.session_id')

# Review contract in multiple steps
claude -p --resume "$session_id" "Review contract.pdf for liability clauses"
claude -p --resume "$session_id" "Check compliance with GDPR requirements"
claude -p --resume "$session_id" "Generate executive summary of risks"
```

**Automated Code Generation Pipeline:**

```bash
#!/bin/bash

generate_feature() {
    local feature_name="$1"
    local spec_file="$2"

    # Step 1: Generate implementation
    claude -p --output-format json \
      "Generate TypeScript implementation for feature: $feature_name based on spec in $spec_file" \
      --allowedTools "Read,Write(src/**/*)" > impl_result.json

    # Step 2: Generate tests
    claude -p --output-format json \
      "Generate comprehensive tests for the implementation" \
      --allowedTools "Read,Write(tests/**/*)" > test_result.json

    # Step 3: Run tests
    npm test

    # Step 4: Generate documentation
    claude -p --output-format json \
      "Generate API documentation for the new feature" \
      --allowedTools "Read,Write(docs/**/*)" > doc_result.json
}

# Usage
generate_feature "user-authentication" "specs/auth-spec.md"
```

### Streaming JSON Input/Output

**Bidirectional Streaming:**

```bash
# Streaming JSON input (JSONL format)
echo '{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Explain this code"}]}}' | \
  claude -p --output-format=stream-json --input-format=stream-json --verbose
```

**Multi-Turn Streaming:**

```bash
#!/bin/bash

# Interactive streaming conversation
cat <<EOF | claude -p --output-format=stream-json --input-format=stream-json
{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Start code review"}]}}
{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Focus on security"}]}}
{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Generate report"}]}}
EOF
```

### CI/CD Integration

**GitHub Actions:**

```yaml
name: AI Code Review
on: pull_request

jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Claude Code
        run: npm install -g claude-code

      - name: Review PR
        env:
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
        run: |
          claude -p "Review this PR for security issues and best practices" \
            --output-format json \
            --allowedTools "Read,Grep,Glob" \
            --permission-mode readOnly > review.json

      - name: Post Review Comment
        run: |
          result=$(jq -r '.result' review.json)
          gh pr comment ${{ github.event.pull_request.number }} --body "$result"
```

**GitLab CI:**

```yaml
ai-review:
  stage: review
  script:
    - npm install -g claude-code
    - |
      claude -p "Analyze code changes for quality and security" \
        --output-format json \
        --allowedTools "Read,Grep" \
        --permission-mode readOnly > review.json
    - cat review.json
  artifacts:
    reports:
      review: review.json
```

### Monitoring & Logging in Headless Mode

**Structured Logging:**

```bash
# Enable verbose logging
claude -p "Deploy application" \
  --verbose \
  --output-format json 2>&1 | tee deployment.log

# Parse logs with jq
cat deployment.log | jq 'select(.type == "error")'
```

**Cost Tracking:**

```bash
# Extract cost from JSON output
cost=$(claude -p "Generate tests" --output-format json | jq -r '.total_cost_usd')
echo "Operation cost: \$$cost"

# Accumulate daily costs
date=$(date +%Y-%m-%d)
echo "$cost" >> costs-$date.txt
daily_total=$(awk '{sum+=$1} END {print sum}' costs-$date.txt)
echo "Daily total: \$$daily_total"
```

### Error Handling & Retries

**Robust Error Handling:**

```bash
#!/bin/bash

run_with_retry() {
    local max_attempts=3
    local timeout=300
    local attempt=1

    while [ $attempt -le $max_attempts ]; do
        echo "Attempt $attempt of $max_attempts"

        if timeout $timeout claude -p "$1" --output-format json 2>error.log; then
            echo "Success on attempt $attempt"
            return 0
        fi

        echo "Attempt $attempt failed:" >&2
        cat error.log >&2

        if [ $attempt -eq $max_attempts ]; then
            echo "Max attempts reached, failing" >&2
            return 1
        fi

        sleep $((attempt * 10))  # Exponential backoff
        attempt=$((attempt + 1))
    done
}

# Usage
run_with_retry "Analyze codebase and generate report"
```

### Batch Processing

**Process Multiple Files:**

```bash
#!/bin/bash

# Process all TypeScript files
find src -name "*.ts" -type f | while read -r file; do
    echo "Processing: $file"

    result=$(claude -p "Review file $file for issues" \
      --output-format json \
      --allowedTools "Read" \
      --permission-mode readOnly)

    echo "$result" > "reviews/$(basename $file .ts)-review.json"
done

# Aggregate results
jq -s '.' reviews/*.json > final-report.json
```

### Best Practices for Headless Mode

1. **Use JSON Output** - Easier to parse programmatically
2. **Handle Errors Gracefully** - Check exit codes and parse stderr
3. **Implement Retries** - Network and API failures happen
4. **Set Timeouts** - Prevent hanging processes
5. **Rate Limiting** - Add delays between requests
6. **Session Management** - Use --resume for multi-turn conversations
7. **Cost Monitoring** - Track API costs in automation
8. **Logging** - Comprehensive logging for debugging
9. **Permission Control** - Use --allowedTools for security
10. **Idempotency** - Design scripts to be safely re-runnable

**Example Production-Ready Script:**

```bash
#!/bin/bash
set -euo pipefail

# Configuration
MAX_RETRIES=3
TIMEOUT=300
LOG_DIR="/var/log/claude-automation"
COST_THRESHOLD=10.0

# Logging setup
mkdir -p "$LOG_DIR"
LOG_FILE="$LOG_DIR/$(date +%Y%m%d-%H%M%S).log"
exec 1> >(tee -a "$LOG_FILE")
exec 2>&1

log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*"
}

# Error handling
trap 'log "ERROR: Script failed on line $LINENO"' ERR

# Cost tracking
TOTAL_COST=0.0

run_claude() {
    local prompt="$1"
    local attempt=1

    while [ $attempt -le $MAX_RETRIES ]; do
        log "Attempt $attempt: $prompt"

        if result=$(timeout $TIMEOUT claude -p "$prompt" \
            --output-format json \
            --allowedTools "Read,Grep,Glob" \
            --permission-mode readOnly 2>&1); then

            # Extract cost
            cost=$(echo "$result" | jq -r '.total_cost_usd // 0')
            TOTAL_COST=$(echo "$TOTAL_COST + $cost" | bc)

            log "Success (cost: \$$cost, total: \$$TOTAL_COST)"

            # Check cost threshold
            if (( $(echo "$TOTAL_COST > $COST_THRESHOLD" | bc -l) )); then
                log "ERROR: Cost threshold exceeded (\$$TOTAL_COST > \$$COST_THRESHOLD)"
                exit 1
            fi

            echo "$result"
            return 0
        fi

        log "Attempt $attempt failed, retrying..."
        sleep $((attempt * 10))
        attempt=$((attempt + 1))
    done

    log "ERROR: Max retries exceeded"
    return 1
}

# Main logic
log "Starting automated analysis"
run_claude "Analyze codebase for security issues" > security-report.json
run_claude "Analyze codebase for performance issues" > performance-report.json
log "Analysis complete (total cost: \$$TOTAL_COST)"
```

---

## Section 9: CLI Reference

### CLI Commands

| Command | Description | Example |
|---------|-------------|---------|
| `claude` | Start interactive REPL | `claude` |
| `claude "query"` | Start REPL with initial prompt | `claude "explain this project"` |
| `claude -p "query"` | Query via SDK, then exit | `claude -p "explain this function"` |
| `cat file \| claude -p "query"` | Process piped content | `cat logs.txt \| claude -p "explain"` |
| `claude -c` | Continue most recent conversation | `claude -c` |
| `claude -c -p "query"` | Continue via SDK | `claude -c -p "Check for type errors"` |
| `claude -r "<session-id>" "query"` | Resume session by ID | `claude -r "abc123" "Finish this PR"` |
| `claude update` | Update to latest version | `claude update` |
| `claude mcp` | Configure MCP servers | See MCP documentation |

### CLI Flags

| Flag | Description | Example |
|------|-------------|---------|
| `--add-dir` | Add additional working directories | `claude --add-dir ../apps ../lib` |
| `--agents` | Define custom subagents dynamically via JSON | `claude --agents '{"reviewer":...}'` |
| `--allowedTools` | List of allowed tools without prompting | `"Bash(git log:*)" "Read"` |
| `--disallowedTools` | List of disallowed tools | `"Bash(rm*)" "Edit"` |
| `--print`, `-p` | Print response without interactive mode | `claude -p "query"` |
| `--append-system-prompt` | Append to system prompt (only with `--print`) | `claude --append-system-prompt "Custom"` |
| `--output-format` | Specify output format (`text`, `json`, `stream-json`) | `claude -p "query" --output-format json` |
| `--input-format` | Specify input format (`text`, `stream-json`) | `claude -p --input-format stream-json` |
| `--include-partial-messages` | Include partial streaming events | `claude -p --output-format stream-json --include-partial-messages` |
| `--verbose` | Enable verbose logging | `claude --verbose` |
| `--max-turns` | Limit number of agentic turns | `claude -p --max-turns 3 "query"` |
| `--model` | Set model for current session | `claude --model claude-opus-4` |
| `--permission-mode` | Begin in specified permission mode | `claude --permission-mode plan` |
| `--permission-prompt-tool` | MCP tool for permission prompts (non-interactive) | `claude -p --permission-prompt-tool mcp_auth` |
| `--resume` | Resume specific session by ID | `claude --resume abc123 "query"` |
| `--continue` | Load most recent conversation | `claude --continue` |
| `--dangerously-skip-permissions` | Skip permission prompts (use with caution) | `claude --dangerously-skip-permissions` |
| `--no-telemetry` | Disable telemetry for session | `claude --no-telemetry` |

### Configuration Commands

```bash
# Get configuration value
claude config get telemetry.enabled

# Set configuration value
claude config set telemetry.enabled false

# List all configuration
claude config list

# Reset configuration
claude config reset

# Validate configuration
claude config validate
```

### Session Management

```bash
# List recent sessions
claude sessions list

# Show session details
claude sessions show abc123

# Delete session
claude sessions delete abc123

# Export session
claude sessions export abc123 > session.json

# Import session
claude sessions import < session.json
```

### MCP Management

```bash
# List configured MCP servers
claude mcp list

# Add MCP server
claude mcp add github npx -y @modelcontextprotocol/server-github

# Remove MCP server
claude mcp remove github

# Test MCP server
claude mcp test github
```

### Usage & Monitoring

```bash
# Show current usage
claude usage

# Show daily usage
claude usage --period daily

# Show monthly usage with breakdown
claude usage --period monthly --breakdown user,model

# Watch real-time usage
claude usage --watch --interval 60

# Export usage report
claude usage --period monthly --format csv > usage.csv
```

### Compliance & Privacy

```bash
# Check telemetry status
claude config get telemetry

# Opt out of training
claude config set telemetry.trainingOptOut true

# Export user data (GDPR)
claude gdpr export --user user@company.com --format json

# Erase user data (GDPR)
claude gdpr erase --user user@company.com --confirm

# Show compliance status
claude compliance status
```

### Troubleshooting

```bash
# Show version and build info
claude --version
claude --version --verbose

# Test connectivity
claude test connection

# Validate settings
claude config validate

# Show effective settings
claude config show-effective

# Clear cache
claude cache clear

# Reset to defaults
claude reset --confirm

# Show logs
tail -f ~/.claude/logs/claude.log
```

---

## Section 7: Claude Agent SDK (December 2025)

### Overview

The Claude Agent SDK (formerly Claude Code SDK) enables building custom agents with Claude's capabilities. It's the foundation for IDE integrations and autonomous agent development.

### Three Advanced Tool Use Features

| Feature | Purpose | Use Case |
|---------|---------|----------|
| **Tool Search Tool** | Access thousands of tools via regex/BM25 search | When you need specific tools without loading all |
| **Programmatic Tool Calling** | Execute tools in code environment | Reduces context window impact |
| **Tool Use Examples** | Universal examples for effective patterns | Demonstrate tool usage |

### Custom Tools with In-Process MCP

Build tools that run directly in your Python application:

```python
from anthropic import Anthropic
from anthropic.mcp import MCPServer

# Define custom tool
server = MCPServer()

@server.tool("calculate")
def calculate(expression: str) -> float:
    return eval(expression)  # Simplified example

# Claude can now use this tool
```

**Benefits:**
- No separate process management
- Direct Python integration
- Extend Claude Code with custom functionality

### Long-Running Agent Capabilities

Build agents that work across multiple sessions:

**Context Compaction:**
- Work on tasks without exhausting context window
- Automatic summarization of long conversations
- Preserves essential information

**Initializer Agent:**
- Setup environment on first run
- Install dependencies
- Configure project structure

**Coding Agent Pattern:**
- Make incremental progress each session
- Clear artifacts for resumption
- Checkpoint-based workflow

### SDK Integration Points

| Integration | Description |
|-------------|-------------|
| **JetBrains IDEs** | Built on Agent SDK with IDE-specific tools |
| **VS Code** | Extension uses SDK for context management |
| **Custom Agents** | Build specialized agents for your workflow |
| **Automation** | Create autonomous development pipelines |

### Getting Started

```bash
# Install the SDK
pip install anthropic-agent-sdk

# Or via npm for JavaScript
npm install @anthropic-ai/agent-sdk
```

**Documentation:**
- [Agent SDK Overview](https://platform.claude.com/docs/en/agent-sdk/overview)
- [Building Agents](https://www.anthropic.com/engineering/building-agents-with-the-claude-agent-sdk)
- [Advanced Tool Use](https://www.anthropic.com/engineering/advanced-tool-use)

---

## Related Resources

**Official Documentation:**
- [Claude Code Documentation](https://docs.claude.com/en/docs/claude-code)
- [Sandboxing Guide](https://docs.claude.com/en/docs/claude-code/sandboxing)
- [Security Documentation](https://docs.claude.com/en/docs/claude-code/security)
- [Network Configuration](https://docs.claude.com/en/docs/claude-code/network-config)
- [Monitoring & Usage](https://docs.claude.com/en/docs/claude-code/monitoring-usage)
- [Data Privacy](https://docs.claude.com/en/docs/claude-code/data-usage)

**Local Documentation:**
- [Settings Guide](/mnt/c/Projects/DevForgeAI2/devforgeai/specs/Terminal/settings.md)
- [Headless Mode Guide](/mnt/c/Projects/DevForgeAI2/devforgeai/specs/Terminal/headless-mode.md)
- [CLI Reference](/mnt/c/Projects/DevForgeAI2/devforgeai/specs/Terminal/cli-reference.md)
- [MCP Configuration](/mnt/c/Projects/DevForgeAI2/devforgeai/specs/Terminal/mcp.md)
- [Hooks Reference](/mnt/c/Projects/DevForgeAI2/devforgeai/specs/Terminal/hooks-reference.md)
- [Common Workflows](/mnt/c/Projects/DevForgeAI2/devforgeai/specs/Terminal/common-workflows.md)

**Community Resources:**
- [Claude Code GitHub](https://github.com/anthropics/claude-code)
- [Claude API Documentation](https://docs.anthropic.com)
- [Anthropic Support](https://support.anthropic.com)

---

**Document Version:** 2.0 (2025-12-20)
**Claude Code Version:** 2.0.74
**Sections:** 7 (Sandboxing, Security, Monitoring, Network, Privacy, Enterprise, Agent SDK)
