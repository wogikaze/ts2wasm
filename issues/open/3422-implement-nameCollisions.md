---
id: 3422
title: "Implement Namecollisions"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed after splitting the first concrete diagnostic family into
implementation-ready child issue
`5439-report-namespace-value-duplicate-identifiers.md`.

Fresh triage shows `nameCollisions.ts` currently build-passes while TypeScript
reports multiple diagnostics. The first family is TS2300 duplicate identifiers
for same-scope namespace/value collisions (`var x` with `namespace x`, and
`namespace z` with `var z`).

## Problem

Reference test results show 1 cases fail in directory `nameCollisions` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: nameCollisions had 1 generated reference failure and needed
smart-triage evidence before implementation starts.

Disposition: the first executable implementation slice is child issue `5439`.
Later oracle diagnostics are either already tracked by existing owners or
should be re-triaged after `5439` advances.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nameCollisions.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nameCollisions.ts --detail
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
- [x] At least one child issue contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nameCollisions.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nameCollisions.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5439-report-namespace-value-duplicate-identifiers.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nameCollisions.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nameCollisions.ts --detail --no-dashboard-data

result:
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
reference/typescript/tests/cases/compiler/nameCollisions.ts: build_pass
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nameCollisions.ts

result:
BuildPass: ts2wasm build succeeded
TypeScript oracle reports TS2300, TS2564, TS2434, TS2813, and TS2814 diagnostics.
```

Source context:

```ts
namespace T {
    var x = 2;

    namespace x { // error
        export class Bar {
            test: number;
        }
    }

    namespace z {
        var t;
    }
    var z;

    function f() {}
    function f() {}
    function f2() {}
    var f2;

    class C {}
    function C() {}
    function C2() {}
    class C2 {}
}
```

Compiler evidence:

```text
tokens: ok through namespace T and the nested declarations
ast/resolved: ok but empty after namespace erasure
visible symbols include x, Bar, t, z, f, f2, C, C2, fi, cli, and cli2 before build-pass classification
coverage: executed=1, build_pass=1, unsupported=0
```

TypeScript oracle evidence:

```text
TS2300: Duplicate identifier 'x'.            // var x
TS2300: Duplicate identifier 'x'.            // namespace x
TS2564: Property 'test' has no initializer.
TS2300: Duplicate identifier 'z'.            // namespace z
TS2300: Duplicate identifier 'z'.            // var z
TS2434: A namespace declaration cannot be located prior to a class or function with which it is merged.
TS2300: Duplicate identifier 'f'.
TS2300: Duplicate identifier 'f2'.
TS2813/TS2814: invalid class/function merge diagnostics for C and C2.
```

Split result:

- `5439-report-namespace-value-duplicate-identifiers.md` owns the first TS2300 namespace/value duplicate family.
- `5307-report-var-function-duplicate-identifier-diagnostics.md` owns var/function duplicate identifier diagnostics.
- `5330-report-namespace-before-class-merge-diagnostic.md` owns TS2434 namespace-before-class/function ordering.
- `5356-report-uninitialized-generic-class-fields.md` owns the TS2564 strict-property-initialization family.

## Completion evidence

Split into:

- `5439-report-namespace-value-duplicate-identifiers.md`

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nameCollisions.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nameCollisions.ts
result: pass; BuildPass with TypeScript oracle diagnostics TS2300/TS2564/TS2434/TS2813/TS2814
date: 2026-05-08
```

Remaining risks:

- Later class/function merge diagnostics may need a separate child after the
  namespace/value duplicate family advances.
