---
id: 5486
title: "Report non-exported merged namespace value member access"
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

Report a TS2339-equivalent diagnostic when a merged namespace block accesses a
member that was local to a previous same-name namespace block.

## Problem

`nonExportedElementsOfMergedModules.ts` now build-passes, but TypeScript
reports that `B.x` is not visible from the later `namespace One` block:

```ts
namespace One {
    namespace B {
        export var x;
    }
}

namespace One {
    namespace B {
        export var y;
    }
    B.x;
    B.y;
}
```

Current triage:

```text
ts2wasm: BuildPass
TypeScript oracle: TS2339 Property 'x' does not exist on type 'typeof B'.
```

Problem: namespace member export/visibility state is not checked when merged
namespace value members are read through qualified value access.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts
```

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts --detail --no-dashboard-data
```

Observed:

```text
coverage: build_pass=1
triage: BuildPass
oracle diagnostic: TS2339 for `B.x`
visible symbols: x, y
```

## Desired final state

The semantic checker rejects `B.x` in the second merged namespace block while
still accepting exported member `B.y`.

## Scope

In scope:

- [ ] Track exported namespace members across same-name namespace merges.
- [ ] Report a source-spanned TS2339-equivalent diagnostic for `B.x`.
- [ ] Add one focused semantic regression for `namespace M { namespace N { export var x; } } namespace M { namespace N { export var y; } N.x; N.y; }`.

Out of scope:

- Namespace member type annotations, tracked by `issues/open/5409-report-non-exported-namespace-member-type-annotations.md`.
- Qualified heritage diagnostics, tracked by `issues/open/5313-report-non-exported-namespace-member-qualified-heritage.md`.
- Mixed exported/local same-name var diagnostics, tracked by `issues/open/5436-report-mixed-exported-local-namespace-vars.md`.
- Runtime namespace member lowering.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/semantic.rs`
- focused semantic tests or fixtures

Do not touch:

- backend wasm lowering
- package/module resolution
- unrelated namespace parser behavior

## Acceptance criteria

- [ ] `nonExportedElementsOfMergedModules.ts` no longer silently build-passes when TypeScript reports TS2339 for `B.x`.
- [ ] `B.y` remains accepted in the representative shape.
- [ ] A focused regression covers rejected `N.x` and accepted `N.y` in merged namespace value access.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(semantic)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nonExportedElementsOfMergedModules.ts --detail --no-dashboard-data
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

Split from `issues/done/3591-implement-nonExportedElementsOfMergedModules.md`.

Related but not duplicates:

- `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`
  covers unresolved namespace roots; this representative already build-passes.
- `issues/open/5409-report-non-exported-namespace-member-type-annotations.md`
  covers type annotations, not value access.
- `issues/open/5313-report-non-exported-namespace-member-qualified-heritage.md`
  covers class heritage, not value access.
- `issues/open/5436-report-mixed-exported-local-namespace-vars.md` covers
  TS2395 same-name export/local var conflicts, not missing member access.

## Completion evidence

Fill when implemented.
