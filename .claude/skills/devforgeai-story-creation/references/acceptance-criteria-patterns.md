# Acceptance Criteria Patterns

Comprehensive reference for writing testable, unambiguous acceptance criteria following Given/When/Then format.

## Purpose

This guide provides proven patterns for acceptance criteria across different story types (CRUD, authentication, workflow, reporting, integration) to ensure all criteria are testable and complete.

---

## Core Principles

### Testability

**Every acceptance criterion must be verifiable through automated testing.**

✅ **Testable:**
```
Given user is logged in
When user clicks "Logout" button
Then user session is terminated and redirected to login page
```
**Why:** Can write test that verifies session termination and redirect

❌ **Not Testable:**
```
User should have a good experience when logging out
```
**Why:** "good experience" is subjective, cannot automate

### Completeness

**Acceptance criteria must cover:**
- Happy path (successful completion)
- Error scenarios (validation failures, system errors)
- Edge cases (boundary conditions, concurrent access)
- Security scenarios (unauthorized access, injection attempts)

### Unambiguity

**Avoid vague language:**
- ❌ "should", "might", "could", "probably"
- ❌ "fast", "slow", "easy", "intuitive"
- ❌ "user-friendly", "responsive", "efficient"

**Use specific, measurable terms:**
- ✅ "within 500ms", "less than 2 seconds"
- ✅ "displays error message 'Invalid email format'"
- ✅ "supports 1,000 concurrent users"

---

## Given/When/Then Format

### Structure

**Given** - Context/Precondition
- System state before action
- User authentication/authorization state
- Data that already exists
- External system state

**When** - Action/Trigger
- User action (click, type, submit, navigate)
- System event (timer, webhook, message)
- External trigger (API call, scheduled job)

**Then** - Expected Outcome
- System response
- Data changes
- UI updates
- Side effects (emails sent, logs created)

### Examples by Complexity

**Simple (Single action, single outcome):**
```
Given user is on homepage
When user clicks "Sign Up" button
Then registration form displays
```

**Moderate (Multiple conditions):**
```
Given user is logged in as admin
And there are 10 users in the database
When user navigates to "/admin/users" page
Then all 10 users are displayed in a table with columns: Name, Email, Role, Status
```

**Complex (Multiple steps and validations):**
```
Given user has items in shopping cart totaling $150
And user has selected express shipping ($25)
And user has applied coupon "SAVE10" (10% off)
When user proceeds to checkout
Then order total displays as $160.50 ($150 - $15 discount + $25 shipping)
And payment page loads within 2 seconds
And order summary shows itemized breakdown
```

---

## CRUD Operation Patterns

### Create (C)

**Happy Path:**
```
### AC1: Successful creation
Given user is on {entity} creation form
When user submits valid {entity} data
Then {entity} is created in database
And success message displays "{Entity} created successfully"
And user is redirected to {entity} list page
And new {entity} appears in the list
```

**Validation Errors:**
```
### AC2: Required field validation
Given user is on {entity} creation form
When user submits form with {field} empty
Then form validation displays error "{Field} is required"
And form remains on page (no submission)
And focus moves to {field} input
And screen reader announces error
```

**Duplicate Prevention:**
```
### AC3: Duplicate {unique_field} handling
Given {entity} with {unique_field} "value" already exists
When user tries to create {entity} with same {unique_field}
Then validation error displays "{Unique_field} already exists"
And form remains editable
And existing {entity} is not modified
```

**Business Rule Validation:**
```
### AC4: {Business rule} validation
Given user is creating {entity}
When user enters {field} that violates {business_rule}
Then validation error displays "{Rule description}"
And field is highlighted in red
And user can correct and resubmit
```

### Read (R)

**List View:**
```
### AC1: Display all {entities}
Given database contains {count} {entities}
When user navigates to {entity} list page
Then all {count} {entities} are displayed
And each {entity} shows {field1}, {field2}, {field3}
And list is paginated (20 per page) if count > 20
```

