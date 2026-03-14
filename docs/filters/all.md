# Filters Overview

Filters transform values in templates. They are applied using the pipe (`|`) operator.

## Basic Usage

```html
{{ value | filter_name }}
```

## Chaining Filters

Multiple filters can be chained:

```html
{{ value | filter1 | filter2 | filter3 }}
```

## Filter Arguments

Many filters accept arguments:

```html
{{ text | truncate(length=50, end="...") }}
{{ items | slice(start=1, length=10) }}
```

## Filter Categories

- [String Filters](filters/string.html) - Text transformation
- [Collection Filters](filters/collection.html) - Array/object operations
- [Number Filters](filters/number.html) - Math operations
- [Date Filters](filters/date.html) - Date formatting
- [HTML Filters](filters/html.html) - HTML processing
- [Encoding Filters](filters/encoding.html) - Encoding/decoding
- [Conditional Filters](filters/conditional.html) - Conditional logic

## Helper Directives

- [@map](filters/helpers.html#map) - Transform array elements
- [@filter](filters/helpers.html#filter) - Filter array elements
- [@each](filters/helpers.html#each) - Iterate with index
- [@reduce](filters/helpers.html#reduce) - Reduce to value
- [@flatten](filters/helpers.html#flatten) - Flatten arrays
- [@partition](filters/helpers.html#partition) - Split array
