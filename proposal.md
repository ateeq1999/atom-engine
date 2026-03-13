# Atom Engine

### A Pure, General-Purpose Template Engine in Rust

**Version 4.0 · March 2026**

> `.atom` templates · `@directive{}` system · React-inspired components · EdgeJS-inspired syntax  
> Format-agnostic · Zero-tier architecture · Composable · Extensible

---

## 1. What Atom Engine Is

Atom Engine is a **pure template engine**. It takes a template file and a data context, evaluates all directives and expressions, and returns a rendered string. That is the entire job.

There is no tier system. There are no atoms, layouts, features, or configs. There are only **templates** — `.atom` files — and they can render anything: HTML, TypeScript, SQL, YAML, Markdown, JSON, shell scripts, config files, or any other text format.

The design is inspired by two things:

**EdgeJS**: `@directive(){}` syntax that is terse, readable, and feels like writing code rather than learning a templating dialect. Directives are the only special syntax — everything else is literal output. Custom directives can be registered at runtime.

**React**: The component model. Templates are components. Components accept props, render children, define named slots, and compose other components. The component tree is the mental model — not inheritance chains or include graphs.

Where Atom Engine goes further than both: it is implemented in **Rust**, compiles templates to an internal AST at parse time (not at runtime), provides a rich built-in helper library covering strings, collections, math, encoding, and formatting, and exposes a clean extension API for custom directives.

---

## 2. Template Files

Every template is a `.atom` file. The file extension is the only convention. There is no filename-based tier, no required front-matter block, no required structure. A valid `.atom` file can be as simple as:

```atom
Hello, {{ name }}!
```

Templates that need to declare props, define slots, or specify output metadata do so using the `@props`, `@slots`, and `@meta` header directives at the top of the file. These are optional. A template without them is still a valid template.

### 2.1 Template Header (Optional)

```atom
@props({
  name: String,
  items: Array,
  count: Number = 0,
  active: Boolean = true,
  label: String = "default",
})

@slots([header, footer, content?])

@meta({
  description: "Renders a list with a header and footer",
})
```

`@props` declares the variables this template accepts and their types. Props with `= value` are optional with a default. Props without defaults are required — passing a context that is missing a required prop is a render error with a clear message.

`@slots` declares named slot outlets this template exposes to its callers. The `?` suffix marks a slot as optional.

`@meta` is a free-form key-value block used for documentation and tooling. It has no effect on rendering.

---

## 3. Interpolation

Values are interpolated with `{{ expr }}`. Any expression valid in Atom Engine's expression language is valid between the double braces.

```atom
Hello, {{ name }}!
Today is {{ date.format("YYYY-MM-DD") }}.
You have {{ items.len() }} items.
The total is {{ (price * quantity).round(2) }}.
```

Expressions support property access, method calls (helpers), arithmetic, comparisons, logical operators, and ternary expressions. They do not support arbitrary code execution — the expression language is intentionally sandboxed.

### 3.1 Raw Output

`{{{ expr }}}` emits the value without any escaping. `{{ expr }}` escapes HTML-special characters by default when the engine is configured for HTML output. In non-HTML modes (the default), both forms are identical.

---

## 4. Directives — The Full System

Every directive starts with `@`. A directive either:

- **Takes arguments** in `()` and optionally a **block body** in `{}`
- **Is self-closing** with `@!` prefix (no block body, renders immediately)

The general form:

```
@directiveName(args) {
  block body
}
```

Or inline for directives that produce a value:

```
{{ @directiveName(args) }}
```

### 4.1 Control Flow

#### `@if(expr) { }` / `@elseif(expr) { }` / `@else { }`

```atom
@if(user.isAdmin) {
  <span class="badge">Admin</span>
} @elseif(user.isModerator) {
  <span class="badge">Moderator</span>
} @else {
  <span class="badge">Member</span>
}
```

Chains are formed by `@elseif` and `@else` immediately following the closing `}` of the prior branch. Any number of `@elseif` branches are allowed. Only one `@else` is allowed and it must be last.

`@if` can also be used inline as a **value expression** on the right-hand side of `@let`:

```atom
@let(role = @if(user.isAdmin) { "admin" } @elseif(user.isModerator) { "mod" } @else { "member" })
```

#### `@unless(expr) { }`

Semantic inverse of `@if`. Renders its body when `expr` is falsy.

```atom
@unless(user.verified) {
  Please verify your email.
}
```

#### `@each(item in collection) { }`

Iterates over arrays, objects, and any iterable value.

```atom
@each(user in users) {
  <li>{{ user.name }} — {{ user.email }}</li>
}
```

Inside `@each`, three loop variables are automatically available:

| Variable | Type | Description |
|---|---|---|
| `$index` | `Number` | Zero-based position |
| `$number` | `Number` | One-based position (`$index + 1`) |
| `$first` | `Boolean` | `true` for the first element |
| `$last` | `Boolean` | `true` for the last element |
| `$even` | `Boolean` | `true` on even iterations |
| `$odd` | `Boolean` | `true` on odd iterations |
| `$total` | `Number` | Total count of the collection |
| `$key` | `String` | The key when iterating over an object |

```atom
@each(item in items) {
  @if(!$first) { , }{{ item.name }}@if($last) { . }
}
```

#### `@for(i from start to end) { }` / `@for(i from start to end step n) { }`

Numeric range loop. `start` inclusive, `end` exclusive. Optional `step` (default `1`, may be negative for countdown).

```atom
@for(i from 1 to 6) {
  <h{{ i }}>Heading {{ i }}</h{{ i }}>
}

@for(i from 10 to 0 step -2) {
  {{ i }}...
}
```

`$index`, `$first`, `$last` are available inside `@for` as in `@each`.

#### `@while(expr) { }`

Renders its body repeatedly while `expr` is truthy. Requires `@set` inside the body to prevent infinite loops — the engine will enforce a maximum iteration count (configurable, default 10,000).

```atom
@let(n = 1)
@while(n <= 5) {
  {{ n }}
  @set(n = n + 1)
}
```

#### `@switch(expr) { }` with `@case(val) { }` / `@default { }`

```atom
@switch(status) {
  @case("active") {
    <span class="green">Active</span>
  }
  @case("pending") {
    <span class="yellow">Pending</span>
  }
  @case("banned") {
    <span class="red">Banned</span>
  }
  @default {
    <span class="grey">Unknown</span>
  }
}
```

### 4.2 Variable Directives

#### `@let(name = expr)`

Declares an immutable variable in the current scope. The variable is visible in the rest of the current block and all nested blocks. Re-declaring the same name in the same scope is an error.

```atom
@let(greeting = "Hello, " + name + "!")
@let(count = items.len())
@let(isLong = count > 100)
```

#### `@set(name = expr)`

Mutates an existing variable declared by `@let` in any enclosing scope. Props are always immutable — `@set` targeting a prop name is an error.

