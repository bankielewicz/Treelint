# UI Specification Guide

Comprehensive reference for documenting user interface components, layouts, interactions, and accessibility requirements in user stories.

## Purpose

This guide provides templates and patterns for UI specifications that enable frontend developers to implement interfaces without ambiguity, following WCAG accessibility standards and modern UX best practices.

---

## ASCII Mockup Patterns

### Box-Drawing Character Reference

**Basic Characters:**
```
Horizontal line: ─
Vertical line: │
Corners: ┌ ┐ └ ┘
T-junctions: ├ ┤ ┬ ┴
Cross: ┼
Thick borders: ═ ║ ╔ ╗ ╚ ╝
```

**Alternative (ASCII-safe):**
```
Horizontal: -
Vertical: |
Corners: + (all four corners)
```

### Simple Form Layout

```
┌──────────────────────────────────────┐
│         Page Title                   │
├──────────────────────────────────────┤
│ Field Label                          │
│ [ Input field text            ]      │
│                                      │
│ Another Label                        │
│ [ Input field                 ]      │
│                                      │
│ [ ] Checkbox with label text         │
│                                      │
│ [Button: Submit] [Button: Cancel]    │
└──────────────────────────────────────┘
```

### Form with Validation

```
┌──────────────────────────────────────┐
│      User Registration               │
├──────────────────────────────────────┤
│ Email Address *                      │
│ [ user@example.com            ]      │
│ ✓ Valid email format                 │
│                                      │
│ Password *                           │
│ [ ••••••••••                  ]      │
│ [████████────────] Medium            │
│ Requirements:                        │
│  ✓ 8+ characters                     │
│  ✓ Uppercase letter                  │
│  ✗ Special character                 │
│                                      │
│ Full Name *                          │
│ [ John Doe                    ]      │
│                                      │
│ [ ] I agree to Terms of Service      │
│                                      │
│ [   Create Account   ]               │
│                                      │
│ Already have account? [Log in]       │
└──────────────────────────────────────┘

Legend:
* = Required field
[ ] = Input field
[x] = Checked checkbox
[ ] = Unchecked checkbox
✓ = Valid/Complete
✗ = Invalid/Incomplete
████ = Filled progress bar
──── = Empty progress bar
```

### Data Table

```
┌────────────────────────────────────────────────────────────┐
│ Users (150 total)              [Search: ______] [+ Add]    │
├────────┬──────────────┬────────┬──────────┬───────────────┤
│ Name   │ Email        │ Role   │ Status   │ Actions       │
├────────┼──────────────┼────────┼──────────┼───────────────┤
│ John D │ john@ex.com  │ Admin  │ ● Active │ [Edit][Delete]│
│ Jane S │ jane@ex.com  │ User   │ ● Active │ [Edit][Delete]│
│ Bob J  │ bob@ex.com   │ User   │ ○ Inact. │ [Edit][Delete]│
├────────┴──────────────┴────────┴──────────┴───────────────┤
│ [Prev] [1] [2] [3] [4] [5] [Next]         Showing 1-20    │
└────────────────────────────────────────────────────────────┘

Legend:
● = Active (green indicator)
○ = Inactive (gray indicator)
```

### Modal Dialog

```
┌────────────────────────────────────────────────────┐
│ Background page (dimmed/blurred)                   │
│                                                    │
│    ┌──────────────────────────────────┐           │
│    │ ✕ Confirm Delete                 │           │
│    ├──────────────────────────────────┤           │
│    │                                  │           │
│    │ Are you sure you want to delete  │           │
│    │ "John Doe"?                      │           │
│    │                                  │           │
│    │ This action cannot be undone.    │           │
│    │                                  │           │
│    │ [    Cancel    ] [    Delete    ]│           │
│    │                                  │           │
│    └──────────────────────────────────┘           │
│                                                    │
└────────────────────────────────────────────────────┘

Interaction:
- Escape key or ✕ closes modal
- Click outside modal closes
- Tab cycles focus within modal (focus trap)
- Delete button is red/warning color
```

### Dashboard Layout

