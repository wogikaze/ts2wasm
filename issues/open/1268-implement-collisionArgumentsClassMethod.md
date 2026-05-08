---
id: 1268
title: "Implement Collisionargumentsclassmethod"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1268.

## Summary

Triage collisionArgumentsClassMethod across 1 reference case and close it as
superseded by the existing class method overload-signature issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionArgumentsClassMethod` with diagnostics: arguments-object. Fresh
triage shows the current first blocker is `DuplicateFunction` for bodyless class
method overload signatures, before the intended strict-mode `arguments`
diagnostics.

Problem: valid TypeScript class method overload signatures are treated as
duplicate method definitions.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsClassMethod.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsClassMethod.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5198 covers the current first blocker
- [x] Supersede this generated bucket without creating a duplicate child
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
- [x] Superseding issue 5198 owns class method overload signatures being treated as duplicate methods
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsClassMethod.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsClassMethod.ts
```

Not run:

- `cargo fmt --all --check`; issue close only, no Rust code changed
- `cargo nextest run`; issue close only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionArgumentsClassMethod.ts`

## Duplicate detection

- `issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md` - exact owner for valid class method overload signatures currently reported as duplicate methods
- `issues/open/5327-report-class-method-overload-wrong-implementation-name.md` - related invalid wrong-name implementation diagnostics, not this valid overload signature shape
- broad duplicate-function generated buckets are not exact owners for this current first blocker

## Smart triage

Fresh triage shows this generated arguments-object bucket is currently blocked
by valid class method overload-signature handling.

### Smart triage: collisionArgumentsClassMethod

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Current compiler message: `duplicate method definition: c1.f4`
- Path: `reference/typescript/tests/cases/compiler/collisionArgumentsClassMethod.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsClassMethod.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsClassMethod.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=DuplicateFunction:1
unsupported_features=duplicate-function:1
```

Source context:

```ts
class c1 {
    public f4(i: number, ...arguments);
    public f4(i: string, ...arguments);
    public f4(i: any, ...arguments) {
        var arguments: any[];
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; ClassDecl c1 contains multiple Function members named `f4`
resolved/lowering: fails with DuplicateFunction duplicate method definition `c1.f4`
```

TypeScript oracle evidence:

```text
TS1210: Code contained in a class is evaluated in JavaScript's strict mode which does not allow this use of 'arguments'.
```

Superseding owner:

- `issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md`

## Completion evidence

Commits:

- Superseded by `issues/open/5198-support-class-method-overload-signatures-for-element-access-calls.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsClassMethod.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction class method overload blocker superseded by issue 5198
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsClassMethod.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction/duplicate-function
date: 2026-05-07
```

Remaining risks:

- After issue 5198 lands, this reference will likely expose TS1210 strict-mode
  class `arguments` binding diagnostics.