```atom
@let(sep = "")
@each(tag in tags) {
  {{ sep }}{{ tag }}
  @set(sep = ", ")
}
```

#### `@const(name = expr)`

Equivalent to `@let` but communicates intent: this value will never be `@set`. The engine enforces this — any `@set` targeting a `@const` name is an error.

```atom
@const(BASE_URL = "https://api.example.com")
```

### 4.3 Component Directives

#### `@use('template/path') { }` or `@use('template/path', props) { }`

Renders another template as a component, optionally passing props and a slot body.

```atom
@use('components/card', { title: "Welcome" }) {
  <p>This is the card body — it fills the main slot.</p>
}
```

The path is relative to the template root directory (configurable). Named disks are supported with a `disk::` prefix: `@use('ui::components/button')`.

Self-closing form (no slot body):

```atom
@!use('components/spinner', { size: "lg" })
```

#### `@slot(name?) { default_content }`

Used inside a component template to mark where slot content is injected. The optional block body is the default content rendered when the caller provides nothing for this slot.

```atom
{{-- components/card.atom --}}
<div class="card">
  <div class="card-header">
    @slot(header) {
      <h3>Default Title</h3>
    }
  </div>
  <div class="card-body">
    @slot() {
      <p>No content provided.</p>
    }
  </div>
  <div class="card-footer">
    @slot(footer) {}
  </div>
</div>
```

`@slot()` with no name is the **main slot** — it receives the block body of the `@use` directive directly.

#### `@fill(slot_name) { }` — Filling Named Slots from the Caller

When calling a component that has named slots, use `@fill` inside the `@use` body to target specific slots:

```atom
@use('components/card', { elevated: true }) {
  @fill(header) {
    <h2>My Custom Title</h2>
  }
  @fill(footer) {
    <button>Close</button>
  }
  <p>This goes into the main slot (no @fill needed).</p>
}
```

Content not inside a `@fill` block goes to the main slot.

#### `@provide(key, value)` and `@inject(key)`

React-style context: `@provide` makes a value available to all descendant templates in the render tree without passing it as an explicit prop. `@inject` reads it in any descendant.

```atom
{{-- In a parent template --}}
@provide(theme, "dark")
@provide(locale, user.locale)

@use('components/layout') {
  @use('components/nav') {}
}
```

```atom
{{-- components/nav.atom — can be deeply nested --}}
@inject(theme)
@inject(locale)

<nav class="nav-{{ theme }}">
  <span>{{ locale }}</span>
</nav>
```

### 4.4 Layout Directives

#### `@extends('layout/path')`

Declares that this template extends a layout. When `@extends` is present, the entire template body is treated as slot content for the layout. The layout controls the outer structure.

```atom
@extends('layouts/base')

@fill(title) {
  My Page Title
}

@fill(content) {
  <article>
    <h1>Hello World</h1>
    <p>Page content goes here.</p>
  </article>
}
```

```atom
{{-- layouts/base.atom --}}
<!DOCTYPE html>
<html>
<head>
  <title>@slot(title) { Untitled }</title>
</head>
<body>
  <main>
    @slot(content) {}
  </main>
</body>
</html>
```

#### `@section(name) { }` and `@yield(name)`

An alternative to `@fill`/`@slot` for layout inheritance. `@section` in a child template defines a named section; `@yield` in the layout emits it.

```atom
{{-- In child template --}}
@extends('layouts/app')

@section(styles) {
  <link rel="stylesheet" href="/page.css">
}

@section(body) {
  <h1>Hello</h1>
}
```

```atom
{{-- layouts/app.atom --}}
<head>
  @yield(styles)
</head>
<body>
  @yield(body)
</body>
```

### 4.5 Include Directives

#### `@include('partial/path')` / `@include('partial/path', data)`

Includes another template inline. The included template shares the current scope by default. An optional second argument merges additional data into the scope for the included template only.

```atom
@include('partials/header')
@include('partials/user-card', { user: currentUser, compact: true })
```

#### `@includeWhen(condition, 'partial/path')` / `@includeUnless(condition, 'partial/path')`

Conditionally includes a partial. Shorthand for `@if(cond) { @include(...) }`.

```atom
@includeWhen(user.isAdmin, 'partials/admin-bar')
@includeUnless(user.verified, 'partials/verify-notice')
```

#### `@includeFirst(['path1', 'path2', 'fallback'])`

Tries each path in order and includes the first one that exists in the template registry.

```atom
@includeFirst(['themes/custom/header', 'themes/default/header', 'partials/header'])
```

#### `@each` + `@include` combined: `@includeEach('partial', collection)`

Renders a partial once for each item in the collection, with the item bound to `item` (or a named variable).

```atom
@includeEach('partials/comment', post.comments)
@includeEach('partials/row', { collection: rows, as: 'row' })
```

### 4.6 Stack Directives

Stacks allow templates at any depth to push content into named buckets that a layout drains once. This is the standard pattern for CSS/JS includes in HTML templates, but works for any accumulation pattern.

#### `@push(stack_name) { }`

Pushes content into a named stack. May be called from any depth in the render tree.

```atom
@push(scripts) {
  <script src="/page-specific.js"></script>
}
```

#### `@prepend(stack_name) { }`

Pushes content to the **front** of the stack (before any content already pushed).

```atom
@prepend(scripts) {
  <script src="/must-load-first.js"></script>
}
```

#### `@stack(name)`

Emits all content pushed into a named stack, in push order. Typically placed in a layout.

```atom
<head>
  @stack(styles)
</head>
<body>
  @yield(body)
  @stack(scripts)
</body>
```

### 4.7 Utility Directives

#### `@dump(expr)`

Pretty-prints the value of an expression as formatted text for debugging. Does not affect output in production mode (disabled when engine `debug: false`).

```atom
@dump(user)
@dump(items.filter(x => x.active))
```

#### `@log(expr)`

Emits a message to stderr. Does not affect rendered output. Always active regardless of debug mode.

```atom
@log("Rendering user: " + user.id)
```

#### `@comment { }`

Block comment — content is never rendered.

```atom
@comment {
  This entire block is ignored.
  TODO: replace with real data
}
```

Single-line comment shorthand: `{{-- this is a comment --}}`

#### `@raw { }`

Emits its content verbatim, treating all `@` and `{{` sequences as literal text.

```atom
@raw {
  Use {{ name }} to interpolate a variable.
  Use @if(cond) { } for conditionals.
}
```

#### `@once { }`

Renders its content only the first time it is encountered in a render pass. Subsequent encounters are silently skipped. Useful for deduplicating includes in complex layouts.

```atom
@once {
  <script src="/heavy-library.js"></script>
}
```

---

## 5. Component Tags — File-Based Tag Resolution

Any template file inside the `components/` directory of a disk can be referenced directly as a tag, without using `@use`. The filename becomes the tag name following these rules:

| File path | Tag name |
|---|---|
| `components/button.atom` | `@button()` |
| `components/form/input.atom` | `@form.input()` |
| `components/ui/modal_dialog.atom` | `@ui.modalDialog()` |
| `components/ui/modal/index.atom` | `@ui.modal()` |

```atom
{{-- Instead of: @use('components/button', { label: "Submit", type: "submit" }) --}}
@button({ label: "Submit", type: "submit" })

{{-- With a slot body --}}
@form.input({ name: "email", type: "email" }) {
  @fill(icon) {
    <i class="icon-email"></i>
  }
}

{{-- Self-closing --}}
@!ui.spinner({ size: "sm" })
```

This is the React mental model: components are the primary abstraction, and you just write them like HTML tags.

Named disk prefix: `@ui.button()` where `ui` is a registered disk name.

---

## 6. Expression Language

The expression language is a sandboxed subset of a JavaScript-like syntax. It supports everything needed for template logic without allowing arbitrary code execution.

### 6.1 Primitives

```atom
{{ "a string" }}
{{ 'also a string' }}
{{ `template literal with {{ name }}` }}
{{ 42 }}
{{ 3.14 }}
{{ true }}
{{ false }}
{{ null }}
```

### 6.2 Operators

```
Arithmetic:   +  -  *  /  %  **
Comparison:   ==  !=  <  >  <=  >=
Logical:      &&  ||  !
Null-coalesce: ??
Ternary:      condition ? then : else
String concat: +
```

### 6.3 Property Access and Optional Chaining

```atom
{{ user.name }}
{{ user.address.city }}
{{ user?.profile?.avatar }}     {{-- optional chaining --}}
{{ items[0] }}
{{ map["key"] }}
{{ config[dynamicKey] }}
```

### 6.4 Method Calls (Helpers on Values)

Values have built-in methods. These are the primary way helpers are applied in expressions.

```atom
{{ name.upper() }}
{{ items.len() }}
{{ price.round(2) }}
{{ tags.join(", ") }}
{{ text.truncate(100) }}
```

Method calls chain naturally: `{{ name.trim().lower().replace("-", "_") }}`

### 6.5 Global Helper Calls

Global helpers are called with the `h.` prefix (or the bare name for the most common ones):

```atom
{{ h.snakeCase(name) }}
{{ h.pluralize(count, "item") }}
{{ now() }}
{{ range(1, 10) }}
```

---

## 7. Built-in Helper Library

All helpers are available both as methods on values (when the receiver type matches) and as global `h.*` functions.

### 7.1 String Helpers

| Method / `h.*` | Description | Example |
|---|---|---|
| `.upper()` / `h.upper(s)` | UPPER CASE | `"hello".upper()` → `"HELLO"` |
| `.lower()` / `h.lower(s)` | lower case | `"HELLO".lower()` → `"hello"` |
| `.trim()` | Strip whitespace | `"  hi  ".trim()` → `"hi"` |
| `.trimStart()` / `.trimEnd()` | Trim one side | |
| `.capitalize()` | First letter upper | `"hello world".capitalize()` → `"Hello world"` |
| `.titleCase()` | Title Case | `"hello world".titleCase()` → `"Hello World"` |
| `.camelCase()` | camelCase | `"hello-world".camelCase()` → `"helloWorld"` |
| `.pascalCase()` | PascalCase | `"hello-world".pascalCase()` → `"HelloWorld"` |
| `.snakeCase()` | snake_case | `"HelloWorld".snakeCase()` → `"hello_world"` |
| `.kebabCase()` | kebab-case | `"HelloWorld".kebabCase()` → `"hello-world"` |
| `.screamingSnake()` | SCREAMING_SNAKE | `"helloWorld".screamingSnake()` → `"HELLO_WORLD"` |
| `.dotCase()` | dot.case | `"hello world".dotCase()` → `"hello.world"` |
| `.sentenceCase()` | Sentence case | `"helloWorld".sentenceCase()` → `"Hello world"` |
| `.replace(from, to)` | Replace all | `"a-b-c".replace("-", "_")` → `"a_b_c"` |
| `.replaceFirst(from, to)` | Replace first | |
| `.startsWith(s)` | Bool: starts with | |
| `.endsWith(s)` | Bool: ends with | |
| `.contains(s)` | Bool: contains | |
| `.padStart(n, char?)` | Pad left | `"7".padStart(3, "0")` → `"007"` |
| `.padEnd(n, char?)` | Pad right | |
| `.repeat(n)` | Repeat n times | `"ab".repeat(3)` → `"ababab"` |
| `.split(sep)` | Split to array | `"a,b".split(",")` → `["a","b"]` |
| `.chars()` | Split to chars | `"abc".chars()` → `["a","b","c"]` |
| `.lines()` | Split by newline | |
| `.len()` / `.length` | Character count | |
| `.isEmpty()` | Bool: zero length | |
| `.truncate(n, suffix?)` | Truncate to n chars | `"hello world".truncate(5)` → `"hello..."` |
| `.excerpt(n)` | Strip tags then truncate | |
| `.indent(n, char?)` | Indent every line | |
| `.dedent()` | Remove common indentation | |
| `.wrap(n)` | Word-wrap at n chars | |
| `h.pluralize(n, word, plural?)` | "1 item" / "2 items" | |
| `h.singularize(word)` | Remove English plural | |
| `h.slugify(s)` | URL-safe slug | `"Hello World!"` → `"hello-world"` |
| `h.escape(s)` | HTML escape | |
| `h.unescape(s)` | HTML unescape | |
| `h.encodeUri(s)` | URI encode | |
| `h.decodeUri(s)` | URI decode | |
| `h.base64Encode(s)` | Base64 encode | |
| `h.base64Decode(s)` | Base64 decode | |
| `h.sha256(s)` | SHA-256 hex | |
| `h.md5(s)` | MD5 hex | |
| `h.uuid()` | Random UUID v4 | |
| `h.random(n)` | Random alphanumeric string | |

### 7.2 Collection Helpers (Arrays / Lists)