**Detail View:**
```
### AC2: View {entity} details
Given {entity} with ID {id} exists
When user clicks on {entity} in list
Then detail page displays showing all {entity} fields
And related {related_entities} are displayed
And action buttons show: Edit, Delete, {custom_actions}
```

**Search/Filter:**
```
### AC3: Search {entities} by {field}
Given database contains 100 {entities}
When user enters "query" in search box
Then {entities} matching "query" in {field} are displayed
And results update in real-time (debounced 300ms)
And "X results found" message displays
```

**Empty State:**
```
### AC4: Empty state handling
Given database contains zero {entities}
When user navigates to {entity} list page
Then empty state message displays "No {entities} found. Create your first {entity}?"
And "Create {Entity}" button is prominently displayed
```

### Update (U)

**Happy Path:**
```
### AC1: Successful update
Given {entity} with ID {id} exists
And user is on {entity} edit form
When user modifies {field} and submits
Then {entity} is updated in database
And success message displays "{Entity} updated successfully"
And user is redirected to {entity} detail page
And updated {field} value is displayed
```

**Concurrent Modification (Optimistic Locking):**
```
### AC2: Concurrent update detection
Given {entity} was loaded at timestamp T1
And another user updated {entity} at timestamp T2 (T2 > T1)
When user submits changes
Then error message displays "This {entity} was modified by another user. Please refresh and retry."
And user's changes are not lost (displayed for review)
And user can choose to overwrite or merge
```

**Partial Update:**
```
### AC3: Update specific fields only
Given {entity} with 10 fields exists
When user modifies only {field1} and {field2}
Then only {field1} and {field2} are updated
And other 8 fields remain unchanged
And {updated_at} timestamp is updated
And {updated_by} is set to current user
```

**Validation on Update:**
```
### AC4: Update validation
Given {entity} exists with valid data
When user modifies {field} to invalid value
Then validation error displays "{Field} validation message"
And update is rejected (database not modified)
And form shows current valid value
And user can correct and resubmit
```

### Delete (D)

**Soft Delete (Preferred):**
```
### AC1: Soft delete {entity}
Given {entity} with ID {id} exists
When user clicks "Delete" and confirms
Then {entity}.deleted_at is set to current timestamp
And {entity}.deleted_by is set to current user
And {entity} no longer appears in list view
And {entity} can be restored (undelete functionality)
```

**Hard Delete (Permanent):**
```
### AC2: Hard delete {entity}
Given {entity} with ID {id} exists
And {entity} has no dependent records
When user clicks "Delete" and confirms twice
Then {entity} is permanently removed from database
And success message displays "{Entity} deleted"
And user is redirected to {entity} list
And {entity} cannot be recovered
```

**Delete Confirmation:**
```
### AC3: Delete confirmation required
Given user clicks "Delete" button
When confirmation modal displays
Then modal shows: "Are you sure you want to delete '{entity_name}'? This cannot be undone."
And modal has "Cancel" and "Delete" buttons
And "Delete" button is styled in warning color (red)
And pressing Escape or clicking Cancel closes modal without deleting
```

**Cascade Delete Protection:**
```
### AC4: Prevent delete if dependencies exist
Given {entity} with ID {id} exists
And {entity} has {count} related {dependent_entities}
When user tries to delete {entity}
Then error message displays "Cannot delete {entity} - {count} {dependent_entities} depend on it"
And {entity} is not deleted
And user is shown list of dependent records
```

---

## Authentication & Authorization Patterns

### Registration

**Happy Path:**
```
### AC1: Successful registration
Given user is on registration page
When user submits valid email, password, and name
Then new user account is created
And verification email is sent within 30 seconds
And success message displays "Account created. Check your email to verify."
And user is redirected to /verify-email-sent page
```

**Password Strength:**
```
### AC2: Password strength validation
Given user is filling registration form
When user types password in password field
Then real-time strength indicator updates (weak/medium/strong)
And requirements checklist shows: 8+ chars, uppercase, lowercase, number, special char
And each requirement shows ✓ (green) when met or ✗ (red) when not met
And submit button is disabled until all requirements met
```