```
┌──────────────────────────────────────────────────────────┐
│ [☰ Menu] Dashboard                    [🔔] [👤 Profile]  │
├──────────────────────────────────────────────────────────┤
│ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐        │
│ │ 1,234   │ │ $45.6K  │ │ 89.5%   │ │ 456     │        │
│ │ Users   │ │ Revenue │ │ Uptime  │ │ Orders  │        │
│ │ ↑ 12%   │ │ ↑ 8%    │ │ ↓ 0.5%  │ │ ↑ 15%   │        │
│ └─────────┘ └─────────┘ └─────────┘ └─────────┘        │
│                                                          │
│ ┌────────────────────────┐ ┌────────────────────────┐  │
│ │ Revenue Chart (30d)    │ │ Recent Orders          │  │
│ │                        │ │                        │  │
│ │      ╱╲                │ │ ORD-042  $187  Shipped │  │
│ │   ╱─╯  ╲╱╲             │ │ ORD-041  $99   Pending │  │
│ │ ─╯        ╲╱           │ │ ORD-040  $256  Deliv.  │  │
│ │                        │ │ [View all orders...]   │  │
│ └────────────────────────┘ └────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

### Mobile Responsive

```
Desktop (>768px):
┌────────────────────────────────────┐
│ [Logo]    Nav  Nav  Nav  [Profile] │
├────────────────────────────────────┤
│ ┌─────────┐ ┌─────────┐           │
│ │ Card 1  │ │ Card 2  │  (2 col)  │
│ └─────────┘ └─────────┘           │
└────────────────────────────────────┘

Mobile (<768px):
┌──────────────┐
│ [☰] [Logo]   │
├──────────────┤
│ ┌──────────┐ │
│ │ Card 1   │ │ (1 col)
│ └──────────┘ │
│ ┌──────────┐ │
│ │ Card 2   │ │
│ └──────────┘ │
└──────────────┘
```

---

## Component Type Catalog

### Form Components

**Text Input:**
```markdown
#### Component: TextInput

**Type:** Form Input (text)

**Purpose:** Single-line text entry

**Props:**
- label (string, required): Field label
- value (string, required): Current value
- onChange (function, required): Value change handler
- placeholder (string, optional): Placeholder text
- required (boolean, default: false): Is field required
- disabled (boolean, default: false): Is field disabled
- error (string, optional): Validation error message
- helpText (string, optional): Helper text below field
- maxLength (number, optional): Maximum character count

**State:**
- isFocused (boolean): Whether field has focus
- isDirty (boolean): Whether user has modified value

**Events:**
- onChange(newValue): Fired on every keystroke
- onBlur(): Fired when field loses focus (trigger validation)
- onFocus(): Fired when field gains focus

**Accessibility:**
- aria-label: Use label prop
- aria-required: Set if required=true
- aria-invalid: Set if error present
- aria-describedby: Link to error message ID

**Example Interface (TypeScript):**
```typescript
interface TextInputProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  error?: string;
  helpText?: string;
  maxLength?: number;
}
```
```

**Dropdown/Select:**
```markdown
#### Component: Select

**Type:** Form Input (dropdown)

**Purpose:** Select single option from list

**Props:**
- label (string, required): Field label
- value (string, required): Currently selected value
- options (array, required): List of options
- onChange (function, required): Selection change handler
- required (boolean, default: false): Is field required
- disabled (boolean, default: false): Is field disabled
- error (string, optional): Validation error
- placeholder (string, optional): "Select an option..."

**State:**
- isOpen (boolean): Whether dropdown is expanded
- highlightedIndex (number): Keyboard navigation index

**Events:**
- onChange(selectedValue): Fired when user selects option
- onOpen(): Dropdown expanded
- onClose(): Dropdown collapsed

**Accessibility:**
- role="combobox" on trigger
- aria-expanded: Reflects isOpen state
- aria-activedescendant: Points to highlighted option
- Keyboard:
  - Enter/Space: Open dropdown
  - Arrow down/up: Navigate options
  - Escape: Close dropdown
  - Type to search: Filter options

**Example Interface:**
```typescript
interface SelectProps {
  label: string;
  value: string;
  options: SelectOption[];
  onChange: (value: string) => void;
  required?: boolean;
  disabled?: boolean;
  error?: string;
  placeholder?: string;
}

interface SelectOption {
  value: string;
  label: string;
  disabled?: boolean;
}
```
```

**Checkbox:**
```markdown
#### Component: Checkbox