| Method / `h.*` | Description | Example |
|---|---|---|
| `.len()` / `.length` | Count | `items.len()` → `3` |
| `.isEmpty()` | Bool: zero length | |
| `.first()` | First element or null | |
| `.last()` | Last element or null | |
| `.first(n)` | First n elements | |
| `.last(n)` | Last n elements | |
| `.nth(i)` | Element at index | |
| `.slice(start, end?)` | Subarray | |
| `.take(n)` | Alias for `.first(n)` | |
| `.skip(n)` | Drop first n | |
| `.includes(val)` | Bool: contains value | |
| `.indexOf(val)` | Index or -1 | |
| `.push(val)` | New array with val appended | (immutable) |
| `.prepend(val)` | New array with val prepended | (immutable) |
| `.concat(arr)` | Merge two arrays | |
| `.flatten()` | One level of nesting removed | |
| `.flatMap(fn)` | Map then flatten | |
| `.unique()` | Deduplicate (preserves order) | |
| `.uniqueBy(key)` | Deduplicate by field | |
| `.compact()` | Remove null/undefined | |
| `.sort()` | Sort (lexicographic) | |
| `.sortBy(key, dir?)` | Sort objects by field | `users.sortBy("name")` |
| `.reverse()` | Reversed array | |
| `.shuffle()` | Random order | |
| `.zip(arr)` | Zip two arrays into pairs | |
| `.chunk(n)` | Split into chunks of n | `[1,2,3,4].chunk(2)` → `[[1,2],[3,4]]` |
| `.groupBy(key)` | Group objects by field | → `{ "admin": [...], "user": [...] }` |
| `.countBy(key)` | Count by field value | |
| `.keyBy(key)` | Index by field → map | |
| `.join(sep)` | Join to string | |
| `.joinWith(sep, lastSep?)` | "a, b and c" | `arr.joinWith(", ", " and ")` |
| `.sum()` / `.sum(key)` | Numeric sum | `prices.sum()` |
| `.min()` / `.min(key)` | Minimum value | |
| `.max()` / `.max(key)` | Maximum value | |
| `.avg()` / `.avg(key)` | Average | |
| `.pluck(key)` | Extract field from each | `users.pluck("name")` → `["Alice","Bob"]` |
| `.filter(key, val?)` | Filter by field or truthy | `users.filter("active")` |
| `.reject(key, val?)` | Inverse of filter | |
| `.where(key, op, val)` | Filter with operator | `items.where("price", ">", 10)` |
| `.find(key, val)` | First matching element | |
| `.findIndex(key, val)` | Index of first match | |
| `.map(key)` | Alias for `.pluck(key)` | |
| `.pick(keys)` | Keep only listed keys | `user.pick(["name", "email"])` |
| `.omit(keys)` | Remove listed keys | |
| `h.range(start, end, step?)` | Numeric range array | `h.range(1, 5)` → `[1,2,3,4]` |
| `h.zip(a, b)` | Zip two arrays | |

### 7.3 Object / Map Helpers

| Method / `h.*` | Description |
|---|---|
| `.keys()` | Array of keys |
| `.values()` | Array of values |
| `.entries()` | Array of `[key, val]` pairs |
| `.has(key)` | Bool: key exists |
| `.get(key, default?)` | Value or default |
| `.set(key, val)` | New object with key set (immutable) |
| `.merge(obj)` | Shallow merge |
| `.deepMerge(obj)` | Deep merge |
| `.pick(keys)` | Keep only listed keys |
| `.omit(keys)` | Remove listed keys |
| `.len()` | Number of keys |
| `.isEmpty()` | Bool: no keys |
| `h.fromEntries(pairs)` | Build object from pairs |

### 7.4 Number Helpers

| Method / `h.*` | Description | Example |
|---|---|---|
| `.round(n?)` | Round to n decimals | `3.14159.round(2)` → `3.14` |
| `.floor()` | Floor | |
| `.ceil()` | Ceiling | |
| `.abs()` | Absolute value | |
| `.clamp(min, max)` | Clamp to range | |
| `.toFixed(n)` | Format with n decimal places | `3.1.toFixed(2)` → `"3.10"` |
| `.format(pattern?)` | Locale number format | `1234567.format()` → `"1,234,567"` |
| `.currency(symbol?)` | Currency format | `9.99.currency("$")` → `"$9.99"` |
| `.percent(decimals?)` | Percentage format | `0.42.percent()` → `"42%"` |
| `.isEven()` | Bool | |
| `.isOdd()` | Bool | |
| `.pad(n, char?)` | Zero-pad | `7.pad(3)` → `"007"` |
| `h.min(a, b)` | Minimum of two | |
| `h.max(a, b)` | Maximum of two | |
| `h.random(min, max)` | Random integer in range | |

### 7.5 Date / Time Helpers

| Method / `h.*` | Description | Example |
|---|---|---|
| `.format(pattern)` | Format date | `date.format("YYYY-MM-DD")` |
| `.toRelative()` | Relative time | `date.toRelative()` → `"3 days ago"` |
| `.toIso()` | ISO 8601 string | |
| `.toUnix()` | Unix timestamp (seconds) | |
| `.add(n, unit)` | Add duration | `date.add(7, "days")` |
| `.sub(n, unit)` | Subtract duration | |
| `.startOf(unit)` | Start of unit | `date.startOf("month")` |
| `.endOf(unit)` | End of unit | |
| `.isBefore(d)` | Bool | |
| `.isAfter(d)` | Bool | |
| `.isSame(d, unit?)` | Bool | |
| `.diff(d, unit)` | Difference | `a.diff(b, "days")` → `14` |
| `h.now()` | Current datetime | |
| `h.today()` | Current date (midnight) | |
| `h.parseDate(s, fmt?)` | Parse string to date | |

Date format tokens: `YYYY` year, `MM` month, `DD` day, `HH` hour (24h), `mm` minute, `ss` second.

### 7.6 Type / Value Helpers

| Helper | Description |
|---|---|
| `h.isDefined(x)` | True if not null or undefined |
| `h.isNull(x)` | True if null |
| `h.isEmpty(x)` | True if null, empty string, empty array, or empty object |
| `h.isString(x)` | Type check |
| `h.isNumber(x)` | Type check |
| `h.isBool(x)` | Type check |
| `h.isArray(x)` | Type check |
| `h.isObject(x)` | Type check |
| `h.default(x, fallback)` | x if defined, else fallback |
| `h.coalesce(a, b, c, ...)` | First defined value |
| `h.str(x)` | Coerce to string |
| `h.int(x)` | Coerce to integer |
| `h.float(x)` | Coerce to float |
| `h.bool(x)` | Coerce to boolean |
| `h.json(x)` | JSON stringify |
| `h.parseJson(s)` | JSON parse |
| `h.inspect(x)` | Pretty-print for debugging |
| `h.typeof(x)` | Type name as string |

---

## 8. Props System

### 8.1 Declaring Props

```atom
@props({
  title: String,
  items: Array,
  count: Number = 0,
  visible: Boolean = true,
  theme: String = "light",
  user: Object,
  callback: Function?,
})
```

Type names are: `String`, `Number`, `Boolean`, `Array`, `Object`, `Function`, `Any`. The `?` suffix makes a prop optional without a default (it will be `null` if not passed).

Props are always immutable. Any attempt to `@set` a prop name is a parse error.

### 8.2 `$props` — The Props Object

Inside a component template, `$props` provides programmatic access to props:

```atom
{{-- Access a prop by name --}}
{{ $props.get('theme') }}

{{-- Check if a prop was provided by the caller (not just using its default) --}}
@if($props.has('user')) {
  Welcome, {{ user.name }}
}

{{-- Serialize props to HTML attributes --}}
<div {{ $props.only(['class', 'id', 'style']).toAttrs() }}>

{{-- Merge and serialize --}}
<button {{ $props.merge({ class: ['btn', 'btn-primary'] }).toAttrs() }}>
  {{ label }}
</button>

{{-- All except listed keys --}}
<input {{ $props.except(['label', 'helpText']).toAttrs() }} />
```

