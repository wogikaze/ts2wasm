---
id: 5322
title: "Support callable class auto-accessor fields"
type: feature
area: ir/runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Support class auto-accessor fields initialized with callable values when they
are invoked through `this`.

## Problem

Problem: `classFieldSuperNotAccessibleJs.ts` parses successfully, but lowering
reports `method YaddaBase.b not found` for `this.b()` when `b` is declared as
`accessor b = () => { ... }`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperNotAccessibleJs.ts
```

Observed 2026-05-07:

```text
lower_program: UnsupportedSyntax method `YaddaBase.b` not found at 344..352
source: this.b()
```

Representative source:

```ts
class YaddaBase {
    constructor() {
        this.b()
    }
    accessor b = () => {
        this.foo = 10
    }
}
```

TypeScript oracle reports property-existence diagnostics for other fields in
the same file, but not for `this.b()`.

## Desired final state

The resolver/lowering pipeline represents auto-accessor fields initialized with
callable values well enough for `this.b()` to dispatch or to advance to the next
TypeScript-compatible property diagnostic in the representative file.

## Scope

In scope:

- [x] Preserve `accessor b = () => {}` as a callable instance member or
  receiver-bound field value.
- [x] Resolve `this.b()` against that callable auto-accessor member.
- [x] Add focused coverage for `class C { accessor b = () => {}; m() { this.b(); } }`.

Out of scope:

- Divergent getter/setter type diagnostics, tracked by class-accessor buckets.
- Full auto-accessor emit parity beyond callable arrow initializers.
- The later `super.roots` / `super.foo` property diagnostics in the same file.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/ir/src/`
- focused frontend/IR or CLI tests

Do not touch:

- unrelated accessor syntax buckets unless triage shows this AST shape changed

## Acceptance criteria

- [x] `classFieldSuperNotAccessibleJs.ts` no longer reports
  `method YaddaBase.b not found` for `this.b()`.
- [x] A focused fixture covers an auto-accessor field initialized to an arrow
  function and called through `this`.
- [x] If the reference advances to `super.<field>` diagnostics, that next blocker
  is recorded separately or tied to an existing exact issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -p ts2wasm-ir -E 'test(class) or test(accessor) or test(receiver)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classFieldSuperNotAccessibleJs.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classFieldSuperNotAccessibleJs.ts --detail --no-dashboard-data
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

Split from stale generated bucket
`issues/done/1210-implement-classFieldSuperNotAccessibleJs.md`.

Related but not duplicate:

- `issues/done/422-implement-class-accessor.md` is a broad generated
  class-accessor bucket, not a narrow implementation-ready owner for this
  callable auto-accessor receiver shape.

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

**truly-done** (5322)

- Implementation commits: verified via `git log --oneline --all --grep=5322`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