**Type:** Form Input (boolean)

**Purpose:** Toggle binary choice (checked/unchecked)

**Props:**
- label (string, required): Checkbox label
- checked (boolean, required): Current state
- onChange (function, required): State change handler
- disabled (boolean, default: false): Is checkbox disabled
- error (string, optional): Validation error
- required (boolean, default: false): Is checkbox required (e.g., agree to terms)

**Events:**
- onChange(newCheckedState): Fired when user toggles
- Can be triggered by Space key or click

**Accessibility:**
- role="checkbox"
- aria-checked: Reflects checked state (true/false)
- aria-required: Set if required=true
- Keyboard: Space toggles, Tab navigates

**Example Interface:**
```typescript
interface CheckboxProps {
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
  error?: string;
  required?: boolean;
}
```
```

### Display Components

**Data Table:**
```markdown
#### Component: DataTable

**Type:** Display (tabular data)

**Purpose:** Display list of records with sorting, pagination, and actions

**Props:**
- columns (array, required): Column definitions
- data (array, required): Row data
- onSort (function, optional): Sort handler
- onPageChange (function, optional): Pagination handler
- onRowClick (function, optional): Row click handler
- actions (array, optional): Row-level actions
- loading (boolean, default: false): Loading state
- emptyMessage (string, default: "No data"): Empty state message

**State:**
- sortColumn (string): Currently sorted column
- sortDirection (asc|desc): Sort direction
- currentPage (number): Current page (for pagination)

**Events:**
- onSort(column, direction): User clicks column header
- onPageChange(page): User navigates pages
- onRowClick(row): User clicks table row
- onAction(action, row): User clicks action button

**Accessibility:**
- role="table"
- Column headers: role="columnheader", scope="col"
- Rows: role="row"
- Cells: role="cell"
- Sortable headers: aria-sort (ascending|descending|none)
- Keyboard: Tab navigates cells, Enter activates

**Example Interface:**
```typescript
interface DataTableProps<T> {
  columns: ColumnDef<T>[];
  data: T[];
  onSort?: (column: string, direction: 'asc' | 'desc') => void;
  onPageChange?: (page: number) => void;
  onRowClick?: (row: T) => void;
  actions?: Action<T>[];
  loading?: boolean;
  emptyMessage?: string;
  pagination?: {
    currentPage: number;
    totalPages: number;
    pageSize: number;
  };
}

interface ColumnDef<T> {
  key: string;
  header: string;
  sortable?: boolean;
  render?: (value: any, row: T) => ReactNode;
}

interface Action<T> {
  label: string;
  icon?: string;
  onClick: (row: T) => void;
  disabled?: (row: T) => boolean;
}
```
```

**Card:**
```markdown
#### Component: Card

**Type:** Display (content container)

**Purpose:** Group related content with optional header, body, footer

**Props:**
- title (string, optional): Card title
- children (ReactNode, required): Card content
- actions (array, optional): Action buttons in header
- variant (string, default: 'default'): Visual style (default, outlined, elevated)
- padding (string, default: 'medium'): Inner padding (small, medium, large, none)

**Layout:**
```
┌────────────────────────────┐
│ Title          [Action] [X]│
├────────────────────────────┤
│                            │
│ Card content/body          │
│                            │
├────────────────────────────┤
│ Footer area (optional)     │
└────────────────────────────┘
```

**Accessibility:**
- Semantic HTML: <article> or <section>
- Heading level appropriate to page hierarchy
- Action buttons keyboard accessible

**Example Interface:**
```typescript
interface CardProps {
  title?: string;
  children: ReactNode;
  actions?: CardAction[];
  variant?: 'default' | 'outlined' | 'elevated';
  padding?: 'none' | 'small' | 'medium' | 'large';
  footer?: ReactNode;
}

interface CardAction {
  label: string;
  icon?: string;
  onClick: () => void;
}
```
```

**Modal/Dialog:**
```markdown
#### Component: Modal

**Type:** Overlay (dialog)

**Purpose:** Display content above page, requires user interaction to dismiss