### 8.3 `$attrs` — Passthrough Attributes

`$attrs` contains all props not declared in `@props`. This allows a component to receive and forward arbitrary HTML attributes:

```atom
@props({ label: String, disabled: Boolean = false })

<button {{ $attrs.toAttrs() }} @if(disabled) { disabled }>
  {{ label }}
</button>
```

---

## 9. Slots System

### 9.1 Declaring and Using Slots

```atom
{{-- components/dialog.atom --}}
@props({ open: Boolean = false })
@slots([title, content, actions?])

@if(open) {
  <dialog open>
    <header>
      @slot(title) { <span>Dialog</span> }
    </header>
    <section>
      @slot(content) {}
    </section>
    <footer>
      @slot(actions) {
        <button onclick="this.closest('dialog').close()">Close</button>
      }
    </footer>
  </dialog>
}
```

### 9.2 Filling Slots

```atom
@dialog({ open: showModal }) {
  @fill(title) {
    <strong>Confirm deletion</strong>
  }
  @fill(content) {
    <p>This action cannot be undone.</p>
  }
  @fill(actions) {
    <button class="btn-danger">Delete</button>
    <button class="btn-secondary">Cancel</button>
  }
}
```

### 9.3 Scoped Slots — Component-to-Caller Data Sharing

A slot can receive data from the component that declares it, exposing internal state to the caller:

```atom
{{-- components/list.atom --}}
@props({ items: Array })

<ul>
  @each(item in items) {
    <li>
      @slot(item, { item, index: $index, isLast: $last }) {
        {{ item.toString() }}
      }
    </li>
  }
</ul>
```

The caller binds the slot data with `as`:

```atom
@list({ items: products }) {
  @fill(item, as: row) {
    <span class="@if(row.isLast) { last }">{{ row.item.name }}</span>
  }
}
```

---

## 10. `@each` with Collections — Advanced Patterns

### Empty State

```atom
@each(user in users) {
  <li>{{ user.name }}</li>
} @empty {
  <li class="empty">No users found.</li>
}
```

### Destructuring

```atom
@each([key, value] in config.entries()) {
  {{ key }}: {{ value }}
}

@each({ name, email } in users) {
  {{ name }} <{{ email }}>
}
```

### Nested `@each` with Outer Loop Variables

```atom
@each(group in groups) {
  @let(outerIndex = $index)
  <h2>{{ group.name }}</h2>
  @each(member in group.members) {
    <p>Group {{ outerIndex }}, Member {{ $index }}: {{ member.name }}</p>
  }
}
```

---

## 11. Custom Directives API

Any consumer of Atom Engine can register custom directives. This is how 100% of the built-in block directives are implemented internally — the engine provides no privileged directives unavailable to user code.

### 11.1 Directive Contract

```rust
// atom-engine/src/directives/mod.rs

pub trait Directive: Send + Sync {
    /// Name of the directive (without @)
    fn name(&self) -> &str;

    /// Does this directive accept a block body `{ }`?
    fn is_block(&self) -> bool { false }

    /// Does this directive accept arguments `()`?
    fn is_seekable(&self) -> bool { true }

    /// Compile the directive into a sequence of Nodes.
    /// Called at parse time — not render time.
    fn compile(
        &self,
        args: Option<&ArgList>,
        body: Option<&[Node]>,
        ctx: &CompileContext,
    ) -> Result<Vec<Node>, CompileError>;
}
```

### 11.2 Example — Custom `@authorize` Directive

```rust
pub struct AuthorizeDirective;

impl Directive for AuthorizeDirective {
    fn name(&self) -> &str { "authorize" }
    fn is_block(&self) -> bool { true }
    fn is_seekable(&self) -> bool { true }

    fn compile(&self, args: Option<&ArgList>, body: Option<&[Node]>, ctx: &CompileContext)
        -> Result<Vec<Node>, CompileError>
    {
        let ability = args.required_string("ability")?;
        let resource = args.optional_expr("resource");

        // Wrap body in a conditional: only render if ctx.can(ability, resource)
        let condition = Expr::call("$can", vec![
            Expr::str(ability),
            resource.unwrap_or(Expr::null()),
        ]);

        Ok(vec![Node::If {
            condition,
            body: body.unwrap_or_default().to_vec(),
            else_body: vec![],
        }])
    }
}
```

Registration:

```rust
let mut engine = Engine::new();
engine.register_directive(AuthorizeDirective);
engine.register_helper("$can", |args, ctx| {
    let ability = args[0].as_str()?;
    let user = ctx.get("$user")?;
    Ok(Value::Bool(user.can(ability)))
});
```

Usage in templates:

```atom
@authorize("delete", post) {
  <button class="btn-danger">Delete Post</button>
}
```

### 11.3 Example — Custom `@markdown` Directive

```rust
pub struct MarkdownDirective;

impl Directive for MarkdownDirective {
    fn name(&self) -> &str { "markdown" }
    fn is_block(&self) -> bool { true }
    fn is_seekable(&self) -> bool { false }

    fn compile(&self, _args: Option<&ArgList>, body: Option<&[Node]>, ctx: &CompileContext)
        -> Result<Vec<Node>, CompileError>
    {
        // Capture body as raw text, then wrap in a render-time markdown transform
        let raw_body = ctx.nodes_to_raw_text(body.unwrap_or_default())?;
        Ok(vec![Node::RawTransform {
            content: raw_body,
            transform: "markdown",
        }])
    }
}
```

Usage:

```atom
@markdown {
  # Hello World

  This **Markdown** is rendered to HTML at render time.

  - Item 1
  - Item 2
}
```

### 11.4 Example — Self-Closing `@!svg` Directive

```rust
pub struct SvgDirective;

impl Directive for SvgDirective {
    fn name(&self) -> &str { "svg" }
    fn is_block(&self) -> bool { false }
    fn is_seekable(&self) -> bool { true }

    fn compile(&self, args: Option<&ArgList>, _body: Option<&[Node]>, _ctx: &CompileContext)
        -> Result<Vec<Node>, CompileError>
    {
        let icon_name = args.required_string("name")?;
        let size = args.optional_string("size").unwrap_or("24");
        Ok(vec![Node::RawTransform {
            content: format!("icons/{}.svg", icon_name),
            transform: "inline_svg",
        }])
    }
}
```

Usage:

```atom
@!svg("arrow-right")
@!svg("chevron", { size: "16" })
```

---

## 12. Engine Architecture

### 12.1 Processing Pipeline

