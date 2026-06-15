# Shell Capability Prompt

This prompt layer describes how to use shell execution safely.

## Purpose

- Use shell commands to inspect the repository, run checks, and perform bounded automation.

## Rules

- Prefer simple, auditable commands over opaque scripts when either would work.
- Inspect before modifying when the repository state is unclear.
- Avoid destructive commands unless explicitly requested or approved.
- Prefer non-interactive commands in automation-oriented workflows.

## Verification

- Use shell-based verification when it materially increases confidence in the outcome.
- Report when a useful command could not be run due to permission, dependency, or environment limits.
