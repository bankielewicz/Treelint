# Web UI Best Practices

This guide provides best practices for generating web-based user interfaces across all web technologies (React, Blazor, ASP.NET, HTML).

---

## Semantic HTML

Use semantic HTML elements that convey meaning about the content structure.

### ✅ DO: Use Semantic Elements

```html
<!-- Forms -->
<form action="/login" method="post">
  <label for="email">Email</label>
  <input type="email" id="email" name="email" required>

  <button type="submit">Login</button>
</form>

<!-- Navigation -->
<nav>
  <ul>
    <li><a href="/home">Home</a></li>
    <li><a href="/about">About</a></li>
  </ul>
</nav>

<!-- Content Structure -->
<article>
  <header>
    <h1>Article Title</h1>
  </header>
  <section>
    <p>Article content...</p>
  </section>
</article>
```

### ❌ DON'T: Use Generic Divs Everywhere

```html
<!-- Bad: No semantic meaning -->
<div class="form">
  <div class="label">Email</div>
  <div class="input">
    <input type="text" name="email">
  </div>
  <div class="button" onclick="submit()">Login</div>
</div>
```

**Why:** Semantic HTML improves accessibility, SEO, and code maintainability.

---

## Accessibility (ARIA & A11y)

Ensure all UI components are accessible to users with disabilities.

### Form Accessibility

```html
<!-- Associate labels with inputs -->
<label for="username">Username</label>
<input type="text" id="username" name="username" aria-required="true">

<!-- Provide descriptive errors -->
<input
  type="email"
  id="email"
  aria-invalid="true"
  aria-describedby="email-error"
>
<span id="email-error" role="alert">Please enter a valid email address</span>

<!-- Group related inputs -->
<fieldset>
  <legend>Contact Information</legend>
  <label for="phone">Phone</label>
  <input type="tel" id="phone" name="phone">
</fieldset>
```

### Button Accessibility

```html
<!-- Always use <button> for clickable elements -->
<button type="submit" aria-label="Submit login form">
  Login
</button>

<!-- Icon buttons need labels -->
<button aria-label="Close dialog">
  <span aria-hidden="true">×</span>
</button>

<!-- Loading states -->
<button aria-busy="true" disabled>
  Loading...
</button>
```

### Image Accessibility

```html
<!-- Always provide alt text -->
<img src="logo.png" alt="Company Logo">

<!-- Decorative images -->
<img src="decorative-line.png" alt="" role="presentation">

<!-- Complex images need descriptions -->
<figure>
  <img src="chart.png" alt="Sales chart showing 20% increase">
  <figcaption>
    Sales increased from $100K to $120K in Q1 2024
  </figcaption>
</figure>
```

### Landmark Roles

```html
<!-- Use landmark roles for page structure -->
<header role="banner">
  <nav role="navigation">...</nav>
</header>

<main role="main">
  <article>...</article>
</main>

<aside role="complementary">
  <section aria-labelledby="related-title">
    <h2 id="related-title">Related Articles</h2>
  </section>
</aside>

<footer role="contentinfo">...</footer>
```

---

## Form Validation

Implement client-side validation with clear error messages.

### HTML5 Validation

```html
<!-- Use native validation attributes -->
<input
  type="email"
  name="email"
  required
  pattern="[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$"
  minlength="5"
  maxlength="100"
>

<!-- Provide custom error messages -->
<input
  type="text"
  name="username"
  required
  pattern="[a-zA-Z0-9_]{3,16}"
  title="Username must be 3-16 characters (letters, numbers, underscore only)"
>
```

### React Validation Example

```jsx
function LoginForm() {
  const [errors, setErrors] = useState({});

  const validateForm = (data) => {
    const newErrors = {};

    if (!data.email) {
      newErrors.email = 'Email is required';
    } else if (!/\S+@\S+\.\S+/.test(data.email)) {
      newErrors.email = 'Email is invalid';
    }

    if (!data.password) {
      newErrors.password = 'Password is required';
    } else if (data.password.length < 8) {
      newErrors.password = 'Password must be at least 8 characters';
    }

    return newErrors;
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    const formData = new FormData(e.target);
    const data = Object.fromEntries(formData);

    const validationErrors = validateForm(data);
    if (Object.keys(validationErrors).length > 0) {
      setErrors(validationErrors);
      return;
    }

    // Submit form
    onSubmit(data);
  };

  return (
    <form onSubmit={handleSubmit} noValidate>
      <div>
        <label htmlFor="email">Email</label>
        <input
          type="email"
          id="email"
          name="email"
          aria-invalid={errors.email ? 'true' : 'false'}
          aria-describedby={errors.email ? 'email-error' : undefined}
        />
        {errors.email && (
          <span id="email-error" role="alert" className="error">
            {errors.email}
          </span>
        )}
      </div>

      <button type="submit">Login</button>
    </form>
  );
}
```