```
Source string  (.atom file content)
       │
       ▼
   Lexer  (src/parser/lexer.rs)
       │  Produces: Vec<Token>
       │  Tokens: Text, OpenDirective, CloseDirective,
       │          OpenInterp, CloseInterp, OpenRawInterp,
       │          At, Ident, LParen, RParen, LBrace, RBrace,
       │          Comma, Dot, StringLit, NumLit, BoolLit,
       │          NullLit, Whitespace
       ▼
   Parser  (src/parser/parser.rs)
       │  Produces: Vec<Node>
       │  Validates: directive syntax, block matching, expr grammar
       │  Dispatches to registered directives for compile()
       ▼
   Template  { nodes: Vec<Node>, source_map: SourceMap }
       │  Stored in TemplateRegistry (Arc<Template>)
       ▼
   Renderer  (src/renderer/mod.rs)
       │  Receives: Arc<Template>, RenderContext
       │  Evaluates nodes against context
       │  Manages scope stack, slot buffers, stacks, provide/inject chain
       ▼
   String  (complete rendered output)
```

### 12.2 Node Types

```rust
pub enum Node {
    // Literal text — emitted verbatim
    Text(String),

    // {{ expr }} and {{{ expr }}}
    Interpolation { expr: Expr, raw: bool },

    // @directiveName(args) { body }
    Directive {
        name:  String,
        args:  Option<ArgList>,
        body:  Option<Vec<Node>>,
        span:  Span,
    },

    // @use('path', props) { body with @fill blocks }
    Component {
        path:    String,
        props:   Option<ArgList>,
        fills:   HashMap<String, Vec<Node>>,    // named slot content
        main:    Vec<Node>,                     // main slot content
        span:    Span,
    },

    // @slot(name?) { default }
    Slot {
        name:    Option<String>,
        default: Vec<Node>,
        scoped:  Option<(String, Expr)>,        // (binding_name, data_expr)
        span:    Span,
    },

    // @extends('path')
    Extends {
        path: String,
        span: Span,
    },

    // @section(name) { body }
    Section {
        name: String,
        body: Vec<Node>,
    },

    // @yield(name)
    Yield {
        name: String,
        span: Span,
    },

    // @include('path', data?)
    Include {
        path: String,
        data: Option<Expr>,
        span: Span,
    },

    // @push(stack) { body }
    Push {
        stack:   String,
        prepend: bool,
        body:    Vec<Node>,
    },

    // @stack(name)
    Stack {
        name: String,
        span: Span,
    },

    // Raw transform (e.g. markdown rendering, svg inlining)
    RawTransform {
        content:   String,
        transform: String,
    },
}
```

### 12.3 Render Context and Scope Stack

```rust
// src/renderer/scope.rs

pub struct Scope {
    frames: Vec<Frame>,
}

struct Frame {
    vars:       IndexMap<String, Value>,   // @let / @set bindings
    is_props:   bool,                      // true for the props frame — immutable
}

impl Scope {
    pub fn push_frame(&mut self);
    pub fn pop_frame(&mut self);
    pub fn push_props(&mut self, props: IndexMap<String, Value>);

    /// @let — declare in current frame. Error if name already exists in current frame.
    pub fn declare(&mut self, name: &str, value: Value) -> Result<(), RenderError>;

    /// @set — update nearest frame owning the name. Error if not found or is props frame.
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), RenderError>;

    /// Resolve name — walk frames from innermost to outermost.
    pub fn get(&self, name: &str) -> Option<&Value>;

    /// Inject loop variables ($index, $first, etc.) into the current frame.
    pub fn inject_loop_vars(&mut self, index: usize, total: usize);
}
```

### 12.4 Stack System

```rust
// src/renderer/stack_buffer.rs

pub struct StackBuffer {
    stacks: IndexMap<String, Vec<String>>,  // stack_name → Vec<rendered_content>
}

impl StackBuffer {
    pub fn push(&mut self, name: &str, content: String);
    pub fn prepend(&mut self, name: &str, content: String);
    pub fn drain(&mut self, name: &str) -> String;
    pub fn peek(&self, name: &str) -> &[String];
}
```

### 12.5 Provide/Inject Chain

```rust
// src/renderer/context_chain.rs

pub struct ContextChain {
    layers: Vec<IndexMap<String, Value>>,  // outermost first
}

impl ContextChain {
    pub fn provide(&mut self, key: &str, value: Value);
    pub fn inject(&self, key: &str) -> Option<&Value>;
    pub fn push_layer(&mut self);
    pub fn pop_layer(&mut self);
}
```

The `ContextChain` is separate from the `Scope`. Scope is for local variables (`@let`/`@set`). The chain is for cross-component values (`@provide`/`@inject`). A provided value is visible to all descendants in the render tree — it does not participate in scope resolution.

### 12.6 Public API

```rust
pub struct Engine {
    registry:   TemplateRegistry,
    helpers:    HelperRegistry,
    directives: DirectiveRegistry,
    transforms: TransformRegistry,    // "markdown", "inline_svg", etc.
    config:     EngineConfig,
}

pub struct EngineConfig {
    pub escape_html:   bool,          // default: false (opt-in)
    pub max_loop_iter: usize,         // default: 10_000
    pub debug:         bool,          // default: false (enables @dump)
    pub strict_props:  bool,          // default: true (error on missing required props)
}

impl Engine {
    pub fn new() -> Self;
    pub fn with_config(config: EngineConfig) -> Self;

    /// Load a directory of .atom files into the registry.
    pub fn add_disk(&mut self, name: &str, dir: &Path) -> Result<(), ParseError>;

    /// Load from embedded (filename, source) pairs.
    pub fn add_embedded(&mut self, name: &str, files: &[(&str, &str)]) -> Result<(), ParseError>;

    /// Register a custom directive.
    pub fn register_directive(&mut self, directive: impl Directive + 'static);

    /// Register a global helper function.
    pub fn register_helper<F>(&mut self, name: &str, f: F)
    where F: Fn(&[Value], &RenderCtx) -> Result<Value, RenderError> + Send + Sync + 'static;

    /// Register a raw transform (e.g. markdown → html).
    pub fn register_transform<F>(&mut self, name: &str, f: F)
    where F: Fn(&str) -> Result<String, RenderError> + Send + Sync + 'static;

    /// Register a domain type that implements IntoValue.
    pub fn register_type<T: IntoValue + 'static>(&mut self, name: &str);

    /// Render a template by path with given data.
    pub fn render(&self, path: &str, data: Value) -> Result<String, RenderError>;

    /// Render a raw template string directly.
    pub fn render_raw(&self, source: &str, data: Value) -> Result<String, RenderError>;

    /// Parse a template without rendering (for validation / precompilation).
    pub fn parse(&self, source: &str) -> Result<Template, ParseError>;
}
```

### 12.7 Crate Structure

