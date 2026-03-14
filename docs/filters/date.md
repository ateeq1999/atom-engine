# Date Filters

## date

Formats a date value.

```html
{{ now() | date }}
{# Output: 2024-01-15 #}

{{ now() | date(format="%Y-%m-%d %H:%M") }}
{# Output: 2024-01-15 14:30 #}
```

## Available Format Tokens

| Token | Description | Example |
|-------|-------------|---------|
| `%Y` | Year (4 digits) | 2024 |
| `%m` | Month (01-12) | 01 |
| `%d` | Day (01-31) | 15 |
| `%H` | Hour (00-23) | 14 |
| `%M` | Minute (00-59) | 30 |
| `%S` | Second (00-59) | 45 |
| `%A` | Weekday name | Monday |
| `%B` | Month name | January |
