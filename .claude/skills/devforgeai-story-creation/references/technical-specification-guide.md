# Technical Specification Guide

Comprehensive reference for documenting API contracts, data models, business rules, and dependencies in user stories.

## Purpose

This guide provides templates and examples for technical specifications that enable developers to implement stories without ambiguity, following spec-driven development principles.

---

## API Contract Templates

### REST API Specification (OpenAPI 3.0 Style)

#### HTTP Methods & Use Cases

**GET** - Retrieve data (safe, idempotent)
- List resources: `GET /api/users`
- Get single resource: `GET /api/users/{id}`
- Search/filter: `GET /api/users?role=admin&status=active`

**POST** - Create new resource (not idempotent)
- Create: `POST /api/users`
- Custom actions: `POST /api/users/{id}/verify-email`

**PUT** - Replace entire resource (idempotent)
- Full update: `PUT /api/users/{id}`
- Requires all fields in request body

**PATCH** - Partial update (idempotent)
- Update specific fields: `PATCH /api/users/{id}`
- Only modified fields in request body

**DELETE** - Remove resource (idempotent)
- Delete: `DELETE /api/users/{id}`
- Soft delete: `DELETE /api/users/{id}?soft=true`

#### Complete Endpoint Documentation Template

```markdown
#### Endpoint: {METHOD} /api/{resource}/{id}

**Description:** {What this endpoint does}

**Authentication:** {Required|Optional|None}
- Method: {Bearer Token|API Key|OAuth2|None}
- Required scopes: {scope1, scope2}

**Request Headers:**
```http
Content-Type: application/json
Authorization: Bearer {token}
X-API-Key: {api_key}
```

**Path Parameters:**
- `id` (UUID, required): {Resource identifier}

**Query Parameters:**
- `param1` (string, optional): {Description, default value}
- `param2` (integer, optional): {Description, range: 1-100}

**Request Body:**
```json
{
  "field1": "string (required, max 100 chars, email format)",
  "field2": 42,
  "field3": true,
  "nested": {
    "subfield": "value"
  }
}
```

**Validation Rules:**
- field1: Required, email format, unique in database
- field2: Optional, integer, range 1-100
- field3: Required, boolean

**Success Response (200 OK):**
```json
{
  "id": "uuid-v4",
  "field1": "string",
  "field2": 42,
  "field3": true,
  "created_at": "2025-11-05T14:30:00Z",
  "updated_at": "2025-11-05T14:30:00Z"
}
```

**Error Responses:**

**400 Bad Request (Validation Error):**
```json
{
  "error": "Validation failed",
  "details": [
    {
      "field": "field1",
      "message": "Email format is invalid"
    }
  ]
}
```

**401 Unauthorized (Authentication Failed):**
```json
{
  "error": "Unauthorized",
  "message": "Invalid or expired token"
}
```

**403 Forbidden (Authorization Failed):**
```json
{
  "error": "Forbidden",
  "message": "Insufficient permissions. Required scope: admin:write"
}
```

**404 Not Found (Resource Not Found):**
```json
{
  "error": "Not found",
  "message": "{Resource} with id {id} not found"
}
```

**422 Unprocessable Entity (Business Rule Violation):**
```json
{
  "error": "Business rule violation",
  "message": "Cannot delete user with active orders",
  "details": {
    "active_orders": 3
  }
}
```

**500 Internal Server Error:**
```json
{
  "error": "Internal server error",
  "message": "An unexpected error occurred. Request ID: {request_id}",
  "request_id": "uuid-v4"
}
```

**Rate Limiting (429 Too Many Requests):**
```json
{
  "error": "Rate limit exceeded",
  "message": "Too many requests. Try again in 60 seconds.",
  "retry_after": 60
}
```
```

---

## REST API Examples by Operation

### Create Resource (POST)

