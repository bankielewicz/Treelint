# Domain-Specific Patterns

Common features, user flows, and regulatory considerations for major application domains.

## Table of Contents

1. [E-commerce](#e-commerce)
2. [SaaS Applications](#saas-applications)
3. [Fintech Platforms](#fintech-platforms)
4. [Healthcare Systems](#healthcare-systems)
5. [Content Management Systems](#content-management-systems)
6. [Marketplaces](#marketplaces)
7. [Workflow & Automation](#workflow--automation)
8. [Social Networks](#social-networks)
9. [IoT & Real-Time Monitoring](#iot--real-time-monitoring)
10. [Education & Learning Platforms](#education--learning-platforms)

---

## E-commerce

### Standard Features

#### Product Catalog
- **Product Listing:** Browse by category, search, filters (price, brand, rating, availability)
- **Product Detail:** Images (gallery), description, specifications, reviews, related products
- **Inventory Management:** Real-time stock tracking, low-stock alerts, backorders, pre-orders
- **Product Variants:** Size, color, material (SKU management)
- **Pricing:** Base price, sale price, bulk discounts, promotional pricing

#### Shopping Experience
- **Shopping Cart:** Add/remove items, update quantities, save for later
- **Wishlist/Favorites:** Save products for future purchase
- **Product Recommendations:** "Customers also bought", "You might like", personalized recommendations
- **Reviews & Ratings:** Customer reviews, star ratings, verified purchase badges, review moderation

#### Checkout & Payment
- **Guest Checkout:** Option to checkout without account
- **Shipping:** Address validation, multiple shipping addresses, shipping method selection (standard, expedited)
- **Payment:** Credit card, PayPal, Apple Pay, Google Pay, buy-now-pay-later (Klarna, Affirm)
- **Order Summary:** Itemized cart, shipping cost, taxes, discounts, total
- **Order Confirmation:** Confirmation page, email receipt, order tracking link

#### Post-Purchase
- **Order Management:** Order history, order status tracking, invoice download
- **Shipment Tracking:** Integration with shipping providers (USPS, UPS, FedEx)
- **Returns & Refunds:** Return request, return label generation, refund processing

#### Admin Features
- **Product Management:** Add/edit products, bulk upload (CSV), image management
- **Order Management:** Order processing, fulfillment, shipping label generation
- **Customer Management:** Customer list, order history, support tickets
- **Analytics:** Sales reports, top products, customer acquisition, conversion rates
- **Promotions:** Coupon codes, percentage discounts, BOGO, flash sales

### User Flows

**Customer Purchase Flow:**
1. Browse products or search
2. View product details
3. Add to cart
4. Continue shopping or checkout
5. Enter shipping address
6. Select shipping method
7. Enter payment information
8. Review order
9. Place order
10. Receive confirmation email
11. Track shipment
12. Receive product
13. (Optional) Leave review

**Admin Order Fulfillment Flow:**
1. Receive order notification
2. Verify inventory availability
3. Pick and pack items
4. Generate shipping label
5. Mark order as shipped
6. Customer receives tracking update

### Data Model

**Core Entities:**
- **User:** Customer info, addresses, payment methods
- **Product:** SKU, name, description, price, images, inventory
- **Category:** Hierarchical categories, subcategories
- **Cart:** User → CartItems (Product, quantity)
- **Order:** User, status (pending, paid, shipped, delivered), total
- **OrderItem:** Order → Product, quantity, price at purchase
- **Payment:** Order, method, transaction ID, status
- **Shipment:** Order, tracking number, carrier, status
- **Review:** User → Product, rating, text, verified purchase

### Regulatory Considerations

**Consumer Protection:**
- Return policies (EU: 14-day return right)
- Price display regulations (show taxes, shipping costs upfront)
- Refund processing timelines

**Data Privacy (GDPR):**
- Customer data collection consent
- Right to data deletion
- Cookie consent for analytics/advertising

**Payment Security (PCI-DSS):**
- If storing credit card data: PCI-DSS Level 1-4 compliance
- Recommend: Use payment gateway (Stripe, PayPal) to avoid direct card handling

**Accessibility:**
- WCAG 2.1 AA compliance for public-facing websites

---

## SaaS Applications

### Standard Features

#### Tenant Management (Multi-Tenancy)
- **Tenant Onboarding:** Organization sign-up, workspace creation, subdomain/custom domain
- **User Invitation:** Invite team members, role assignment
- **User Roles:** Owner, admin, member, viewer (custom roles for enterprise)
- **Team Management:** User list, role changes, user deactivation

#### Subscription & Billing
- **Subscription Plans:** Free, starter, professional, enterprise (feature-based tiers)
- **Plan Selection:** Compare plans, select plan during sign-up or upgrade later
- **Payment Method:** Credit card, invoice (enterprise), usage-based billing
- **Billing Cycle:** Monthly, annual (annual discount)
- **Usage Tracking:** API calls, storage, users, compute time
- **Invoicing:** Invoice generation, payment history, downloadable receipts
- **Plan Changes:** Upgrade (immediate), downgrade (end of billing cycle), cancellation

#### Core SaaS Features
- **Dashboard:** Key metrics, recent activity, quick actions
- **Settings:** Account settings, team settings, integrations, API keys
- **Activity Log:** Audit trail of user actions
- **Notifications:** In-app notifications, email notifications, notification preferences
- **Help & Support:** Help center, documentation, chat support, ticket system

#### Collaboration
- **Shared Workspaces:** Multiple users collaborating on shared resources
- **Permissions:** Resource-level permissions (view, edit, delete)
- **Comments & Mentions:** Commenting on resources, @mentions for collaboration
- **Real-Time Updates:** WebSockets for live updates when collaborators make changes

#### Integrations
- **REST API:** Public API for developers
- **Webhooks:** Event notifications to external systems
- **OAuth:** Third-party app authorization
- **SSO:** SAML/OAuth for enterprise (Google Workspace, Okta, Azure AD)
- **Zapier/Make:** Integration with automation platforms

#### Admin Features
- **Tenant Management:** View all tenants, tenant analytics, tenant support
- **Usage Analytics:** Per-tenant usage, growth trends, churn analysis
- **Feature Flags:** Enable/disable features per tenant or globally
- **System Health:** Uptime monitoring, error tracking, performance metrics

### User Flows

**Customer Onboarding Flow:**
1. Sign up (email, password, organization name)
2. Email verification
3. Workspace created
4. Onboarding tutorial (feature highlights)
5. Invite team members (optional)
6. Select subscription plan or start trial
7. Enter payment method (if paid plan)
8. Start using product

**Subscription Upgrade Flow:**
1. User navigates to billing settings
2. Compare current plan vs. higher tiers
3. Select new plan
4. Review pricing (prorated if mid-cycle)
5. Confirm upgrade
6. Payment processed
7. Plan features unlocked immediately
8. Confirmation email sent

### Data Model

**Core Entities:**
- **Tenant/Organization:** Name, subdomain, plan, billing info
- **User:** Email, password, role, tenant membership
- **Subscription:** Tenant, plan, status (active, trialing, cancelled), billing cycle
- **Invoice:** Tenant, amount, status (paid, pending, failed), period
- **Resource:** Generic term for tenant's data (projects, documents, workspaces)
- **Activity:** User, tenant, action, timestamp (audit log)
- **Integration:** Tenant, service, API keys, webhook URLs

### Regulatory Considerations

**Data Privacy (GDPR):**
- Tenant data isolation (never show Tenant A's data to Tenant B)
- Data export (tenant can download all their data)
- Data deletion (tenant can delete account and all data)
- Subprocessors disclosure (list of third-party services)

**Security (SOC 2):**
- For enterprise SaaS: SOC 2 Type II certification
- Access controls, encryption, audit logging
- Annual security audits

**Service Level Agreement (SLA):**
- Uptime guarantees (99.9%, 99.95%, 99.99%)
- Support response times
- Downtime credits

---

## Fintech Platforms

### Standard Features

#### Account Management
- **Account Opening:** KYC verification, identity documents, address verification
- **Account Types:** Checking, savings, investment, credit
- **Account Dashboard:** Balance, recent transactions, pending transactions
- **Linked Accounts:** Link external bank accounts for transfers
- **Account Closure:** Close account, transfer remaining balance

#### Transactions
- **Send Money:** P2P transfers, bank transfers, international transfers
- **Receive Money:** Generate payment links, request money
- **Transaction History:** List transactions, filters (date, type, status), search
- **Transaction Details:** Sender, recipient, amount, fee, status, timestamp
- **Scheduled Transfers:** Recurring transfers, future-dated transfers

#### Payments
- **Payment Methods:** Bank transfer (ACH), debit card, credit card, crypto
- **Payment Processing:** Instant (card), same-day (ACH), 1-3 day (standard ACH)
- **Payment Confirmation:** Confirmation page, email receipt, push notification
- **Refunds:** Initiate refund, refund status tracking

#### Security & Fraud
- **Multi-Factor Authentication:** SMS, authenticator app, biometric
- **Fraud Detection:** Transaction monitoring, velocity checks, anomaly detection
- **Dispute Resolution:** Dispute transaction, upload evidence, decision notification
- **Account Security:** Login alerts, device management, session management

#### Regulatory & Compliance
- **KYC (Know Your Customer):** Identity verification, document upload, liveness check
- **AML (Anti-Money Laundering):** Transaction monitoring, suspicious activity detection
- **OFAC Screening:** Sanctions list screening for transactions
- **SAR Filing:** Suspicious Activity Reports for flagged transactions
- **CTR Filing:** Currency Transaction Reports for large cash transactions (> $10k)

#### Reporting
- **Statements:** Monthly statements, downloadable PDF
- **Tax Documents:** 1099 forms for interest income, capital gains
- **Transaction Export:** CSV, PDF export for accounting
- **Analytics:** Spending by category, income/expense trends

### User Flows

**Account Opening Flow:**
1. User submits application (personal info)
2. Identity verification (document upload: driver's license, passport)
3. Liveness check (selfie video)
4. Address verification (utility bill upload)
5. KYC check (automated + manual review if flagged)
6. Account approval or rejection
7. If approved: Account created, welcome email
8. User sets up funding source (link bank account or debit card)

**Money Transfer Flow:**
1. User selects "Send Money"
2. Enter recipient (email, phone, account number)
3. Enter amount
4. Select funding source (bank account, card balance)
5. Review transfer details (amount, fee, estimated arrival)
6. Confirm transfer (may require MFA)
7. Transaction initiated
8. Sender and recipient receive notifications
9. Funds transferred (instant, same-day, or 1-3 days)

### Data Model

**Core Entities:**
- **User:** Personal info, KYC status, documents
- **Account:** User, account type, balance, status (active, frozen, closed)
- **Transaction:** From account, to account, amount, fee, type (transfer, payment), status (pending, completed, failed)
- **Payment Method:** User, type (bank account, card), external account details, verification status
- **KYC Document:** User, document type, file, verification status
- **Fraud Alert:** Transaction, rule triggered, risk score, status (cleared, blocked)
- **Statement:** Account, period (month/year), PDF file

### Regulatory Considerations

**KYC (Know Your Customer):**
- Identity verification required before account opening
- Document verification: government-issued ID
- Address verification: utility bill, bank statement
- Liveness check: prevent identity fraud

**AML (Anti-Money Laundering):**
- Transaction monitoring: flag suspicious patterns (structuring, rapid movement of funds)
- Suspicious Activity Reports (SAR): file with FinCEN for flagged activity
- Customer Due Diligence (CDD): ongoing monitoring of customer behavior

**OFAC Compliance:**
- Screen all transactions against OFAC sanctions lists
- Block transactions involving sanctioned individuals/entities
- Report blocked transactions to OFAC

**PCI-DSS (if handling cards):**
- Never store CVV/CVC
- Tokenize card numbers
- Encrypt cardholder data at rest and in transit

**Banking Regulations:**
- If offering bank accounts: Partner with licensed bank (Banking-as-a-Service) or obtain banking license
- FDIC insurance (if applicable)
- Regulation E compliance (electronic fund transfers)

**Data Retention:**
- Retain financial records for 7 years (IRS requirement)
- Retain KYC documents for 5 years after account closure

---

## Healthcare Systems

### Standard Features

#### Patient Management
- **Patient Registration:** Demographics, insurance info, emergency contact
- **Patient Portal:** View medical records, lab results, prescriptions, appointments
- **Patient Chart:** Medical history, diagnoses, medications, allergies, immunizations
- **Patient Search:** Search by name, DOB, MRN (Medical Record Number)

#### Scheduling
- **Appointment Scheduling:** Book appointments with providers, view availability
- **Appointment Reminders:** Email/SMS reminders 24 hours before appointment
- **Check-In:** Patient check-in (kiosk or front desk), insurance verification
- **Waitlist:** Add to waitlist if no slots available

#### Clinical Workflows
- **Electronic Health Records (EHR):** Comprehensive patient chart
- **Clinical Notes:** SOAP notes (Subjective, Objective, Assessment, Plan), progress notes
- **E-Prescribing:** Send prescriptions electronically to pharmacy
- **Lab Orders:** Order labs, view results, flag abnormal values
- **Imaging Orders:** Order imaging studies (X-ray, MRI, CT), view reports
- **Referrals:** Refer patient to specialist, track referral status

#### Telehealth
- **Video Visits:** Secure video consultations (HIPAA-compliant)
- **Virtual Waiting Room:** Patients wait for provider to join
- **Screen Sharing:** Share educational materials
- **Prescriptions:** E-prescribe during video visit

#### Billing
- **Insurance Verification:** Check eligibility before appointment
- **Claim Submission:** Submit claims to insurance (EDI)
- **Patient Billing:** Generate patient statements, payment processing
- **Payment Plans:** Set up payment plans for large balances

#### Reporting
- **Clinical Reports:** Quality measures, patient outcomes
- **Financial Reports:** Revenue, collections, payer mix
- **Compliance Reports:** HIPAA audit logs, breach reports

### User Flows

**Patient Appointment Flow:**
1. Patient calls or uses online portal
2. Select provider and reason for visit
3. View available time slots
4. Book appointment
5. Receive confirmation email/SMS
6. Receive reminder 24 hours before
7. Check-in at appointment time
8. Provider sees patient
9. Provider documents encounter in EHR
10. E-prescribe medications if needed
11. Schedule follow-up if needed
12. Patient receives after-visit summary

**Clinical Documentation Flow:**
1. Provider opens patient chart
2. Review medical history, medications, allergies
3. Document visit (SOAP note)
4. Select diagnoses (ICD-10 codes)
5. Document procedures (CPT codes)
6. Order labs/imaging if needed
7. E-prescribe medications
8. Finalize note (signs off)
9. Billing codes captured for claim submission

### Data Model

**Core Entities:**
- **Patient:** Demographics, MRN, insurance
- **Provider:** Name, specialty, NPI (National Provider Identifier)
- **Appointment:** Patient, provider, date/time, reason, status (scheduled, completed, cancelled, no-show)
- **Encounter:** Appointment, clinical notes, diagnoses (ICD-10), procedures (CPT)
- **Medication:** Patient, drug name, dosage, frequency, prescriber, status (active, discontinued)
- **Lab Order:** Patient, provider, test type, status (ordered, completed), results
- **Insurance:** Patient, payer, policy number, group number

### Regulatory Considerations

**HIPAA (Health Insurance Portability and Accountability Act):**
- **Privacy Rule:** Protect patient health information (PHI)
- **Security Rule:** Administrative, physical, technical safeguards for PHI
- **Breach Notification Rule:** Notify patients of breaches affecting 500+ individuals
- **Enforcement:** Penalties up to $1.5M per violation category per year

**PHI (Protected Health Information):**
- Names, addresses, dates (birth, admission, discharge, death)
- Phone numbers, email addresses, SSN
- Medical record numbers, health plan numbers
- Diagnoses, treatments, lab results, prescriptions

**Required Security Controls:**
- Encryption: At rest (AES-256), in transit (TLS 1.3)
- Access controls: Role-based, minimum necessary
- Audit logging: All PHI access logged
- Authentication: Strong passwords, MFA for remote access
- Automatic logoff: 15-minute inactivity timeout
- Business Associate Agreements (BAA): With all vendors handling PHI

**Interoperability Standards:**
- **HL7 v2.x:** Messaging standard (ADT, ORM, ORU messages)
- **HL7 FHIR:** REST API standard for health data exchange
- **Direct Messaging:** Secure email for provider communication
- **CDA (Clinical Document Architecture):** Structured clinical documents

**Patient Rights:**
- Right to access medical records (within 30 days)
- Right to request amendment
- Right to accounting of disclosures
- Right to request restrictions on uses/disclosures

**Data Retention:**
- Retain medical records for 6 years minimum (state laws vary)
- Retain billing records for 7 years

---

## Content Management Systems

### Standard Features

#### Content Creation
- **Rich Text Editor:** WYSIWYG editor with formatting (bold, italic, headings, lists)
- **Media Library:** Upload images, videos, documents, organize in folders
- **Content Types:** Pages, posts, articles, custom content types
- **Metadata:** Title, description, keywords (SEO), author, publish date
- **Categories & Tags:** Organize content, hierarchical categories, flat tags
- **Content Relationships:** Related posts, parent-child pages

#### Workflow
- **Draft/Publish:** Save as draft, publish immediately, schedule publication
- **Revisions:** Version history, compare versions, restore previous version
- **Approval Workflow:** Submit for review, approve/reject, feedback comments
- **Multi-Author:** Multiple authors collaborate on same content

#### Publishing
- **URL Slugs:** SEO-friendly URLs, automatic generation from title, custom slugs
- **Templates:** Page templates, post templates, custom layouts
- **Menus:** Navigation menus, drag-and-drop menu builder
- **Widgets/Blocks:** Reusable content blocks, sidebar widgets

#### Media Management
- **Image Editor:** Crop, resize, rotate images
- **Image Optimization:** Automatic compression, responsive images
- **Video Embedding:** YouTube, Vimeo embed
- **Document Management:** PDF, Word, Excel files

#### SEO & Analytics
- **SEO Metadata:** Meta title, meta description, Open Graph tags
- **Sitemap:** Auto-generated XML sitemap
- **Analytics Integration:** Google Analytics, page view tracking
- **Search Console:** Submit sitemap, monitor indexing

#### User Management
- **Roles:** Admin, editor, author, contributor, viewer
- **Permissions:** Create, edit, delete content, manage users, manage settings

### User Flows

**Content Publishing Flow (Editor):**
1. Click "New Post" or "New Page"
2. Enter title (auto-generates URL slug)
3. Write content in rich text editor
4. Upload media (images, videos)
5. Add categories and tags
6. Set SEO metadata (title, description)
7. Preview content
8. Save as draft or publish immediately
9. If workflow enabled: Submit for review
10. Reviewer approves or requests changes
11. Content published

**Content Versioning Flow:**
1. Editor makes changes to published content
2. Save changes (new version created)
3. View version history
4. Compare current version vs. previous
5. If mistake: Restore previous version
6. Publish restored version

### Data Model

**Core Entities:**
- **Content:** Title, slug, body, author, status (draft, published, archived), publish date
- **ContentType:** Page, post, article, product (custom types)
- **Category:** Name, slug, parent category (hierarchical)
- **Tag:** Name, slug (flat taxonomy)
- **Media:** Filename, URL, type (image, video, document), size, upload date
- **User:** Name, email, role (admin, editor, author)
- **Revision:** Content, version number, author, timestamp, changes

### Regulatory Considerations

**Accessibility (WCAG 2.1 AA):**
- Alt text for images
- Keyboard navigation
- Color contrast ratios
- Screen reader compatibility

**GDPR (if EU users):**
- Cookie consent banner
- Privacy policy
- User data export/deletion

**Copyright & DMCA:**
- DMCA takedown procedures
- User-generated content moderation

---

## Marketplaces

### Standard Features

#### Seller Onboarding
- **Seller Registration:** Business info, verification (business license, tax ID)
- **Seller Profile:** Store name, logo, description, policies (shipping, returns)
- **Payout Setup:** Bank account for payouts, payout schedule (daily, weekly)

#### Listing Management
- **Create Listing:** Product name, description, images, price, inventory, SKU
- **Listing Categories:** Assign to marketplace categories
- **Pricing:** Base price, sale price, shipping cost
- **Inventory:** Stock quantity, backorders, low-stock alerts

#### Buyer Experience
- **Search & Discovery:** Keyword search, filters (price, category, seller rating, location)
- **Listing Detail:** Images, description, price, shipping, seller info, reviews
- **Add to Cart:** Multi-seller cart (items from different sellers)
- **Checkout:** Single checkout for items from multiple sellers
- **Order Tracking:** Track orders from each seller

#### Transaction Management
- **Order Processing:** Seller receives order notification, processes order
- **Payment Splitting:** Platform collects payment, splits between seller and platform (commission)
- **Escrow:** Hold funds until buyer receives item (optional)
- **Payouts:** Transfer funds to seller after delivery confirmation

#### Trust & Safety
- **Reviews & Ratings:** Buyer reviews seller, product ratings, verified purchase badges
- **Seller Performance:** Fulfillment rate, response time, cancellation rate
- **Dispute Resolution:** Buyer opens dispute, seller responds, platform mediates
- **Fraud Detection:** Flag suspicious listings, monitor seller behavior

#### Platform Features
- **Commission Model:** Percentage commission, flat fee per transaction, subscription fee
- **Featured Listings:** Paid promotion for sellers
- **Seller Dashboard:** Sales reports, order management, payout history

### User Flows

**Seller Onboarding Flow:**
1. Seller creates account
2. Complete business verification (upload business license, tax ID)
3. Platform reviews and approves (1-3 days)
4. Seller creates store profile
5. Set up payout method (bank account)
6. Create first listing
7. Listing goes live (if auto-approval) or pending review

**Buyer Purchase Flow (Multi-Seller):**
1. Buyer searches for product
2. Views listings from multiple sellers
3. Adds items to cart from Seller A and Seller B
4. Proceeds to checkout
5. Single payment for all items
6. Platform splits payment: Seller A's cut, Seller B's cut, platform commission
7. Seller A and Seller B each receive order notification
8. Each seller ships their items independently
9. Buyer receives separate shipments
10. Buyer can review each seller

### Data Model

**Core Entities:**
- **Seller:** Business name, verification status, payout info, commission rate
- **Listing:** Seller, title, description, price, inventory, status (active, sold out, removed)
- **Order:** Buyer, order items, total, platform fee, seller payouts
- **OrderItem:** Order, listing, quantity, price (at purchase), seller
- **Payout:** Seller, amount, status (pending, paid), payout date
- **Review:** Buyer, seller, listing, rating, text, verified purchase
- **Dispute:** Order, buyer, seller, reason, status (open, resolved), resolution

### Regulatory Considerations

**Marketplace Liability:**
- Platform vs. seller liability (who is responsible for product issues?)
- Terms of Service: Define responsibilities
- Consumer protection laws (vary by region)

**Seller Verification:**
- Business license verification
- Tax ID (for sellers above threshold)
- Identity verification (prevent fraud)

**Payment Processing:**
- Platform acts as payment facilitator (Stripe Connect, PayPal Marketplace)
- 1099-K reporting for sellers earning > $600/year (US)

**Content Moderation:**
- Prohibited items (weapons, drugs, counterfeit goods)
- Listing review (automated + manual)
- Takedown procedures (copyright, trademark)

**Data Privacy:**
- Buyer and seller data protection
- Limit data sharing between buyers and sellers

---

## Workflow & Automation

### Standard Features

#### Workflow Builder
- **Visual Designer:** Drag-and-drop workflow builder, node-based interface
- **Triggers:** Manual, scheduled (cron), event-based (webhook, database change)
- **Actions:** Send email, API call, database operation, conditional logic, loops
- **Variables:** Pass data between steps, transformations
- **Error Handling:** Retry logic, error notifications, fallback actions

#### Workflow Management
- **Workflow Library:** Browse pre-built templates, create from scratch
- **Workflow Versioning:** Publish new versions, rollback to previous version
- **Workflow Testing:** Test with sample data before activation
- **Workflow Activation:** Enable/disable workflows

#### Execution Monitoring
- **Execution History:** List all executions, filters (status, date, workflow)
- **Execution Details:** Input, output, logs, duration, errors
- **Real-Time Monitoring:** Live execution status
- **Notifications:** Email/SMS on workflow failure

#### Integrations
- **Pre-Built Connectors:** Popular services (Gmail, Slack, Salesforce, Stripe, Google Sheets)
- **Custom Integrations:** HTTP requests, custom authentication (API key, OAuth)
- **Webhooks:** Receive webhooks from external systems as triggers

### User Flows

**Workflow Creation Flow:**
1. User clicks "Create Workflow"
2. Select trigger (e.g., "New row in Google Sheets")
3. Configure trigger (select sheet, authenticate Google account)
4. Add action (e.g., "Send email via SendGrid")
5. Configure action (recipient, subject, body with variables)
6. Add conditional logic (if/else)
7. Test workflow with sample data
8. Review results
9. Activate workflow
10. Workflow runs automatically on trigger

**Troubleshooting Flow:**
1. User receives failure notification
2. Navigate to execution history
3. Find failed execution
4. View execution details (error message, logs)
5. Identify issue (e.g., API rate limit, invalid data)
6. Fix workflow (adjust retry logic, data validation)
7. Retry failed execution or wait for next trigger

### Data Model

**Core Entities:**
- **Workflow:** Name, description, trigger, actions, status (active, inactive)
- **Trigger:** Type (manual, scheduled, webhook), configuration (cron expression, webhook URL)
- **Action:** Type (email, API call, database), configuration (API endpoint, request body)
- **Execution:** Workflow, trigger data, status (running, succeeded, failed), start time, end time
- **ExecutionLog:** Execution, step, output, error, timestamp

### Regulatory Considerations

**Data Privacy:**
- Workflows may process sensitive data (customer info, financial data)
- Encrypt data in transit and at rest
- Access controls: Users can only access their workflows

**Third-Party Integrations:**
- OAuth scopes: Request minimum necessary permissions
- Store tokens securely (encrypted)
- Revoke tokens on workflow deletion

---

## Additional Domains

(Abbreviated for space, but follow similar structure as above)

### Social Networks

**Key Features:** User profiles, friend/follow relationships, news feed, posts/comments, direct messaging, notifications, content moderation

**Data Model:** User, Post, Comment, Like, Friendship, Message, Notification

**Regulatory:** GDPR (EU), content moderation (illegal content, hate speech), data portability

---

### IoT & Real-Time Monitoring

**Key Features:** Device registration, telemetry ingestion (MQTT, HTTP), real-time dashboards, alerts, historical data analysis

**Data Model:** Device, Telemetry, Alert, Dashboard, Rule

**Regulatory:** Data retention policies, device security (firmware updates), data privacy (location data)

---

### Education & Learning Platforms

**Key Features:** Course catalog, enrollment, video lessons, quizzes/assignments, grading, discussion forums, certificates

**Data Model:** Course, Lesson, Enrollment, Assignment, Submission, Grade, Certificate

**Regulatory:** FERPA (student data privacy), accessibility (508 compliance), content licensing

---

**Use these domain-specific patterns in Phase 2 (Requirements Elicitation) to quickly identify standard features and avoid reinventing the wheel.**
