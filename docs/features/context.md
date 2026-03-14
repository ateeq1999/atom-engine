# Context (Provide/Inject)

React-style context for passing data to templates.

## Providing Values

```rust
engine.provide("theme", json!("dark"));
engine.provide("user", json!({
    "name": "Alice",
    "role": "admin"
}));
engine.provide("config", json!({
    "site_name": "My Site",
    "version": "1.0.0"
}));
```

## Using in Templates

```html
{# Access provided values #}
Theme: {{ __context.theme }}
User: {{ __context.user.name }}
Role: {{ __context.user.role }}

{# Nested access #}
{{ __context.config.site_name }}
```

## Use Cases

### Theme Switching

```rust
// In your app
engine.provide("theme", json!(current_theme));

// In template
<body class="{{ __context.theme }}">
    {% if __context.theme == "dark" %}
        <!-- Dark mode styles -->
    {% endif %}
</body>
```

### User Authentication

```rust
engine.provide("user", json!({
    "id": user.id,
    "name": user.name,
    "role": user.role,
    "permissions": user.permissions
}));
```

```html
{% if __context.user %}
    Welcome, {{ __context.user.name }}!
    {% if __context.user.role == "admin" %}
        <a href="/admin">Admin Panel</a>
    {% endif %}
{% else %}
    <a href="/login">Login</a>
{% endif %}
```