**Props:**
- isOpen (boolean, required): Whether modal is visible
- onClose (function, required): Close handler
- title (string, required): Modal title
- children (ReactNode, required): Modal content
- size (string, default: 'medium'): Modal size (small, medium, large, fullscreen)
- closeOnEscape (boolean, default: true): Close on Escape key
- closeOnBackdrop (boolean, default: true): Close on backdrop click

**Layout:**
```
Background page (dimmed, pointer-events: none)

    ┌──────────────────────┐
    │ ✕ Modal Title        │
    ├──────────────────────┤
    │                      │
    │ Modal content here   │
    │                      │
    ├──────────────────────┤
    │ [Cancel] [  Confirm ]│
    └──────────────────────┘
```

**State:**
- previousFocusElement: Element that opened modal (restore focus on close)

**Accessibility:**
- role="dialog"
- aria-modal="true"
- aria-labelledby: Points to title element ID
- Focus trap: Tab cycles within modal
- Escape key: Closes modal (if closeOnEscape)
- Focus: Automatically focus first input or close button
- On close: Restore focus to element that opened modal

**Example Interface:**
```typescript
interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: ReactNode;
  size?: 'small' | 'medium' | 'large' | 'fullscreen';
  closeOnEscape?: boolean;
  closeOnBackdrop?: boolean;
  footer?: ReactNode;
}
```
```

### Navigation Components

**Top Navigation Bar:**
```
┌─────────────────────────────────────────────────────┐
│ [Logo]   Nav1  Nav2  Nav3     [Search] [🔔] [👤]   │
└─────────────────────────────────────────────────────┘
```

**Sidebar Navigation:**
```
┌────┬──────────────────────┐
│ [☰]│ Main Content         │
│ Nav│                      │
│ 1  │                      │
│ 2  │                      │
│ 3  │                      │
│    │                      │
└────┴──────────────────────┘

Collapsed:
┌──┬────────────────────────┐
│☰ │ Main Content           │
│  │                        │
│  │                        │
└──┴────────────────────────┘
```

**Breadcrumbs:**
```
Home > Products > Electronics > Laptops > Product Name
```

**Tabs:**
```
┌──────────────────────────────┐
│ [Tab 1] Tab 2  Tab 3         │
├──────────────────────────────┤
│                              │
│ Tab 1 Content                │
│                              │
└──────────────────────────────┘

Interaction:
- Click tab to switch
- Arrow left/right to navigate (keyboard)
- Active tab highlighted
```

---

## Component Interface Patterns

### React (TypeScript)

**Functional Component with Props:**
```typescript
interface UserFormProps {
  initialData?: User;
  onSubmit: (user: User) => Promise<void>;
  onCancel: () => void;
  isLoading: boolean;
  error: string | null;
}

interface User {
  id?: string;
  email: string;
  name: string;
  role: 'customer' | 'admin' | 'moderator';
}

export function UserForm({
  initialData,
  onSubmit,
  onCancel,
  isLoading,
  error
}: UserFormProps) {
  // Component implementation
}
```

**Component with State:**
```typescript
interface UserListState {
  users: User[];
  loading: boolean;
  error: string | null;
  selectedUserId: string | null;
  filters: FilterState;
}

interface FilterState {
  role: string | null;
  status: string | null;
  searchQuery: string;
}
```

### Vue (TypeScript)

**Component Props:**
```typescript
interface Props {
  user: User;
  editable: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  save: [user: User];
  cancel: [];
}>();
```

### Angular (TypeScript)

**Component Class:**
```typescript
@Component({
  selector: 'app-user-form',
  templateUrl: './user-form.component.html'
})
export class UserFormComponent {
  @Input() initialData?: User;
  @Input() isLoading: boolean = false;
  @Output() submit = new EventEmitter<User>();
  @Output() cancel = new EventEmitter<void>();

  formGroup: FormGroup;
  error: string | null = null;
}
```

### C# (WPF/MAUI)

**ViewModel:**
```csharp
public class UserFormViewModel : INotifyPropertyChanged
{
    private User _user;
    private bool _isLoading;
    private string _error;

    public User User
    {
        get => _user;
        set { _user = value; OnPropertyChanged(); }
    }

    public bool IsLoading
    {
        get => _isLoading;
        set { _isLoading = value; OnPropertyChanged(); }
    }

