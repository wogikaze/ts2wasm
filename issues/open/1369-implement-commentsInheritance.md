---
id: 1369
title: "Implement Commentsinheritance"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: [5286]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1369.

## Summary

Closed by splitting the current representative failure into a narrow
implementation-ready constructor-arity issue.

## Problem

Reference test results originally showed 1 case failing in directory
`commentsInheritance` with diagnostics: parser-syntax. Fresh focused triage on
2026-05-07 shows the parser and AST now advance through the file; the current
blocker is a lowered constructor arity mismatch for `new` calls with arguments.

Problem: `commentsInheritance.ts` now fails because class constructors with
parameters are validated as zero-argument constructors.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsInheritance.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsInheritance.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=ArityMismatch:1
unsupported_features=arity:1
```

Smart triage reaches backend emission and reports:

```text
error: [ArityMismatch] constructor 19 expects between 0 and 0 argument(s), got 1
/tmp/ts2wasm-4-0.wat:2282:26: error: type mismatch at end of function, expected [] but got [i32]
```

## Desired final state

This generated bucket is split into implementation-ready child issues or
superseded by existing work. Do not implement directly from this bucket.

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
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsInheritance.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsInheritance.ts
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

- [x] created: `issues/open/5286-preserve-class-constructor-parameters-for-new-arity.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsInheritance.ts`

## Duplicate detection

No exact duplicate found by path/title/feature scan. Existing arity issues such
as `issues/open/413-implement-arity.md`, `issues/open/5135-fix-builtin-arity-validation-coercion-globals.md`,
`issues/open/5136-fix-arity-validation-regexp-string-prototype.md`, and the
generated `issues/open/2460-implement-functionParameterArityMismatch.md` cover
builtins or function-call arity, not class constructor parameter arity for
`new C(arg)`. Related constructor issues cover parameter properties or later
class binding and are not exact matches.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage backend io: commentsInheritance

- Issue class: backend-io
- Feature label: backend-io
- Diagnostic: BackendIo / backend-io
- Path: reference/typescript/tests/cases/compiler/commentsInheritance.ts
```

Source context:

```ts
class c2 {
    public field2: number;
    constructor(a: number) {
        this.field2 = a;
    }
}
var c2_i = new c2(10);
```

Compiler evidence:

```text
tokens: ok through interfaces, classes, constructors, and new expressions
ast: ok; class declarations and let/var bindings are present
resolved: fails during lowered validation with ArityMismatch
wat: wat2wasm then reports a stack type mismatch caused by the extra constructor argument
```

Visible symbols include `c1`, `c2`, `c3`, `c4`, `c1_i`, `c2_i = new c2(10)`,
`c3_i = new c3()`, `c4_i = new c4(10)`, `i2_i`, and `i3_i`.

TypeScript oracle:

```text
The source parses and types far enough to infer c2_i: c2, c3_i: c3, and c4_i: c4.
Oracle diagnostics are strict-property initialization and used-before-assigned diagnostics,
not constructor arity diagnostics.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsInheritance.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=ArityMismatch:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsInheritance.ts
result: BackendIo/ArityMismatch; constructor expects 0 arguments but got 1
date: 2026-05-07
```

Remaining risks:

- The child issue only targets constructor arity. Full inheritance semantics,
  strict property initialization parity, and comment/declaration emit fidelity
  remain outside this generated-bucket closure.