**Duplicate Email:**
```
### AC3: Duplicate email prevention
Given user with email "existing@example.com" already exists
When new user tries to register with "existing@example.com"
Then validation error displays "Email already registered"
And link displays "Already have an account? Log in"
And existing account is not modified
```

**Email Verification:**
```
### AC4: Email verification workflow
Given user has registered but not verified
When user clicks verification link in email
Then user account is marked as verified
And success message displays "Email verified! You can now log in."
And user is redirected to login page
```

### Login/Logout

**Successful Login:**
```
### AC1: Valid credentials login
Given user with email "user@example.com" and password "Pass123!" exists
And user account is verified
When user submits login form with correct credentials
Then JWT token is generated and returned
And token expires in 24 hours
And user is redirected to dashboard
And user session is established (user_id stored)
```

**Invalid Credentials:**
```
### AC2: Invalid password handling
Given user with email "user@example.com" exists
When user submits login form with incorrect password
Then error message displays "Invalid email or password"
And login form remains visible
And password field is cleared
And no information leaks about whether email exists (security)
```

**Account Lockout:**
```
### AC3: Account lockout after failed attempts
Given user has failed login 4 times
When user fails login 5th time
Then account is locked for 15 minutes
And error message displays "Account locked due to too many failed attempts. Try again in 15 minutes."
And lockout is logged for security audit
```

**Logout:**
```
### AC4: User logout
Given user is logged in with active session
When user clicks "Logout" button
Then user session is terminated (token invalidated)
And user is redirected to login page
And subsequent API calls with that token return 401 Unauthorized
```

### Authorization (RBAC)

**Role-Based Access:**
```
### AC1: Admin-only access enforcement
Given user is logged in as "Customer" role
When user attempts to access /admin/users page
Then 403 Forbidden error displays
And error message shows "Access denied. Admin privileges required."
And user is redirected to their dashboard
And unauthorized access attempt is logged
```

**Permission Checking:**
```
### AC2: Edit permission validation
Given user is logged in with "Editor" role
And "Editor" role has "edit:articles" permission
When user clicks "Edit" on article
Then article edit page loads
And user can modify and save article
```

**Missing Permission:**
```
### AC3: Missing permission handling
Given user is logged in with "Viewer" role
And "Viewer" role does NOT have "delete:articles" permission
When user attempts to delete article
Then 403 Forbidden response
And UI hides "Delete" button (preventive measure)
And error message displays "You don't have permission to delete articles"
```

---

## Workflow & State Transition Patterns

### Order Processing

**Submit Order:**
```
### AC1: Submit order with items
Given user has 3 items in cart totaling $120
And user has selected standard shipping
When user completes checkout
Then order is created with status "Pending"
And order number is generated (e.g., ORD-2025-00042)
And confirmation email is sent within 60 seconds
And cart is emptied
And user is redirected to order confirmation page
```

**Payment Processing:**
```
### AC2: Payment authorization
Given order ORD-2025-00042 exists with status "Pending"
When payment gateway authorizes payment
Then order status changes to "Processing"
And payment_authorized_at timestamp is recorded
And inventory is reserved for order items
And warehouse notification is sent
```

**State Transition Rules:**
```
### AC3: Order can only be cancelled if pending or processing
Given order with status "Pending"
When user clicks "Cancel Order"
Then order status changes to "Cancelled"
And payment is refunded (if already charged)
And inventory reservations are released

Given order with status "Shipped"
When user clicks "Cancel Order"
Then error displays "Cannot cancel order - already shipped. Please initiate return."
And order status remains "Shipped"
```

### Approval Workflows

**Submit for Approval:**
```
### AC1: Submit document for approval
Given document has status "Draft"
And current user is document owner
When user clicks "Submit for Approval"
Then document status changes to "Pending Approval"
And assigned approver receives email notification
And document becomes read-only for owner
And timestamp {submitted_at} is recorded
```

