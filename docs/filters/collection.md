# Collection Filters

Collection filters work with arrays and objects.

## first

Returns the first element of an array.

```html
{{ items | first }}
{# If items = ["a", "b", "c"], output: a #}
```

## last

Returns the last element of an array.

```html
{{ items | last }}
{# If items = ["a", "b", "c"], output: c #}
```

## length

Returns the number of elements.

```html
{{ items | length }}
{# If items = ["a", "b", "c"], output: 3 #}
```

## reverse

Reverses an array or string.

```html
{{ items | reverse | join(",") }}
{# If items = [1, 2, 3], output: 3,2,1 #}

{{ "hello" | reverse }}
{# Output: olleh #}
```

## sort

Sorts an array.

```html
{{ items | sort | join(",") }}
{# If items = [3, 1, 2], output: 1,2,3 #}
```

## group_by

Groups array elements by a field.

```html
{% for role, users in users_list | group_by(attribute="role") %}
    {{ role }}: {% for u in users %}{{ u.name }}{% endfor %}
{% endfor %}
```

## where

Filters array elements by a condition.

```html
{{ users | where(attribute="active") | map(prop="name") | join(", ") }}
{# Returns only active users #}

{{ users | where(attribute="role", value="admin") }}
{# Filters by specific value #}
```

## pluck

Extracts a field from all objects in an array.

```html
{{ users | pluck(attribute="name") | join(", ") }}
{# If users = [{"name": "Alice"}, {"name": "Bob"}], output: Alice, Bob #}
```

## join

Joins array elements with a separator.

```html
{{ items | join("-") }}
{# If items = ["a", "b", "c"], output: a-b-c #}
```

## slice

Returns a portion of an array or string.

```html
{{ items | slice(start=1, length=2) | join(",") }}
{# If items = [1, 2, 3, 4, 5], output: 2,3 #}
```

## uniq

Returns unique elements.

```html
{{ items | uniq | join(",") }}
{# If items = [1, 2, 1, 3], output: 1,2,3 #}
```

## shuffle

Randomly shuffles array elements.

```html
{{ items | shuffle | join(",") }}
{# Output: random order #}
```