    public string Error
    {
        get => _error;
        set { _error = value; OnPropertyChanged(); }
    }

    public ICommand SubmitCommand { get; }
    public ICommand CancelCommand { get; }
}
```

---

## User Interaction Flow Patterns

### Linear Flow

```
Flow: User Registration

1. User navigates to /register
   → State: Empty form displays

2. User enters email
   → System validates format in real-time
   → State: Validation icon shows (✓ or ✗)

3. User enters password
   → System calculates strength
   → State: Strength indicator updates (weak/medium/strong)
   → State: Requirements checklist updates

4. User enters name
   → System validates length (2+ chars)
   → State: Validation passes

5. User checks "Agree to Terms" checkbox
   → State: Submit button becomes enabled

6. User clicks "Create Account"
   → State: Button shows loading spinner, form disabled
   → System: POST /api/auth/register
   → System: Send verification email (async)

7a. Success path:
    → System: Returns 201 Created
    → State: Success message displays
    → State: Redirect to /verify-email-sent page (3 second delay)

7b. Error path (validation):
    → System: Returns 400 Bad Request
    → State: Error messages display near fields
    → State: Form remains editable
    → State: Focus moves to first invalid field
```

### Branching Flow

```
Flow: E-commerce Checkout

1. User clicks "Proceed to Checkout"
   → System checks: User logged in?

2a. If logged in:
    → Navigate to shipping address page
    → Pre-fill with saved address (if exists)

2b. If not logged in:
    → Show modal: "Log in or Continue as Guest?"
    → User chooses:
      - Log in → Navigate to /login with return URL
      - Guest → Navigate to shipping address (empty form)

3. User enters/confirms shipping address
   → Validates address format
   → If valid: Navigate to payment page
   → If invalid: Show errors, stay on page

4. User enters payment details
   → System validates card format (client-side)
   → User clicks "Place Order"
   → System: Create payment intent (Stripe)

5a. Payment succeeds:
    → Create order in database
    → Send confirmation email
    → Redirect to /order-confirmation

5b. Payment fails:
    → Show error message
    → User can retry with same or different card
    → Order saved as "Payment Failed" (user can resume later)
```

### Error Recovery Flow

```
Flow: Form Submission with Network Error

1. User fills form completely
2. User clicks "Submit"
   → State: Loading spinner, form disabled
   → System: POST /api/resource

3. Network error occurs (timeout, disconnection)
   → State: Error message displays "Connection error. Please check internet and try again."
   → State: Form re-enables
   → State: Data preserved (not lost)
   → State: "Retry" button displays

4a. User clicks "Retry":
    → Same request retries
    → If succeeds: Normal success flow
    → If fails again: Same error handling (max 3 retries)

4b. User navigates away:
    → Browser warns "Unsaved changes. Leave anyway?"
    → Data saved to sessionStorage (recoverable)
```

---

## Accessibility Requirements (WCAG 2.1 Level AA)

### Keyboard Navigation

**Standard Interactions:**
- **Tab**: Move focus forward
- **Shift+Tab**: Move focus backward
- **Enter**: Activate button, submit form, follow link
- **Space**: Activate button, toggle checkbox, expand dropdown
- **Escape**: Close modal, cancel action, clear selection
- **Arrow keys**: Navigate lists, dropdowns, tabs, radio groups

**Focus Indicators:**
```
Visual focus indicator required for all interactive elements:
- Outline: 3px solid {color}
- Color contrast: 3:1 minimum against background
- Visible when element has focus
- Not removed with outline: none (accessibility violation)

Example CSS:
button:focus {
  outline: 3px solid #0066CC;
  outline-offset: 2px;
}
```

**Focus Trap (Modals):**
```
When modal is open:
1. Focus moves to first focusable element in modal
2. Tab cycles through modal elements only (not page behind)
3. Shift+Tab from first element moves to last element
4. Tab from last element moves to first element
5. Escape closes modal
6. On close: Focus returns to element that opened modal

Implementation:
- Capture all Tab keypress events
- Get list of focusable elements in modal
- Manually manage focus cycling
```

### Screen Reader Support

**ARIA Labels:**
```html
<!-- Icon-only buttons need labels -->
<button aria-label="Close modal">✕</button>
<button aria-label="Search">🔍</button>