**Approve:**
```
### AC2: Manager approves document
Given document has status "Pending Approval"
And current user is assigned approver with "approve:documents" permission
When user clicks "Approve" and adds optional comment
Then document status changes to "Approved"
And timestamp {approved_at} is recorded
And {approved_by} is set to current user
And document owner receives approval notification
And document becomes available for next workflow step
```

**Reject:**
```
### AC3: Manager rejects document
Given document has status "Pending Approval"
And current user is assigned approver
When user clicks "Reject" and provides reason
Then document status changes to "Rejected"
And {rejected_at} timestamp is recorded
And {rejected_by} and {rejection_reason} are recorded
And document owner receives rejection notification with reason
And document status returns to "Draft" for owner to revise
```

---

## Data Validation Patterns

### Email Format

```
### ACX: Email format validation
Given user is entering email in {form}
When user enters "{invalid_email}" (e.g., "notanemail", "missing@domain", "@example.com")
Then validation error displays "Invalid email format"
And field is highlighted in red
And submit button is disabled
And error announcement for screen readers

When user enters valid email (e.g., "user@example.com")
Then validation passes
And field highlight returns to normal
And submit button becomes enabled
```

### Date Range Validation

```
### ACX: Start date before end date
Given user is selecting date range
When user selects end_date that is before start_date
Then validation error displays "End date must be after start date"
And date picker highlights error
And submit is disabled

When user corrects dates (end_date > start_date)
Then validation passes
And date range is accepted
```

### Numeric Range Validation

```
### ACX: Quantity within allowed range
Given product has max_quantity = 10
When user enters quantity 15
Then validation error displays "Maximum quantity is 10"
And quantity field shows error state
And user cannot add to cart

When user enters quantity 5 (within range 1-10)
Then validation passes
And "Add to Cart" button is enabled
```

### String Length Validation

```
### ACX: Password length validation
Given user is setting password
When user enters password with 6 characters
Then validation error displays "Password must be at least 8 characters"
And strength indicator shows "Too short"

When user enters password with 8+ characters
Then validation passes
And strength indicator evaluates other criteria (uppercase, lowercase, etc.)
```

### Regex Pattern Validation

```
### ACX: Phone number format
Given user is entering phone number
When user enters "123-456-7890" (valid US format)
Then validation passes
And formatted as "(123) 456-7890" for display

When user enters "12345" (invalid format)
Then validation error displays "Phone number must be 10 digits (e.g., 123-456-7890)"
And field shows error state
```

---

## Search & Filtering Patterns

### Text Search

```
### AC1: Search by keyword
Given database contains 100 products
And 15 products have "laptop" in name or description
When user enters "laptop" in search box
Then 15 matching products are displayed
And search term is highlighted in results
And "15 results found" message displays
And results load within 500ms
```

### Filter Combination

```
### AC2: Multiple filter criteria
Given user is viewing product list
When user selects category "Electronics" AND price range "$500-$1000"
Then only products matching BOTH criteria are displayed
And filter tags show "Electronics" and "$500-$1000" with X to remove
And result count updates (e.g., "23 products")
And URL updates to include filter parameters (shareable)
```

### No Results

```
### AC3: No search results
Given user searches for "xyz123abc"
And no products match query
Then empty state displays "No products found for 'xyz123abc'"
And suggestions display "Try different keywords" or "Clear filters"
And search box remains filled with query (user can edit)
```

### Real-Time Search

```
### AC4: Real-time search suggestions
Given user is typing in search box
When user types "lap" (3+ characters)
Then dropdown shows top 5 suggestions within 200ms
And suggestions highlight matching text ("lap" in bold)
And user can click suggestion to search
And user can press Enter to search for typed text
And dropdown closes on Escape key
```

---

## Pagination & Sorting Patterns

### Pagination

```
### AC1: Paginated list navigation
Given database contains 95 {entities}
And page size is 20 items
When user views {entity} list page
Then first 20 items display (page 1 of 5)
And pagination controls show: [Prev] [1] [2] [3] [4] [5] [Next]
And current page (1) is highlighted
And "Prev" button is disabled (already on first page)

When user clicks "Next" or page "2"
Then items 21-40 display
And URL updates to include ?page=2
And "Prev" button becomes enabled
```

