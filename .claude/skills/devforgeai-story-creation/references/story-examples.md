# Story Examples

Complete user story examples demonstrating best practices for different story types.

## Purpose

These examples show complete, production-ready stories across common types (CRUD, authentication, workflow, reporting, integration) to serve as templates and reference implementations.

---

## Example 1: CRUD Story - User Management

**Story Type:** Create/Read/Update/Delete operations
**Complexity:** 5 points (standard)
**Has API:** Yes
**Has UI:** Yes

```markdown
---
id: STORY-001
title: User management CRUD operations
epic: EPIC-001
sprint: SPRINT-001
status: Backlog
priority: High
points: 5
created: 2025-11-05
updated: 2025-11-05
assigned_to: null
tags: [crud, users, admin]
---

## User Story

As an administrator,
I want to create, view, update, and delete user accounts,
So that I can manage system access and maintain accurate user records.

## Acceptance Criteria

### AC1: Create new user
**Given** I am logged in as administrator
**When** I navigate to /admin/users/new and submit form with valid email, name, and role
**Then** new user account is created in database
And success message displays "User '{name}' created successfully"
And I am redirected to user list page
And new user appears in the list

### AC2: View user list
**Given** database contains 50 users
**When** I navigate to /admin/users page
**Then** all 50 users are displayed in table with columns: Name, Email, Role, Status, Actions
And table is sorted by created_at descending (newest first)
And pagination shows "Showing 1-20 of 50"
And each row has Edit and Delete action buttons

### AC3: View single user details
**Given** user with ID "uuid-123" exists
**When** I click on user row in list
**Then** detail page displays showing: Name, Email, Role, Status, Created Date, Last Login
And "Edit" and "Delete" buttons are visible
And user's order history is displayed (if any orders)

### AC4: Update user information
**Given** I am on user edit page for user "uuid-123"
**When** I modify name from "John Doe" to "John Smith" and submit
**Then** user record is updated in database
And success message displays "User updated successfully"
And I am redirected to user list
And updated name "John Smith" is displayed in list

### AC5: Delete user confirmation
**Given** I am viewing user "John Smith" in list
**When** I click "Delete" button
**Then** confirmation modal displays "Are you sure you want to delete 'John Smith'? This cannot be undone."
And modal has "Cancel" and "Delete" buttons
And "Delete" button is styled in red (warning color)

### AC6: Delete user (no dependencies)
**Given** user "John Smith" has no associated orders or data
And I have confirmed deletion
**When** I click "Delete" in confirmation modal
**Then** user is soft-deleted (deleted_at timestamp set)
**And user no longer appears in active user list
And success message displays "User deleted successfully"
And modal closes

### AC7: Prevent delete if dependencies exist
**Given** user "Jane Doe" has 5 active orders
**When** I try to delete user
**Then** error message displays "Cannot delete user - 5 active orders exist"
And user is NOT deleted
And link displays "View orders" (navigates to user's orders)

### AC8: Duplicate email validation
**Given** user with email "existing@example.com" exists
**When** I try to create new user with same email
**Then** validation error displays "Email already registered"
And form remains open for correction
And no new user is created

## Technical Specification

### API Contracts

#### Endpoint: POST /api/admin/users

**Description:** Create new user account

**Authentication:** Required (Bearer token, admin:write scope)

**Request Body:**
```json
{
  "email": "user@example.com",
  "name": "John Doe",
  "role": "customer",
  "password": "TempPass123!"
}
```

**Validation Rules:**
- email: Required, email format, unique, max 255 chars
- name: Required, 2-100 chars
- role: Required, one of: customer, admin, moderator
- password: Required, min 8 chars (if setting initial password)

**Success Response (201 Created):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "name": "John Doe",
  "role": "customer",
  "status": "active",
  "created_at": "2025-11-05T14:30:00Z"
}
```

**Error Response (400 Bad Request):**
```json
{
  "error": "Validation failed",
  "details": [
    {"field": "email", "message": "Email already exists"}
  ]
}
```

#### Endpoint: GET /api/admin/users

**Description:** List all users with pagination

**Authentication:** Required (Bearer token, admin:read scope)

**Query Parameters:**
- page (integer, default: 1): Page number
- limit (integer, default: 20, max: 100): Items per page
- role (string, optional): Filter by role
- status (string, optional): Filter by status (active, inactive)
- search (string, optional): Search in name or email

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
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total_items": 50,
    "total_pages": 3
  }
}
```

#### Endpoint: GET /api/admin/users/{id}

**Description:** Get single user by ID

**Authentication:** Required (admin:read scope)

**Path Parameters:**
- id (UUID, required): User identifier

**Success Response (200 OK):**
```json
{
  "id": "uuid-123",
  "email": "user@example.com",
  "name": "John Doe",
  "role": "customer",
  "status": "active",
  "created_at": "2025-11-01T10:00:00Z",
  "updated_at": "2025-11-05T14:30:00Z",
  "last_login": "2025-11-05T09:00:00Z",
  "order_count": 5
}
```

**Error Response (404 Not Found):**
```json
{
  "error": "Not found",
  "message": "User with id 'uuid-123' not found"
}
```

#### Endpoint: PATCH /api/admin/users/{id}

**Description:** Update user fields

**Authentication:** Required (admin:write scope)

**Request Body (Partial):**
```json
{
  "name": "John Smith",
  "role": "moderator"
}
```

**Success Response (200 OK):**
```json
{
  "id": "uuid-123",
  "email": "user@example.com",
  "name": "John Smith",
  "role": "moderator",
  "status": "active",
  "updated_at": "2025-11-05T14:45:00Z"
}
```

#### Endpoint: DELETE /api/admin/users/{id}

**Description:** Soft delete user

**Authentication:** Required (admin:write scope)

**Success Response (204 No Content):**
- Empty body
- User marked as deleted (deleted_at timestamp set)

**Error Response (422 Unprocessable Entity):**
```json
{
  "error": "Cannot delete user",
  "message": "User has 5 active orders",
  "dependencies": {
    "active_orders": 5
  }
}
```

### Data Models

#### Entity: User

**Purpose:** Represents a registered user account

**Attributes:**

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | Required, PK, Auto-generated | Unique identifier |
| email | String(255) | Required, Unique, Email format | User email address |
| password_hash | String(255) | Required | Bcrypt hashed password (cost factor 10) |
| name | String(100) | Required | Full name |
| role | Enum | Required, Default: 'customer' | User role (customer, admin, moderator) |
| status | Enum | Required, Default: 'active' | Account status (active, inactive, suspended) |
| created_at | DateTime | Required, Auto-generated | Account creation timestamp |
| updated_at | DateTime | Required, Auto-updated | Last modification timestamp |
| last_login | DateTime | Optional | Last login timestamp |
| deleted_at | DateTime | Optional | Soft delete timestamp (null if active) |

**Relationships:**
- Has many: Orders (one-to-many)
- Has one: UserProfile (one-to-one, optional)
- Belongs to many: Roles (many-to-many via UserRoles, if advanced RBAC)

**Indexes:**
- email (unique): Fast lookup, enforce uniqueness
- role (non-unique): Fast filtering by role
- status (non-unique): Fast filtering by status
- (deleted_at IS NULL) partial: Only active users

**Constraints:**
- Unique: email (case-insensitive)
- Check: email matches email regex
- Check: role IN ('customer', 'admin', 'moderator')
- Check: status IN ('active', 'inactive', 'suspended')

### Business Rules

1. **Email uniqueness:** Email addresses are case-insensitive, stored as lowercase, must be unique across all users (active and deleted)
2. **Password strength:** Passwords must be at least 8 characters with uppercase, lowercase, number, and special character
3. **Soft delete:** Users are soft-deleted (deleted_at timestamp) to preserve historical data, can be restored within 30 days
4. **Role default:** New users default to 'customer' role unless specified
5. **Status default:** New users default to 'active' status
6. **Dependency check:** Users cannot be deleted if they have active orders (status not 'delivered' or 'cancelled')

### Dependencies

**None** - Core feature with no external dependencies

## UI Specification

### Components

#### Component: UserListTable
**Type:** Data Table
**Purpose:** Display paginated list of users with actions
**Data Bindings:**
- Input: users (array), loading (boolean), pagination (object)
- Output: onEdit(userId), onDelete(userId), onPageChange(page)
- State: selectedUserId, sortColumn, sortDirection

#### Component: UserForm
**Type:** Form
**Purpose:** Create or edit user (reusable for both operations)
**Data Bindings:**
- Input: initialData (User | null), isLoading (boolean), error (string | null)
- Output: onSubmit(userData), onCancel()
- State: formData, validationErrors

#### Component: DeleteConfirmationModal
**Type:** Modal
**Purpose:** Confirm user deletion with warning
**Data Bindings:**
- Input: isOpen (boolean), userName (string)
- Output: onConfirm(), onCancel()

### Layout Mockup

**User List Page:**
```
┌──────────────────────────────────────────────────────────────┐
│ Admin Dashboard > Users                    [🔔] [👤 Profile] │
├──────────────────────────────────────────────────────────────┤
│ Users (50 total)        [Search: ______]  [+ Add New User]   │
├──────┬────────────────┬──────────┬─────────┬────────────────┤
│ Name │ Email          │ Role     │ Status  │ Actions        │
├──────┼────────────────┼──────────┼─────────┼────────────────┤
│ John │ john@ex.com    │ Customer │ ● Active│ [Edit][Delete] │
│ Jane │ jane@ex.com    │ Admin    │ ● Active│ [Edit][Delete] │
│ Bob  │ bob@ex.com     │ Customer │ ○ Inact.│ [Edit][Delete] │
├──────┴────────────────┴──────────┴─────────┴────────────────┤
│ [Prev] [1] [2] [3] [Next]                  Showing 1-20 of 50│
└──────────────────────────────────────────────────────────────┘