<!-- Form inputs need labels (visible or aria-label) -->
<label for="email">Email Address</label>
<input id="email" type="email" />

<!-- Or use aria-label if no visible label -->
<input type="email" aria-label="Email Address" />
```

**ARIA Described By:**
```html
<!-- Link input to help text -->
<label for="password">Password</label>
<input
  id="password"
  type="password"
  aria-describedby="password-requirements"
/>
<div id="password-requirements">
  Must be 8+ characters with uppercase, lowercase, number, special char
</div>

<!-- Link input to error message -->
<input
  id="email"
  type="email"
  aria-invalid="true"
  aria-describedby="email-error"
/>
<div id="email-error" role="alert">
  Invalid email format
</div>
```

**ARIA Live Regions:**
```html
<!-- Announce dynamic content changes -->
<div aria-live="polite" aria-atomic="true">
  <!-- Content updates announced to screen readers -->
  3 results found
</div>

<!-- For urgent announcements -->
<div aria-live="assertive" role="alert">
  Error: Form submission failed
</div>
```

**ARIA Roles:**
```html
<!-- Custom components need semantic roles -->
<div role="button" tabindex="0" aria-pressed="false">
  Toggle Button
</div>

<div role="tablist">
  <button role="tab" aria-selected="true" aria-controls="panel1">Tab 1</button>
  <button role="tab" aria-selected="false" aria-controls="panel2">Tab 2</button>
</div>
<div role="tabpanel" id="panel1" aria-labelledby="tab1">
  Panel 1 content
</div>
```

### Color Contrast

**WCAG AA Requirements:**
- **Normal text** (<18px or <14px bold): Minimum 4.5:1 contrast ratio
- **Large text** (≥18px or ≥14px bold): Minimum 3:1 contrast ratio
- **UI components**: Minimum 3:1 contrast ratio (buttons, inputs, focus indicators)

**Color Combinations:**

**Text on White (#FFFFFF):**
- Black (#000000): 21:1 ✅ Excellent
- Dark Gray (#333333): 12.6:1 ✅ Great
- Medium Gray (#767676): 4.5:1 ✅ Minimum for normal text
- Light Gray (#AAAAAA): 2.3:1 ❌ Fails AA

**Error Colors:**
- Red (#D32F2F) on White: 7:1 ✅ High contrast
- Orange (#F57C00) on White: 4.5:1 ✅ Acceptable
- Light Red (#FFEBEE) on White: 1.2:1 ❌ Too light (use for background only)

**Success Colors:**
- Green (#2E7D32) on White: 4.9:1 ✅ Good
- Dark Green (#1B5E20) on White: 8.6:1 ✅ Excellent

**Interactive Colors:**
- Primary Button (#1976D2) on White: 4.6:1 ✅ Acceptable
- Focus Outline (#0066CC) on White: 8.6:1 ✅ Excellent

**Testing:**
Use browser DevTools or online tools:
- WebAIM Contrast Checker: https://webaim.org/resources/contrastchecker/
- Browser DevTools: Accessibility panel shows contrast ratios

### Form Validation Accessibility

**Real-Time Validation Announcement:**
```html
<form>
  <label for="email">Email</label>
  <input
    id="email"
    type="email"
    aria-invalid="true"
    aria-describedby="email-error"
  />
  <!-- aria-live announces errors to screen readers -->
  <div id="email-error" role="alert" aria-live="polite">
    Invalid email format
  </div>
</form>
```

**Error Summary (Top of Form):**
```html
<!-- Announce form errors on submit -->
<div role="alert" aria-live="assertive" class="error-summary">
  <h2>Form has 3 errors</h2>
  <ul>
    <li><a href="#email">Email: Invalid format</a></li>
    <li><a href="#password">Password: Too short</a></li>
    <li><a href="#name">Name: Required field</a></li>
  </ul>
</div>

<!-- Links jump to invalid fields -->
```

**Success Confirmation:**
```html
<!-- Announce success -->
<div role="status" aria-live="polite">
  User created successfully