### Sorting

```
### AC2: Sort by column
Given {entity} list displays 50 items
When user clicks "Name" column header
Then items are sorted alphabetically by name (A-Z)
And column header shows ↑ icon
And sort direction is ascending

When user clicks "Name" header again
Then items are sorted reverse alphabetically (Z-A)
And column header shows ↓ icon
And sort direction is descending
```

### Combined Pagination + Sorting

```
### AC3: Pagination persists through sorting
Given user is on page 3 of {entity} list
When user changes sort order
Then results re-sort
And user remains on page 3 (if enough items)
Or moves to last available page (if items reduced)
And pagination controls update accordingly
```

---

## File Upload Patterns

### Single File Upload

```
### AC1: Upload single file
Given user is on upload form
When user selects valid file (JPG, PNG, max 5MB)
Then file uploads within 10 seconds
And progress bar shows upload percentage
And success message displays "File uploaded: {filename}"
And thumbnail preview displays (for images)
And file is stored in {storage_location}
```

### File Validation

```
### AC2: File type validation
Given user selects file "document.exe"
And allowed types are: JPG, PNG, PDF
When user attempts to upload
Then validation error displays "Invalid file type. Allowed: JPG, PNG, PDF"
And file is rejected (not uploaded)
And user can select different file
```

### File Size Limit

```
### AC3: File size validation
Given max file size is 5MB
When user selects file larger than 5MB
Then validation error displays "File too large. Maximum size: 5MB"
And file is rejected before upload starts
And user can compress or select different file
```

### Multiple Files

```
### AC4: Upload multiple files
Given user can upload up to 10 files
When user selects 5 files via file picker
Then all 5 files upload in parallel
And individual progress bars show for each file
And overall progress shows "3 of 5 uploaded"
And user can cancel individual uploads
And user can continue using app while uploads continue (background)
```

---

## Real-Time & Async Patterns

### WebSocket Updates

```
### AC1: Real-time notifications
Given user is logged in with WebSocket connection
When another user creates {entity} relevant to this user
Then notification appears in UI within 2 seconds
And notification count increments
And notification shows: Icon, message, timestamp, action button
And notification persists until user dismisses or acts on it
```

### Background Jobs

```
### AC2: Long-running task with progress
Given user initiates {long_task} (e.g., "Generate report")
When task is submitted
Then task starts in background
And task_id is returned immediately
And user can navigate away (task continues)
And progress updates via polling (every 2 seconds) or WebSocket
And progress bar shows "35% complete - Estimated 2 minutes remaining"

When task completes:
Then success notification displays
And result is available for download
And task status is "Completed"
```

### Polling for Updates

```
### AC3: Poll for {entity} status changes
Given {entity} has external processing (e.g., payment gateway)
When user is on {entity} detail page
Then status is polled every 5 seconds
And UI updates when status changes
And polling stops when terminal status reached ("Completed", "Failed")
And polling stops if user navigates away
```

---

## Error Handling Patterns

### Network Errors

```
### AC1: API request fails (network error)
Given user submits form
When network request fails (no connection)
Then error message displays "Connection error. Please check your internet and try again."
And form data is preserved (not lost)
And "Retry" button is displayed
And user can retry without re-entering data
```

### Server Errors (5XX)

```
### AC2: Server error (500)
Given user submits request
When server returns 500 Internal Server Error
Then error message displays "Something went wrong. Our team has been notified. Please try again later."
And error is logged with request ID for debugging
And user can retry or contact support
And request ID is shown to user (for support reference)
```

### Validation Errors (400)

```
### AC3: Multiple validation errors
Given user submits form with 3 invalid fields
When server returns 400 with validation errors
Then all 3 errors are displayed near their respective fields
And first invalid field receives focus
And error summary displays at top: "Please correct 3 errors below"
And screen reader announces "Form has errors"
```

