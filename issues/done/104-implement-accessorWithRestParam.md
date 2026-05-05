---
id: 104
title: "Implement Accessorwithrestparam"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage accessorWithRestParam across 1 failing reference test case and split this generated bucket into an implementation-ready child issue.

## Problem

Fresh smart triage shows the parser now accepts this source and the current build reaches backend WAT generation. The TypeScript oracle, however, rejects both setter declarations with TS1053 because `set` accessors cannot have rest parameters.

Problem: `accessorWithRestParam` is not a standalone implementation order; the executable frontend diagnostic slice is split to issue 5157.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorWithRestParam.ts
```

Current compiler diagnostic:

```text
BackendIo: wat2wasm failed
```

Direct build evidence:

```sh
cargo run -q -p ts2wasm-cli -- build reference/typescript/tests/cases/compiler/accessorWithRestParam.ts -o /tmp/ts2wasm-104-accessorWithRestParam.wasm
```

```text
error: [BackendIo] wat2wasm failed
/tmp/ts2wasm-2-0.wat:789:21: error: undefined global variable "$exception_pending"
    (if (global.get $exception_pending)
                    ^^^^^^^^^^^^^^^^^^
```

## Desired final state

This generated bucket is closed as superseded by `issues/open/5157-report-set-accessor-rest-parameter-diagnostic.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one observable frontend diagnostic behavior into a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/frontend/src/diagnostic.rs`
- `fixtures/`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue 5157 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorWithRestParam.ts
cargo run -q -p ts2wasm-cli -- build reference/typescript/tests/cases/compiler/accessorWithRestParam.ts -o /tmp/ts2wasm-104-accessorWithRestParam.wasm
```

Not run:

- `cargo fmt --all --check`; issue split only, no owned Rust implementation changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5157-report-set-accessor-rest-parameter-diagnostic.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/accessorWithRestParam.ts`

## Duplicate detection

- `issues/open/422-implement-class-accessor.md` is the broad class-accessor triage bucket, not an executable child for this exact setter rest-parameter diagnostic.
- No existing implementation-ready issue matched the exact TS1053 setter rest-parameter shape.

## Smart triage

### Smart triage: Triage backend io: accessorWithRestParam

- Issue class: `triage-needed`
- Feature label: `backend-io`
- Diagnostic: `BackendIo` / `backend-io`
- Path: `reference/typescript/tests/cases/compiler/accessorWithRestParam.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorWithRestParam.ts
```

Source context:

```text
4 | class C {
5 |     set X(...v) { }
6 |     static set X(...v2) { }
7 | }
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C",
    "line": 4,
    "column": 1
  }
]
```

Parser/resolver evidence:

```text
tokens: ok
ast: ok; ClassDecl C has methods `set X` and `static::set X`, both with rest parameters
resolved: ok; ClassMethod entries preserve rest parameters `v` and `v2`
wat: generated, then final build fails in wat2wasm
```

TypeScript oracle evidence:

```text
TS1053: A 'set' accessor cannot have rest parameter.
line 5, character 11: ...v
line 6, character 18: ...v2
```

Resolution:

```text
Issue 5157 now owns the concrete frontend diagnostic contract. The shared backend `$exception_pending` WAT validity failure is tracked separately by issue 5155.
```

## Completion evidence

Commits:

- superseded by `issues/open/5157-report-set-accessor-rest-parameter-diagnostic.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorWithRestParam.ts
result: pass; reproduced current BackendIo/wat2wasm state and TypeScript TS1053 oracle evidence
date: 2026-05-06

command: cargo run -q -p ts2wasm-cli -- build reference/typescript/tests/cases/compiler/accessorWithRestParam.ts -o /tmp/ts2wasm-104-accessorWithRestParam.wasm
result: fail as expected; stderr reports undefined global variable "$exception_pending"
date: 2026-05-06
```

Remaining risks:

- Issue 5157 still needs implementation.
