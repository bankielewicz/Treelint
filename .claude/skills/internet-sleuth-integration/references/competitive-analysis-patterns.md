# Competitive Analysis Patterns - Internet-Sleuth

**Purpose:** Analyze market landscape, competitive positioning, and SWOT analysis for technology/product decisions.

**When to Use:** devforgeai-ideation Phase 5 (market viability assessment), strategic planning, vendor selection, differentiation analysis

**Loaded:** Conditionally (when research_mode = "competitive-analysis")

---

## Competitive Analysis Overview

**Scope:** Market landscape and strategic positioning (competitive intelligence)
**Duration:** 6-8 minutes (p95)
**Output:** Competitive matrix, SWOT analysis, market positioning recommendations

**Research Questions Answered:**
- "Who are our competitors in [market]?"
- "What are their strengths and weaknesses?"
- "How do we differentiate from competitors?"
- "What are market opportunities and threats?"

**Not Answered (Use discovery mode instead):**
- "Is [technology] technically feasible?"
- "How to implement [feature]?"

---

## Competitive Analysis Workflow (7 Steps)

### Step 1: Define Competitive Scope

**Goal:** Identify market segment and competitive boundaries.

**Scope Dimensions:**

**1. Market Segment**
- Industry: SaaS, E-commerce, FinTech, HealthTech, etc.
- Target audience: B2B, B2C, B2B2C
- Geography: Regional, national, global
- Price tier: Free/Freemium, SMB ($10-100/mo), Enterprise ($1K+/mo)

**2. Competitive Set**
- Direct competitors (same market, same solution)
- Indirect competitors (same market, different solution)
- Substitute products (different market, same problem)
- Adjacent players (potential future competitors)