### Timeout Errors

```
### AC4: Request timeout
Given user submits request
When request takes longer than 30 seconds
Then timeout error displays "Request taking longer than expected"
And options displayed: "Keep waiting" or "Cancel"
And if user keeps waiting, timeout extends by 30 seconds
And if user cancels, request is aborted (if possible)
```

---

## Reporting & Analytics Patterns

### Dashboard Display

```
### AC1: Display key metrics
Given user is admin with access to dashboard
When user navigates to /dashboard
Then 4 metric cards display:
  - Total Users: {count} (↑ {increase}% from last month)
  - Revenue: ${amount} (↑ {increase}% from last month)
  - Orders: {count} (↓ {decrease}% from last month)
  - Conversion Rate: {percentage}%
And each metric shows trend indicator (↑ up, ↓ down, → flat)
And metrics are cached (refreshed every 5 minutes)
```

### Chart Visualization

```
### AC2: Revenue chart over time
Given dashboard displays revenue data
When user selects "Last 30 days" filter
Then line chart displays daily revenue for past 30 days
And chart shows: X-axis (dates), Y-axis (revenue in $)
And data points are labeled on hover
And chart loads within 1 second
And chart is responsive (adapts to screen size)
```

### Export Data

```
### AC3: Export report to CSV
Given user is viewing {entity} list with 500 items
When user clicks "Export to CSV"
Then CSV file downloads within 5 seconds
And filename includes timestamp: "{entity}-export-2025-11-05.csv"
And CSV includes all columns from table
And CSV respects current filters/sorting
And user can continue using app while export generates
```

### Drill-Down

```
### AC4: Drill down from summary to details
Given dashboard shows "Revenue by Category" chart
When user clicks "Electronics" bar in chart
Then detailed view displays all Electronics orders
And filters are auto-applied (Category = Electronics)
And breadcrumb shows: Dashboard > Revenue > Electronics
And user can click breadcrumb to navigate back
```

---

## Integration & External Service Patterns

### Payment Gateway

```
### AC1: Process payment via Stripe
Given order total is $99.50
And user has entered valid credit card details
When user clicks "Pay Now"
Then Stripe API is called to create payment intent
And if payment succeeds:
  - Order status changes to "Paid"
  - Confirmation email sent
  - Receipt generated
  - User redirected to /order/success

And if payment fails:
  - Error message displays Stripe error (e.g., "Card declined")
  - Order status remains "Pending Payment"
  - User can retry with same or different card
```

### Third-Party API

```
### AC2: Fetch data from external API
Given user requests weather for "New York"
When weather data is fetched from OpenWeather API
Then current temperature, conditions, and forecast display
And data is cached for 30 minutes (avoid repeated API calls)
And if API is unavailable:
  - Cached data is shown (if available)
  - Or message displays "Weather data temporarily unavailable"

And API response time < 2 seconds
And API key is NOT exposed in client-side code
```

### Webhook Handling

```
### AC3: Receive webhook from external service
Given system is registered for {service} webhooks
When {service} sends webhook POST to /api/webhooks/{service}
Then webhook payload is validated (signature check)
And if valid:
  - Webhook is processed asynchronously (background job)
  - 200 OK response returned immediately (<100ms)
  - Relevant entity is updated based on webhook data
  - Notification sent to affected users (if applicable)

And if invalid signature:
  - 401 Unauthorized response returned
  - Webhook is logged but not processed
  - Security alert is triggered
```

---

## Edge Case Patterns

### Boundary Conditions

```
### ACX: Empty input handling
Given user submits form with all fields empty
Then validation errors display for all required fields
And form is not submitted
And focus moves to first invalid field

Given user submits form with whitespace-only values ("   ")
Then validation treats as empty
And error displays "{Field} cannot be blank"
```

```
### ACX: Maximum length handling
Given field has max_length = 100 characters
When user types 100 characters
Then input is accepted
And character counter shows "100/100"

When user tries to type 101st character
Then input is rejected (character not entered)
And counter shows "100/100" in warning color
```

