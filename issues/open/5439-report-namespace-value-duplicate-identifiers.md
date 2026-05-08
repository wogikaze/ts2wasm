---
id: 5439
title: "Report namespace/value duplicate identifiers"
type: bug
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report TS2300-style duplicate identifier diagnostics when an instantiated
namespace declaration collides with a same-scope value binding.

Split from generated bucket `3422`.

## Problem

`nameCollisions.ts` now build-passes in ts2wasm, but TypeScript reports
duplicate identifier diagnostics for namespace/value collisions before later
merge diagnostics:

```ts
namespace T {
    var x = 2;

    namespace x {
        export class Bar {
            test: number;
        }
    }

    namespace z {
        var t;
    }
    var z;
}
```

Current compiler evidence shows the namespace bodies are erased and the final
AST/resolved dumps are empty, so the duplicate namespace/value names are never
diagnosed.

Problem: namespace declarations that collide with `var` bindings currently build-pass silently instead of reporting duplicate identifier diagnostics.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nameCollisions.ts
```

Observed result:

```text
BuildPass: ts2wasm build succeeded
```

Focused coverage:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
```

TypeScript oracle diagnostics in scope:

```text
TS2300: Duplicate identifier 'x'.  // var x
TS2300: Duplicate identifier 'x'.  // namespace x
TS2300: Duplicate identifier 'z'.  // namespace z
TS2300: Duplicate identifier 'z'.  // var z
```

Compiler evidence:

```text
tokens: ok through namespace T, var x, namespace x, namespace z, and var z
ast/resolved: ok but empty after namespace erasure
visible symbols include binding x, class Bar, binding t, and binding z before build-pass classification
```

## Desired final state

The frontend/resolver preserves enough namespace declaration metadata to detect
same-scope namespace/value duplicate identifiers and report source-spanned
diagnostics instead of silently build-passing.

## Scope

In scope:

- [ ] Detect `var x` followed by `namespace x` in the same namespace scope.
- [ ] Detect `namespace z` followed by `var z` in the same namespace scope.
- [ ] Report TS2300-style duplicate identifier diagnostics at the duplicated names.
- [ ] Add focused resolver coverage for both declaration orders.
- [ ] Re-triage `nameCollisions.ts` and record the next diagnostic after the namespace/value duplicate blockers.

Out of scope:

- Var/function duplicate identifiers, tracked by `issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`.
- Namespace-before-class/function TS2434, tracked by `issues/open/5330-report-namespace-before-class-merge-diagnostic.md`.
- Class/namespace duplicate members, tracked by `issues/open/5329-report-class-namespace-duplicate-member-diagnostics.md`.
- Strict property initialization TS2564, tracked by `issues/open/5356-report-uninitialized-generic-class-fields.md`.
- Full namespace runtime emission.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused resolver/reference tests

Do not touch:

- backend namespace emit unless fresh implementation evidence proves it is required
- static ES module resolution

## Acceptance criteria

- [ ] `nameCollisions.ts` no longer build-passes silently for the `var x` / `namespace x` collision.
- [ ] A focused test reports duplicate identifiers for `var x = 1; namespace x {}`.
- [ ] A focused test reports duplicate identifiers for `namespace z {} var z;`.
- [ ] Diagnostics are source-spanned at the duplicated identifiers, not only at the enclosing namespace or statement.
- [ ] Re-triage records either the next narrower diagnostic or the remaining oracle diagnostics after this duplicate family advances.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(duplicate) or test(resolver)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nameCollisions.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nameCollisions.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

This issue owns the first duplicate family in `nameCollisions.ts`. Later oracle
diagnostics include TS2434 namespace ordering, TS2564 uninitialized class field,
var/function duplicate identifiers, and class/function merge diagnostics.

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
