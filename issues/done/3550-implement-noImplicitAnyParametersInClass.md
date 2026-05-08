---
id: 3550
title: "Implement Noimplicitanyparametersinclass"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5198]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows the current blocker is the
existing class method overload signature issue 5198.

## Problem

Fresh triage shows this fixture tokenizes and parses class methods, rest
parameters, method overload signatures, and class-field arrow initializers.
Lowering then fails at the first bodyless class method overload group:

```text
DuplicateFunction: duplicate method definition: `C.pub_f8`
```

Problem: this generated bucket is superseded by issue 5198, which owns valid
class method overload signatures being treated as duplicate methods.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyParametersInClass.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyParametersInClass.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=DuplicateFunction:1 unsupported_features=duplicate-function:1
triage: DuplicateFunction duplicate method definition: `C.pub_f8`
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md`.
Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fold into existing issue 5198 for the same observable behavior
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
- [x] Existing issue 5198 contains the implementation owner; this done issue contains the exact focused triage command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5198 acceptance covers the class-method overload diagnostic family; noImplicitAny parameter diagnostics must be re-triaged after 5198 advances

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyParametersInClass.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyParametersInClass.ts
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

- [x] folded into: `issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyParametersInClass.ts`

## Duplicate detection

- `issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md`
  is the exact owner for valid bodyless class method overload signatures being
  treated as duplicate method definitions.
- `issues/open/5199-report-function-overload-list-class-merge-diagnostics.md`
  covers top-level function overload declarations followed by a class of the
  same name, not class member overload groups.
- `issues/open/5200-validate-top-level-function-overload-implementations.md`
  covers top-level functions, not class methods.
- Folded into issue 5198.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage duplicate function: noImplicitAnyParametersInClass

- Issue class: triage-needed
- Feature label: duplicate-function
- Diagnostic: DuplicateFunction / compiler-diagnostic
- Path: reference/typescript/tests/cases/compiler/noImplicitAnyParametersInClass.ts
```

Current compiler message:

```text
duplicate method definition: `C.pub_f8`
```

Source context:

```ts
class C {
    public pub_f8(x1, y1: number): any;
    public pub_f8(x2: string, y2): any;
    public pub_f8(x3, y3): any { }
}
```

Compiler evidence:

```text
tokens: ok through public/private methods, overload signatures, rest parameters, and class-field arrows
ast: ok; ClassDecl C includes three Function entries named pub_f8 before later class fields/private methods
resolved/lowering: fails with DuplicateFunction for C.pub_f8
visible symbols: class C
```

TypeScript oracle:

```text
diagnostics: []
parameters on pub_f8 overloads are typed as any/number/string as expected by the reference
```

Remaining risks:

- After issue 5198 advances, this fixture may expose class-field arrow parser,
  implicit-any diagnostic, or private member overload boundaries.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyParametersInClass.ts --detail --no-dashboard-data
result: pass; reproduced current DuplicateFunction boundary for C.pub_f8
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyParametersInClass.ts
result: pass; generated smart triage evidence and duplicate review material
date: 2026-05-08
```
