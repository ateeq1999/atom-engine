# Helper Directives

Helper directives provide powerful array manipulation capabilities.

## @map

Transforms array elements by extracting a property or applying a transform.

### Extract Property

```html
{{ users | map(prop="name") | join(", ") }}
{# If users = [{"name": "Alice"}, {"name": "Bob"}], output: Alice, Bob #}
```

### With Transform

```html
{{ users | map(prop="name", transform="upper") | join(", ") }}
{# Output: ALICE, BOB #}

{{ users | map(prop="name", transform="lower") | join(", ") }}
{# Output: alice, bob #}

{{ items | map(prop="tags", transform="length") | join(", ") }}
```

## @filter

Filters array elements by key/value with optional operator.

### Basic Filter

```html
{{ users | filter(key="active", value=true) | map(prop="name") }}
{# Returns only active users #}
```

### With Operators

```html
{{ users | filter(key="age", value=18, op="gte") }}
{# Users with age >= 18 #}

{{ users | filter(key="name", value="Alice", op="ne") }}
{# Users where name != Alice #}

{{ items | filter(key="price", value=100, op="gt") }}
{# Items with price > 100 #}
```

### Available Operators

| Operator | Description |
|----------|-------------|
| `eq` | Equal (default) |
| `ne` | Not equal |
| `gt` | Greater than |
| `gte` | Greater than or equal |
| `lt` | Less than |
| `lte` | Less than or equal |
| `contains` | Contains substring |
| `exists` | Key exists |

## @each

Iterates with index and metadata.

```html
{% for item in items | each(index=true) %}
    {{ item.index }}: {{ item.value }}
    {% if item.first %} (first){% endif %}
    {% if item.last %} (last){% endif %}
{% endfor %}
```

## @reduce

Reduces array to a single value.

```html
{{ numbers | reduce(initial=0) }}
{# Sum of all numbers #}

{{ users | reduce(initial=0, prop="age") }}
{# Sum of all ages #}
```

## @flatten

Flattens nested arrays.

```html
{{ [[1, 2], [3, 4], [5]] | flatten | join(", ") }}
{# Output: 1, 2, 3, 4, 5 #}
```

## @partition

Splits array into matched and rest.

```html
{% set partitioned = users | partition(key="active") %}
Active: {{ partitioned.matched | length }}
Inactive: {{ partitioned.rest | length }}

{% for user in partitioned.matched %}
    {{ user.name }} (active)
{% endfor %}
```

## Pipeline Examples

### Complex Data Processing

```html
{# Get names of active users, sorted alphabetically #}
{{ users 
    | filter(key="active", value=true) 
    | map(prop="name") 
    | sort 
    | join(", ") 
}}

{# Get average age of adult users #}
{{ users 
    | filter(key="age", value=18, op="gte") 
    | map(prop="age") 
    | avg 
}}

{# Get unique tags from all posts #}
{{ posts 
    | map(prop="tags") 
    | flatten 
    | uniq 
    | sort 
    | join(", ") 
}}
```
