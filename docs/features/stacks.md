# Stack System

The stack system allows accumulating content from multiple places and rendering it together.

## Push to Stack

```html
{% call push(name="scripts", content="<script>console.log('a')</script>") %}
{% call push(name="scripts", content="<script>console.log('b')</script>") %}
{% call push(name="scripts", content="<script>console.log('c')</script>") %}
```

## Prepend to Stack

```html
{% call prepend(name="styles", content="<link rel='stylesheet' href='a.css'>") %}
{% call prepend(name="styles", content="<link rel='stylesheet' href='b.css'>") %}
```

## Render Stack

```html
{{ "scripts" | stack }}

{{ "styles" | stack }}
```

## Use Cases

### Collecting Page Scripts

```html
{# layout.html #}
<html>
<head>
    {{ "styles" | stack }}
</head>
<body>
    {{ content }}
</body>
<script>
    {{ "scripts" | stack }}
</script>
</html>
```

```html
{# page.html #}
{% extends "layout.html" %}

{% block content %}
    <h1>My Page</h1>
{% endblock %}

{# Push scripts from components or partials #}
{% call push(name="scripts", content="<script>initPage()</script>") %}
```

### Page-specific Styles

```html
{# In different templates #}
{% call push(name="page_styles", content="<style>.red { color: red }</style>") %}
{% call push(name="page_styles", content="<style>.blue { color: blue }</style>") %}

{# In base template #}
<head>
    {{ "page_styles" | stack }}
</head>
```

### Meta Tags

```html
{# In templates #}
{% call push(name="meta", content="<meta name='description' content='...'>") %}

{# In head #}
{{ "meta" | stack }}
```