```
### ACX: Minimum value handling
Given quantity field has minimum = 1
When user enters 0 or negative number
Then validation error displays "Quantity must be at least 1"
And submit is disabled

When user enters 1
Then validation passes
```

### Concurrent Access

```
### ACX: Two users edit same record
Given User A loads {entity} at time T1
And User B loads same {entity} at time T2
And User B saves changes at time T3
When User A tries to save changes at time T4
Then conflict detection triggers
And message displays "This {entity} was modified by {User B} at {T3}. Please refresh and retry."
And User A's changes are preserved (shown in comparison view)
And User A can choose: Overwrite, Merge, or Cancel
```

### Race Conditions

```
### ACX: Prevent duplicate submission
Given user clicks "Submit" button on payment form
When network is slow (2 second delay)
And user clicks "Submit" button again (double-click)
Then only ONE payment request is sent
And button is disabled after first click
And loading spinner displays
And duplicate click is ignored (no second request)
```

### Session Expiration

```
### ACX: Session expires during form fill
Given user session expires after 30 minutes of inactivity
And user has been filling form for 35 minutes
When user submits form
Then 401 Unauthorized response
And modal displays "Your session has expired. Please log in again to continue."
And form data is preserved in session storage
And after login, user is returned to form with data intact
```

### Data Loss Prevention

```
### ACX: Browser refresh with unsaved changes
Given user has modified form fields
And changes are not saved
When user attempts to refresh page or navigate away
Then browser confirmation displays "You have unsaved changes. Are you sure you want to leave?"
And if user confirms: Changes are lost
And if user cancels: User remains on page with changes intact
```

---

## Performance & Scalability Patterns

### Response Time

```
### ACX: API response time under load
Given system is handling 500 concurrent requests
When user makes API request
Then response is returned within 500ms (95th percentile)
And if response time exceeds 1 second:
  - Warning is logged
  - Performance monitoring alert triggers
```

### Large Dataset Handling

```
### ACX: Display large list efficiently
Given database contains 10,000 {entities}
When user navigates to {entity} list
Then first 20 items load within 1 second
And virtual scrolling is enabled (only render visible items)
And total count displays "{10,000} total items"
And scrolling is smooth (no lag)
And items load progressively as user scrolls
```

### Debouncing

```
### ACX: Debounce search input
Given user is typing in search box
When user types "laptop"
Then search does NOT execute on every keystroke
And search waits 300ms after user stops typing
And then search executes once
And prevents excessive API calls
```

---

## Security Patterns

### SQL Injection Prevention

```
### ACX: Prevent SQL injection
Given user enters "'; DROP TABLE users; --" in search field
When search is executed
Then parameterized query prevents injection
And search treats input as literal string
And no SQL commands are executed
And no database damage occurs
And security audit logs the attempt
```

### XSS Prevention

```
### ACX: Prevent cross-site scripting
Given user enters "<script>alert('XSS')</script>" in name field
When name is saved and displayed
Then script is NOT executed
And output is HTML-escaped: "&lt;script&gt;alert('XSS')&lt;/script&gt;"
And user sees literal text, not executed script
```

### CSRF Protection

```
### ACX: CSRF token validation
Given user is on form requiring state change (POST/PUT/DELETE)
When form loads
Then CSRF token is included in hidden field
And token is validated on submission
And if token missing or invalid:
  - 403 Forbidden response
  - Error: "Security token validation failed. Please refresh page and retry."
```

### Rate Limiting

```
### ACX: Rate limit protection
Given rate limit is 100 requests per user per hour
And user has made 99 requests this hour
When user makes 100th request
Then request succeeds

When user makes 101st request
Then 429 Too Many Requests response
And error message: "Rate limit exceeded. Try again in {minutes} minutes."
And Retry-After header indicates wait time
```

---

## Accessibility Patterns

### Keyboard Navigation

