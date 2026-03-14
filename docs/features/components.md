# Components

Components are reusable template blocks with props and slots.

## Registering Components

```rust
engine.register_component(
    "button",
    r#"<button class="btn">{{ text }}</button>"#
).unwrap();
```

## Component with Props

```rust
engine.register_component(
    "card",
    r#"<div class="card">
        <h3>{{ title }}</h3>
        <div class="content">{{ $slots.default() }}</div>
    </div>"#
).unwrap();
```

## Using Components

```html
{% call button(text="Click Me") %}

{% call card(title="My Card") %}
    This is the card content.
{% endcall %}
```

## Props Definition

Define props with type and validation:

```rust
engine.register_component(
    "user-card",
    r#"{%-- atom: @props(name:String, age:Number, active:Boolean=true) --%}
    <div class="user-card">
        <h3>{{ name }}</h3>
        <p>Age: {{ age }}</p>
        {% if active %}<span class="badge">Active</span>{% endif %}
    </div>"#
).unwrap();
```

### Prop Types

- `String` - Text values
- `Number` - Numeric values
- `Boolean` - True/false
- `Array` - Array values
- `Object` - Object values
- `Any` - Any value (default)

### Prop Modifiers

- `?` - Optional: `name?`
- `=default` - Default value: `name="Guest"`

```rust
{%-- atom: @props(name:String, theme:String="dark", debug:Boolean=false) --%}
```

## Slots

### Default Slot

```html
{# Component #}
<div class="card">
    {{ $slots.default() }}
</div>

{# Usage #}
{% call card() %}
    Card content here
{% endcall %}
```

### Named Slots

```html
{# Component #}
<div class="modal">
    <header>{{ $slots.header() }}</header>
    <body>{{ $slots.body() }}</body>
    <footer>{{ $slots.footer() }}</footer>
</div>

{# Usage #}
{% call modal() %}
    {% fill header() %}Title{% endfill %}
    {% fill body() %}Content{% endfill %}
    {% fill footer() %}<button>Close</button>{% endfill %}
{% endcall %}
```

### Slot with Fallback

```html
{# Component #}
<div class="icon">
    {{ $slots.icon(default="default.png") }}
</div>
```

## Scoped Slots

Pass data from component to slot:

```html
{# Component #}
{% for item in items %}
    {{ $slots.item(item=item) }}
{% endfor %}

{# Usage #}
{% call list(items=users) %}
    {% fill item(item=user) %}{{ user.name }}{% endfill %}
{% endcall %}
```

## Component Caching

Enable caching for performance:

```rust
engine.enable_component_cache(true);

// Render - will be cached
let result = engine.render("page.html", &ctx).unwrap();

// Check cache stats
println!("Cache size: {}", engine.component_cache_len());

// Clear cache when needed
engine.clear_component_cache();
```
