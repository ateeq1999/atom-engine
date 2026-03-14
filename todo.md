# Atom Engine v5 — Implementation Todo

> Rust crate: `atom-engine`. Template engine built on Tera with component system.  
> Work top-to-bottom. Each task is discrete and testable. Check the box when done.

---

## Phase 1 — Project Bootstrap (DONE)

- [x] `cargo new atom-engine --lib` — library crate, not binary
- [x] Set `edition = "2021"` in `Cargo.toml`
- [x] Add all `[dependencies]`: `tera`, `anyhow`, `thiserror`, `indexmap`, `heck`, `chrono`, `serde`, `serde_json`, `glob`
- [x] Add `[dev-dependencies]`: `insta`, `tempfile`
- [x] Run `cargo build` — confirm zero errors

---

## Phase 2 — Core Engine Structure (DONE)

### 2.1 Atom Engine Wrapper (`src/lib.rs`)

- [x] Create `Atom` struct wrapping Tera
- [x] Implement `Atom::new()` - initialize Tera with built-in filters
- [x] Implement `load_templates(glob)` - load templates from filesystem
- [x] Implement `add_template(name, content)` - add raw template
- [x] Implement `render(template, context)` - render template with data

### 2.2 Component System (DONE)

- [x] Create `ComponentRegistry` for managing components
- [x] Implement `Component` struct with props, template, slots
- [x] Implement props parsing from `{%-- atom: @props({...}) --%}` headers
- [x] Implement slot parsing from `slot_name()` patterns
- [x] Implement file-based component tag resolution

### 2.3 Context Chain (Provide/Inject) (DONE)

- [x] Create `ContextChain` for React-style context
- [x] Implement `provide(key, value)` - make value available to descendants
- [x] Implement `inject(key)` - read provided value
- [x] Implement layer management (push/pop)

### 2.4 Error Types (DONE)

- [x] Define error enum with variants: TemplateLoad, TemplateParse, Render, Context, Component, Slot, Props, Filter, Function

---

## Phase 3 — Built-in Filters (DONE)

### 3.1 String Filters

- [x] `upper` - uppercase
- [x] `lower` - lowercase  
- [x] `capitalize` - first letter uppercase
- [x] `title` - Title Case
- [x] `camel_case` - camelCase
- [x] `pascal_case` - PascalCase
- [x] `snake_case` - snake_case
- [x] `kebab_case` - kebab-case
- [x] `truncate(length, end?)` - truncate string
- [x] `slugify` - URL-safe slug
- [x] `pluralize(n, singular?, plural?)` - pluralization

### 3.2 Collection Filters

- [x] `first` - first element
- [x] `last` - last element
- [x] `length` - count elements
- [x] `reverse` - reverse array/string
- [x] `sort` - sort array
- [x] `group_by(attribute)` - group by field
- [x] `where(attribute, value?)` - filter array
- [x] `pluck(attribute)` - extract field

### 3.3 Number Filters

- [x] `round(precision?)` - round number
- [x] `abs` - absolute value
- [x] `format(format?)` - number formatting

### 3.4 Date Filters

- [x] `date(format?)` - format date

### 3.5 HTML Filters

- [x] `escape_html` - escape HTML characters
- [x] `safe` - mark as safe (no escaping)

### 3.6 Global Functions

- [x] `dump(...)` - debug output to stderr
- [x] `log(...)` - log to stderr
- [x] `range(start?, end, step_by?)` - generate range
- [x] `now(utc?, timestamp?)` - current datetime

---

## Phase 4 — Component Features (DONE)

### 4.1 Props System (DONE)

- [x] Define `PropType` enum (String, Number, Boolean, Array, Object, Any)
- [x] Implement prop validation at render time
- [x] Implement `$props` access in components
- [x] Implement `$attrs` for passthrough attributes

### 4.2 Slot System (DONE)

- [x] Implement slot resolution at render time
- [x] Implement named slots (`@slot(name)`)
- [x] Implement default slot content
- [x] Implement scoped slots (component passes data to caller)

### 4.3 Component Directives (DONE)

- [x] Implement `@component(path, props)` usage
- [x] Implement `@slot(name)` definition
- [x] Implement `@fill(name)` filling
- [x] Implement `@provide(key, value)` context
- [x] Implement `@inject(key)` context access

---

## Phase 5 — Enhanced Control Flow (DONE)

### 5.1 Custom Tera Extensions (DONE)

- [x] Register custom Tera filters for enhanced control flow
- [x] Implement `@each` with destructuring support (via Tera native)
- [x] Implement conditional filters (`when`, `default`, `coalesce`, `defined`, `undefined`, `empty`, `not_empty`)

### 5.2 Stack System (DONE)

- [x] Implement `@push(stack_name)` - push to stack (via `push()` function)
- [x] Implement `@prepend(stack_name)` - prepend to stack (via `prepend()` function)
- [x] Implement `stack(stack_name)` - render stack (via `stack` filter)

---

## Phase 6 — Template Features (DONE)

### 6.1 Layout Inheritance

- [x] Support Tera's native `extends`/`block` system
- [x] Document usage patterns

### 6.2 Includes

- [x] Support Tera's native `include` directive
- [x] Support `includeIf`, `includeFirst`

### 6.3 Macros

- [x] Support Tera's native macro system
- [x] Document usage patterns

---

## Phase 7 — Testing (DONE)

### 7.1 Unit Tests

- [x] Test all filters with various inputs
- [x] Test edge cases (empty arrays, null values, etc.)
- [x] Test component registration and rendering

### 7.2 Integration Tests

- [x] Test component with slots
- [x] Test component with props
- [x] Test layout inheritance
- [x] Test include directives
- [x] Test error handling

### 7.3 Example Templates

- [x] Create `examples/basic.rs` - simple template usage
- [x] Create `examples/components.rs` - component usage
- [x] Create `examples/layouts.rs` - layout inheritance

---

## Phase 8 — Documentation

### 8.1 API Documentation

- [x] Add doc comments to all public types
- [x] Add `README.md` with quickstart
- [x] Document filter reference
- [x] Document component system

### 8.2 Code Quality

- [x] Run `cargo clippy -- -D warnings` - zero warnings
- [x] Run `cargo fmt --check` - fully formatted
- [x] Run `cargo test` - all tests pass (16/16)

---

## Checklist Summary

| Phase | Description | Tasks | Done |
|-------|-------------|-------|------|
| 1 | Project Bootstrap | 5 | 5/5 |
| 2 | Core Engine Structure | 13 | 13/13 |
| 3 | Built-in Filters | 26 | 26/26 |
| 4 | Component Features | 14 | 14/14 |
| 5 | Enhanced Control Flow | 7 | 7/7 |
| 6 | Template Features | 6 | 6/6 |
| 7 | Testing | 12 | 12/12 |
| 8 | Documentation | 5 | 5/5 |
| **Total** | | **88** | **88/88** |

---

## Key Differences from v4

v5 builds on Tera instead of a custom parser:

- **Tera handles**: Lexing, parsing, expression evaluation, template caching, inheritance
- **Atom adds**: Component system with props/slots, provide/inject context, enhanced filters

This dramatically reduces implementation complexity while adding powerful component features.