</div>
```

---

## Responsive Design Patterns

### Breakpoint Strategy

**Standard Breakpoints:**
```
Extra Small (XS): <576px (mobile portrait)
Small (SM): 576px-768px (mobile landscape, small tablets)
Medium (MD): 768px-992px (tablets)
Large (LG): 992px-1200px (desktops)
Extra Large (XL): ≥1200px (large desktops)
```

**Layout Changes by Breakpoint:**

**Desktop (≥992px):**
```
┌────────────────────────────────────┐
│ Header with full navigation        │
├──────┬─────────────────────────────┤
│ Side │ Main Content (2-3 columns)  │
│ Nav  │ ┌──────┐ ┌──────┐ ┌──────┐ │
│ ├──  │ │Card 1│ │Card 2│ │Card 3│ │
│ ├──  │ └──────┘ └──────┘ └──────┘ │
│ ├──  │                             │
└──────┴─────────────────────────────┘
```

**Tablet (768px-991px):**
```
┌──────────────────────────┐
│ Header with nav          │
├──────────────────────────┤
│ Main Content (2 columns) │
│ ┌─────────┐ ┌─────────┐ │
│ │ Card 1  │ │ Card 2  │ │
│ └─────────┘ └─────────┘ │
│ ┌─────────┐              │
│ │ Card 3  │              │
│ └─────────┘              │
└──────────────────────────┘
```

**Mobile (<768px):**
```
┌─────────────┐
│ [☰] Header  │
├─────────────┤
│ ┌─────────┐ │
│ │ Card 1  │ │ (1 column)
│ └─────────┘ │
│ ┌─────────┐ │
│ │ Card 2  │ │
│ └─────────┘ │
│ ┌─────────┐ │
│ │ Card 3  │ │
│ └─────────┘ │
└─────────────┘
```

### Touch Interactions (Mobile)

**Minimum Touch Target Size:**
- 44px × 44px (iOS guidelines)
- 48px × 48px (Android guidelines)
- Use larger target for small visual elements

**Touch Gestures:**
```
- Tap: Primary action (equivalent to click)
- Long press: Secondary action (equivalent to right-click)
- Swipe: Navigate, delete, reveal actions
- Pinch: Zoom (images, maps)
- Pull to refresh: Reload content
```

**Touch-Specific Components:**
```
Mobile: Bottom Sheet (drawer from bottom)
┌─────────────┐
│ Content     │
│             │
│ ┌─────────┐ │
│ │ Handle  │ │ ← User drags to expand
│ ├─────────┤ │
│ │ Sheet   │ │
│ │ Content │ │
│ └─────────┘ │
└─────────────┘
```

---

## State Management Patterns

### Component State Documentation

**Local State (Component-level):**
```typescript
interface ComponentState {
  // UI state (doesn't need global store)
  isOpen: boolean;
  selectedTab: number;
  inputValue: string;
  validationErrors: Record<string, string>;
}
```

**Global State (Application-level):**
```typescript
interface AppState {
  // Shared across components
  currentUser: User | null;
  isAuthenticated: boolean;
  notifications: Notification[];
  theme: 'light' | 'dark';
}
```

**Server State (API data):**
```typescript
interface ServerState {
  // Data fetched from API
  users: {
    data: User[];
    loading: boolean;
    error: string | null;
    lastFetched: timestamp;
  };
}
```

### State Updates in Interaction Flows

```
Document state changes:

1. User clicks "Add to Cart"
   State changes:
   - cart.items: Add new item
   - cart.count: Increment by 1
   - cart.total: Recalculate
   - addToCartButton.loading: true (during API call)
   - addToCartButton.disabled: true (prevent double-click)

2. API call succeeds
   State changes:
   - addToCartButton.loading: false
   - addToCartButton.disabled: false
   - notification: Show "Item added to cart" (3 second auto-dismiss)

3. API call fails
   State changes:
   - cart.items: Rollback (remove added item)
   - cart.count: Decrement (revert)
   - addToCartButton.loading: false
   - addToCartButton.disabled: false
   - error: Show "Failed to add item. Try again."
```

---

## Loading States

### Skeleton Screens

```
While data loading, show skeleton:

┌────────────────────────┐
│ ░░░░░░░  ░░░░░  ░░░   │ (Animated shimmer)
│ ░░░░░░░░░░░░░░░░░░░   │
│                        │
│ ░░░░  ░░░░░░  ░░░░░   │
│ ░░░░░░░░░░░░░░░░░░░   │
└────────────────────────┘

