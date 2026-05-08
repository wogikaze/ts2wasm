---
id: 5157
title: "Report set accessor rest parameter diagnostics"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

The parser now accepts class `set` accessors with rest parameters and lowers them as methods, but TypeScript rejects this source with TS1053. The compiler should report a precise frontend diagnostic before the program reaches backend WAT generation.

## Problem

Problem: `reference/typescript/tests/cases/compiler/accessorWithRestParam.ts` currently reaches backend emission and fails with `BackendIo`, even though TypeScript reports that `set` accessors cannot have rest parameters.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorWithRestParam.ts
```

Current compiler diagnostic:

```text
BackendIo: wat2wasm failed
```

Direct build stderr also shows the shared backend global failure tracked by issue 5155:

```text
error: [BackendIo] wat2wasm failed
/tmp/ts2wasm-2-0.wat:789:21: error: undefined global variable "$exception_pending"
    (if (global.get $exception_pending)
                    ^^^^^^^^^^^^^^^^^^
```

Representative source:

```ts
class C {
    set X(...v) { }
    static set X(...v2) { }
}
```

Current compiler evidence:

- Tokens succeed.
- AST succeeds with methods named `set X` and `static::set X`, each with one rest parameter.
- Resolved IR succeeds and preserves both rest parameters.
- Backend WAT validation fails before a TypeScript-compatible diagnostic is reported.

TypeScript oracle evidence:

```text
TS1053: A 'set' accessor cannot have rest parameter.
line 5, character 11: ...v
line 6, character 18: ...v2
```

## Desired final state

The frontend reports a source-spanned diagnostic for rest parameters on instance and static `set` accessors. The representative case should no longer reach `BackendIo` for this invalid TypeScript source.

## Scope

In scope:

- [x] Detect rest parameters in class `set` accessors during parsing or validation.
- [x] Cover both instance `set X(...v)` and static `static set X(...v2)`.
- [x] Add a focused parser/frontend test for the TS1053 shape.
- [x] Re-run the representative triage and confirm it no longer reports `BackendIo`.

Out of scope:

- General accessor runtime semantics.
- Object literal accessors.
- The shared `$exception_pending` runtime-link bug, tracked by issue 5155.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/frontend/src/diagnostic.rs`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/src/` unless triage still reaches backend after the frontend diagnostic is added.

## Acceptance criteria

- [x] `set X(...v) {}` reports a source-spanned frontend diagnostic matching the TypeScript oracle shape.
- [x] `static set X(...v2) {}` reports the same diagnostic.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorWithRestParam.ts` no longer reports `BackendIo`.
- [x] Existing valid getter/setter parsing tests continue to pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorWithRestParam.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

The current backend error overlaps with issue 5155, but the TypeScript oracle proves this specific reference case should be rejected earlier as invalid accessor syntax.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none


## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in issues/done/. Implementation commits confirmed.