---

## Responsive Design

Ensure UI works across all screen sizes.

### Mobile-First Approach

```css
/* Base styles for mobile */
.container {
  padding: 1rem;
  width: 100%;
}

/* Tablet and larger */
@media (min-width: 768px) {
  .container {
    padding: 2rem;
    max-width: 720px;
    margin: 0 auto;
  }
}

/* Desktop */
@media (min-width: 1024px) {
  .container {
    max-width: 960px;
  }
}
```

### Viewport Meta Tag

```html
<!-- Always include in <head> -->
<meta name="viewport" content="width=device-width, initial-scale=1.0">
```

### Responsive Images

```html
<!-- Use srcset for different screen sizes -->
<img
  src="image-small.jpg"
  srcset="image-small.jpg 480w, image-medium.jpg 768w, image-large.jpg 1200w"
  sizes="(max-width: 768px) 100vw, 50vw"
  alt="Responsive image"
>
```

---

## Component Structure (React)

Follow consistent component patterns.

### Functional Component Template

```jsx
import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';

/**
 * LoginForm - User authentication form
 *
 * @param {Function} onSubmit - Callback when form is submitted
 * @param {Boolean} isLoading - Shows loading state
 */
function LoginForm({ onSubmit, isLoading }) {
  const [formData, setFormData] = useState({
    email: '',
    password: ''
  });

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    onSubmit(formData);
  };

  return (
    <form onSubmit={handleSubmit}>
      {/* Form content */}
    </form>
  );
}

LoginForm.propTypes = {
  onSubmit: PropTypes.func.isRequired,
  isLoading: PropTypes.bool
};

LoginForm.defaultProps = {
  isLoading: false
};

export default LoginForm;
```

---

## Component Structure (Blazor)

Follow Blazor conventions for C# components.

### Blazor Component Template

```razor
@page "/login"
@inject NavigationManager Navigation
@inject IAuthService AuthService

<div class="login-container">
    <h2>Login</h2>

    <EditForm Model="loginModel" OnValidSubmit="HandleLogin">
        <DataAnnotationsValidator />
        <ValidationSummary />

        <div class="form-group">
            <label for="email">Email</label>
            <InputText id="email" @bind-Value="loginModel.Email" class="form-control" />
            <ValidationMessage For="@(() => loginModel.Email)" />
        </div>

        <div class="form-group">
            <label for="password">Password</label>
            <InputText id="password" type="password" @bind-Value="loginModel.Password" class="form-control" />
            <ValidationMessage For="@(() => loginModel.Password)" />
        </div>

        <button type="submit" disabled="@isLoading">
            @if (isLoading)
            {
                <span>Loading...</span>
            }
            else
            {
                <span>Login</span>
            }
        </button>
    </EditForm>

    @if (!string.IsNullOrEmpty(errorMessage))
    {
        <div class="alert alert-danger" role="alert">
            @errorMessage
        </div>
    }
</div>

@code {
    private LoginModel loginModel = new();
    private bool isLoading = false;
    private string? errorMessage;

    private async Task HandleLogin()
    {
        isLoading = true;
        errorMessage = null;

        try
        {
            var result = await AuthService.LoginAsync(loginModel);
            if (result.Success)
            {
                Navigation.NavigateTo("/dashboard");
            }
            else
            {
                errorMessage = result.ErrorMessage;
            }
        }
        catch (Exception ex)
        {
            errorMessage = "An error occurred. Please try again.";
        }
        finally
        {
            isLoading = false;
        }
    }
}
```

---

## Styling Best Practices

### Tailwind CSS

```html
<!-- Use utility classes for common patterns -->
<div class="container mx-auto px-4 py-8">
  <form class="max-w-md mx-auto space-y-4">
    <div class="space-y-2">
      <label for="email" class="block text-sm font-medium text-gray-700">
        Email
      </label>
      <input
        type="email"
        id="email"
        class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
      >
    </div>

    <button
      type="submit"
      class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
    >
      Submit
    </button>
  </form>
</div>
```

