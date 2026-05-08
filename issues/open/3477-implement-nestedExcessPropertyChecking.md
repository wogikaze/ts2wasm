---
id: 3477
title: "Implement Nestedexcesspropertychecking"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
status: done
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed after folding the current plain-enum binding blocker into
`issues/done/5284-bind-plain-enum-declarations-before-member-access.md`.

## Problem

Reference test results previously showed 1 case failing in directory
`nestedExcessPropertyChecking` with diagnostics: parser-syntax.

Fresh triage on 2026-05-08 shows the first current blocker is not parser syntax
or excess-property checking yet. The file stops because plain enum declaration
`enum E { A = "A" }` is not resolver-visible before `E.A`, already tracked by
issue 5284.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedExcessPropertyChecking.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedExcessPropertyChecking.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedExcessPropertyChecking.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedExcessPropertyChecking.ts
```

Not run:

- `cargo fmt --all --check` (issue-only duplicate fold; no Rust changes)
- `cargo nextest run` (issue-only duplicate fold; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] updated: `issues/done/5284-bind-plain-enum-declarations-before-member-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nestedExcessPropertyChecking.ts`

## Duplicate detection

- folded into `issues/done/5284-bind-plain-enum-declarations-before-member-access.md`
  because the current failure is a plain enum declaration not binding before
  `E.A`.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedExcessPropertyChecking.ts
```

Result:

```text
Feature label: name-resolution
Diagnostic: UnresolvedName / resolver-symbol
Message: unresolved name: `E` at 363..364
Source: enum E { A = "A" }; let x: { nope?: any } = E.A;
tokens: ok; includes enum E, member A, typed lets, intersections, and object literals
ast: ok; enum declaration is omitted, later `E.A` member access remains
resolved: fails in resolve_names
TypeScript oracle: TS2559 for assigning enum value to `{ nope?: any }`
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedExcessPropertyChecking.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
semantic_enabled=0
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedExcessPropertyChecking.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, current blocker folded into issue 5284
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedExcessPropertyChecking.ts
result: pass; UnresolvedName for plain enum value `E`
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- Nested excess-property diagnostics are not proven; this bucket currently
  stops before the type-checking diagnostics because plain enum values are not
  resolver-visible.