```markdown
#### Endpoint: POST /api/users

**Description:** Create new user account

**Authentication:** Required (admin:write scope)

**Request Body:**
```json
{
  "email": "john.doe@example.com",
  "password": "SecurePass123!",
  "name": "John Doe",
  "role": "customer"
}
```

**Validation Rules:**
- email: Required, email format, unique, max 255 chars
- password: Required, min 8 chars, must include uppercase, lowercase, number, special char
- name: Required, 2-100 chars
- role: Required, one of: customer, admin, moderator

**Success Response (201 Created):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "john.doe@example.com",
  "name": "John Doe",
  "role": "customer",
  "created_at": "2025-11-05T14:30:00Z"
}
```

**Response Headers:**
```http
Location: /api/users/550e8400-e29b-41d4-a716-446655440000
```

**Error Response (400 Bad Request):**
```json
{
  "error": "Validation failed",
  "details": [
    {
      "field": "email",
      "message": "Email already exists"
    },
    {
      "field": "password",
      "message": "Password must include at least one uppercase letter"
    }
  ]
}
```
```

### List Resources (GET)

```markdown
#### Endpoint: GET /api/users

**Description:** List all users with pagination and filtering

**Authentication:** Required (admin:read scope)

**Query Parameters:**
- `page` (integer, optional, default: 1): Page number
- `limit` (integer, optional, default: 20, max: 100): Items per page
- `role` (string, optional): Filter by role (customer, admin, moderator)
- `status` (string, optional): Filter by status (active, inactive, suspended)
- `search` (string, optional): Search in name or email
- `sort` (string, optional, default: created_at): Sort field
- `order` (string, optional, default: desc): Sort order (asc, desc)

**Success Response (200 OK):**
```json
{
  "data": [
    {
      "id": "uuid-1",
      "email": "user1@example.com",
      "name": "User One",
      "role": "customer",
      "status": "active",
      "created_at": "2025-11-05T10:00:00Z"
    },
    {
      "id": "uuid-2",
      "email": "user2@example.com",
      "name": "User Two",
      "role": "admin",
      "status": "active",
      "created_at": "2025-11-04T15:30:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total_items": 150,
    "total_pages": 8
  },
  "links": {
    "self": "/api/users?page=1&limit=20",
    "next": "/api/users?page=2&limit=20",
    "prev": null,
    "first": "/api/users?page=1&limit=20",
    "last": "/api/users?page=8&limit=20"
  }
}
```
```

### Get Single Resource (GET)

```markdown
#### Endpoint: GET /api/users/{id}

**Description:** Retrieve single user by ID

**Authentication:** Required (read:users scope)

**Path Parameters:**
- `id` (UUID, required): User identifier

**Success Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "john.doe@example.com",
  "name": "John Doe",
  "role": "customer",
  "status": "active",
  "created_at": "2025-11-05T14:30:00Z",
  "updated_at": "2025-11-05T14:30:00Z",
  "last_login": "2025-11-05T16:00:00Z"
}
```

**Error Response (404 Not Found):**
```json
{
  "error": "Not found",
  "message": "User with id '550e8400-e29b-41d4-a716-446655440000' not found"
}
```
```

### Update Resource (PUT/PATCH)

```markdown
#### Endpoint: PATCH /api/users/{id}

**Description:** Update user fields (partial update)

**Authentication:** Required (write:users scope or self)

**Path Parameters:**
- `id` (UUID, required): User identifier

**Request Body (Partial - only include fields to update):**
```json
{
  "name": "John Smith",
  "role": "moderator"
}
```

**Success Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "john.doe@example.com",
  "name": "John Smith",
  "role": "moderator",
  "status": "active",
  "created_at": "2025-11-05T14:30:00Z",
  "updated_at": "2025-11-05T18:45:00Z"
}
```

**Error Response (409 Conflict - Concurrent Modification):**
```json
{
  "error": "Conflict",
  "message": "Resource was modified by another user",
  "current_version": {
    "updated_at": "2025-11-05T18:40:00Z",
    "updated_by": "admin@example.com"
  }
}
```
```

### Delete Resource (DELETE)

```markdown
#### Endpoint: DELETE /api/users/{id}

