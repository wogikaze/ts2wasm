---
id: 3579
title: "Implement Nodenextesmimportsofpackageswithextensionlessmains"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5402]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nodeNextEsmImportsOfPackagesWithExtensionlessMains across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows the representative path currently stops while parsing a
virtual `node_modules/@types/ip/package.json` section as TypeScript source.

Problem: package `main` / `types` resolution is not actionable until issue
`5402` skips or stores virtual package.json sections as metadata.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nodeNextEsmImportsOfPackagesWithExtensionlessMains.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextEsmImportsOfPackagesWithExtensionlessMains.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by issue `5402`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue `5402`
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing issue `5402` owns the current package.json section parse blocker
- [x] Closed bucket includes failing path, diagnostic code, source context, visible symbols, parser evidence, and TypeScript oracle evidence
- [x] No child issue is needed from `3579` because the current blocker is already implementation-ready in issue `5402`

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextEsmImportsOfPackagesWithExtensionlessMains.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextEsmImportsOfPackagesWithExtensionlessMains.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata-only close.
- `cargo nextest run`; issue metadata-only close.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by issue `5402`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nodeNextEsmImportsOfPackagesWithExtensionlessMains.ts`

## Duplicate detection

- `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`
  owns the current virtual `package.json` parsing boundary.

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextEsmImportsOfPackagesWithExtensionlessMains.ts
```

Coverage:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-resolution:1
```

Current diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(Colon) at 155..156
```

Compiler evidence:

```text
tokens: starts with package.json object tokens from node_modules/@types/ip/package.json
ast/resolved: fail on the `"name": "@types/ip"` property colon
visible symbols: []
```

Later package `main` / `types` resolution for `ip` and `nullthrows` is hidden
until issue `5402` stops parsing package.json bodies as executable source.

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextEsmImportsOfPackagesWithExtensionlessMains.ts --detail --no-dashboard-data
result: pass; UnsupportedSyntax/module-resolution in virtual package.json section
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextEsmImportsOfPackagesWithExtensionlessMains.ts
result: pass; package.json property colon blocker superseded by issue 5402
date: 2026-05-08
```

Remaining risks:

- After issue `5402`, this path may expose package `main` / `types`
  resolution for extensionless mains or CommonJS default interop blockers.
