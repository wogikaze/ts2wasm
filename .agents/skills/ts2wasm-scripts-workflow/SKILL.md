---
name: ts2wasm-scripts-workflow
description: "Use when adding or editing scripts under scripts/ in ts2wasm. Trigger phrases: add script, edit script, update script, benchmark script, coverage script, regression gate script."
---

# ts2wasm Scripts Workflow

Use this skill only for scripts/ changes.

## Scope

- scripts/*.sh maintenance
- usage comment updates
- script reliability and reproducibility improvements

Do not use this skill for fixture content/path changes.

## Rules

1. Prefer POSIX-safe shell unless file explicitly depends on bash features.
2. Keep `set -e` (or stricter existing options) intact.
3. Quote paths and variables.
4. Keep output stable for CI/parsers.
5. If behavior changes, update the usage header in the same file.

## Validation

- cargo fmt --all --check
- Run the touched script with a representative command
- cargo nextest run (full or impacted tests)

## Common Traps

- Script updates that silently change output format
- Hardcoded absolute paths
- Changing fixture paths inside scripts without coordinating fixture workflow

## Output Checklist

1. Which script files changed
2. Behavior delta (before/after)
3. Commands run for validation
4. Remaining risks
