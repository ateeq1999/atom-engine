# Number Filters

Number filters perform mathematical operations.

## round

Rounds a number.

```html
{{ 3.7 | round }}
{# Output: 4 #}

{{ 3.14159 | round(precision=2) }}
{# Output: 3.14 #}
```

## abs

Returns absolute value.

```html
{{ -42 | abs }}
{# Output: 42 #}
```

## format

Formats a number with thousand separators.

```html
{{ 1234567 | format }}
{# Output: 1,234,567 #}

{{ 1234567 | format(format="_") }}
{# Output: 1_234_567 #}
```

## min

Returns minimum value in array.

```html
{{ numbers | min }}
{# If numbers = [3, 1, 2], output: 1 #}
```

## max

Returns maximum value in array.

```html
{{ numbers | max }}
{# If numbers = [3, 1, 2], output: 3 #}
```

## sum

Returns sum of all values.

```html
{{ numbers | sum }}
{# If numbers = [1, 2, 3], output: 6 #}
```

## avg

Returns average of values.

```html
{{ numbers | avg }}
{# If numbers = [1, 2, 3], output: 2 #}
```

## ceil

Returns ceiling of number.

```html
{{ 3.1 | ceil }}
{# Output: 4 #}
```

## floor

Returns floor of number.

```html
{{ 3.9 | floor }}
{# Output: 3 #}
```