**Description:** Soft delete user (mark as deleted, preserve data)

**Authentication:** Required (admin:write scope)

**Path Parameters:**
- `id` (UUID, required): User identifier

**Success Response (204 No Content):**
- Empty body
- User record updated with deleted_at timestamp

**Alternative Response (200 OK with body):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "message": "User deleted successfully",
  "deleted_at": "2025-11-05T19:00:00Z"
}
```

**Error Response (422 Unprocessable Entity - Dependencies Exist):**
```json
{
  "error": "Cannot delete user",
  "message": "User has 5 active orders. Cancel orders before deleting user.",
  "dependencies": {
    "active_orders": 5,
    "order_ids": ["ORD-001", "ORD-002", "ORD-003", "ORD-004", "ORD-005"]
  }
}
```
```

---

## Data Model Documentation

### Entity Documentation Template

```markdown
#### Entity: {EntityName}

**Purpose:** {1-2 sentence description of what this entity represents}

**Attributes:**

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | Required, PK, Auto-generated | Unique identifier |
| {field} | {Type} | {Constraints} | {Description} |

**Relationships:**
- {Relationship type}: {Related entity} ({cardinality})
  - Example: Has many: Orders (one-to-many)

**Indexes:**
- {field} ({unique|non-unique}): {Purpose}
  - Example: email (unique): Fast lookup, enforce uniqueness

**Constraints:**
- Unique: {fields that must be unique together}
- Check: {custom constraints}
  - Example: CHECK (start_date < end_date)

**Lifecycle:**
- Created when: {Trigger}
- Updated when: {Triggers}
- Deleted when: {Soft delete or hard delete strategy}

**Example Record:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "field1": "value",
  "field2": 42,
  "created_at": "2025-11-05T14:30:00Z"
}
```
```

### Data Type Reference

**Common Data Types:**

| Type | Description | Examples | Validation |
|------|-------------|----------|------------|
| **UUID** | Universally unique identifier | `550e8400-e29b-41d4-a716-446655440000` | UUID v4 format |
| **String** | Text data | `"John Doe"`, `"test@example.com"` | Max length, pattern |
| **Integer** | Whole numbers | `42`, `-10`, `0` | Min/max range |
| **Float/Decimal** | Decimal numbers | `99.99`, `3.14159` | Precision, range |
| **Boolean** | True/false | `true`, `false` | N/A |
| **DateTime** | Timestamp | `"2025-11-05T14:30:00Z"` | ISO 8601 format |
| **Date** | Date only | `"2025-11-05"` | YYYY-MM-DD |
| **Time** | Time only | `"14:30:00"` | HH:MM:SS |
| **Enum** | Fixed set of values | `"active"`, `"inactive"`, `"suspended"` | One of allowed values |
| **JSON** | Structured data | `{"key": "value"}` | Valid JSON |
| **Array** | List of items | `[1, 2, 3]`, `["a", "b"]` | Item type, length |

**Constraint Notation:**

- **Required**: Cannot be null or empty
- **Optional**: Can be null or omitted
- **Unique**: Must be unique across all records
- **PK**: Primary key
- **FK**: Foreign key (references another table)
- **Auto-generated**: System generates value (user doesn't provide)
- **Auto-updated**: System updates automatically (e.g., updated_at timestamp)
- **Min/Max**: Range constraints
- **Pattern**: Regex validation
- **Default**: Default value if not provided

### Comprehensive Entity Example

```markdown
#### Entity: Order

**Purpose:** Represents a customer purchase order with items, shipping, and payment details

