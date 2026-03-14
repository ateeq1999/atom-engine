# Atom Engine v5 - Comprehensive Test Plan

## Overview
Test all Atom Engine v5 features using template files in `examples/templates/` directory.

## Test Categories

### 1. Core Template Features
- [ ] Variable interpolation `{{ variable }}`
- [ ] Comments `{# comment #}`
- [ ] Whitespace control `-` modifier

### 2. Filters

#### String Filters
- [ ] `upper` - uppercase
- [ ] `lower` - lowercase
- [ ] `capitalize` - first letter uppercase
- [ ] `title` - Title Case
- [ ] `camel_case` - camelCase
- [ ] `pascal_case` - PascalCase
- [ ] `snake_case` - snake_case
- [ ] `kebab_case` - kebab-case
- [ ] `truncate(length, end?)` - truncate string
- [ ] `slugify` - URL-safe slug

#### Collection Filters
- [ ] `first` - first element
- [ ] `last` - last element
- [ ] `length` - count elements
- [ ] `reverse` - reverse array/string
- [ ] `sort` - sort array
- [ ] `group_by(attribute)` - group by field
- [ ] `where(attribute, value?)` - filter array
- [ ] `pluck(attribute)` - extract field

#### Number Filters
- [ ] `round(precision?)` - round number
- [ ] `abs` - absolute value
- [ ] `format(format?)` - number formatting

#### Date Filters
- [ ] `date(format?)` - format date

#### HTML Filters
- [ ] `escape_html` - escape HTML characters
- [ ] `safe` - mark as safe (no escaping)

#### Conditional Filters
- [ ] `default(value)` - default value for null/empty
- [ ] `when(then, else)` - ternary conditional
- [ ] `coalesce(value, other)` - first non-null
- [ ] `defined` - check if defined
- [ ] `undefined` - check if undefined
- [ ] `empty` - check if empty
- [ ] `not_empty` - check if not empty

### 3. Control Flow

#### Conditionals
- [ ] `{% if %}` - basic if
- [ ] `{% elif %}` - else if
- [ ] `{% else %}` - else
- [ ] Boolean operators: `and`, `or`, `not`
- [ ] Comparison operators: `==`, `!=`, `>`, `<`, `>=`, `<=`

#### Loops
- [ ] `{% for item in items %}` - basic for loop
- [ ] `{% for i in range(end) %}` - range loop
- [ ] `{% for i in range(start, end, step_by) %}` - range with step
- [ ] `loop.index` - current iteration (1-based)
- [ ] `loop.index0` - current iteration (0-based)
- [ ] `loop.first` - first iteration
- [ ] `loop.last` - last iteration
- [ ] `loop.length` - total length
- [ ] `loop.revindex` - reverse index (1-based)
- [ ] `{% empty %}` - empty loop handling
- [ ] Nested loops

### 4. Template Inheritance

- [ ] `{% extends "base.html" %}` - extends parent
- [ ] `{% block name %}{% endblock %}` - define block
- [ ] `{{ super() }}` - render parent block content

### 5. Includes

- [ ] `{% include "template.html" %}` - include template
- [ ] `{% includeIf "template.html" %}` - conditional include
- [ ] `{% includeFirst ["a.html", "b.html"] %}` - try multiple

### 6. Macros

- [ ] `{% macro name(args) %}{% endmacro %}` - define macro
- [ ] `{{ macro_name(args) }}` - call macro
- [ ] Macro with default arguments
- [ ] Macro with nested content

### 7. Components (via register_component)

- [ ] Register component with `register_component(path, template)`
- [ ] Component with props
- [ ] Component with `$slots.default()`
- [ ] Component with named slots

### 8. Context (provide/inject)

- [ ] `engine.provide(key, value)` - provide value
- [ ] Access provided values in template
- [ ] Nested context override

### 9. Global Functions

- [ ] `dump(...)` - debug output
- [ ] `log(...)` - log to stderr
- [ ] `range(start?, end, step_by?)` - generate range
- [ ] `now(utc?, timestamp?)` - current datetime
- [ ] `push(stack, content)` - push to stack
- [ ] `prepend(stack, content)` - prepend to stack
- [ ] `set_slot(name, content)` - set slot
- [ ] `once(key)` - render once

### 10. Stack System

- [ ] `{% call push("name") %}{% endcall %}` - push to stack
- [ ] `{% call prepend("name") %}{% endcall %}` - prepend to stack
- [ ] `{{ "name" | stack }}` - render stack

### 11. File-based Templates

- [ ] Load templates from filesystem with glob pattern
- [ ] Templates in subdirectories
- [ ] Extends with file-based templates
- [ ] Include with file-based templates

### 12. Error Handling

- [ ] Missing template
- [ ] Missing variable
- [ ] Invalid filter arguments
- [ ] Invalid loop variable

## Template Files to Create

1. `test_variables.html` - Variable interpolation, comments
2. `test_filters_string.html` - All string filters
3. `test_filters_collection.html` - All collection filters
4. `test_filters_number.html` - Number and date filters
5. `test_filters_conditional.html` - Conditional filters
6. `test_conditionals.html` - If/elif/else, operators
7. `test_loops.html` - For loops, loop variables
8. `test_inheritance.html` - Extends, block, super
9. `test_includes.html` - Include, includeIf, includeFirst
10. `test_macros.html` - Macro definition and usage
11. `test_context.html` - Provide/inject
12. `test_stacks.html` - Push, prepend, stack
13. `test_full.html` - Full page demonstrating multiple features

## Test Execution

```bash
# Load all templates
engine.load_templates("examples/templates/test_*.html")

# Run each test
engine.render("test_variables.html", &context)
engine.render("test_filters_string.html", &context)
# ... etc
```

## Expected Output

Each template should render without errors and produce expected output based on test data provided in context.
