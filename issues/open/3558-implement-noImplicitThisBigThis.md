---
id: 3558
title: "Implement Noimplicitthisbigthis"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: [5218]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows the current blocker is the
existing nested-function `this` closure issue 5218.

## Problem

Fresh triage shows this fixture parses object literals with method shorthand,
then lowering rejects the first object-literal method returning `this`:

```text
UnsupportedRuntimeSubset: issue-062e: nested function `func1` closures with `this` or `arguments` are not supported in this slice
```

Problem: this generated bucket is superseded by issue 5218, which owns
nested ordinary function/object-literal member closures that capture `this`.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitThisBigThis.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitThisBigThis.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
triage: UnsupportedRuntimeSubset issue-062e nested function func1 closures with this or arguments
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5218-support-nested-function-closures-capturing-this.md`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5218 covers this bucket
- [x] Fold into existing issue 5218 for the same observable runtime boundary
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing issue 5218 contains the implementation owner; this done issue contains the exact focused triage command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5218 acceptance covers the generic issue-062e nested-function `this` closure guard

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitThisBigThis.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitThisBigThis.ts
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

- [x] folded into: `issues/open/5218-support-nested-function-closures-capturing-this.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitThisBigThis.ts`

## Duplicate detection

- `issues/open/5218-support-nested-function-closures-capturing-this.md` is the
  exact owner for the current `issue-062e` nested-function/object-literal
  member `this` capture boundary.
- `issues/open/5179-report-implicit-this-before-closure-runtime-guard.md` is
  related but not exact: it owns cases where TypeScript reports TS2683 before
  the runtime guard. `noImplicitThisBigThis.ts` has TypeScript diagnostics `[]`.
- Folded into issue 5218.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage runtime subset: noImplicitThisBigThis

- Issue class: triage-needed
- Feature label: runtime-subset
- Diagnostic: UnsupportedRuntimeSubset / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/noImplicitThisBigThis.ts
```

Current compiler message:

```text
issue-062e: nested function `func1` closures with `this` or `arguments` are not supported in this slice
```

Source context:

```ts
function createObj() {
    return {
        func1() {
            return this;
        },
        func2() {
            return this;
        },
        func3() {
            return this;
        }
    };
}
```

Compiler evidence:

```text
tokens: ok; includes object-literal method shorthand and This tokens
ast: ok; object properties func1/func2/func3 are FunctionExpr values returning This
resolved/lowered: issue-062e nested function closure with this/arguments at func1
visible symbols: createObj, createObjNoCrash
```

TypeScript oracle:

```text
diagnostics: []
```

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitThisBigThis.ts --detail --no-dashboard-data
result: pass; reproduced current unsupported boundary
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitThisBigThis.ts
result: pass; generated smart triage evidence and duplicate review material
date: 2026-05-08
```

Remaining risks:

- After issue 5218 advances, this fixture may expose declaration/noImplicitThis
  semantic parity details, but the current first blocker is the issue-062e
  object-literal `this` closure boundary.