**Attributes:**

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | Required, PK, Auto-generated | Unique order identifier |
| order_number | String(20) | Required, Unique, Auto-generated | Human-readable order number (ORD-2025-00042) |
| customer_id | UUID | Required, FK → User.id | Customer who placed order |
| status | Enum | Required, Default: 'pending' | Order status (pending, processing, shipped, delivered, cancelled) |
| subtotal | Decimal(10,2) | Required, Min: 0.01 | Sum of item prices (before tax and shipping) |
| tax | Decimal(10,2) | Required, Min: 0 | Sales tax amount |
| shipping | Decimal(10,2) | Required, Min: 0 | Shipping cost |
| total | Decimal(10,2) | Required, Min: 0.01 | Final total (subtotal + tax + shipping - discount) |
| discount | Decimal(10,2) | Optional, Min: 0, Default: 0 | Discount amount applied |
| coupon_code | String(50) | Optional | Coupon code used (if any) |
| shipping_address | JSON | Required | Shipping address object |
| billing_address | JSON | Required | Billing address object |
| payment_method | Enum | Required | Payment method (credit_card, paypal, bank_transfer) |
| payment_status | Enum | Required, Default: 'pending' | Payment status (pending, authorized, captured, failed, refunded) |
| notes | Text | Optional | Customer notes or special instructions |
| created_at | DateTime | Required, Auto-generated | Order creation timestamp |
| updated_at | DateTime | Required, Auto-updated | Last modification timestamp |
| shipped_at | DateTime | Optional | Shipping timestamp (null until shipped) |
| delivered_at | DateTime | Optional | Delivery timestamp (null until delivered) |

**Relationships:**
- Belongs to: Customer (many-to-one via customer_id → User.id)
- Has many: OrderItems (one-to-many)
- Has one: Payment (one-to-one, optional)
- Has many: Shipments (one-to-many, for split shipments)

**Indexes:**
- order_number (unique): Fast lookup by human-readable number
- customer_id (non-unique): Fast customer order history queries
- status (non-unique): Fast filtering by status
- created_at (non-unique): Fast sorting by date
- (customer_id, created_at) composite: Optimized customer order history with date sort

**Constraints:**
- Unique: order_number
- Check: total = subtotal + tax + shipping - discount (enforce calculation correctness)
- Check: subtotal >= 0, tax >= 0, shipping >= 0, total >= 0.01
- Foreign Key: customer_id references User.id (cascade on update, restrict on delete)

**Lifecycle:**
- Created when: Customer completes checkout
- Updated when: Status changes, payment processed, shipping updates
- Soft deleted: Never (orders are historical records, kept forever)
- Archived: After 7 years (compliance requirement)

**Example Record:**
```json
{
  "id": "a1b2c3d4-e5f6-4789-0123-456789abcdef",
  "order_number": "ORD-2025-00042",
  "customer_id": "c1d2e3f4-a5b6-4789-0123-456789abcdef",
  "status": "delivered",
  "subtotal": 150.00,
  "tax": 12.50,
  "shipping": 25.00,
  "total": 187.50,
  "discount": 0.00,
  "coupon_code": null,
  "shipping_address": {
    "street": "123 Main St",
    "city": "New York",
    "state": "NY",
    "zip": "10001",
    "country": "USA"
  },
  "billing_address": {
    "street": "123 Main St",
    "city": "New York",
    "state": "NY",
    "zip": "10001",
    "country": "USA"
  },
  "payment_method": "credit_card",
  "payment_status": "captured",
  "notes": "Please leave package at front door",
  "created_at": "2025-11-01T10:30:00Z",
  "updated_at": "2025-11-05T14:30:00Z",
  "shipped_at": "2025-11-03T09:15:00Z",
  "delivered_at": "2025-11-05T14:00:00Z"
}
```
```

---

## Business Rules Documentation

### Business Rule Template

```markdown
### Business Rule: {Rule Name}

**Description:** {What this rule enforces}

**Trigger:** {When this rule is evaluated}

**Logic:**
{Step-by-step rule logic}

**Validation:**
{How to validate compliance}

**Error Handling:**
{What happens if rule is violated}

**Example:**
{Concrete example showing rule in action}
```

### Common Business Rules

**Calculation Rules:**
```
### Business Rule: Order Total Calculation