```
crates/
  atom-engine/
    src/
      lib.rs                        # Public API: Engine, RenderError, Value
      parser/
        mod.rs
        lexer.rs                    # Source → Vec<Token> with spans
        parser.rs                   # Tokens → Vec<Node>, dispatches directives
        expr_parser.rs              # Expression grammar parser
        arg_list.rs                 # Parses (key: val, ...) argument lists
        source_map.rs               # Span → (file, line, col) mapping
      directives/
        mod.rs                      # DirectiveRegistry, Directive trait
        builtin/
          control.rs                # @if, @unless, @elseif, @else, @switch, @case
          loops.rs                  # @each, @for, @while
          variables.rs              # @let, @set, @const
          includes.rs               # @include, @includeWhen, @includeUnless,
                                    # @includeFirst, @includeEach
          layout.rs                 # @extends, @section, @yield
          components.rs             # @use, @slot, @fill, @provide, @inject
          stacks.rs                 # @push, @prepend, @stack
          utils.rs                  # @dump, @log, @comment, @raw, @once
      renderer/
        mod.rs                      # Renderer entry point
        scope.rs                    # Scope stack: @let / @set
        context_chain.rs            # Provide/inject chain
        stack_buffer.rs             # @push / @stack accumulator
        slot_resolver.rs            # Resolves @fill content for @slot outlets
        section_map.rs              # @section / @yield for layout inheritance
        component_renderer.rs       # Handles @use / component tag rendering
        layout_renderer.rs          # Handles @extends layout rendering
        output.rs                   # OutputBuffer: collects rendered text
        expr_eval.rs                # Evaluates Expr nodes → Value
        helper_eval.rs              # Dispatches method calls and h.* calls
      helpers/
        mod.rs                      # HelperRegistry
        strings.rs                  # All string helpers
        collections.rs              # Array + object helpers
        numbers.rs                  # Number helpers
        dates.rs                    # Date/time helpers
        types.rs                    # Type/value helpers
        encoding.rs                 # base64, sha256, md5, uuid, uri
      types/
        mod.rs
        value.rs                    # Value enum: Str, Num, Bool, Array, Object, Null, Date
        coerce.rs                   # IntoValue, FromValue
        props.rs                    # Props struct, $props/$attrs API
      error.rs                      # ParseError, RenderError with span + call trace
    tests/
      directives/                   # One test file per directive
      helpers/                      # One test file per helper group
      components/                   # Slot, fill, scoped slot tests
      layouts/                      # @extends, @yield tests
      stacks/                       # @push, @prepend, @stack tests
      integration/                  # Full template render snapshot tests (insta)
    Cargo.toml
```

---

## 13. Error Model

Every error carries a source span (file, line, column range), a structured kind, and an optional hint.

### 13.1 Parse Errors

```rust
pub struct ParseError {
    pub file:  String,
    pub span:  Span,
    pub kind:  ParseErrorKind,
    pub hint:  Option<String>,
}

pub enum ParseErrorKind {
    UnknownDirective        { name: String, similar: Vec<String> },
    UnclosedBlock           { directive: String, opened_at: Span },
    UnexpectedToken         { expected: String, found: String },
    ElseifWithoutIf,
    ElseWithoutIf,
    FillOutsideComponent,
    SlotOutsideComponent,
    SetTargetingProp        { name: String },
    ConstReassignment       { name: String },
    DuplicatePropDeclaration{ name: String },
    MissingRequiredArg      { directive: String, arg: String },
    ExprError               { message: String },
    InvalidSlotName         { name: String },
    MultipleExtends,
    ExtendsNotFirst,
}
```

**Example:**

```
error[P007]: unclosed block directive
  --> templates/pages/home.atom:42:1
   |
42 | @each(item in items) {
   | ^^^^^^^^^^^^^^^^^^^^^ `@each` block opened here
   |
   = hint: add a closing `}` before end of file
```

### 13.2 Render Errors

```rust
pub struct RenderError {
    pub file:  String,
    pub span:  Span,
    pub kind:  RenderErrorKind,
    pub trace: Vec<CallFrame>,     // component call stack
    pub hint:  Option<String>,
}

pub enum RenderErrorKind {
    MissingRequiredProp     { name: String, template: String },
    PropTypeMismatch        { name: String, expected: String, found: String },
    UndefinedVariable       { name: String },
    SetUndeclaredVariable   { name: String },
    UnknownTemplate         { path: String, disk: Option<String> },
    MaxLoopIterationsExceeded { limit: usize },
    HelperError             { name: String, message: String },
    TransformError          { name: String, message: String },
    DivisionByZero,
    InjectWithoutProvide    { key: String },
    RequiredSlotUnfilled    { name: String, component: String },
    CircularInclude         { a: String, b: String },
}
```

**Example with component trace:**

```
error[R003]: missing required prop `items`
  --> templates/components/list.atom:1:1
   |
 1 | @props({ items: Array })
   |          ^^^^^ prop `items` is required but was not provided
   |
   = hint: pass `items` when calling this component

component call trace:
  templates/pages/home.atom:15       @use('components/sidebar')
  templates/components/sidebar.atom:8  @!use('components/list')
  templates/components/list.atom:1   ← error here
```

---

## 14. Cargo.toml

```toml
[package]
name    = "atom-engine"
version = "4.0.0"
edition = "2021"

[dependencies]
# Error handling
anyhow    = "1"
thiserror = "1"

# Ordered maps (deterministic rendering)
indexmap = "2"

# Case conversion helpers
heck = "0.5"

# Date handling
chrono = { version = "0.4", features = ["serde"] }

# YAML front-matter / config
serde      = { version = "1", features = ["derive"] }
serde_json = "1"   # Value type JSON interop

# String utilities
unicode-segmentation = "1"  # correct Unicode char/grapheme handling
regex                = "1"  # h.matches(), h.replace() regex variants

# Encoding
base64 = "0.22"
sha2   = "0.10"
md-5   = "0.10"
uuid   = { version = "1", features = ["v4"] }

# Fuzzy matching for directive name suggestions in errors
strsim = "0.11"

# Optional parallel rendering of independent subtrees
rayon = { version = "1", optional = true }

[features]
default  = []
parallel = ["dep:rayon"]
html     = []              # Enables HTML escaping by default + html.* helpers

[dev-dependencies]
insta    = "1"     # snapshot testing
tempfile = "3"
```

---

## 15. Full Template Example — HTML Page

A realistic HTML page template showing the full system working together.

```atom
{{-- pages/post.atom --}}

@extends('layouts/app')

@push(styles) {
  <link rel="stylesheet" href="/css/post.css">
}

@push(scripts) {
  <script type="module" src="/js/comments.js"></script>
}

@section(title) {
  {{ post.title }} — {{ site.name }}
}

@section(body) {
  <article class="post">
    <header class="post-header">
      <h1>{{ post.title }}</h1>

      <div class="post-meta">
        <span>By {{ post.author.name }}</span>
        <time datetime="{{ post.publishedAt.toIso() }}">
          {{ post.publishedAt.format("MMMM D, YYYY") }}
        </time>
        <span>{{ h.pluralize(post.readingTime, "min read") }}</span>
      </div>

      @if(post.tags.len() > 0) {
        <div class="tags">
          @each(tag in post.tags) {
            <a href="/tags/{{ tag.slug }}" class="tag">{{ tag.name }}</a>
          }
        </div>
      }
    </header>

    <div class="post-body">
      @markdown {
        {{ post.content }}
      }
    </div>

    <footer class="post-footer">
      @includeWhen(post.relatedPosts.len() > 0, 'partials/related-posts', {
        posts: post.relatedPosts.take(3)
      })
    </footer>
  </article>

  <section class="comments" id="comments">
    <h2>{{ h.pluralize(post.comments.len(), "Comment") }}</h2>

    @each(comment in post.comments) {
      @!use('components/comment', {
        comment,
        level: 0,
        canReply: user?.isLoggedIn ?? false,
      })
    } @empty {
      <p class="no-comments">No comments yet. Be the first!</p>
    }

    @authorize("create", "comment") {
      @use('components/comment-form', { postId: post.id }) {}
    }
  </section>
}
```