**3. Comparison Dimensions**
- Features (what capabilities exist)
- Pricing (how much they charge)
- Technology stack (what they're built on)
- User experience (ease of use, design)
- Market share (adoption metrics)
- Differentiation (unique value props)

**Example Scope Definition:**
```markdown
## Competitive Scope: SaaS Authentication Providers

**Market Segment:**
- Industry: B2B SaaS (dev tools)
- Target: Startups + SMBs (10-1000 employees)
- Geography: Global (English-speaking markets)
- Price tier: $0-500/month

**Competitive Set:**
- **Direct:** Auth0, AWS Cognito, Firebase Authentication, Supertokens
- **Indirect:** Keycloak (self-hosted), Ory (open-source)
- **Substitutes:** Custom auth implementation
- **Adjacent:** SSO providers (Okta, OneLogin) - may enter developer auth market

**Comparison Dimensions:**
1. Features (OAuth 2.0, MFA, social logins, APIs)
2. Pricing (free tier, pay-as-you-go, tiered plans)
3. Developer experience (documentation quality, integration time)
4. Performance (latency, uptime SLAs)
5. Security (compliance certifications, audit logs)
6. Scalability (MAU limits, rate limits)
```

**Outputs:**
- Market segment definition
- Competitive set (4-8 competitors)
- Comparison dimensions (5-8 dimensions)

---

### Step 2: Competitor Research

**Goal:** Gather comprehensive data on each competitor.

**Research Sources:**

**1. Official Sources (Quality Score: 10/10)**
- Company websites (pricing, feature pages)
- Official documentation (API docs, guides)
- Product marketing materials (whitepapers, case studies)
- Investor presentations (growth metrics, strategy)

**2. Third-Party Reviews (Quality Score: 7-8/10)**
- G2, Capterra, TrustRadius (user reviews)
- Stack Overflow Survey (developer preferences)
- State of [Industry] reports (adoption trends)
- Analyst reports (Gartner, Forrester) - if accessible

**3. Community Discussions (Quality Score: 5-6/10)**
- Reddit (r/SaaS, r/webdev, r/startups)
- HackerNews (Show HN, Ask HN threads)
- Product Hunt (launches, comments)
- Twitter/X (product announcements, user feedback)

**4. Technical Analysis (Quality Score: 8/10)**
- GitHub repositories (if open-source)
- Stack Overflow questions (common issues)
- API documentation quality
- Uptime monitoring sites (status pages)

**Data Collection Template:**

```markdown
### Competitor: Auth0

**Company Info:**
- Founded: 2013
- Headquarters: Bellevue, WA, USA
- Ownership: Okta (acquired 2021, $6.5B)
- Employees: ~500 (estimate)
- Funding: Acquired (was $330M+ raised pre-acquisition)

**Product:**
- **Description:** Authentication and authorization platform for web, mobile, and legacy apps
- **Target Market:** Startups, SMBs, Enterprise
- **Key Features:**
  - Universal Login (hosted login page)
  - Social connections (Google, Facebook, GitHub, etc.)
  - Enterprise connections (SAML, LDAP, AD)
  - Multi-factor authentication (SMS, email, authenticator apps)
  - Fine-grained authorization (RBAC, ABAC)
  - Extensibility (Rules, Hooks, Actions)
  - APIs and SDKs (20+ languages/frameworks)

**Pricing:**
- Free: Up to 7,000 MAU
- Essentials: $35/month + $0.05/MAU
- Professional: $240/month + $0.13/MAU
- Enterprise: Custom pricing

**Strengths:**
- ✅ Comprehensive feature set (industry leader)
- ✅ Excellent documentation (high praise from developers)
- ✅ Wide platform support (20+ SDKs)
- ✅ Enterprise-ready (SOC 2, HIPAA, GDPR compliance)
- ✅ Strong ecosystem (marketplace, integrations)

**Weaknesses:**
- ❌ Expensive at scale (cost escalates quickly beyond 10K MAU)
- ❌ Vendor lock-in (proprietary platform, migration challenges)
- ❌ Okta acquisition concerns (product direction uncertainty)
- ❌ Complex pricing (hidden costs, add-ons expensive)
- ❌ Overkill for simple use cases (steep learning curve)

**Market Position:**
- **Market share:** ~35-40% of SaaS dev auth market (estimate)
- **Growth:** Stable (mature product, large customer base)
- **Adoption:** High among startups and scale-ups

**Differentiation:**
- Premium features (advanced auth flows, extensibility)
- Developer experience focus (docs, SDKs, community)
- Enterprise compliance and support

**Sources:**
- https://auth0.com/pricing (pricing, features)
- G2 Auth0 Reviews (4.5/5 stars, 1200+ reviews)
- Stack Overflow Developer Survey 2024 (14% of devs use Auth0)
```

**Actions:**
1. Research each competitor (4-8 total)
2. Document company info, product, pricing, strengths/weaknesses
3. Cite all sources with quality scores
4. Validate claims with multiple sources (cross-reference)

**Outputs:**
- Competitor profiles (4-8 detailed profiles)
- Source citations (official + third-party)
- Initial insights (patterns, gaps, opportunities)

---

### Step 3: Feature Comparison Matrix

**Goal:** Create side-by-side comparison of capabilities.

**Matrix Structure:**

| Feature | Auth0 | AWS Cognito | Firebase Auth | Supertokens | Custom Build |
|---------|-------|-------------|---------------|-------------|--------------|
| **OAuth 2.0** | ✅ Full | ✅ Full | ✅ Full | ✅ Full | ⚠️ DIY |
| **Social logins** | ✅ 30+ | ✅ 15+ | ✅ 10+ | ✅ 10+ | ⚠️ DIY |
| **MFA** | ✅ SMS/Email/TOTP | ✅ SMS/TOTP | ✅ Phone/Email | ✅ Email/TOTP | ⚠️ DIY |
| **SSO (SAML)** | ✅ Enterprise | ✅ Yes | ❌ No | ✅ Yes | ⚠️ Complex |
| **API first** | ✅ Yes | ⚠️ Limited | ⚠️ Firebase-centric | ✅ Yes | ✅ Full control |
| **Self-hosted** | ❌ No | ❌ No | ❌ No | ✅ Yes | ✅ Yes |
| **Free tier** | 7K MAU | Pay-as-you-go | 10K MAU | Unlimited (self-host) | Free (own infra) |
| **Pricing (1K MAU)** | $35-60/mo | ~$5.50/mo | ~$6/mo | $0 (self-host) | $20-50/mo (infra) |
| **Documentation** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |
| **Setup time** | 1-2 hours | 4-8 hours | 1-2 hours | 2-4 hours | 40-80 hours |
| **Learning curve** | Moderate | Steep | Easy | Moderate | Steep |
| **Vendor lock-in** | High | Medium | High | Low | None |

**Legend:**
- ✅ = Fully supported
- ⚠️ = Limited or requires workarounds
- ❌ = Not supported
- ⭐ = Quality rating (1-5 stars)

**Actions:**
1. Identify 10-15 key features
2. Research each competitor's support for each feature
3. Score features (✅ / ⚠️ / ❌)
4. Add notes for nuances (e.g., "SAML only in Enterprise plan")

**Outputs:**
- Feature comparison matrix (table format)
- Feature gaps (missing capabilities per competitor)
- Feature parity assessment (who has most complete offering)

---

### Step 4: Pricing Analysis

**Goal:** Compare total cost of ownership across competitors.

**Pricing Dimensions:**

**1. Base Plans**
- Free tier limits (MAU, features, support)
- Starter/Essentials tier ($10-100/mo)
- Professional tier ($100-500/mo)
- Enterprise tier (custom pricing)

**2. Usage-Based Costs**
- Per MAU (monthly active user)
- Per API call
- Per SMS (for MFA)
- Per email (for notifications)

**3. Add-On Costs**
- Advanced features (SAML SSO, custom domains)
- Additional storage/bandwidth
- Premium support
- SLA upgrades

**4. Hidden Costs**
- Implementation/integration time (developer hours)
- Training and onboarding
- Migration costs (if switching)
- Downtime risks

**Cost Scenario Analysis:**

```markdown
## Cost Comparison: 1,000 MAU Scenario

| Provider | Base Plan | Per MAU | MFA SMS | Total (1K MAU) | Notes |
|----------|-----------|---------|---------|----------------|-------|
| **Auth0 Essentials** | $35/mo | $0.05 | $0.05/SMS | $85/mo | Assumes 1K MAU, 20% use MFA (200 SMS) |
| **AWS Cognito** | $0 | $0.0055 | $0.00645/SMS | $6.78/mo | Pay-as-you-go, no base cost |
| **Firebase Auth** | $0 (free tier) | $0.006 | $0.01/SMS | $2/mo | Within free tier (10K MAU) |
| **Supertokens** | $0 (self-host) | $0 | SMS via Twilio | $20/mo | Infra costs (server, DB) |
| **Custom Build** | $0 | $0 | SMS via Twilio | $50/mo | Dev time amortized |

## Cost Comparison: 10,000 MAU Scenario

| Provider | Base Plan | Per MAU | Total (10K MAU) | Annual Cost |
|----------|-----------|---------|-----------------|-------------|
| **Auth0 Essentials** | $35/mo | $0.05 * 10K = $500 | $535/mo | **$6,420/yr** |
| **AWS Cognito** | $0 | $0.0055 * 10K = $55 | $55/mo | **$660/yr** |
| **Firebase Auth** | $0 | $0.006 * 10K = $60 | $60/mo | **$720/yr** |
| **Supertokens** | $0 | $0 | $100/mo | **$1,200/yr** (infra scales) |

## Cost Comparison: 100,000 MAU Scenario (Enterprise)

| Provider | Total (100K MAU) | Annual Cost | Notes |
|----------|------------------|-------------|-------|
| **Auth0 Professional** | $13K+/mo | **$156K+/yr** | Volume discounts may apply |
| **AWS Cognito** | $550/mo | **$6,600/yr** | Linear scaling |
| **Firebase Auth** | $600/mo | **$7,200/yr** | Linear scaling |
| **Supertokens** | $500-1K/mo | **$6-12K/yr** | Infra + DB costs |
```

**Total Cost of Ownership (TCO) Analysis:**

```markdown
## 3-Year TCO Comparison (10K MAU average)

| Provider | Year 1 | Year 2 | Year 3 | Total TCO | Notes |
|----------|--------|--------|--------|-----------|-------|
| **Auth0** | $6,420 + $5K (setup) | $6,420 | $6,420 | **$24,260** | Higher upfront, stable costs |
| **AWS Cognito** | $660 + $8K (setup) | $660 | $660 | **$9,980** | Complex setup, low operating cost |
| **Firebase** | $720 + $2K (setup) | $720 | $720 | **$4,160** | Easy setup, but Google lock-in |
| **Supertokens** | $1,200 + $10K (setup) | $1,200 + $3K (maintenance) | $1,200 + $3K | **$19,600** | High setup + ongoing maintenance |
```

**Outputs:**
- Pricing comparison table (multiple scenarios)
- TCO analysis (3-year projection)
- Cost drivers identified (base, usage, hidden)
- Recommendation (most cost-effective option per scenario)

---

### Step 5: SWOT Analysis

**Goal:** Synthesize strengths, weaknesses, opportunities, threats for competitive positioning.

**SWOT Template:**

```markdown
## SWOT Analysis: Our SaaS Authentication Solution

### Strengths (Internal Advantages)
- **S1: AWS Integration** - Native AWS ecosystem fit (Cognito, Lambda, RDS)
  - Evidence: AWS tech-stack.md lock, existing AWS infrastructure
  - Impact: Faster integration, lower learning curve for team

- **S2: Cost Efficiency** - Lower operating costs than Auth0
  - Evidence: Cognito $55/mo vs Auth0 $535/mo at 10K MAU
  - Impact: Better unit economics, higher profit margins

- **S3: Customization** - Full control over auth flows (vs SaaS limitations)
  - Evidence: Can modify source code, not restricted by vendor APIs
  - Impact: Unique features, differentiation

- **S4: Team Expertise** - In-house Python/FastAPI skills
  - Evidence: Team has 3 years FastAPI experience
  - Impact: Lower implementation risk, faster development

### Weaknesses (Internal Disadvantages)
- **W1: Feature Parity** - Missing some Auth0 advanced features (extensibility, marketplace)
  - Evidence: Auth0 has 100+ marketplace integrations, we have 0
  - Impact: May lose enterprise customers who need marketplace apps

- **W2: Time to Market** - Longer initial development (8 weeks vs 2 weeks for SaaS)
  - Evidence: Custom build requires auth service, UI, tests, docs
  - Impact: Delayed launch, opportunity cost

- **W3: Maintenance Overhead** - Ongoing security updates, feature additions
  - Evidence: Auth0 handles this, we must allocate dev time
  - Impact: ~1 dev week/month for auth maintenance vs zero for SaaS

- **W4: Limited Social Logins** - Fewer social providers initially (5 vs Auth0's 30+)
  - Evidence: Will start with Google, GitHub, Microsoft only
  - Impact: May miss users who prefer Twitter, LinkedIn, Apple logins

### Opportunities (External Advantages)
- **O1: Open-Source Adoption** - Growing preference for self-hosted auth
  - Evidence: Supertokens 10K+ GitHub stars, 2023-2024 growth
  - Impact: Market demand for AWS Cognito / self-hosted hybrid

- **O2: Auth0 Pricing Backlash** - Users seeking cheaper alternatives
  - Evidence: G2 reviews cite "expensive at scale" (30% of negative reviews)
  - Impact: Opportunity to position as "affordable Auth0 alternative"

- **O3: AWS Ecosystem Expansion** - More AWS-native SaaS tools emerging
  - Evidence: AWS marketplace growth, Amplify adoption
  - Impact: Can ride AWS ecosystem wave, easier sales to AWS shops

- **O4: Compliance Requirements** - GDPR, HIPAA demand for data sovereignty
  - Evidence: 40% of enterprise buyers require EU/US data residency
  - Impact: Self-hosted AWS gives full control, Auth0 has limits

### Threats (External Disadvantages)
- **T1: Auth0 Feature Innovation** - May release killer features we can't match
  - Evidence: Okta backing, $500M+ R&D budget
  - Impact: Feature gap widens, harder to compete

- **T2: AWS Cognito Improvements** - AWS may improve Cognito DX (reducing our value-add)
  - Evidence: AWS invests heavily in developer experience
  - Impact: Our wrapper/abstraction becomes less valuable

- **T3: New Entrants** - More open-source auth solutions (Ory, Logto, Clerk)
  - Evidence: 5+ new auth startups launched in 2023-2024
  - Impact: Market fragmentation, price pressure

- **T4: Security Vulnerabilities** - Custom auth has higher breach risk vs mature SaaS
  - Evidence: Auth0 has dedicated security team, we have generalists
  - Impact: Reputational damage if breach occurs
```

**SWOT Visualization (Matrix):**

```
                HELPFUL                     HARMFUL
          ┌────────────────────┬──────────────────────┐
          │                    │                      │
INTERNAL  │   STRENGTHS (S)    │   WEAKNESSES (W)     │
          │  - AWS Integration │  - Feature Parity    │
          │  - Cost Efficiency │  - Time to Market    │
          │  - Customization   │  - Maintenance       │
          │  - Team Expertise  │  - Limited Socials   │
          │                    │                      │
          ├────────────────────┼──────────────────────┤
          │                    │                      │
EXTERNAL  │  OPPORTUNITIES (O) │    THREATS (T)       │
          │  - Open-Source ↑   │  - Auth0 Innovation  │
          │  - Pricing Backl.  │  - AWS Cognito ↑     │
          │  - AWS Ecosystem   │  - New Entrants      │
          │  - Compliance      │  - Security Risk     │
          │                    │                      │
          └────────────────────┴──────────────────────┘
```

**Strategic Positioning:**
- **Leverage S1 + O3:** "AWS-native auth solution for SaaS builders"
- **Address W1 + T1:** Prioritize marketplace integrations roadmap
- **Exploit S2 + O2:** "Auth0 features at Cognito prices"
- **Mitigate W4 + T4:** Partner with security firm for audits

**Outputs:**
- SWOT analysis (4 quadrants with 4-5 items each)
- SWOT matrix (visual representation)
- Strategic positioning recommendations (SO, WO, ST, WT strategies)

---

### Step 6: Market Positioning Map

**Goal:** Visualize competitive landscape on 2-axis positioning map.

**Common Positioning Axes:**

**Axis Combinations:**
1. **Price (Low ↔ High) vs Features (Basic ↔ Advanced)**
2. **Ease of Use (Simple ↔ Complex) vs Customization (Low ↔ High)**
3. **Self-Hosted (Yes ↔ No) vs Managed (No ↔ Yes)**
4. **Developer-Focused (Yes ↔ No) vs Enterprise-Focused (No ↔ Yes)**

**Example Positioning Map:**

```
        Features (Advanced)
               ↑
               │
               │    Auth0 🟦
               │
               │           AWS Cognito 🟨
               │
               │                       Keycloak 🟪
Price (High) ──┼────────────────────────────────→ Price (Low)
               │
               │  Firebase 🟩
               │
               │              Supertokens 🟧
               │
               │
               ↓
        Features (Basic)
```

**Competitive Clustering:**
- **Premium Cluster** (High price, Advanced features): Auth0, Okta
- **Cloud-Native Cluster** (Mid price, Good DX): Firebase, AWS Cognito
- **Open-Source Cluster** (Low price, High complexity): Keycloak, Supertokens, Ory

**Whitespace Opportunities:**
- **Gap 1:** Mid-price + AWS-native + Excellent DX (our target!)
- **Gap 2:** Enterprise features + Self-hosted + Affordable
- **Gap 3:** Simple setup + Advanced RBAC + Pay-as-you-go

**Strategic Positioning Recommendation:**
```markdown
## Recommended Positioning: "AWS-Native Auth for Modern SaaS"

**Target Quadrant:** Mid-price, Advanced features (Auth0 alternative)

**Positioning Statement:**
"[Our Product] combines Auth0's advanced authentication features with AWS Cognito's cost efficiency and AWS-native integration. Built for SaaS startups who need enterprise-grade auth without enterprise pricing."

**Differentiation:**
1. **vs Auth0:** 90% cheaper at 10K MAU, AWS-native (no vendor lock-in to Okta)
2. **vs AWS Cognito:** Better DX (simpler APIs, better docs, faster setup)
3. **vs Firebase:** AWS ecosystem (not Google lock-in), better for backend-heavy apps
4. **vs Supertokens:** Managed service (no ops overhead), enterprise support available

**Target Customer:**
- SaaS startups (post-seed to Series B)
- AWS-first teams (already on AWS infrastructure)
- Developer-focused (values code quality, documentation)
- Budget-conscious (wants premium features, can't afford Auth0 scaling costs)
```

**Outputs:**
- Positioning map (2-axis visual)
- Competitive clusters identified
- Whitespace gaps highlighted
- Positioning statement (target quadrant, differentiation)

---

### Step 7: Report Generation

**Goal:** Create comprehensive competitive analysis report.

**Report Sections (9 Required):**
1. **Executive Summary** (competitive landscape overview + recommendation)
2. **Research Scope** (market segment, competitive set, comparison dimensions)
3. **Methodology Used** ("Competitive Analysis" + research sources)
4. **Findings** (competitor profiles, feature matrix, pricing analysis, SWOT)
5. **Framework Compliance Check** (validation against context files)
6. **Workflow State** (current phase: Backlog / Architecture)
7. **Recommendations** (positioning strategy, differentiation, go-to-market)
8. **Risk Assessment** (competitive threats, market risks, mitigation)
9. **ADR Readiness** (if applicable: vendor selection, build vs buy decision)

**Report Template:**
```markdown
---
research_id: RESEARCH-003
epic_id: EPIC-007
workflow_state: Backlog
research_mode: competitive-analysis
timestamp: 2025-11-17T17:10:45Z
quality_gate_status: PASS
version: 2.0
---

# Competitive Analysis Report: SaaS Authentication Market

## Executive Summary

Analyzed 5 major competitors in SaaS authentication market (Auth0, AWS Cognito, Firebase, Supertokens, Keycloak). Market opportunity identified: "AWS-native auth with Auth0 features at Cognito prices" positioning gap. Recommended strategy: Target AWS-first SaaS startups with mid-market pricing ($50-200/mo) and advanced features (RBAC, SSO, custom flows). Key risk: Auth0 price cuts (mitigate with faster feature velocity and AWS ecosystem integration).

## Research Scope

[... market segment, competitive set ...]

## Methodology Used

**Research Mode:** Competitive Analysis (market landscape)
**Duration:** 7 minutes 32 seconds
**Sources:**
- Official websites (5 competitors)
- G2 reviews (1,200+ reviews analyzed)
- Stack Overflow Survey 2024
- Pricing pages (as of 2025-11-17)
- GitHub repositories (open-source competitors)

## Findings

[... competitor profiles, feature matrix, pricing analysis, SWOT ...]

## Framework Compliance Check

[... validation against context files ...]

## Workflow State

**Current State:** Backlog
**Research Focus:** Market viability and competitive positioning

## Recommendations

[... positioning strategy, differentiation, target customer ...]

## Risk Assessment

| Risk | Severity | Mitigation |
|------|----------|------------|
| Auth0 price cuts | MEDIUM | Differentiate on AWS integration, faster feature velocity |
| AWS Cognito DX improvements | MEDIUM | Build on top of Cognito (not compete), add value via abstraction |
| Security breach | HIGH | Hire security firm for audits, bug bounty program |

## ADR Readiness

**ADR Required:** Yes (Build vs Buy decision for auth)
**Evidence:** ✅ Competitive analysis, cost comparison, SWOT complete

---

**Report Generated:** 2025-11-17 17:15:42
**Report Location:** devforgeai/specs/research/feasibility/EPIC-007-competitive-analysis.md
```

**Outputs:**
- Complete competitive analysis report
- Positioning recommendation
- Strategic SWOT insights
- Report saved to devforgeai/specs/research/

---

## Success Criteria

Competitive analysis succeeds when:
- [ ] Competitive scope defined (market segment, 4-8 competitors, comparison dimensions)
- [ ] Competitor research complete (detailed profiles with sources)
- [ ] Feature comparison matrix created (10-15 features, all competitors)
- [ ] Pricing analysis complete (3 scenarios, TCO calculated)
- [ ] SWOT analysis synthesized (4 quadrants, 4-5 items each)
- [ ] Market positioning map created (2-axis visual, whitespace identified)
- [ ] Report generated (9 sections, positioning recommendation)
- [ ] Duration <8 minutes (p95 threshold)
- [ ] Token usage <50K (within budget)

---

## Related Documentation

- `research-principles.md` - Core research methodology (always loaded)
- `discovery-mode-methodology.md` - Feasibility research (technical focus)
- `skill-coordination-patterns.md` - Task invocation examples
- `research-report-template.md` - Standard report structure
- `.claude/skills/devforgeai-ideation/SKILL.md` - Ideation integration

---

**Created:** 2025-11-17
**Version:** 1.0
**Lines:** 515 (target: 500 ✓)
**Purpose:** Market landscape analysis and competitive positioning