**Description:** Order total must equal subtotal + tax + shipping - discount

**Trigger:** When order is created or updated

**Logic:**
1. Calculate items_subtotal = sum(item.price * item.quantity for all items)
2. Calculate tax = items_subtotal * tax_rate
3. Calculate total = items_subtotal + tax + shipping_cost - discount_amount
4. Validate: total >= 0.01 (minimum order total)

**Validation:**
- Verify calculation on server-side (never trust client calculation)
- Round to 2 decimal places for currency
- Ensure no negative totals

**Error Handling:**
If total < 0.01:
  - Reject order creation
  - Error: "Order total must be at least $0.01"

**Example:**
Items: $100.00
Tax (8%): $8.00
Shipping: $15.00
Discount (10% coupon): $10.00
Total: $100 + $8 + $15 - $10 = $113.00
```

**State Transition Rules:**
```
### Business Rule: Order Status Transitions

**Description:** Order status can only transition through valid states

**Trigger:** When order status is updated

**Valid Transitions:**
- pending → processing (payment authorized)
- processing → shipped (items dispatched)
- shipped → delivered (delivery confirmed)
- pending → cancelled (user cancels before processing)
- processing → cancelled (admin cancels)
- shipped → returned (user initiates return)
- delivered → returned (user returns items)

**Invalid Transitions:**
- delivered → cancelled (cannot cancel delivered order)
- cancelled → processing (cannot un-cancel)

**Validation:**
```python
VALID_TRANSITIONS = {
    'pending': ['processing', 'cancelled'],
    'processing': ['shipped', 'cancelled'],
    'shipped': ['delivered', 'returned'],
    'delivered': ['returned'],
    'cancelled': [],  # Terminal state
    'returned': []    # Terminal state
}

if new_status not in VALID_TRANSITIONS[current_status]:
    raise BusinessRuleViolation(
        f"Cannot transition from {current_status} to {new_status}"
    )
```

**Error Handling:**
If invalid transition attempted:
  - 422 Unprocessable Entity
  - Error: "Cannot change order from {current} to {new}. Valid transitions: {valid_list}"
```

**Inventory Rules:**
```
### Business Rule: Inventory Reservation

**Description:** Inventory is reserved when order is placed, decremented when shipped

**Trigger:** Order status changes

**Logic:**
1. When order created (pending):
   - Reserve inventory (product.reserved += quantity)
   - Do NOT decrement available count yet

2. When order ships (shipped):
   - Decrement inventory (product.stock -= quantity)
   - Release reservation (product.reserved -= quantity)

3. When order cancelled:
   - Release reservation (product.reserved -= quantity)
   - Inventory returns to available pool

4. Prevent overselling:
   - Before creating order, check: product.stock - product.reserved >= order_quantity
   - If insufficient: Reject order with "Insufficient inventory"

**Validation:**
- product.stock >= 0 (cannot go negative)
- product.reserved >= 0
- product.stock >= product.reserved (reserved is subset of stock)

**Error Handling:**
If inventory insufficient:
  - 422 Unprocessable Entity
  - Error: "Only {available} items available. You requested {requested}."
  - Suggest: "Update quantity or remove from cart"
```

---

## Dependency Documentation

### Dependency Template

```markdown
### Dependency: {Service/Library Name}

**Type:** {External Service | Third-Party API | Database | Infrastructure | Library}

**Purpose:** {Why this dependency is needed}

**Integration Method:**
- {REST API | SDK | Library | Database connection | Message queue}
- Authentication: {API Key | OAuth2 | None}
- Endpoint/Connection: {URL or connection string}

**SLA Requirements:**
- Availability: {99.9% uptime}
- Response Time: {<500ms}
- Rate Limits: {100 requests/second}

**Fallback Behavior:**
{What happens if dependency is unavailable}

**Configuration:**
- Environment variables: {ENV_VAR_NAME}
- Configuration file: {path/to/config}