Legend:
● = Active
○ = Inactive
```

**Create/Edit User Form:**
```
┌──────────────────────────────────┐
│ ✕ {Create | Edit} User           │
├──────────────────────────────────┤
│ Email *                          │
│ [ user@example.com           ]   │
│                                  │
│ Full Name *                      │
│ [ John Doe                   ]   │
│                                  │
│ Role *                           │
│ [ Customer              ▼]       │
│   Options: Customer, Admin,      │
│            Moderator             │
│                                  │
│ Status                           │
│ ( ) Active  ( ) Inactive         │
│                                  │
│ [   Cancel   ] [   Save User  ]  │
└──────────────────────────────────┘
```

**Delete Confirmation Modal:**
```
┌────────────────────────────────────┐
│                                    │
│    ┌──────────────────────────┐   │
│    │ ⚠ Confirm Delete         │   │
│    ├──────────────────────────┤   │
│    │ Are you sure you want to │   │
│    │ delete "John Doe"?       │   │
│    │                          │   │
│    │ This cannot be undone.   │   │
│    │                          │   │
│    │ [Cancel] [    Delete   ] │   │
│    └──────────────────────────┘   │
│                                    │
└────────────────────────────────────┘
```

### Component Interfaces

```typescript
// User List Table
interface UserListTableProps {
  users: User[];
  loading: boolean;
  onEdit: (userId: string) => void;
  onDelete: (userId: string) => void;
  pagination: {
    currentPage: number;
    totalPages: number;
    totalItems: number;
  };
  onPageChange: (page: number) => void;
}

// User Form
interface UserFormProps {
  initialData?: User | null;
  onSubmit: (user: UserFormData) => Promise<void>;
  onCancel: () => void;
  isLoading: boolean;
  error: string | null;
}

interface UserFormData {
  email: string;
  name: string;
  role: 'customer' | 'admin' | 'moderator';
  status: 'active' | 'inactive';
}

// Delete Confirmation Modal
interface DeleteConfirmationModalProps {
  isOpen: boolean;
  userName: string;
  onConfirm: () => void;
  onCancel: () => void;
}
```

### User Interactions

**Create User Flow:**
1. Admin clicks "+ Add New User" button
2. Modal opens with empty UserForm
3. Admin enters email → Real-time validation (email format)
4. Admin enters name → Validation (2+ chars)
5. Admin selects role from dropdown → Default "Customer"
6. Admin clicks "Save User"
7. Loading state: Button shows spinner, form disabled
8. Success: Modal closes, user list refreshes, new user appears, success notification
9. Error: Error message displays in modal, form remains editable

**Delete User Flow:**
1. Admin clicks "Delete" button on user row
2. DeleteConfirmationModal opens with user name
3. Admin clicks "Delete" to confirm (or "Cancel" to abort)
4. If dependencies exist: Error displays, modal remains open
5. If no dependencies: User is deleted, modal closes, list refreshes, success notification

### Accessibility

**Keyboard Navigation:**
- Tab navigates: Search box → Add button → Table rows → Pagination → Edit/Delete buttons
- Enter on row: Opens user detail page
- Arrow up/down: Navigate table rows
- Escape: Close modals

**Screen Reader:**
- Table: role="table", headers have scope="col"
- Row actions: aria-label="Edit {user_name}", aria-label="Delete {user_name}"
- Pagination: aria-label="Page {number}"
- Modal: role="dialog", aria-modal="true", aria-labelledby="modal-title"
- Success notifications: aria-live="polite"
- Error notifications: aria-live="assertive", role="alert"

**Focus Management:**
- Modal open: Focus moves to first input (email field)
- Modal close: Focus returns to trigger button
- Delete confirm: Focus on "Cancel" button (safe default)
- After delete: Focus on first row in updated table

**Color Contrast:**
- Active status (green): #2E7D32 on #FFFFFF (4.9:1) ✅
- Inactive status (gray): #757575 on #FFFFFF (4.6:1) ✅
- Delete button (red): #D32F2F on #FFFFFF (7:1) ✅
- Error text: #D32F2F on #FFFFFF (7:1) ✅

## Non-Functional Requirements

### Performance
- User list loads within 1 second for 1,000 users
- Table sorting/filtering updates within 200ms
- Create/update operations complete within 500ms
- Search autocomplete responds within 300ms (debounced)

### Security
- Authentication: Bearer token (JWT), 24-hour expiry
- Authorization: admin:read for list/view, admin:write for create/update/delete
- HTTPS required for all endpoints
- Input validation: Sanitize all inputs, parameterized SQL queries
- CSRF protection: Token required for all state-changing operations
- Audit logging: Log all user CRUD operations with admin user ID and timestamp

### Usability
- Clear success/error messages (no technical jargon)
- Form validation shows errors inline (near fields)
- Destructive actions (delete) require confirmation
- Loading states for all async operations
- Empty state message if no users: "No users found. Create your first user."

### Scalability
- Support 1,000 concurrent admin users
- Handle 100,000 total users in database
- Table pagination prevents loading all users at once
- Database indexes on email, role, status for fast queries

## Edge Cases & Error Handling

1. **Case:** Admin creates user with email that exists in deleted users
   **Expected:** Error message "Email was previously used by deleted account. Contact support to restore account."

2. **Case:** Two admins try to delete same user simultaneously
   **Expected:** Second delete returns 404 Not Found "User already deleted"

3. **Case:** Admin updates user while another admin is also editing
   **Expected:** Optimistic locking detects conflict, shows "User was modified by another admin. Refresh and retry."

4. **Case:** Database connection fails during create
   **Expected:** Error "Unable to create user. Please try again." and retry option, operation is idempotent

5. **Case:** Admin session expires while on user edit page
   **Expected:** On submit, 401 Unauthorized triggers re-login, after login user returns to edit page with data preserved

## Definition of Done

### Implementation
- [x] All 8 acceptance criteria implemented
- [ ] Unit tests for CRUD operations (95% coverage)
- [ ] Integration tests for API endpoints
- [ ] UI components implemented (UserListTable, UserForm, DeleteModal)
- [ ] Input validation implemented client and server-side
- [ ] Error handling for all edge cases
- [ ] Audit logging implemented

### Code Quality
- [ ] Code follows coding-standards.md
- [ ] No violations of architecture-constraints.md
- [ ] No God Objects (classes >500 lines)
- [ ] Dependency injection used (no direct instantiation)
- [ ] Cyclomatic complexity <10 per method

### Testing
- [ ] All 8 acceptance criteria have automated tests
- [ ] Edge cases have tests (5 edge cases = 5 tests minimum)
- [ ] Test coverage: 95% business logic, 85% API layer, 80% UI components
- [ ] All tests passing (100% pass rate)
- [ ] E2E test covering full CRUD cycle

### Documentation
- [ ] API documentation generated (Swagger/OpenAPI)
- [ ] Code comments for complex logic (e.g., dependency check)
- [ ] README updated with CRUD API endpoints

### Security
- [ ] Input validation prevents SQL injection
- [ ] CSRF tokens required for state changes
- [ ] Authorization checks on all endpoints
- [ ] Audit logging captures admin actions
- [ ] No hardcoded credentials
- [ ] Security scan passed (zero CRITICAL, zero HIGH)

## Workflow History

- **2025-11-05 14:30:00** - Story created, status: Backlog
```