Better than:
- Blank screen
- Spinner only (no context)
- Flash of empty content before data loads
```

### Progressive Loading

```
1. Critical content loads first (above fold)
2. Images load with lazy loading (as user scrolls)
3. Below-fold content loads on scroll or delay
4. Heavy components (charts) load last

Example:
Hero section → Navigation → Main content → Sidebar → Footer → Analytics
  (instant)    (instant)   (200ms)       (500ms)   (1s)    (background)
```

### Optimistic Updates

```
User action appears instant (update UI immediately), then confirm with server:

1. User clicks "Like" button
   → Immediate UI update: Heart icon fills, count increments
   → Background: POST /api/posts/{id}/like

2a. API succeeds:
    → No visible change (already updated)
    → State confirmed

2b. API fails:
    → Revert UI: Heart icon unfills, count decrements
    → Show error: "Failed to like post. Try again."
    → User can retry
```

---

## Animation & Transition Patterns

### Micro-Interactions

```
Button hover:
- Scale: 1.0 → 1.05 (subtle growth)
- Transition: 150ms ease-out

Button click:
- Scale: 1.05 → 0.95 (press down)
- Transition: 100ms ease-in

Modal open:
- Opacity: 0 → 1 (fade in)
- Transform: scale(0.9) → scale(1) (zoom in)
- Duration: 200ms ease-out

Modal close:
- Opacity: 1 → 0 (fade out)
- Transform: scale(1) → scale(0.9) (zoom out)
- Duration: 150ms ease-in

List item add:
- Opacity: 0 → 1
- Transform: translateY(-20px) → translateY(0)
- Duration: 300ms ease-out
- Delay: Stagger by 50ms per item
```

### Loading Indicators

```
Spinner: Rotating circle
Duration: Infinite
Speed: 1 rotation per second

Progress bar: Determinate (0-100%)
Update: Every 500ms
Animation: Smooth transition (not jumpy)

Skeleton: Shimmer effect
Duration: 1.5 seconds per cycle
Direction: Left to right
```

---

## Error Message Patterns

### Inline Validation Errors

```
Error message placement:
- Below input field (preferred)
- Red text (#D32F2F or similar)
- Icon: ✗ or ⚠
- role="alert" for screen readers

Example:
┌──────────────────────┐
│ Email                │
│ [ not-an-email    ]  │
│ ✗ Invalid email      │ ← Error message
└──────────────────────┘
```

### Error Message Tone

**❌ Bad (Technical jargon):**
```
Error: CONSTRAINT_VIOLATION_002
SQLException: duplicate key value violates unique constraint "users_email_key"
```

**✅ Good (User-friendly):**
```
Email already registered
Already have an account? Log in instead.
```

**❌ Bad (Vague):**
```
Something went wrong
```

**✅ Good (Specific with action):**
```
Connection error. Please check your internet connection and try again.
[Retry]
```

### Error Recovery Actions

```
Every error should provide:
1. What went wrong (clear message)
2. Why it happened (if helpful)
3. How to fix it (actionable steps)
4. Action button (Retry, Cancel, Contact Support)

Example:
┌────────────────────────────────────┐
│ ⚠ Payment Processing Failed        │
├────────────────────────────────────┤
│ Your card was declined.            │
│                                    │
│ Reason: Insufficient funds         │
│                                    │
│ Options:                           │
│ • Try a different payment method   │
│ • Contact your bank                │
│ • Contact support (Ref: #12345)    │
│                                    │
│ [Try Another Card] [Contact Us]    │
└────────────────────────────────────┘
```

---

## Progressive Disclosure

**When to load this reference:**
- Phase 4: UI Specification (component patterns, mockup syntax, accessibility requirements)
- Phase 7: Self-Validation (validation rules for UI specs)

**Why progressive:**
- Not needed during Phase 1-3 (story discovery, requirements, technical spec)
- Not needed if story has no UI components
- Loaded only when documenting UI specifications
- Saves ~700 lines from loading until needed

---

**Use these patterns to create complete, accessible, user-friendly UI specifications that enable frontend implementation without ambiguity.**
