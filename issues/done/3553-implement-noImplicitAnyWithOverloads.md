---
id: 3553
title: "Implement Noimplicitanywithoverloads"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5200]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows the current blocker is the
existing top-level function overload implementation issue 5200.

## Problem

Fresh triage shows this fixture tokenizes and parses interface declarations,
two bodyless `callb` overload signatures, one `callb` implementation, and the
lambda call site. AST validation then rejects the second bodyless overload as a
duplicate function:

```text
DuplicateFunction: duplicate function definition: `callb` at 130..138
```

Problem: this generated bucket is superseded by issue 5200, which owns
bodyless top-level function overload signatures plus one implementation being
treated as duplicate functions.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyWithOverloads.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyWithOverloads.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=DuplicateFunction:1 unsupported_features=duplicate-function:1
triage: DuplicateFunction duplicate function definition: `callb` at 130..138
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5200-validate-top-level-function-overload-implementations.md`.
Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fold into existing issue 5200 for the same observable behavior
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
- [x] Existing issue 5200 contains the implementation owner; this done issue contains the exact focused triage command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5200 acceptance covers the top-level overload implementation family; contextual noImplicitAny behavior must be re-triaged after 5200 advances

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyWithOverloads.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyWithOverloads.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only.
- `cargo nextest run`; issue metadata only.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into: `issues/open/5200-validate-top-level-function-overload-implementations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyWithOverloads.ts`

## Duplicate detection

- `issues/open/5200-validate-top-level-function-overload-implementations.md`
  is the exact owner for top-level bodyless overload signatures followed by one
  implementation.
- `issues/open/5199-report-function-overload-list-class-merge-diagnostics.md`
  covers overload lists followed by a class declaration, not this valid
  overload implementation group.
- `issues/open/5226-allow-ambient-function-overload-declarations.md` covers
  ambient declarations, not this non-ambient implementation group.
- Folded into issue 5200.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage duplicate function: noImplicitAnyWithOverloads

- Issue class: triage-needed
- Feature label: duplicate-function
- Diagnostic: DuplicateFunction / compiler-diagnostic
- Path: reference/typescript/tests/cases/compiler/noImplicitAnyWithOverloads.ts
```

Current compiler message:

```text
duplicate function definition: `callb` at 130..138
```

Source context:

```ts
function callb(lam: (l: A) => void);
function callb(lam: (n: B) => void);
function callb(a) { }
callb((a) => { a.foo; });
```

Compiler evidence:

```text
tokens: ok through interfaces, overloads, implementation, and call site
ast: ok; two bodyless Function callb entries, one implemented Function callb, then callb((a) => ...)
validate_ast: fails with DuplicateFunction at the second bodyless overload signature
visible symbols: first callb overload signature
```

TypeScript oracle:

```text
diagnostics: []
lambda parameter a is contextually typed as A
```

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyWithOverloads.ts --detail --no-dashboard-data
result: pass; reproduced current DuplicateFunction boundary for callb overloads
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyWithOverloads.ts
result: pass; generated smart triage evidence and duplicate review material
date: 2026-05-08
```

Remaining risks:

- After issue 5200 advances, this fixture may expose contextual lambda member
  diagnostics or noImplicitAny overload resolution behavior.
