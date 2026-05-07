---
id: 1236
title: "Implement Classsideinheritance Name Resolution"
type: spike
area: frontend/resolver
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
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1236.

## Summary

Triage classSideInheritance-name-resolution across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 2 cases failing in directory
`classSideInheritance-name-resolution` with diagnostics: name-resolution. Fresh
triage shows the two files now stop at separate already-owned blockers:
class-constructor value use and ambient value declarations.

Problem: `classSideInheritance3.ts` currently stops on `issue-5011` for using a
class constructor as a value, and `classSideInheritance1.ts` currently stops on
an ambient `declare var` name that is erased before name resolution. Existing
issues 5192 and 5161 already own those blockers, so this generated bucket is not
a standalone implementation issue.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classSideInheritance3.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classSideInheritance1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classSideInheritance3.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classSideInheritance1.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5192 covers the `issue-5011` class-constructor value blocker
- [x] Confirm existing issue 5161 covers the ambient declaration name-resolution blocker
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
- [x] Superseding issues contain implementation-ready scopes for current blockers
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference paths and diagnostic/stdout changes

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classSideInheritance3.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classSideInheritance3.ts
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

- [x] superseded by `issues/done/5192-support-first-class-class-constructor-values.md`
- [x] superseded by `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`
- [x] related later semantic blocker: `issues/open/5261-report-class-typed-missing-instance-method-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classSideInheritance3.ts`
- `reference/typescript/tests/cases/compiler/classSideInheritance1.ts`

## Duplicate detection

- `issues/done/5192-support-first-class-class-constructor-values.md` - exact current owner for `classSideInheritance3.ts`, where assigning `B` as a value reports `issue-5011`
- `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md` - exact current owner for `classSideInheritance1.ts`, where `declare var a: A` is visible to TypeScript but not resolver-visible
- `issues/open/5261-report-class-typed-missing-instance-method-calls.md` - related later TypeScript diagnostic owner for static members called through class-typed instances (`a.bar()` / `c.bar()`)
- `issues/open/064-implement-name-resolution.md` and other broad name-resolution buckets are no-match for this current evidence because the failing paths map to narrower open issues.

## Smart triage

Fresh triage shows this generated name-resolution bucket is a duplicate of
existing narrower implementation-ready issues.

### Smart triage: classSideInheritance3

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `unsupported-feature-boundary`
- Current compiler message: `issue-5011: class B cannot be used as a value - class runtime is not yet supported`
- Path: `reference/typescript/tests/cases/compiler/classSideInheritance3.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classSideInheritance3.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classSideInheritance3.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
class A {
    foo() { }
}

class B extends A {
    constructor(x: string) {
        super();
    }
}

class C extends A {
}

var r1: typeof A = B;
var r2: new (x: string) => A = B;
var r3: typeof A = C;
```

Compiler evidence:

```text
tokens: ok
ast: ok; top-level ClassDecl A, ClassDecl B extends A, ClassDecl C extends A
resolved: issue-5011 at identifier B in `var r2: new (x: string) => A = B`
```

TypeScript oracle evidence:

```text
diagnostics: TS2322 for assigning `typeof B` to `typeof A` and assigning `typeof B`
to `new (x: string) => A`
```

Superseding owner:

- `issues/done/5192-support-first-class-class-constructor-values.md`

### Smart triage: classSideInheritance1

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `compiler-diagnostic`
- Current compiler message: `unresolved name: a`
- Path: `reference/typescript/tests/cases/compiler/classSideInheritance1.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classSideInheritance1.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classSideInheritance1.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Source context:

```ts
class A {
    static bar() { }
    foo() { }
}

class C2 extends A {
}

declare var a: A;
declare var c: C2;

a.bar();
c.bar();
A.bar();
C2.bar();
```

Compiler evidence:

```text
tokens: ok
ast: ok; class declarations parse, ambient declarations are erased, calls remain
resolved: UnresolvedName for ambient value `a`
visible-symbol extraction: `a` and `c` are visible in the TypeScript source
```

TypeScript oracle evidence:

```text
diagnostics: TS2576 for `a.bar()` and `c.bar()`, because `bar` is static and
should be accessed through the class side rather than class-typed instances
```

Superseding owners:

- `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`
- later semantic diagnostic owner: `issues/open/5261-report-class-typed-missing-instance-method-calls.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by existing issues 5192 and 5161; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classSideInheritance3.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; current issue-5011 class-constructor value boundary superseded by 5192
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classSideInheritance1.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; current ambient declaration name-resolution boundary superseded by 5161, with later TS2576 behavior covered by 5261
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classSideInheritance3.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; unsupported=1
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classSideInheritance1.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; unsupported=1
date: 2026-05-07
```

Remaining risks:

- After issues 5192 and 5161 land, these references may expose class-side
  assignability, constructor-signature compatibility, or static-member diagnostic
  parity as later blockers.
