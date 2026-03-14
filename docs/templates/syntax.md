# Template Syntax

Atom Engine supports a rich template syntax for building dynamic pages.

## Variables

Access variables using double curly braces:

```html
{{ user.name }}
{{ items[0] }}
{{ company.address.city }}
{{ items | length }}
```

### Variable Types

- **Strings**: `"Hello"`
- **Numbers**: `42`, `3.14`
- **Booleans**: `true`, `false`
- **Arrays**: `["a", "b", "c"]`
- **Objects**: `{"name": "Alice", "age": 30}`

## Comments

Use `{# ... #}` for comments:

```html
{# This is a comment and won't appear in output #}
{{ name }}
```

## Filters

Filters transform values. Chain multiple filters:

```html
{{ name | upper | capitalize }}
{{ text | truncate(length=50, end="...") }}
{{ items | sort | reverse | first }}
```

### Filter Syntax

```html
{{ value | filter_name }}
{{ value | filter_name(arg1, arg2) }}
{{ value | filter1 | filter2 | filter3 }}
```

## Control Flow

### Conditionals

```html
{% if user.is_active %}
    Welcome, {{ user.name }}!
{% elif user.is_pending %}
    Please verify your email.
{% else %}
    <a href="/login">Login</a>
{% endif %}
```

### Boolean Operators

```html
{% if user.is_active and user.is_verified %}
    Verified user
{% endif %}

{% if is_admin or is_editor %}
    Can edit content
{% endif %}

{% if not user.is_banned %}
    Not banned
{% endif %}
```

### Comparison Operators

```html
{% if count > 10 %}Many items{% endif %}
{% if count < 5 %}Few items{% endif %}
{% if count >= 10 %}At least 10{% endif %}
{% if count <= 100 %}At most 100{% endif %}
{% if status == "active" %}Active{% endif %}
{% if role != "guest" %}Not a guest{% endif %}
```

## Loops

### Basic For Loop

```html
<ul>
{% for item in items %}
    <li>{{ item }}</li>
{% endfor %}
</ul>
```

### Loop Variables

```html
{% for item in items %}
    Index (1-based): {{ loop.index }}
    Index (0-based): {{ loop.index0 }}
    First item: {{ loop.first }}
    Last item: {{ loop.last }}
    Total items: {{ loop.length }}
{% endfor %}
```

### Loop with Condition

```html
{% for item in items %}
    {% if loop.first %}<ul>{% endif %}
    <li>{{ item }}</li>
    {% if loop.last %}</ul>{% endif %}
{% endfor %}
```

### Empty Handling

```html
{% for item in items %}
    {{ item }}
{% empty %}
    No items found
{% endfor %}
```

### Nested Loops

```html
{% for outer in outer_items %}
    {% for inner in inner_items %}
        {{ outer }}-{{ inner }}
    {% endfor %}
{% endfor %}
```

## Template Inheritance

### Base Template

```html
<!DOCTYPE html>
<html>
<head>
    <title>{% block title %}Default Title{% endblock %}</title>
    {% block extra_head %}{% endblock %}
</head>
<body>
    <header>{% block header %}Header{% endblock %}</header>
    <main>{% block content %}{% endblock %}</main>
    <footer>{% block footer %}Footer{% endblock %}</footer>
</body>
</html>
```

### Child Template

```html
{% extends "base.html" %}

{% block title %}{{ page_title }}{% endblock %}

{% block content %}
    <h1>{{ article.title }}</h1>
    <p>{{ article.content }}</p>
{% endblock %}
```

### Super Block

```html
{% block head %}
    {{ super() }}
    <meta name="custom">
{% endblock %}
```

## Includes

```html
{% include "header.html" %}

{% includeIf "optional/footer.html" %}

{% includeFirst ["primary.html", "fallback.html"] %}
```

## Macros

```html
{% macro button(text, type="primary") %}
<button class="btn btn-{{ type }}">{{ text }}</button>
{% endmacro %}

{% macro input(name, type="text", placeholder="") %}
<input type="{{ type }}" name="{{ name }}" placeholder="{{ placeholder }}">
{% endmacro %}

{{ button("Submit", type="success") }}
{{ input("email", type="email") }}
```

## Set Variable

```html
{% set greeting = "Hello" %}
{% set items = [1, 2, 3] %}
{% set user = {"name": "Alice"} %}

{{ greeting }}, {{ user.name }}!
```

## Whitespace Control

Use `-` to strip whitespace:

```html
{# Strip leading whitespace #}
{%- for item in items -%}
    {{ item }}
{%- endfor -%}

{# Output: item1item2item3 #}
```