---

## Example 2: Authentication Story - User Login

**Story Type:** Authentication workflow
**Complexity:** 8 points (complex)
**Has API:** Yes
**Has UI:** Yes
**Security-sensitive:** Yes

```markdown
---
id: STORY-042
title: User login with email and password
epic: EPIC-003
sprint: SPRINT-005
status: Backlog
priority: High
points: 8
created: 2025-11-05
updated: 2025-11-05
assigned_to: null
tags: [authentication, security, login]
---

## User Story

As a registered user,
I want to log in with my email and password,
So that I can access my account securely and use authenticated features.

## Acceptance Criteria

### AC1: Successful login with valid credentials
**Given** I have registered account with email "user@example.com" and password "Pass123!"
And my account is verified
**When** I submit login form with correct credentials
**Then** JWT access token is generated (24-hour expiry)
And refresh token is generated (30-day expiry)
And I am redirected to /dashboard page
And my user session is established

### AC2: Invalid credentials handling
**Given** I submit login form with incorrect password
**When** authentication fails
**Then** error message displays "Invalid email or password"
And login form remains visible
And password field is cleared
And email field retains value
And no information leaks about whether email exists (security)

### AC3: Account lockout after failed attempts
**Given** I have failed login 4 times in 15 minutes
**When** I fail login 5th time
**Then** account is locked for 15 minutes
And error displays "Too many failed login attempts. Account locked for 15 minutes."
And lockout is logged for security audit
And after 15 minutes, account automatically unlocks

### AC4: Unverified account login blocked
**Given** I have registered but NOT verified my email
**When** I submit login form with correct credentials
**Then** error displays "Please verify your email before logging in. Check your inbox."
And link displays "Resend verification email"
And login is rejected

### AC5: Remember me functionality
**Given** I am on login page
**When** I check "Remember me" checkbox and login successfully
**Then** refresh token is stored in secure httpOnly cookie
And access token auto-refreshes when expired
And I remain logged in for 30 days (or until explicit logout)

### AC6: Logout functionality
**Given** I am logged in with active session
**When** I click "Logout" button
**Then** access token is revoked (added to blacklist)
And refresh token is invalidated
And I am redirected to /login page
And subsequent API calls with that token return 401 Unauthorized

### AC7: Session persistence across page refresh
**Given** I am logged in
**When** I refresh the page
**Then** I remain logged in (session persists)
And current page re-loads
And user context is maintained

### AC8: Redirect to intended page after login
**Given** I am NOT logged in
And I attempt to access /dashboard (requires auth)
**When** I am redirected to /login with returnUrl=/dashboard
And I login successfully
**Then** I am redirected to /dashboard (my original destination)
And not to default post-login page

## Technical Specification

### API Contracts

#### Endpoint: POST /api/auth/login

**Description:** Authenticate user with email and password

**Authentication:** None (public endpoint)

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "Pass123!",
  "remember_me": false
}
```

**Validation Rules:**
- email: Required, email format
- password: Required, min 8 chars
- remember_me: Optional, boolean, default false