**Error Handling:**
{How errors from this dependency are handled}

**Example Usage:**
{Code snippet or API call example}
```

### External Service Examples

**Payment Gateway (Stripe):**
```
### Dependency: Stripe Payment Gateway

**Type:** External Service (Third-Party API)

**Purpose:** Process credit card payments, handle payment authorization, capture, and refunds

**Integration Method:**
- REST API via Stripe SDK (stripe-node for Node.js)
- Authentication: API Secret Key (server-side only)
- Endpoint: https://api.stripe.com/v1/

**SLA Requirements:**
- Availability: 99.99% uptime (Stripe SLA)
- Response Time: <1 second for payment intent creation
- Rate Limits: 100 requests/second (test mode), 1000 req/sec (production)

**Fallback Behavior:**
- If Stripe API unavailable:
  - Queue payment for retry (max 3 attempts over 1 hour)
  - Show user: "Payment processing temporarily unavailable. Your order is saved and will be processed shortly."
  - Send email when payment succeeds
- If payment fails after 3 retries:
  - Mark order as "Payment Failed"
  - Notify user to update payment method

**Configuration:**
- Environment variables:
  - STRIPE_SECRET_KEY (server-side, never client)
  - STRIPE_PUBLISHABLE_KEY (client-side, safe to expose)
  - STRIPE_WEBHOOK_SECRET (for webhook signature verification)

**Error Handling:**
- Card declined: Show user Stripe error message, allow retry
- Invalid card: Show validation error, user corrects
- Network error: Queue for retry, notify user
- Webhook signature invalid: Return 401, log security incident

**Example Usage:**
```javascript
const stripe = require('stripe')(process.env.STRIPE_SECRET_KEY);

const paymentIntent = await stripe.paymentIntents.create({
  amount: 9950, // $99.50 in cents
  currency: 'usd',
  payment_method: paymentMethodId,
  confirmation_method: 'manual',
  confirm: true
});
```
```

**Email Service (SendGrid):**
```
### Dependency: SendGrid Email Service

**Type:** External Service (Email API)

**Purpose:** Send transactional emails (verification, password reset, notifications)

**Integration Method:**
- REST API via SendGrid SDK (@sendgrid/mail for Node.js)
- Authentication: API Key
- Endpoint: https://api.sendgrid.com/v3/

**SLA Requirements:**
- Availability: 99.9% uptime
- Delivery Time: <30 seconds for transactional emails
- Rate Limits: 600 emails/minute (depends on plan)

**Fallback Behavior:**
- If SendGrid unavailable:
  - Queue emails in Redis/database
  - Retry every 5 minutes (max 24 hours)
  - After 24 hours: Mark as failed, alert admin
- If rate limit exceeded:
  - Queue excess emails
  - Send when rate limit resets

**Configuration:**
- Environment variables:
  - SENDGRID_API_KEY
  - FROM_EMAIL (verified sender email)
- Templates: SendGrid Dynamic Templates (template IDs)

**Error Handling:**
- Invalid recipient email: Log error, skip sending
- Template not found: Use fallback plain-text template
- API error: Queue for retry, log error

**Example Usage:**
```javascript
const sgMail = require('@sendgrid/mail');
sgMail.setApiKey(process.env.SENDGRID_API_KEY);

const msg = {
  to: user.email,
  from: 'noreply@example.com',
  templateId: 'd-verification-email-template-id',
  dynamicTemplateData: {
    user_name: user.name,
    verification_link: verificationUrl
  }
};

await sgMail.send(msg);
```
```

### Database Dependencies

**Primary Database (PostgreSQL):**
```
### Dependency: PostgreSQL Database

**Type:** Database (Relational)

**Purpose:** Primary data store for all application data

