---
name: ts2wasm-fixtures-workflow
description: "Use when adding, editing, moving, or renaming fixtures under fixtures/ in ts2wasm. Trigger phrases: add fixture, edit fixture, rename fixture, migrate fixtures, fixture path update."
---

# ts2wasm Fixtures Workflow

Use this skill only for fixtures/ changes.

## Scope

- fixtures/** TypeScript inputs for project-owned smoke/differential tests
- fixture directory naming and fixture migration
- fixture reference synchronization across tests/docs/scripts

Do not use this skill for editing scripts logic itself (use scripts workflow).

## Naming Rules

1. Use semantic, domain-based directory names.
2. Use kebab-case for folder/file names.
3. Avoid milestone-style opaque names such as m1/m2.
4. Keep each fixture focused on a single behavior.

## Required Reference Update Pass

When fixture paths change, update all in one change:

1. crates/cli/tests/**
2. scripts/** that compile/run fixtures
3. docs/** that list internal smoke fixtures
4. TestRecord suite strings and related metadata

Recommended searches:
- `fixtures/<old-path>`
- `"<old-dir>/"` for helper functions joining `fixtures/`

## Validation

- cargo fmt --all --check
- cargo nextest run (impacted tests at minimum)
- Explicitly run fixture-heavy suites when paths moved

## Common Traps

- Directory rename done but suite string left old
- Tests updated but docs or scripts not updated
- Partial migration leaving mixed old/new naming

## Output Checklist

1. Added/renamed fixture paths
2. Updated reference files
3. Validation commands and results
4. Intentional non-updated areas and reason
