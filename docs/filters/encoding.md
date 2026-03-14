# Encoding Filters

## json_decode

Parses a JSON string.

```html
{{ '{"name": "test", "value": 123}' | json_decode | safe }}
{# Output: {"name":"test","value":123} as object #}
```

## urlescape

URL encodes a string.

```html
{{ "hello world! 123" | urlescape }}
{# Output: hello%20world%21%20123 #}
```

## urlunescape

URL decodes a string.

```html
{{ "hello%20world" | urlunescape }}
{# Output: hello world #}
```

## base64_encode

Encodes to Base64.

```html
{{ "Hello World" | base64_encode }}
{# Output: SGVsbG8gV29ybGQ= #}
```

## base64_decode

Decodes from Base64.

```html
{{ "SGVsbG8gV29ybGQ=" | base64_decode }}
{# Output: Hello World #}
```