### Bootstrap

```html
<div class="container">
  <form class="row g-3">
    <div class="col-md-6">
      <label for="email" class="form-label">Email</label>
      <input type="email" class="form-control" id="email" required>
      <div class="invalid-feedback">
        Please provide a valid email.
      </div>
    </div>

    <div class="col-12">
      <button type="submit" class="btn btn-primary">Submit</button>
    </div>
  </form>
</div>
```

### Plain CSS (BEM Methodology)

```css
/* Block */
.login-form {
  max-width: 400px;
  margin: 0 auto;
  padding: 2rem;
}

/* Element */
.login-form__field {
  margin-bottom: 1rem;
}

.login-form__label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
}

.login-form__input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}

/* Modifier */
.login-form__input--error {
  border-color: #e53e3e;
}

.login-form__button {
  width: 100%;
  padding: 0.75rem;
  background-color: #3182ce;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.login-form__button:hover {
  background-color: #2c5aa0;
}

.login-form__button:disabled {
  background-color: #a0aec0;
  cursor: not-allowed;
}
```

---

## Performance Optimization

### Lazy Loading

```jsx
// React lazy loading
import React, { lazy, Suspense } from 'react';

const HeavyComponent = lazy(() => import('./HeavyComponent'));

function App() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <HeavyComponent />
    </Suspense>
  );
}
```

### Image Optimization

```html
<!-- Use modern formats with fallbacks -->
<picture>
  <source srcset="image.webp" type="image/webp">
  <source srcset="image.jpg" type="image/jpeg">
  <img src="image.jpg" alt="Fallback image" loading="lazy">
</picture>
```

---

## Security Best Practices

### XSS Prevention

```jsx
// React automatically escapes content
function UserProfile({ user }) {
  // Safe: React escapes user.name
  return <div>{user.name}</div>;

  // Dangerous: Don't use dangerouslySetInnerHTML unless necessary
  // return <div dangerouslySetInnerHTML={{ __html: user.bio }} />;
}
```

### CSRF Protection

```html
<!-- Include CSRF token in forms -->
<form method="post" action="/login">
  <input type="hidden" name="_csrf" value="{{ csrfToken }}">
  <!-- Form fields -->
</form>
```

---

## Common Patterns

### Loading States

```jsx
function DataTable({ data, isLoading, error }) {
  if (error) {
    return (
      <div role="alert" className="error">
        Error: {error.message}
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="loading" aria-busy="true">
        Loading...
      </div>
    );
  }

  return (
    <table>
      {/* Table content */}
    </table>
  );
}
```

### Modal Dialogs

```jsx
function Modal({ isOpen, onClose, children }) {
  useEffect(() => {
    if (isOpen) {
      // Trap focus in modal
      // Prevent body scroll
      document.body.style.overflow = 'hidden';
    }

    return () => {
      document.body.style.overflow = '';
    };
  }, [isOpen]);

  if (!isOpen) return null;

  return (
    <div
      className="modal-overlay"
      role="dialog"
      aria-modal="true"
      onClick={onClose}
    >
      <div
        className="modal-content"
        onClick={(e) => e.stopPropagation()}
      >
        <button
          className="modal-close"
          onClick={onClose}
          aria-label="Close dialog"
        >
          ×
        </button>
        {children}
      </div>
    </div>
  );
}
```

---

## Summary Checklist

When generating web UI components, ensure:

- [ ] Semantic HTML elements used (`<form>`, `<button>`, `<label>`, etc.)
- [ ] All inputs have associated `<label>` elements
- [ ] ARIA attributes included where needed (`aria-label`, `aria-describedby`, etc.)
- [ ] Form validation implemented (client-side with clear errors)
- [ ] Responsive design patterns applied (mobile-first)
- [ ] Loading and error states handled
- [ ] Keyboard navigation supported
- [ ] Images have alt text
- [ ] Buttons have type attribute (`type="submit"` or `type="button"`)
- [ ] No inline styles (use classes or CSS-in-JS)
- [ ] Security best practices followed (XSS prevention, CSRF tokens)
- [ ] Performance optimizations considered (lazy loading, image optimization)

These best practices ensure generated web UI components are accessible, maintainable, secure, and performant.