```
### ACX: Complete form with keyboard only
Given user is on registration form
And user is using keyboard only (no mouse)
When user presses Tab to navigate fields
Then focus moves through: Email → Password → Name → Terms checkbox → Submit button
And visual focus indicator shows on each element (3px blue outline)
And user can press Enter on Submit button
And user can press Space to check Terms checkbox
And user can press Escape to reset form
```

### Screen Reader Announcements

```
### ACX: Form validation announced to screen readers
Given user is using screen reader
When user submits form with validation errors
Then screen reader announces "Form has 3 errors"
And focus moves to first error
And each error field has aria-describedby linking to error message
And error messages are read aloud when field receives focus
```

### Color Contrast

```
### ACX: Text meets WCAG AA contrast requirements
Given page has text on background
When contrast is measured
Then all text has minimum 4.5:1 contrast ratio
And UI components (buttons, inputs) have minimum 3:1 contrast
And error text (red) on white background has 7:1 ratio
And user can read all text clearly
```

### Focus Management

```
### ACX: Focus trap in modal
Given modal dialog is open
When user presses Tab to navigate
Then focus cycles within modal (does not move to page behind modal)
And first Tab moves to first focusable element
And Shift+Tab from first element moves to last element
And Escape key closes modal
And focus returns to trigger button that opened modal
```

---

## Testing Guidance

### How to Write Tests from Acceptance Criteria

**Each Given/When/Then maps to test structure:**

**Acceptance Criterion:**
```
Given user is logged in
When user clicks "Profile" link
Then profile page displays user's name and email
```

**Corresponding Test (AAA Pattern):**
```javascript
test('displays user profile when logged in', async () => {
  // ARRANGE (Given)
  const user = await createTestUser({ name: 'John Doe', email: 'john@example.com' });
  await loginAs(user);

  // ACT (When)
  await click('Profile');

  // ASSERT (Then)
  expect(screen.getByText('John Doe')).toBeInTheDocument();
  expect(screen.getByText('john@example.com')).toBeInTheDocument();
});
```

### Test Coverage Requirements

**From acceptance criteria:**
- Each AC becomes at least 1 automated test
- Happy path scenarios → positive tests
- Error scenarios → negative tests
- Edge cases → boundary tests

**Example:**
```
Story has 5 acceptance criteria:
- AC1 (happy path) → 1 test
- AC2 (validation error) → 1 test
- AC3 (edge case: empty data) → 1 test
- AC4 (edge case: max data) → 1 test
- AC5 (concurrent access) → 1 test

Minimum: 5 tests from acceptance criteria
Additional: Integration tests, E2E tests
```

---

## Integration with Technical Specification (v2.0)

**For stories using v2.0 structured YAML format:**

**Acceptance criteria drive tech spec components:**

**Example mapping:**

**AC:** "Given user submits registration form, When validation passes, Then account created"

**Generates components:**
```yaml
components:
  - type: "API"
    name: "UserRegistration"
    endpoint: "/api/users/register"
    method: "POST"
    requirements:
      - id: "API-001"
        description: "Must validate email format before account creation"
        test_requirement: "Test: POST with invalid email returns 400 Bad Request"
        priority: "Critical"

  - type: "Service"
    name: "UserRegistrationService"
    file_path: "src/Application/Services/UserRegistrationService.cs"
    requirements:
      - id: "SVC-001"
        description: "Must create user account when validation passes"
        test_requirement: "Test: Valid request creates user in database"
        priority: "Critical"
```

**Each Given/When/Then should map to:**
- Component requirement (what must be built)
- Test requirement (how to verify it)

**See:** `technical-specification-creation.md` for complete v2.0 generation guide

---

## Progressive Disclosure

**When to load this reference:**
- Phase 2: Requirements Analysis (for pattern examples)
- Phase 7: Self-Validation (for validation rules)

**Why progressive:**
- Not needed during Phase 1 (story discovery)
- Not needed during Phase 3-4 (technical/UI spec)
- Loaded only when generating or validating acceptance criteria
- Saves ~500 lines from loading until needed

---

**Use these patterns to write complete, testable, unambiguous acceptance criteria that enable TDD implementation and automated validation.**