**Integration Method:**
- Connection: Prisma ORM (Node.js) or Entity Framework (C#)
- Connection String: postgres://user:pass@host:5432/database
- SSL: Required in production

**SLA Requirements:**
- Availability: 99.9% uptime
- Query Performance: <100ms for indexed queries, <500ms for complex joins
- Concurrent Connections: Support 100 connections

**Backup & Recovery:**
- Automated daily backups
- Point-in-time recovery (7 day retention)
- Backup tested monthly

**Fallback Behavior:**
- If primary unavailable: Fail over to read replica (read-only mode)
- If all databases unavailable: Return 503 Service Unavailable
- Queue write operations for retry when database recovers

**Configuration:**
- Environment variables:
  - DATABASE_URL (connection string)
  - DATABASE_POOL_SIZE (default: 10)
  - DATABASE_TIMEOUT (default: 30s)

**Error Handling:**
- Connection errors: Retry with exponential backoff (3 attempts)
- Query timeout: Return 504 Gateway Timeout
- Deadlock: Retry transaction (max 3 attempts)
- Constraint violation: Return 422 with specific error

**Schema Migration:**
- Managed by: Prisma Migrate or Entity Framework Migrations
- Version controlled: migrations/ directory
```

**Caching Layer (Redis):**
```
### Dependency: Redis Cache

**Type:** In-Memory Database (Caching)

**Purpose:** Cache frequently accessed data, reduce database load, store session data

**Integration Method:**
- Client: ioredis (Node.js) or StackExchange.Redis (C#)
- Connection: redis://host:6379
- Cluster mode: Supported for high availability

**SLA Requirements:**
- Availability: 99.9% uptime
- Response Time: <10ms for cache hits
- Memory: 4GB allocated, eviction policy LRU

**Cache Strategy:**
- TTL: 5 minutes for user sessions, 1 hour for static data
- Invalidation: On data update, delete cache key
- Cache-aside pattern: Check cache first, load from DB on miss, populate cache

**Fallback Behavior:**
- If Redis unavailable:
  - Skip caching, query database directly
  - Performance degrades but app remains functional
  - Log warning for monitoring

**Configuration:**
- Environment variables:
  - REDIS_URL
  - REDIS_TTL_DEFAULT (default: 300 seconds)

**Error Handling:**
- Connection error: Skip cache, use database
- Eviction (memory full): Automatic LRU eviction
- Serialization error: Log error, skip caching for that object

**Example Usage:**
```javascript
const redis = require('ioredis');
const client = new Redis(process.env.REDIS_URL);

// Get from cache
const cached = await client.get(`user:${userId}`);
if (cached) {
  return JSON.parse(cached);
}

// Load from database
const user = await db.user.findUnique({ where: { id: userId } });

// Populate cache (TTL: 5 minutes)
await client.setex(`user:${userId}`, 300, JSON.stringify(user));

return user;
```
```

---

## GraphQL API Patterns

### Query

```markdown
#### GraphQL Query: user

**Description:** Fetch user by ID with related data

**Arguments:**
- `id` (ID!, required): User identifier

**Returns:** User object with fields

**Schema:**
```graphql
type Query {
  user(id: ID!): User
}

type User {
  id: ID!
  email: String!
  name: String!
  role: Role!
  orders(first: Int, after: String): OrderConnection!
  createdAt: DateTime!
}

type OrderConnection {
  edges: [OrderEdge!]!
  pageInfo: PageInfo!
  totalCount: Int!
}

type OrderEdge {
  node: Order!
  cursor: String!
}

type PageInfo {
  hasNextPage: Boolean!
  hasPreviousPage: Boolean!
  startCursor: String
  endCursor: String
}
```

**Example Query:**
```graphql
query GetUser {
  user(id: "550e8400-e29b-41d4-a716-446655440000") {
    id
    email
    name
    role
    orders(first: 10) {
      edges {
        node {
          orderNumber
          total
          status
        }
      }
      totalCount
    }
  }
}
```

**Example Response:**
```json
{
  "data": {
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "email": "john@example.com",
      "name": "John Doe",
      "role": "CUSTOMER",
      "orders": {
        "edges": [
          {
            "node": {
              "orderNumber": "ORD-2025-00042",
              "total": 187.50,
              "status": "DELIVERED"
            }
          }
        ],
        "totalCount": 15
      }
    }
  }
}
```
```

### Mutation

```markdown
#### GraphQL Mutation: createUser

**Description:** Create new user account

**Arguments:**
- `input` (CreateUserInput!, required): User creation data

**Returns:** CreateUserPayload with created user or errors

**Schema:**
```graphql
type Mutation {
  createUser(input: CreateUserInput!): CreateUserPayload!
}

input CreateUserInput {
  email: String!
  password: String!
  name: String!
  role: Role = CUSTOMER
}

type CreateUserPayload {
  user: User
  errors: [UserError!]
}

type UserError {
  field: String!
  message: String!
}
```

**Example Mutation:**
```graphql
mutation CreateUser {
  createUser(input: {
    email: "new@example.com"
    password: "SecurePass123!"
    name: "New User"
    role: CUSTOMER
  }) {
    user {
      id
      email
      name
    }
    errors {
      field
      message
    }
  }
}
```

**Success Response:**
```json
{
  "data": {
    "createUser": {
      "user": {
        "id": "new-uuid",
        "email": "new@example.com",
        "name": "New User"
      },
      "errors": []
    }
  }
}
```

**Error Response (Validation):**
```json
{
  "data": {
    "createUser": {
      "user": null,
      "errors": [
        {
          "field": "email",
          "message": "Email already exists"
        },
        {
          "field": "password",
          "message": "Password must include uppercase letter"
        }
      ]
    }
  }
}
```
```

---

## gRPC API Patterns

### Service Definition

```markdown
#### gRPC Service: UserService

**Description:** User management operations

**Protocol Buffers Definition:**
```protobuf
syntax = "proto3";

package user.v1;

service UserService {
  rpc CreateUser(CreateUserRequest) returns (CreateUserResponse);
  rpc GetUser(GetUserRequest) returns (GetUserResponse);
  rpc ListUsers(ListUsersRequest) returns (ListUsersResponse);
  rpc UpdateUser(UpdateUserRequest) returns (UpdateUserResponse);
  rpc DeleteUser(DeleteUserRequest) returns (DeleteUserResponse);
}

message CreateUserRequest {
  string email = 1;
  string password = 2;
  string name = 3;
  Role role = 4;
}

message CreateUserResponse {
  User user = 1;
  repeated Error errors = 2;
}

message User {
  string id = 1;
  string email = 2;
  string name = 3;
  Role role = 4;
  google.protobuf.Timestamp created_at = 5;
}

enum Role {
  ROLE_UNSPECIFIED = 0;
  ROLE_CUSTOMER = 1;
  ROLE_ADMIN = 2;
  ROLE_MODERATOR = 3;
}

message Error {
  string field = 1;
  string message = 2;
}
```

**Example Request/Response:**
```
Request:
CreateUserRequest {
  email: "user@example.com"
  password: "SecurePass123!"
  name: "John Doe"
  role: ROLE_CUSTOMER
}

Success Response:
CreateUserResponse {
  user: {
    id: "uuid"
    email: "user@example.com"
    name: "John Doe"
    role: ROLE_CUSTOMER
    created_at: {timestamp}
  }
  errors: []
}

Error Response:
CreateUserResponse {
  user: null
  errors: [
    {field: "email", message: "Email already exists"}
  ]
}
```
```

---

## Progressive Disclosure

**When to load this reference:**
- Phase 3: Technical Specification (API contract templates, data model patterns, business rules, dependencies)
- Phase 7: Self-Validation (validation rules for technical specs)

**Why progressive:**
- Not needed during Phase 1-2 (story discovery, requirements analysis)
- Not needed during Phase 4 (UI specification)
- Loaded only when documenting technical specifications
- Saves ~600 lines from loading until needed

---

**Use these templates to create complete, unambiguous technical specifications that enable implementation without back-and-forth clarification.**
