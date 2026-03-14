# Global Functions

Global functions provide utility operations in templates.

## dump

Debug output to stderr.

```html
{% call dump(var=myvar, other=123) %}
{# Outputs to stderr: [dump] var = ..., [dump] other = 123 #}
```

## log

Log to stderr.

```html
{% call log(message="Hello", value=123) %}
```

## range

Generates a range of numbers.

```html
{{ range(end=5) | join(", ") }}
{# Output: 0,1,2,3,4 #}

{{ range(start=1, end=5) | join(", ") }}
{# Output: 1,2,3,4 #}

{{ range(start=0, end=10, step_by=2) | join(", ") }}
{# Output: 0,2,4,6,8 #}
```

## now

Returns current datetime.

```html
{{ now() }}
{# Output: 2024-01-15T14:30:45Z #}

{{ now(utc=true) }}
{# Output: 2024-01-15T14:30:45+00:00 #}

{{ now(timestamp=true) }}
{# Output: 1705329045 #}
```

## uuid

Generates a UUID.

```html
{{ uuid() }}
{# Output: 550e8400-e29b-41d4-a716-446655440000 #}
```

## random

Generates a random number.

```html
{{ random(min=1, max=100) }}
{# Output: random number between 1 and 100 #}
```

## choice

Selects a random element from array.

```html
{{ choice(array=["a", "b", "c"]) }}
{# Output: random choice #}
```

## file_exists

Checks if file exists.

```html
{{ file_exists(path="config.json") }}
{# Output: true or false #}
```

## env

Gets environment variable.

```html
{{ env(key="HOME") }}
{# Output: /home/user #}
```

## md5

Generates MD5 hash.

```html
{{ md5(string="hello") }}
{# Output: 5d41402abc4b2a76b9719d911017c592 #}
```

## sha256

Generates SHA256 hash.

```html
{{ sha256(string="hello") }}
{# Output: 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824 #}
```

## repeat

Repeats a string.

```html
{{ repeat(content="ab", count=3) }}
{# Output: ababab #}

{{ repeat(content="-", count=10, separator="") }}
{# Output: ---------- #}
```

## times

Generates a sequence.

```html
{{ times(times=5) | join(", ") }}
{# Output: 0,1,2,3,4 #}

{{ times(times=3, start=1, step=2) | join(", ") }}
{# Output: 1,3,5 #}
```

## loop

Generates a range with metadata.

```html
{{ loop(from=1, to=5, step=1) | map(prop="value") | join(", ") }}
{# Output: 1,2,3,4,5 #}

{# With inclusive option #}
{{ loop(from=1, to=3, inclusive=true) | map(prop="value") | join(", ") }}
{# Output: 1,2,3 #}
```

## iterate

Iterates array with metadata.

```html
{{ iterate(array=[1,2,3,4,5], limit=3) | map(prop="value") | join(", ") }}
{# Output: 1,2,3 #}

{{ iterate(array=[1,2,3,4,5], skip=2, limit=2) | map(prop="value") | join(", ") }}
{# Output: 3,4 #}
```

## object

Creates object from keys and values.

```html
{{ object(keys=["a", "b", "c"], values=[1, 2, 3]) | json_encode | safe }}
{# Output: {"a":1,"b":2,"c":3} #}
```

## merge

Merges two arrays.

```html
{{ merge(array1=[1,2], array2=[3,4]) | join(", ") }}
{# Output: 1,2,3,4 #}
```

## chunk

Splits array into chunks.

```html
{{ [1,2,3,4,5,6] | chunk(size=2) | json_encode | safe }}
{# Output: [[1,2],[3,4],[5,6]] #}
```

## zip

Combines multiple arrays.

```html
{{ zip(arrays=[[1,2,3], ["a","b","c"]]) | json_encode | safe }}
{# Output: [[1,"a"],[2,"b"],[3,"c"]] #}
```

## compact

Removes null and empty values.

```html
{{ compact(array=[1, null, 2, "", 3]) | join(", ") }}
{# Output: 1,2,3 #}
```
