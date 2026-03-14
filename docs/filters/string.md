# String Filters

String filters transform text values.

## upper

Converts text to uppercase.

```html
{{ "hello" | upper }}
{# Output: HELLO #}
```

## lower

Converts text to lowercase.

```html
{{ "HELLO" | lower }}
{# Output: hello #}
```

## capitalize

Capitalizes the first letter.

```html
{{ "hello world" | capitalize }}
{# Output: Hello world #}
```

## title

Converts to title case.

```html
{{ "hello world" | title }}
{# Output: Hello World #}
```

## camel_case

Converts to camelCase.

```html
{{ "hello_world" | camel_case }}
{# Output: helloWorld #}
```

## pascal_case

Converts to PascalCase.

```html
{{ "hello_world" | pascal_case }}
{# Output: HelloWorld #}
```

## snake_case

Converts to snake_case.

```html
{{ "helloWorld" | snake_case }}
{# Output: hello_world #}
```

## kebab_case

Converts to kebab-case.

```html
{{ "helloWorld" | kebab_case }}
{# Output: hello-world #}
```

## truncate

Truncates text to a maximum length.

```html
{{ "Hello World" | truncate(length=5) }}
{# Output: Hello... #}

{{ "Hello World" | truncate(length=9, end="...") }}
{# Output: Hello... #}
```

## slugify

Creates a URL-safe slug.

```html
{{ "Hello World! 123" | slugify }}
{# Output: hello-world-123 #}
```

## pluralize

Returns empty string for 1, "s" for other numbers.

```html
{{ 1 | pluralize }}
{# Output:  #}

{{ 2 | pluralize }}
{# Output: s #}

{{ 5 | pluralize(singular="item", plural="items") }}
{# Output: items #}
```

## replace

Replaces text.

```html
{{ "Hello World" | replace(old="World", new="Rust") }}
{# Output: Hello Rust #}
```

## remove

Removes text.

```html
{{ "Hello World" | remove(string="o") }}
{# Output: Hell Wrld #}
```

## prepend

Prepends text.

```html
{{ "World" | prepend(string="Hello ") }}
{# Output: Hello World #}
```

## append

Appends text.

```html
{{ "Hello" | append(string=" World") }}
{# Output: Hello World #}
```

## strip

Trims whitespace.

```html
{{ "  hello world  " | strip }}
{# Output: hello world #}
```

## nl2br

Converts newlines to `<br>` tags.

```html
{{ "line1\nline2" | nl2br | safe }}
{# Output: line1<br>line2 #}
```

## word_count

Counts words in text.

```html
{{ "hello world foo bar" | word_count }}
{# Output: 4 #}
```

## char_count

Counts characters.

```html
{{ "hello" | char_count }}
{# Output: 5 #}
```

## starts_with

Checks if string starts with prefix.

```html
{{ "hello" | starts_with(prefix="hel") }}
{# Output: true #}
```

## ends_with

Checks if string ends with suffix.

```html
{{ "hello" | ends_with(suffix="llo") }}
{# Output: true #}
```

## contains

Checks if string contains substring.

```html
{{ "hello world" | contains(substring="world") }}
{# Output: true #}
```
