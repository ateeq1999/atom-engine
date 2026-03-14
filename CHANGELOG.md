# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [5.0.0] - 2024-01-15

### Added
- Initial release of Atom Engine v5
- Built on Tera template engine
- Component system with props, slots, and validation
- Provide/Inject context (React-style)
- Stack system for content accumulation
- 50+ built-in filters:
  - String filters (upper, lower, capitalize, truncate, slugify, etc.)
  - Collection filters (first, last, length, sort, group_by, where, etc.)
  - Number filters (round, abs, format, min, max, sum, avg, etc.)
  - Date filters
  - HTML filters (escape_html, safe, strip_tags)
  - Encoding filters (json_decode, urlescape, base64_encode, etc.)
  - Conditional filters (default, when, coalesce, defined, etc.)
- Helper directives (@map, @filter, @each, @reduce, @flatten, @partition)
- Template inheritance (extends, block, super)
- Include directives (include, includeIf, includeFirst)
- Macro support
- Global functions:
  - dump, log, range, now
  - uuid, random, choice
  - file_exists, env, md5, sha256
  - repeat, times, loop, iterate
  - object, merge, chunk, zip, compact
- Scoped slots support
- Component caching
- Async rendering support (with tokio)
- Parallel rendering support (with rayon)
- Memory pool utilities

### Features
- Full template inheritance system
- Component registration and rendering
- Context-based data passing
- Stack-based content collection
- Comprehensive filter pipeline support

### Dependencies
- tera (template engine)
- serde, serde_json (serialization)
- chrono (dates)
- rand (random numbers)
- uuid (UUID generation)
- Various encoding libraries

### Optional Features
- `parallel` - Enable parallel rendering with Rayon
- `async` - Enable async rendering with Tokio
- `pool-alloc` - Enable memory pooling

## [4.0.0] - Previous Version

See v4 for the legacy version with custom parser (deprecated in favor of v5).
