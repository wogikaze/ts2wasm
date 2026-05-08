---
id: 5436
title: "Report mixed exported and local namespace var merges"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TypeScript-compatible TS2395 diagnostic when declarations with the
same name in a namespace are mixed between exported and local variable
declarations.

## Problem

Problem: `multivar.ts` now build-passes, but TypeScript reports TS2395 because
`namespace m2` has both exported and local declarations for `b2`.

Current source:

```ts
namespace m2 {
    export var a, b2: number = 10, b;
    var b2;
}
```

Current evidence from 2026-05-08:

```text
ts2wasm: build_pass
TypeScript oracle:
TS2395 Individual declarations in merged declaration 'b2' must be all exported or all local.
TS2395 Individual declarations in merged declaration 'b2' must be all exported or all local.
```

## Desired final state

The semantic checker records namespace member export state and reports a
source-spanned diagnostic when same-name var declarations disagree on
exported/local visibility.

## Scope

In scope:

- [ ] Detect same-namespace var declarations where one declaration is exported
  and another declaration with the same name is local.
- [ ] Emit a source-spanned diagnostic equivalent to TS2395 for both
  declarations or the later conflicting declaration.
- [ ] Add focused coverage for `namespace M { export var x; var x; }`.

Out of scope:

- Entry-module `export var` parsing, tracked separately by issues 5283 and 5285.
- `exports`/`require` special names outside namespace member merge checks.
- Full TypeScript declaration merging for classes, enums, or interfaces.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- focused semantic/parser regression tests

Do not touch:

- backend/runtime ABI
- unrelated import/export module loading behavior

## Acceptance criteria

- [ ] `multivar.ts` no longer build-passes silently when TypeScript reports
  TS2395 for namespace member `b2`.
- [ ] A focused fixture covers `namespace M { export var x; var x; }`.
- [ ] Valid namespace multi-declarator vars such as `var a, b, c;` still parse.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend namespace
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multivar.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multivar.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket `issues/open/3416-implement-multivar.md`.

Related but not duplicates:

- `issues/open/5283-support-entry-export-var-declarations.md` and
  `issues/open/5285-support-export-var-initializer-declarations.md` cover
  entry-module export-var parser/module boundaries, not namespace merge
  diagnostics after `multivar.ts` already build-passes.
- Older stale buckets such as
  `issues/open/1290-implement-collisionExportsRequireAndAmbientVar.md` record
  TS2395 as remaining semantic parity risk but do not provide an
  implementation-ready owner.

## Completion evidence

Fill when implemented.