---

## 16. Full Template Example — TypeScript Code Generation

The same engine, generating TypeScript:

```atom
{{-- codegen/drizzle-schema.atom --}}

@props({
  name: String,
  fields: Array,
  timestamps: Boolean = true,
  softDelete: Boolean = false,
})

@let(tableName = name.snakeCase())
@let(typeName  = name.pascalCase())
@let(cols = fields.filter(f => f.type != "relation"))
@let(rels = fields.filter(f => f.type == "relation"))

import { sqliteTable, text, integer, real, numeric } from 'drizzle-orm/sqlite-core'
@each(rel in rels) {
import { {{ rel.references.pascalCase() }} } from './{{ rel.references.snakeCase() }}'
}

export const {{ tableName }} = sqliteTable('{{ tableName }}', {
  id: text('id').primaryKey().$defaultFn(() => crypto.randomUUID()),

  @each(field in cols) {
  @!use('codegen/fragments/drizzle-column', { field })
  }

  @if(timestamps) {
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull().$defaultFn(() => new Date()),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull().$defaultFn(() => new Date()),
  }

  @if(softDelete) {
  deletedAt: integer('deleted_at', { mode: 'timestamp' }),
  }
})

@each(rel in rels) {
export const {{ tableName }}Relations = relations({{ tableName }}, ({ one }) => ({
  {{ rel.name }}: one({{ rel.references.pascalCase() }}, {
    fields: [{{ tableName }}.{{ rel.name }}Id],
    references: [{{ rel.references.snakeCase() }}.id],
  }),
}))
}

export type {{ typeName }}         = typeof {{ tableName }}.$inferSelect
export type New{{ typeName }}      = typeof {{ tableName }}.$inferInsert
export type Update{{ typeName }}   = Partial<New{{ typeName }}>
```

```atom
{{-- codegen/fragments/drizzle-column.atom --}}
@props({ field: Object })

@let(col = field.name)
@let(q   = h.json(col))
@let(nn  = field.required ? ".notNull()" : "")
@let(unq = field.unique   ? ".unique()"  : "")

@switch(field.type) {
  @case("string")  { {{ col }}: text({{ q }}){{ nn }}{{ unq }}, }
  @case("number")  { {{ col }}: real({{ q }}){{ nn }}, }
  @case("boolean") { {{ col }}: integer({{ q }}, { mode: 'boolean' }).notNull().default(false), }
  @case("date")    { {{ col }}: integer({{ q }}, { mode: 'timestamp' }){{ nn }}, }
  @case("decimal") { {{ col }}: numeric({{ q }}){{ nn }}, }
  @case("id")      { {{ col }}: text({{ q }}).references(() => {{ field.references.snakeCase() }}.id){{ nn }}, }
  @case("json")    { {{ col }}: text({{ q }}, { mode: 'json' }){{ nn }}, }
  @case("enum") {
    @let(vals = field.values.map(v => h.json(v)).join(", "))
    {{ col }}: text({{ q }}, { enum: [{{ vals }}] }){{ nn }},
  }
  @default { // unknown type: {{ field.type }} }
}
```

---

## 17. Implementation Plan

### Phase 1 — Lexer and Parser (Week 1)

All lexer tokens, all expression grammar, all node types. Every built-in directive defined as a struct implementing `Directive`. Parse-time validation for all block/arg combinations. 100% unit test coverage on lexer and parser. All parse error kinds have dedicated test cases.

### Phase 2 — Expression Evaluator and Scope (Week 1–2)

`Scope` with `push_frame`/`pop_frame`/`declare`/`assign`/`resolve`. Expression evaluator covering all node types. Method call dispatch to helper registry. `$index`, `$number`, `$first`, `$last`, `$even`, `$odd`, `$total`, `$key` loop variable injection. Props frame immutability enforcement.

### Phase 3 — Built-in Helpers (Week 2)

All helpers from §7 implemented and unit-tested. Every helper tested with: normal input, edge cases (empty string, empty array, null, zero), type mismatch errors.

### Phase 4 — Component and Layout System (Week 2–3)

`@use`, `@slot`, `@fill`, scoped slots, `@provide`/`@inject`, `@extends`, `@section`/`@yield`. Template registry with disk support. Component tag resolution from filename. Slot resolver and section map. Circular include detection. Integration snapshot tests for each component feature.

### Phase 5 — Stack System and Utilities (Week 3)

`@push`, `@prepend`, `@stack`, `@once`, `@raw`, `@dump`, `@log`, `@comment`. Stack buffer per render pass. `@once` deduplication via a `HashSet<u64>` keying on block hash.

### Phase 6 — Custom Directive API and Transforms (Week 3–4)

`Directive` trait, `DirectiveRegistry`, `CompileContext` API. `TransformRegistry` for `@markdown`, `@raw`, inline SVG. Reference implementations: `AuthorizeDirective`, `MarkdownDirective`, `SvgDirective`. Full integration test suite.

### Phase 7 — Error Model and Polish (Week 4)

Structured `ParseError` and `RenderError` with spans and traces. Fuzzy matching for `UnknownDirective` suggestions. `@dump` output formatter. Documentation, examples, and crates.io publication.

---

## 18. Conclusion

Atom Engine v4 is a **pure template engine**. It has one job — render a template with data — and it does that job for any file format, any content type, and any application domain.

The `@directive{}` system gives template authors a consistent, readable syntax for every control structure. Components with props, slots, and provide/inject give templates the same composability as React components without any of the runtime overhead. The helper library covers every common transformation need — strings, collections, numbers, dates, encoding, types — so template authors rarely need to push logic into the application layer.

The custom directive API ensures the engine is open for extension without modification. Domain-specific directives like `@authorize`, `@markdown`, or `@svg` are first-class citizens implemented with the same API as every built-in directive.

The whole system compiles to a single Rust binary. No Node.js runtime, no virtual machine, no garbage collector. Templates are parsed to an AST once and rendered many times. The engine is `Send + Sync` — it can render concurrently across threads without any shared mutable state.

This is the template engine that powers TSX code generation — and it is general enough to power anything else.
