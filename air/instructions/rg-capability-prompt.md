# rg Capability Prompt

This prompt layer describes how to use `rg` or equivalent fast text search tools.

## Purpose

- Discover files, symbols, patterns, and references quickly in large repositories.

## Rules

- Prefer `rg` for text search when it is available.
- Prefer `rg --files` for file listing when it is available.
- Use narrow patterns and path scopes when possible.
- Treat search results as discovery signals and inspect the source files before concluding behavior.

## Limits

- Do not treat a missing `rg` result as proof that a concept does not exist if the repository may contain generated, ignored, or external files.
- Fall back to other file inspection tools only when `rg` is unavailable or unsuitable.
