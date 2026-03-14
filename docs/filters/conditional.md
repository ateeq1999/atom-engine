# Conditional Filters

## default

Returns default value if null or empty.

```html
{{ null_value | default(value="fallback") }}
{# Output: fallback #}

{{ "" | default(value="empty") }}
{# Output: empty #}

{{ "actual" | default(value="fallback") }}
{# Output: actual #}
```

## when

Ternary conditional.

```html
{{ is_active | when(then="yes", else="no") }}
{# If is_active = true, output: yes #}
{# If is_active = false, output: no #}
```

## coalesce

Returns first non-null value.

```html
{{ null_value | coalesce(other=fallback) }}
{# Output: fallback #}

{{ "actual" | coalesce(other=fallback) }}
{# Output: actual #}
```

## defined

Checks if variable is defined.

```html
{% if my_var | defined %}
    Variable is defined
{% endif %}
```

## undefined

Checks if variable is undefined.

```html
{% if my_var | undefined %}
    Variable is not defined
{% endif %}
```

## empty

Checks if value is empty.

```html
{{ [] | empty }}
{# Output: true #}

{{ "" | empty }}
{# Output: true #}

{{ [1] | empty }}
{# Output: false #}
```

## not_empty

Checks if value is not empty.

```html
{{ [1] | not_empty }}
{# Output: true #}

{{ "" | not_empty }}
{# Output: false #}
```
