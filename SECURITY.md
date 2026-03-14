# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 5.0.x   | :white_check_mark: |
| 4.0.x   | :x:                |
| < 4.0   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability within Atom Engine, please send an email to the maintainers. All security vulnerabilities will be promptly addressed.

Please include the following information:

- Type of vulnerability
- Full paths of source file(s) related to the vulnerability
- Location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

## Security Best Practices

When using Atom Engine in your applications:

1. **Template Input**: Always validate and sanitize user input before passing to templates
2. **Context Data**: Don't expose sensitive data in template context unless necessary
3. **Component Registration**: Only register components from trusted sources
4. **File Access**: Be cautious when loading templates from filesystem

## Dependencies

We aim to keep dependencies up-to-date and monitor for security advisories:

- [crates.io advisories](https://crates.io/advisories)
- [RustSec Advisory Database](https://github.com/rustsec/advisory-db)

## Scope

This security policy applies to:
- The core atom-engine library
- All official examples
- Documentation

It does NOT apply to:
- User-created templates
- Third-party components
- Applications built with Atom Engine
