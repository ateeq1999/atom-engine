# HTML Filters

## escape_html

Escapes HTML special characters.

```html
{{ "<script>alert('xss')</script>" | escape_html }}
{# Output: &lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt; #}
```

## safe

Marks content as safe (won't be escaped).

```html
{{ "<b>bold</b>" | safe }}
{# Output: <b>bold</b> (not escaped) #}
```

## strip_tags

Removes HTML tags.

```html
{{ "<p>Hello <b>World</b></p>" | strip_tags }}
{# Output: Hello World #}
```
