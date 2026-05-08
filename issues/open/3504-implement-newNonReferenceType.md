---
id: 3504
title: "Implement Newnonreferencetype"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5005]
blocks: [5468]
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage newNonReferenceType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed after splitting the current direct type-only `new` callee diagnostic
gap to `issues/open/5468-report-direct-new-type-only-callee-diagnostics.md`.

## Problem

Reference test results show 1 cases fail in directory `newNonReferenceType` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: the stale name-resolution bucket now builds successfully, but fresh
TypeScript oracle evidence shows missing TS2693 diagnostics for direct
type-only primitive constructor callees.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newNonReferenceType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newNonReferenceType.ts --detail
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5468-report-direct-new-type-only-callee-diagnostics.md`.

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
- [x] Child issue contains an exact reference-triage command
- [x] Child issue includes failing path, diagnostic code, source context,
  visible symbols, parser/resolver evidence, and TypeScript oracle evidence
- [x] Child issue acceptance names the exact reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/newNonReferenceType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/newNonReferenceType.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5468-report-direct-new-type-only-callee-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/newNonReferenceType.ts`

## Duplicate detection

- No exact existing owner was found for direct `new any()` and
  `new boolean()` type-only callee diagnostics.
- `issues/open/5203-report-indexed-new-type-only-callee-diagnostics.md`
  covers indexed new callees such as `new any[1]`, not direct constructor
  callees.
- `issues/open/5466-report-malformed-new-angle-bracket-casts.md` covers
  malformed `new <any>...` parser diagnostics and explicitly leaves TS2693
  checker parity out of scope.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Build pass: newNonReferenceType

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/newNonReferenceType.ts
```

Focused coverage:

```text
suite=tsc
executed=1
build_pass=1
unsupported=0
semantic_enabled=0

reference/typescript/tests/cases/compiler/newNonReferenceType.ts: build_pass
```

Source context:

```ts
var a = new any();
var b = new boolean(); // error
```

Compiler evidence:

```text
tokens: ok
ast: ok; Let a = New Ident("any"), Let b = New Ident("boolean")
resolved: ok; class_name "any", class_name "boolean"
visible symbols: a initialized by new any(); b initialized by new boolean()
```

TypeScript oracle evidence:

```text
TS2693: 'any' only refers to a type, but is being used as a value here.
TS2693: 'boolean' only refers to a type, but is being used as a value here.
binding hints: a has type any; b has type any
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/newNonReferenceType.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/newNonReferenceType.ts
result: pass; current diagnostic gap split to issue 5468
date: 2026-05-08
```

Remaining risks:

- Issue 5468 still needs implementation to report the TypeScript-style TS2693
  direct-callee diagnostics.