**Success Response (200 OK):**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "uuid-v4-refresh-token",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "name": "John Doe",
    "role": "customer"
  }
}
```

**Response Headers:**
```http
Set-Cookie: refresh_token={token}; HttpOnly; Secure; SameSite=Strict; Max-Age=2592000
```

**Error Response (401 Unauthorized - Invalid Credentials):**
```json
{
  "error": "Unauthorized",
  "message": "Invalid email or password"
}
```

**Error Response (403 Forbidden - Account Locked):**
```json
{
  "error": "Forbidden",
  "message": "Account locked due to too many failed login attempts. Try again in 15 minutes.",
  "locked_until": "2025-11-05T15:45:00Z"
}
```

**Error Response (403 Forbidden - Unverified Account):**
```json
{
  "error": "Forbidden",
  "message": "Email not verified. Check your inbox for verification link.",
  "resend_verification_url": "/api/auth/resend-verification"
}
```

#### Endpoint: POST /api/auth/logout

**Description:** Invalidate user session

**Authentication:** Required (Bearer token)

**Request Body:**
```json
{
  "refresh_token": "uuid-v4-refresh-token"
}
```

**Success Response (204 No Content):**
- Empty body
- Tokens invalidated

**Response Headers:**
```http
Set-Cookie: refresh_token=; HttpOnly; Secure; SameSite=Strict; Max-Age=0
```

#### Endpoint: POST /api/auth/refresh

**Description:** Refresh access token using refresh token

**Authentication:** Required (refresh token in cookie or body)

**Request Body:**
```json
{
  "refresh_token": "uuid-v4-refresh-token"
}
```

**Success Response (200 OK):**
```json
{
  "access_token": "new-jwt-token",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

**Error Response (401 Unauthorized - Invalid Refresh Token):**
```json
{
  "error": "Unauthorized",
  "message": "Invalid or expired refresh token. Please log in again."
}
```

### Data Models

#### Entity: UserSession

**Purpose:** Track active user sessions and refresh tokens

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | Required, PK | Session identifier |
| user_id | UUID | Required, FK → User.id | Associated user |
| access_token_jti | String(255) | Required, Unique | JWT ID for token revocation |
| refresh_token | String(255) | Required, Unique | Refresh token (hashed) |
| ip_address | String(45) | Required | IP address of login |
| user_agent | String(255) | Required | Browser/device info |
| created_at | DateTime | Required | Session start time |
| expires_at | DateTime | Required | Session expiry |
| revoked_at | DateTime | Optional | Token revocation time (logout) |

**Relationships:**
- Belongs to: User (many-to-one)

**Indexes:**
- user_id (non-unique): Fast lookup of user sessions
- refresh_token (unique, hashed): Fast refresh token validation
- access_token_jti (unique): Fast token revocation check

#### Entity: LoginAttempt

**Purpose:** Track login attempts for account lockout

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | Required, PK | Attempt identifier |
| email | String(255) | Required | Email attempted (even if user doesn't exist) |
| success | Boolean | Required | Whether login succeeded |
| ip_address | String(45) | Required | IP address of attempt |
| created_at | DateTime | Required | Attempt timestamp |

**Indexes:**
- (email, created_at) composite: Count recent failed attempts by email

### Business Rules

1. **Password verification:** Use bcrypt.compare() to verify password against hash (never compare plaintext)
2. **Token expiry:** Access tokens expire in 24 hours, refresh tokens in 30 days
3. **Account lockout:** 5 failed login attempts in 15 minutes triggers 15-minute lockout
4. **Failed attempt tracking:** Log all login attempts (success and failure) for security audit
5. **Email verification required:** Users must verify email before login is allowed
6. **Token revocation:** On logout, add access token JTI to blacklist (Redis cache, 24-hour TTL)
7. **Refresh token rotation:** On refresh, issue new refresh token and invalidate old one (security best practice)
8. **Session limits:** Maximum 5 active sessions per user (oldest auto-revoked when limit exceeded)

### Dependencies

**1. JWT Library**
- Type: Library
- Purpose: Generate and verify JWT tokens
- Integration: jsonwebtoken (Node.js), System.IdentityModel.Tokens.Jwt (C#), PyJWT (Python)
- Configuration: SECRET_KEY environment variable (256-bit random key)

**2. Password Hashing**
- Type: Library
- Purpose: Hash and verify passwords securely
- Integration: bcrypt (Node.js/Python), BCrypt.Net (C#)
- Configuration: Cost factor 10 (balances security and performance)

**3. Redis Cache**
- Type: Infrastructure
- Purpose: Token blacklist (revoked access tokens)
- Integration: ioredis (Node.js), StackExchange.Redis (C#)
- SLA: 99.9% uptime, <10ms response time
- Fallback: If Redis unavailable, reject all access tokens (fail secure)

## UI Specification

### Components

#### Component: LoginForm
**Type:** Form
**Purpose:** Authenticate user with email and password

#### Component: PasswordInput
**Type:** Form Input (password)
**Purpose:** Secure password entry with show/hide toggle

### Layout Mockup

```
┌──────────────────────────────────┐
│                                  │
│      [Logo]                      │
│      Welcome Back                │
│                                  │
│ Email Address                    │
│ [ user@example.com           ]   │
│                                  │
│ Password                         │
│ [ ••••••••••                 ] 👁│
│                                  │
│ [ ] Remember me for 30 days      │
│                                  │
│ [Forgot password?]               │
│                                  │
│ [        Log In        ]         │
│                                  │
│ Don't have an account? [Sign up] │
│                                  │
└──────────────────────────────────┘
```

### Component Interfaces

```typescript
interface LoginFormProps {
  onSubmit: (credentials: LoginCredentials) => Promise<void>;
  isLoading: boolean;
  error: string | null;
  returnUrl?: string;
}

interface LoginCredentials {
  email: string;
  password: string;
  rememberMe: boolean;
}

interface PasswordInputProps {
  value: string;
  onChange: (value: string) => void;
  showPassword: boolean;
  onToggleShow: () => void;
  error?: string;
  autoComplete?: string;
}
```

### User Interactions

1. User navigates to /login (or redirected from protected page)
2. Login form displays (email and password fields empty)
3. User enters email → Validation checks email format
4. User enters password → No validation (security - don't reveal password requirements)
5. User optionally checks "Remember me"
6. User clicks "Log In" button
7. Loading state: Button shows spinner, form disabled
8. API call: POST /api/auth/login
9a. Success: Store tokens, redirect to returnUrl or /dashboard
9b. Failure: Show error, re-enable form, clear password field

### Accessibility

**Keyboard:**
- Tab order: Email → Password → Remember me → Forgot password → Log in → Sign up
- Enter on any field: Submit form
- Enter on "Forgot password" or "Sign up": Navigate to those pages

**Screen Reader:**
- Email input: aria-label="Email address"
- Password input: aria-label="Password"
- Password toggle: aria-label="Show password" / "Hide password"
- Remember me: aria-label="Remember me for 30 days"
- Error message: role="alert", aria-live="assertive"

**Focus:**
- Auto-focus on email field when page loads
- After error: Focus on email field (or first invalid field)
- Password toggle button has visible focus indicator

**Contrast:**
- All text: 4.5:1 minimum
- Error message (red): 7:1 contrast
- Links (blue): 4.5:1 contrast

## Non-Functional Requirements

### Performance
- Login API response <300ms (p95)
- Token generation <50ms
- Page load (login form) <1 second
- Form validation (client-side) <50ms

### Security
- Passwords never logged (not in logs, not in errors)
- Tokens never exposed in URLs (only in headers/cookies)
- HTTPS enforced (redirect HTTP to HTTPS)
- Refresh tokens stored in httpOnly cookie (XSS protection)
- Access tokens stored in memory (not localStorage - XSS protection)
- CSRF tokens required for login (prevent login CSRF)
- Rate limiting: 10 login attempts per IP per minute
- Brute force protection: Account lockout after 5 failures
- Audit trail: Log all login attempts with IP, user agent, timestamp

### Usability
- Error messages user-friendly (no technical jargon)
- "Forgot password" link prominent
- "Remember me" option clear
- Loading spinner on submit (user knows request is processing)
- Email field retains value on error (don't make user retype)

### Scalability
- Support 10,000 concurrent logins
- Token generation must scale (no blocking operations)
- Redis cache for token blacklist (distributed, scales horizontally)

## Edge Cases & Error Handling

1. **Case:** User enters email with mixed case "User@Example.COM"
   **Expected:** Email normalized to lowercase "user@example.com" before lookup, login succeeds if credentials valid

2. **Case:** User password contains special characters that need URL encoding
   **Expected:** Password transmitted in JSON body (no URL encoding needed), compared correctly

3. **Case:** Database query timeout during login
   **Expected:** 504 Gateway Timeout after 5 seconds, error "Login service temporarily unavailable. Please try again."

4. **Case:** Redis cache (token blacklist) is unavailable
   **Expected:** Fail secure - reject all tokens, require fresh login, log error for ops team

5. **Case:** User has 5 active sessions (limit reached)
   **Expected:** Oldest session is revoked automatically, new session created, user not aware of limit

6. **Case:** JWT secret key is rotated
   **Expected:** Existing tokens remain valid until expiry (verified with old key for grace period), new tokens use new key

## Definition of Done

### Implementation
- [ ] All 8 acceptance criteria implemented
- [ ] Login API endpoint (/api/auth/login, /api/auth/logout, /api/auth/refresh)
- [ ] Unit tests for authentication logic (95% coverage)
- [ ] Integration tests for login flow
- [ ] UI component (LoginForm) implemented
- [ ] Password hashing with bcrypt (cost 10)
- [ ] JWT token generation with secret key from env var
- [ ] Refresh token rotation implemented
- [ ] Account lockout logic implemented
- [ ] Token blacklist using Redis
- [ ] Audit logging for all login attempts

### Code Quality
- [ ] Code follows coding-standards.md
- [ ] No hardcoded secrets (use environment variables)
- [ ] Dependency injection for auth service
- [ ] Cyclomatic complexity <10 per method
- [ ] No SQL concatenation (parameterized queries only)

### Testing
- [ ] All 8 AC have automated tests
- [ ] Edge cases have tests (6 edge cases = 6 tests)
- [ ] Security tests: SQL injection, timing attacks, brute force
- [ ] Test coverage: 95% auth logic, 85% API, 80% UI
- [ ] All tests passing

### Documentation
- [ ] API docs for login/logout/refresh endpoints
- [ ] Security considerations documented
- [ ] Token lifecycle documented

### Security
- [ ] Passwords hashed with bcrypt (never stored plaintext)
- [ ] JWT tokens signed with secure secret (256-bit)
- [ ] Refresh tokens stored hashed in database
- [ ] HTTPS enforced on all auth endpoints
- [ ] Rate limiting prevents brute force
- [ ] Account lockout after 5 failed attempts
- [ ] CSRF protection on login endpoint
- [ ] Security audit logging implemented
- [ ] No password in logs or error messages
- [ ] Security scan passed (zero CRITICAL/HIGH)

## Workflow History

- **2025-11-05 15:00:00** - Story created, status: Backlog
```

---

## Example 3: Workflow Story - Order Processing

**Story Type:** Multi-step business process with state transitions
**Complexity:** 13 points (very complex)
**Has API:** Yes
**Has UI:** Yes
**Has Background Jobs:** Yes

```markdown
---
id: STORY-087
title: Order processing workflow from pending to delivered
epic: EPIC-012
sprint: SPRINT-008
status: Backlog
priority: Critical
points: 13
created: 2025-11-05
updated: 2025-11-05
assigned_to: null
tags: [workflow, orders, fulfillment, state-machine]
---

## User Story

As a customer,
I want my order to be processed automatically from placement to delivery,
So that I receive my items quickly with real-time status updates.

## Acceptance Criteria

### AC1: Order submitted (Pending)
**Given** I have 3 items in cart totaling $120
**When** I complete checkout with valid payment method
**Then** order is created with status "Pending"
And order number is generated (ORD-2025-00087)
And payment authorization is requested (not captured yet)
And confirmation email is sent within 60 seconds
And I am redirected to /orders/{order_id} status page

### AC2: Payment authorized (Pending → Processing)
**Given** order ORD-2025-00087 has status "Pending"
**When** payment gateway authorizes payment (card valid, funds available)
**Then** order status changes to "Processing"
And payment_authorized_at timestamp is recorded
And inventory is reserved for order items
And warehouse notification is sent (fulfill order)
And customer receives email "Order processing - we're preparing your items"

### AC3: Order packed (Processing → Shipped)
**Given** order has status "Processing"
And warehouse has packed all items
**When** shipping label is generated and order is handed to carrier
**Then** order status changes to "Shipped"
And shipping_carrier and tracking_number are recorded
And customer receives email with tracking link
And estimated delivery date is calculated and shown

### AC4: Order delivered (Shipped → Delivered)
**Given** order has status "Shipped"
**When** carrier confirms delivery (webhook or manual update)
**Then** order status changes to "Delivered"
And delivered_at timestamp is recorded
And payment is captured (charged to customer)
And customer receives email "Order delivered! Thank you for your purchase."
And review request is sent after 3 days

### AC5: Order cancellation (Pending/Processing → Cancelled)
**Given** order has status "Pending" or "Processing"
**When** customer clicks "Cancel Order" and confirms
**Then** order status changes to "Cancelled"
And payment authorization is voided (not charged)
And inventory reservations are released
And cancellation email is sent
And cancellation reason is recorded

### AC6: Cannot cancel shipped order
**Given** order has status "Shipped" or "Delivered"
**When** customer tries to cancel order
**Then** error displays "Cannot cancel order - already {status}. Please initiate return instead."
And order status remains unchanged
And "Return Items" button is shown as alternative

### AC7: Real-time status updates
**Given** I am viewing my order status page
**When** order status changes (any transition)
**Then** status page updates within 30 seconds (WebSocket or polling)
And status timeline shows updated progress
And estimated delivery date adjusts if needed

### AC8: Order status tracking page
**Given** I have order ORD-2025-00087
**When** I navigate to /orders/ORD-2025-00087
**Then** status page displays:
  - Order number and date
  - Current status with progress indicator
  - Timeline: Pending → Processing → Shipped → Delivered
  - Estimated delivery date
  - Order items with images, quantities, prices
  - Shipping address
  - Tracking link (if shipped)
  - Cancel/Return buttons (if applicable)

## Technical Specification

### API Contracts

#### Endpoint: POST /api/orders

**Description:** Create new order from cart contents

**Authentication:** Required (customer token)

**Request Body:**
```json
{
  "cart_id": "uuid",
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
  "payment_method_id": "pm_stripe_token",
  "shipping_method": "standard"
}
```

**Success Response (201 Created):**
```json
{
  "order_id": "uuid",
  "order_number": "ORD-2025-00087",
  "status": "pending",
  "total": 187.50,
  "estimated_delivery": "2025-11-12",
  "created_at": "2025-11-05T15:00:00Z"
}
```

#### Endpoint: PATCH /api/orders/{id}/status

**Description:** Update order status (admin/system use)

**Authentication:** Required (admin or system token)

**Request Body:**
```json
{
  "status": "processing",
  "metadata": {
    "warehouse_id": "WH-001",
    "assigned_at": "2025-11-05T15:30:00Z"
  }
}
```

**Validation:**
- Status must be valid transition (see state machine diagram)
- Invalid transitions return 422 Unprocessable Entity

#### Endpoint: GET /api/orders/{id}

**Description:** Get order details and status

**Authentication:** Required (order owner or admin)

**Success Response (200 OK):**
```json
{
  "order_id": "uuid",
  "order_number": "ORD-2025-00087",
  "status": "shipped",
  "items": [...],
  "subtotal": 150.00,
  "tax": 12.50,
  "shipping": 25.00,
  "total": 187.50,
  "shipping_address": {...},
  "tracking": {
    "carrier": "UPS",
    "tracking_number": "1Z999AA10123456784",
    "tracking_url": "https://ups.com/track?q=1Z999AA10123456784"
  },
  "timeline": [
    {"status": "pending", "timestamp": "2025-11-05T15:00:00Z"},
    {"status": "processing", "timestamp": "2025-11-05T16:00:00Z"},
    {"status": "shipped", "timestamp": "2025-11-06T10:00:00Z"}
  ],
  "estimated_delivery": "2025-11-09",
  "created_at": "2025-11-05T15:00:00Z"
}
```

### Data Models

#### Entity: Order
(See Example 1's comprehensive Order entity - same structure)

#### Entity: OrderStatusHistory

**Purpose:** Track all status changes for order audit trail

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | Required, PK | History record ID |
| order_id | UUID | Required, FK → Order.id | Associated order |
| from_status | Enum | Optional | Previous status (null for first entry) |
| to_status | Enum | Required | New status |
| changed_by | UUID | Required, FK → User.id | User/system that changed status |
| reason | Text | Optional | Reason for change (e.g., cancellation reason) |
| created_at | DateTime | Required | When status changed |

**Relationships:**
- Belongs to: Order (many-to-one)

### Business Rules

1. **State Machine:** Orders follow defined state transitions (see diagram below)
2. **Inventory reservation:** Reserve inventory when order moves to Processing, release on Cancel
3. **Payment timing:** Authorize on Pending, capture on Delivered (reduces fraud, allows easy cancellation)
4. **Cancellation window:** Customers can cancel in Pending/Processing states only
5. **Status history:** All status changes are logged with timestamp, user, and reason
6. **Estimated delivery:** Calculated based on shipping method (standard: 5-7 days, express: 2-3 days)
7. **Automatic transitions:** Some transitions are automatic (payment webhook, carrier delivery confirmation)

**State Machine Diagram:**
```
Pending
  ├─→ Processing (payment authorized)
  ├─→ Cancelled (user cancels or payment fails)

Processing
  ├─→ Shipped (items dispatched)
  ├─→ Cancelled (admin cancels)

Shipped
  ├─→ Delivered (carrier confirms)
  ├─→ Returned (user initiates return)

Delivered
  └─→ Returned (user returns items)

Cancelled (terminal)
Returned (terminal)
```

### Dependencies

**1. Payment Gateway (Stripe)**
- Purpose: Payment authorization and capture
- Integration: Stripe SDK, webhook for async updates
- SLA: 99.99% uptime, <1s response time
- Fallback: Queue payment for retry if Stripe down

**2. Shipping Carrier API**
- Purpose: Generate shipping labels, track packages
- Integration: UPS/FedEx API or EasyPost aggregator
- Fallback: Manual label generation if API unavailable

**3. Background Job Queue**
- Purpose: Async email sending, status transitions
- Integration: Bull (Redis-based, Node.js) or Celery (Python)
- SLA: Process jobs within 60 seconds

**4. WebSocket Server**
- Purpose: Real-time status updates to customer
- Integration: Socket.io (Node.js) or SignalR (C#)
- Fallback: Polling every 30 seconds if WebSocket unavailable

## UI Specification

### Components

#### Component: OrderStatusPage
**Type:** Page Layout
**Purpose:** Display order details and real-time status

#### Component: StatusTimeline
**Type:** Display
**Purpose:** Visual timeline of order progression

#### Component: OrderItemList
**Type:** Display (list)
**Purpose:** Show order items with images and prices

### Layout Mockup

```
┌────────────────────────────────────────────────────────┐
│ Order #ORD-2025-00087                   [Cancel Order] │
├────────────────────────────────────────────────────────┤
│ Status: Shipped                   Est. Delivery: Nov 9 │
│                                                        │
│ Progress:                                              │
│ ●────────●────────●────────○                          │
│ Pending  Processing Shipped  Delivered                │
│ Nov 5    Nov 5      Nov 6    Nov 9 (est)              │
│                                                        │
│ Tracking: UPS 1Z999AA10123456784 [Track Package]      │
│                                                        │
├────────────────────────────────────────────────────────┤
│ Order Items (3 items)                                  │
│                                                        │
│ [Image] Product 1 - Size M                            │
│         Qty: 2 × $40.00 = $80.00                       │
│                                                        │
│ [Image] Product 2                                      │
│         Qty: 1 × $70.00 = $70.00                       │
│                                                        │
│ Subtotal:        $150.00                               │
│ Tax:             $12.50                                │
│ Shipping:        $25.00                                │
│ Total:           $187.50                               │
│                                                        │
├────────────────────────────────────────────────────────┤
│ Shipping Address:                                      │
│ 123 Main St                                            │
│ New York, NY 10001                                     │
│                                                        │
│ [Need help? Contact Support]                           │
└────────────────────────────────────────────────────────┘

● = Completed step (green)
○ = Pending step (gray)
```

### Component Interfaces

```typescript
interface OrderStatusPageProps {
  orderId: string;
}

interface StatusTimelineProps {
  currentStatus: OrderStatus;
  timeline: StatusEvent[];
  estimatedDelivery: string;
}

interface StatusEvent {
  status: OrderStatus;
  timestamp: string;
  message?: string;
}

enum OrderStatus {
  Pending = 'pending',
  Processing = 'processing',
  Shipped = 'shipped',
  Delivered = 'delivered',
  Cancelled = 'cancelled',
  Returned = 'returned'
}

interface OrderItemListProps {
  items: OrderItem[];
  subtotal: number;
  tax: number;
  shipping: number;
  discount: number;
  total: number;
}
```

### User Interactions

1. Customer places order → Redirect to /orders/{id}
2. Status page loads with "Pending" status
3. WebSocket connection established for real-time updates
4. After 1 hour: Payment authorizes → Status updates to "Processing" (WebSocket push or poll)
5. UI updates: Timeline advances, status text changes, cancel button remains
6. After 1 day: Order ships → Status updates to "Shipped"
7. UI updates: Timeline advances, tracking link appears, cancel button replaced with "Return Items"
8. Customer clicks tracking link → Opens carrier tracking page in new tab
9. After 3 days: Delivery confirmed → Status updates to "Delivered"
10. UI updates: Timeline complete, "Order delivered" message, "Return Items" button available

### Accessibility

**Keyboard:**
- Tab navigates: Cancel button → Track package link → Contact support
- Enter activates links and buttons
- Status timeline is informational (not interactive)

**Screen Reader:**
- Timeline: role="list", each status is list item
- Status updates: aria-live="polite" announces status changes
- Progress indicator: aria-valuenow, aria-valuemin, aria-valuemax
- Tracking link: aria-label="Track package with UPS"

**Focus:**
- Cancel button has focus indicator
- After status update: Announce but don't move focus (non-disruptive)

**Contrast:**
- Completed status (green): High contrast
- Pending status (gray): 4.5:1 minimum
- Cancel button (red): 7:1 contrast

## Non-Functional Requirements

### Performance
- Order creation <500ms
- Status page load <2 seconds
- WebSocket connection established <1 second
- Status updates delivered within 30 seconds of change
- Polling fallback every 30 seconds (if WebSocket fails)

### Security
- Order access: Customer sees only their orders, admin sees all
- Payment authorization before inventory reservation (prevent fraud)
- Webhook signature validation (Stripe webhooks)
- HTTPS for all endpoints
- CSRF protection for status changes

### Usability
- Clear status labels (not technical codes)
- Estimated delivery date prominent
- Tracking link opens in new tab (user doesn't lose order page)
- Timeline is visual and text (not just icons)
- Mobile-responsive (order tracking on phone)

### Scalability
- Support 10,000 concurrent order processing operations
- WebSocket server handles 50,000 concurrent connections
- Background jobs process 1,000 status updates per minute
- Database indexes on order_number, customer_id, status for fast queries

## Edge Cases & Error Handling

1. **Case:** Payment authorization fails after order created
   **Expected:** Order remains "Pending", payment_failed flag set, customer receives email "Payment failed. Update payment method to continue."

2. **Case:** Item goes out of stock after order placed but before processing
   **Expected:** Customer notified of backorder, estimated delivery date updated, option to cancel and receive full refund

3. **Case:** Shipping carrier loses package
   **Expected:** Status remains "Shipped", after 14 days customer can file claim, automatic refund processed, new order created

4. **Case:** Customer requests cancellation after order shipped
   **Expected:** Cancellation rejected with message "Order already shipped. Initiate return when delivered.", "Return Items" workflow explained

5. **Case:** WebSocket connection drops during status viewing
   **Expected:** Fallback to polling (every 30 seconds), user not aware of connection method change, reconnect attempt every 5 minutes

## Definition of Done

### Implementation
- [ ] All 8 acceptance criteria implemented
- [ ] State machine implemented with valid transitions only
- [ ] API endpoints for order creation, status update, cancellation
- [ ] Background jobs for async processing (emails, inventory, payments)
- [ ] WebSocket real-time updates (with polling fallback)
- [ ] Unit tests for state machine logic (95% coverage)
- [ ] Integration tests for complete workflow (Pending → Delivered)
- [ ] UI components (OrderStatusPage, StatusTimeline) implemented

### Code Quality
- [ ] State machine is table-driven (not giant if/else)
- [ ] No hardcoded status values (use enum)
- [ ] Event sourcing for status changes (OrderStatusHistory entity)
- [ ] Idempotent status transitions (can safely retry)
- [ ] Cyclomatic complexity <10

### Testing
- [ ] All AC have automated tests (8 tests minimum)
- [ ] State transition tests (valid and invalid transitions)
- [ ] Edge case tests (5 edge cases = 5 tests)
- [ ] E2E test: Full workflow Pending → Delivered
- [ ] Test coverage: 95% business logic

### Documentation
- [ ] State machine diagram in docs/
- [ ] API docs for order endpoints
- [ ] Webhook integration guide (Stripe, Carrier)

### Security
- [ ] Authorization: Customer sees only own orders
- [ ] Webhook signature validation (Stripe, Carrier)
- [ ] Payment data never logged
- [ ] Audit trail for all status changes
- [ ] Security scan passed

## Workflow History

- **2025-11-05 15:30:00** - Story created, status: Backlog

**Note:** This is a complex 13-point story. Consider splitting into:
- STORY-087A: Order creation and payment (5 points)
- STORY-087B: Order fulfillment and shipping (5 points)
- STORY-087C: Delivery and completion (3 points)
```

---

## Example 4: Reporting Story - Analytics Dashboard

**Story Type:** Data visualization and reporting
**Complexity:** 8 points (complex)
**Has API:** Yes
**Has UI:** Yes (heavy)
**Has Charts:** Yes

```markdown
---
id: STORY-125
title: Admin analytics dashboard with key metrics
epic: EPIC-018
sprint: SPRINT-012
status: Backlog
priority: Medium
points: 8
created: 2025-11-05
updated: 2025-11-05
assigned_to: null
tags: [reporting, analytics, dashboard, charts]
---

## User Story

As an administrator,
I want to view key business metrics on a dashboard,
So that I can monitor platform health and make data-driven decisions.

## Acceptance Criteria

### AC1: Display metric cards
**Given** I am logged in as admin
**When** I navigate to /admin/dashboard
**Then** 4 metric cards display:
  - Total Users: {count} (↑ {percent}% from last month)
  - Revenue (30d): ${amount} (↑ {percent}% from previous 30d)
  - Active Orders: {count}
  - Conversion Rate: {percent}%
And each card shows current value, trend indicator (↑↓→), and comparison to previous period
And metrics are cached for 5 minutes (refresh automatically)

### AC2: Revenue chart (line chart)
**Given** I am on dashboard
**When** I view "Revenue Over Time" chart
**Then** line chart displays daily revenue for last 30 days
And chart shows: X-axis (dates), Y-axis (revenue in $), line connecting data points
And hovering data point shows tooltip: "Nov 5: $1,234.56"
And chart loads within 2 seconds
And chart is responsive (adapts to screen size)

### AC3: User growth chart (bar chart)
**Given** I am on dashboard
**When** I view "New Users" chart
**Then** bar chart displays new user signups per day (last 30 days)
And bars are colored by value (higher = darker shade)
And Y-axis shows user count, X-axis shows dates
And clicking bar drills down to user list for that day

### AC4: Filter by date range
**Given** I am viewing dashboard
**When** I select date range "Last 7 days" from dropdown
**Then** all charts update to show 7-day data
And metric cards recalculate for 7-day period
And comparison shows vs previous 7 days
And URL updates with ?range=7d (shareable link)

### AC5: Export dashboard data
**Given** I am viewing dashboard
**When** I click "Export to CSV"
**Then** CSV file downloads within 5 seconds
And filename includes date range: "dashboard-2025-11-01-to-2025-11-05.csv"
And CSV includes all metric values and chart data points
And export continues in background (user can navigate away)

### AC6: Real-time metric updates
**Given** I have dashboard open
**When** new order is placed or user registers (events occur)
**Then** relevant metrics update within 60 seconds
And chart data points update (new bar/line point)
And trend indicators recalculate
And update animation draws attention (subtle pulse)

## Technical Specification

### API Contracts

#### Endpoint: GET /api/admin/analytics/summary

**Description:** Get dashboard summary metrics

**Authentication:** Required (admin:read scope)

**Query Parameters:**
- start_date (date, required): Start of date range (YYYY-MM-DD)
- end_date (date, required): End of date range (YYYY-MM-DD)

**Success Response (200 OK):**
```json
{
  "date_range": {
    "start": "2025-10-06",
    "end": "2025-11-05"
  },
  "metrics": {
    "total_users": {
      "value": 1234,
      "change": 12.5,
      "change_direction": "up",
      "previous_value": 1097
    },
    "revenue_30d": {
      "value": 45678.90,
      "change": 8.3,
      "change_direction": "up",
      "previous_value": 42156.23
    },
    "active_orders": {
      "value": 156,
      "change": -5.4,
      "change_direction": "down",
      "previous_value": 165
    },
    "conversion_rate": {
      "value": 3.45,
      "change": 0.5,
      "change_direction": "up",
      "previous_value": 2.95
    }
  }
}
```

#### Endpoint: GET /api/admin/analytics/revenue-chart

**Description:** Get revenue data for chart visualization

**Query Parameters:**
- start_date, end_date (dates, required)
- interval (string, optional, default: day): Grouping interval (hour, day, week, month)

**Success Response (200 OK):**
```json
{
  "data": [
    {"date": "2025-10-06", "revenue": 1234.56},
    {"date": "2025-10-07", "revenue": 1456.78},
    ...
    {"date": "2025-11-05", "revenue": 1678.90}
  ],
  "summary": {
    "total": 45678.90,
    "average": 1522.63,
    "highest": 2345.67,
    "lowest": 567.89
  }
}
```

### Data Models

**No new entities** - Queries aggregate existing data (User, Order entities)

**Aggregation Queries:**
- Total users: COUNT(*) FROM users WHERE deleted_at IS NULL
- Revenue: SUM(total) FROM orders WHERE status = 'delivered' AND created_at BETWEEN {start} AND {end}
- Active orders: COUNT(*) FROM orders WHERE status IN ('pending', 'processing', 'shipped')
- Conversion rate: (COUNT(orders) / COUNT(DISTINCT user sessions)) * 100

### Business Rules

1. **Metric caching:** Metrics cached for 5 minutes, refresh on cache expiry or manual refresh
2. **Date range limits:** Maximum 365 days (performance constraint)
3. **Real-time threshold:** Metrics update every 60 seconds for real-time feel, actual data lag up to 5 minutes acceptable
4. **Comparison periods:** "Last 30 days" compares to previous 30 days, "Last 7 days" to previous 7 days
5. **Chart data limits:** Maximum 366 data points (1 year daily), use weekly/monthly intervals for longer ranges

### Dependencies

**1. Database (Read Replica)**
- Purpose: Run analytics queries without impacting production database
- Integration: Separate read-only connection string
- Fallback: Use primary database if replica unavailable (with query timeout)

**2. Charting Library**
- Purpose: Render interactive charts (line, bar, pie)
- Integration: Chart.js, Recharts (React), or ApexCharts
- No server dependency (client-side rendering)

## UI Specification

### Components

#### Component: MetricCard
```typescript
interface MetricCardProps {
  title: string;
  value: number | string;
  change: number;
  changeDirection: 'up' | 'down' | 'neutral';
  format: 'number' | 'currency' | 'percentage';
  loading: boolean;
}
```

#### Component: LineChart
```typescript
interface LineChartProps {
  data: ChartDataPoint[];
  xAxis: AxisConfig;
  yAxis: AxisConfig;
  loading: boolean;
  onDataPointClick?: (point: ChartDataPoint) => void;
}

interface ChartDataPoint {
  x: string | number;
  y: number;
  label?: string;
}
```

### Accessibility

**Keyboard:**
- Tab navigates through metric cards, date range picker, export button, charts
- Arrow keys navigate within date picker
- Enter on chart data point: Drill down (if interactive)

**Screen Reader:**
- Metric cards: Announce "Total Users: 1,234, up 12.5% from previous period"
- Charts: Provide data table alternative (hidden visually, available to screen readers)
- Trend indicators: aria-label="Up 12.5 percent"

**Color:**
- Don't rely solely on color for up/down (use ↑↓ symbols too)
- Green/red trends also have symbol
- Chart lines have patterns (solid, dashed) in addition to colors

## Non-Functional Requirements

### Performance
- Dashboard loads within 2 seconds
- Metric queries <500ms each (parallelized)
- Chart rendering <1 second
- Date range change updates within 1 second
- Export CSV <5 seconds for 365 days of data

### Security
- Admin-only access (admin:read permission required)
- No PII in exported CSV (only aggregated data)
- Query timeout 10 seconds (prevent long-running analytics queries from blocking)

### Usability
- Tooltips on hover (show exact values)
- Date range picker intuitive (presets: 7d, 30d, 90d, 365d, custom)
- Loading skeletons while data loads (not blank screen)
- Error message if data unavailable: "Analytics temporarily unavailable. Try again in a few minutes."

### Scalability
- Handle 1M orders in database (analytics queries optimized)
- Aggregation queries use indexes (created_at, status)
- Consider materialized views for expensive aggregations
- Dashboard supports 100 concurrent admins

## Edge Cases & Error Handling

1. **Case:** No data for selected date range
   **Expected:** Empty state "No data available for this period" with suggestion to adjust date range

2. **Case:** Analytics database replica is lagging (replication delay)
   **Expected:** Show last updated timestamp: "Data as of 5 minutes ago"

3. **Case:** Export CSV for 365 days times out
   **Expected:** Error "Export too large. Try shorter date range or contact support.", suggest max 90 days

## Definition of Done

### Implementation
- [ ] All 6 AC implemented
- [ ] API endpoints for metrics and chart data
- [ ] Database queries optimized (indexes, aggregation)
- [ ] UI components (MetricCard, LineChart, BarChart, DateRangePicker)
- [ ] Caching implemented (5-minute cache)
- [ ] CSV export functionality
- [ ] Real-time updates (polling or WebSocket)

### Testing
- [ ] Unit tests for metric calculations
- [ ] Integration tests for API endpoints
- [ ] UI tests for dashboard components
- [ ] Performance tests (queries <500ms)
- [ ] Test coverage 95%

### Security
- [ ] Admin-only access enforced
- [ ] No PII exposure
- [ ] Query timeouts prevent DoS

## Workflow History

- **2025-11-05 16:00:00** - Story created, status: Backlog
```

---

## Progressive Disclosure

**When to load this reference:**
- Phase 2: Requirements Analysis (as examples for requirements-analyst subagent)
- When user needs reference for specific story type
- During validation (compare against best practices)

**Why progressive:**
- Large file (~800 lines)
- Examples are reference material (not always needed)
- Loaded selectively based on story type being created

---

## Example 5: Authentication Story with Email Verification

**Source:** Migrated from /create-story command (original educational reference)

**Domain:** Authentication/Security
**Complexity:** Medium (API + UI + Security)
**Story Points:** 5

**Key Features Demonstrated:**
- Complete authentication flow (registration + email verification)
- Dual API endpoints (POST register, GET verify-email)
- Security-focused data model (verification tokens, expiration)
- Complex UI with real-time validation (password strength indicator)
- Comprehensive WCAG AA accessibility requirements
- Measurable NFRs (response times, rate limiting, queue capacity)
- Security edge cases (SQL injection, token reuse, service failures)

**Use this example for:** Authentication stories, email workflows, security requirements, complex forms, accessibility compliance

---

**File:** `devforgeai/specs/Stories/STORY-042-user-registration-email-verification.story.md`

```markdown
---
id: STORY-042
title: User registration with email verification
epic: EPIC-003
sprint: SPRINT-005
status: Backlog
priority: High
points: 5
created: 2025-10-31
updated: 2025-10-31
assigned_to: null
tags: [authentication, security]
---

## User Story

As a new user,
I want to register with my email and verify it,
So that I can access the platform securely and receive important notifications.

## Acceptance Criteria

### AC1: User submits registration form
**Given** I am on the registration page
**When** I submit valid email, password, and name
**Then** I receive a success message and verification email

### AC2: Email verification link
**Given** I received a verification email
**When** I click the verification link
**Then** My account is activated and I am redirected to login

### AC3: Password strength validation
**Given** I am filling the registration form
**When** I enter a password
**Then** Real-time feedback shows strength (weak/medium/strong)

### AC4: Duplicate email handling
**Given** An account exists with my email
**When** I try to register with the same email
**Then** I see an error "Email already registered"

## Technical Specification

### API Contracts

#### Endpoint: POST /api/auth/register

**Request:**
```json
{
  "email": "user@example.com",
  "password": "SecurePass123!",
  "name": "John Doe"
}
```

**Response (201):**
```json
{
  "user_id": "uuid",
  "email": "user@example.com",
  "verification_sent": true,
  "message": "Verification email sent"
}
```

**Response (400):**
```json
{
  "error": "Validation failed",
  "details": ["Password must be at least 8 characters"]
}
```

#### Endpoint: GET /api/auth/verify-email?token={token}

**Response (200):**
```json
{
  "verified": true,
  "redirect_url": "/login"
}
```

**Response (400):**
```json
{
  "error": "Invalid or expired token"
}
```

### Data Models

#### Model: User
| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| id | UUID | Required, PK | Unique identifier |
| email | String | Required, Unique, Email format | User email |
| password_hash | String | Required | Bcrypt hashed password |
| name | String | Required, 2-100 chars | Full name |
| verified | Boolean | Default: false | Email verification status |
| verification_token | String | Nullable | Email verification token |
| token_expires_at | DateTime | Nullable | Token expiration time |
| created_at | DateTime | Auto | Account creation timestamp |

### Business Rules

1. Password must be 8+ characters with uppercase, lowercase, number, special char
2. Verification token expires after 24 hours
3. Verification email sent asynchronously (background job)
4. Unverified users cannot log in
5. Email is case-insensitive (stored lowercase)

### Dependencies

- Email service: SendGrid or SMTP
- Background job queue: Redis/Bull or similar
- Password hashing: Bcrypt library

## UI Specification

### Components

#### Component: RegistrationForm
**Type:** Form
**Purpose:** Collect user registration details
**Data Bindings:** email, password, name, passwordStrength

#### Component: PasswordStrengthIndicator
**Type:** Visual feedback
**Purpose:** Show real-time password strength
**Data Bindings:** password (input), strength (computed)

### Layout Mockup

```
+------------------------------------------+
|           Create Your Account            |
+------------------------------------------+
| Email Address                            |
| [ user@example.com                    ]  |
|                                          |
| Full Name                                |
| [ John Doe                            ]  |
|                                          |
| Password                                 |
| [ ••••••••••                          ]  |
| [####--------] Medium strength           |
| Must be 8+ chars with uppercase, number  |
|                                          |
| [ ] I agree to Terms of Service          |
|                                          |
| [        Create Account       ]          |
|                                          |
| Already have an account? [Log in]        |
+------------------------------------------+
```

### Component Interface

```typescript
interface RegistrationFormProps {
  onSubmit: (data: RegistrationData) => Promise<void>;
  isLoading: boolean;
  error: string | null;
}

interface RegistrationData {
  email: string;
  password: string;
  name: string;
  agreedToTerms: boolean;
}

interface PasswordStrengthIndicatorProps {
  password: string;
  onChange?: (strength: 'weak' | 'medium' | 'strong') => void;
}
```

### User Interactions

1. User navigates to /register
2. Form displays with empty fields
3. User types email → Real-time validation (email format)
4. User types password → Strength indicator updates
5. User types name → Validation (2+ chars)
6. User checks "Terms" checkbox
7. User clicks "Create Account"
8. Loading state: Button shows spinner, form disabled
9. Success: Redirect to /verify-email-sent page
10. Error: Error message displays above form, form remains editable

### Accessibility

- **Keyboard:** Tab through fields, Enter to submit
- **Screen reader:**
  - aria-label="Email address" on email input
  - aria-describedby="password-requirements" on password field
  - aria-live="polite" on error messages
- **Focus:** Blue outline on focused field (3px solid)
- **Contrast:** Error text #D32F2F on #FFFFFF (7:1 ratio)
- **Error announcements:** Screen reader announces validation errors

## Non-Functional Requirements

### Performance
- Registration API response < 200ms (excluding email send)
- Email delivery within 30 seconds
- Password strength calculation < 50ms

### Security
- HTTPS required for registration endpoint
- Password hashed with Bcrypt (cost factor 10)
- Verification tokens cryptographically random (32 bytes)
- Rate limiting: 5 registration attempts per IP per hour
- CSRF protection enabled

### Usability
- Clear password requirements shown upfront
- Real-time validation feedback (not just on submit)
- Success message confirms email sent
- Verify-email-sent page includes resend option

### Scalability
- Support 1,000 concurrent registrations
- Email queue handles 10,000 messages/hour
- Database indexes on email (unique) and verification_token

## Edge Cases & Error Handling

1. **Case:** User closes browser before verification
   **Expected:** Token remains valid for 24 hours, resend option available

2. **Case:** Verification link clicked twice
   **Expected:** Second click shows "Already verified" message

3. **Case:** Email service is down
   **Expected:** Registration succeeds, email queued for retry (max 3 attempts)

4. **Case:** Token expired
   **Expected:** User can request new verification email from login page

5. **Case:** SQL injection attempt in email field
   **Expected:** Parameterized queries prevent injection, validation rejects

## Workflow History

- **2025-10-31 14:32** - Story created, status: Backlog
```

---

## Progressive Disclosure

**When to load this reference:**
- Phase 2: Requirements Analysis (as examples for requirements-analyst subagent)
- When user needs reference for specific story type
- During validation (compare against best practices)

**Why progressive:**
- Large file (~2,200 lines after Example 5 added)
- Examples are reference material (not always needed)
- Loaded selectively based on story type being created

---

**These examples demonstrate complete, production-ready stories with all required sections, serving as templates for story creation.**
