---
id: 5313
title: "Report non-exported namespace member in qualified heritage"
type: feature
area: frontend/name-resolution
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report a TypeScript-compatible diagnostic when a class heritage clause qualifies
a namespace-private class member, starting with `class D extends M.C`.

## Problem

Problem: `classExtendingQualifiedName.ts` now builds successfully, but
TypeScript reports TS2339 because `C` is not exported from namespace `M`.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingQualifiedName.ts
```

Observed 2026-05-07:

```text
ts2wasm: BuildPass
source:
namespace M {
    class C {
    }

    class D extends M.C {
    }
}
TypeScript oracle: TS2339 Property 'C' does not exist on type 'typeof M'.
```

The companion case with an exported class already builds and matches the
TypeScript no-diagnostic shape:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingQualifiedName2.ts
```

## Desired final state

`class D extends M.C` reports a source-spanned missing exported member
diagnostic when `C` is namespace-private. `class D extends M.C` continues to
build for `namespace M { export class C {} }`.

## Scope

In scope:

- [ ] Detect namespace-private class members used through qualified heritage.
- [ ] Report TS2339-equivalent missing property/member diagnostic for `M.C`.
- [ ] Preserve the current build-pass behavior for exported `M.C`.

Out of scope:

- General qualified class heritage lowering; issue 5225 owns the current
  implementation blocker for exported qualified heritage in another path.
- Full namespace emit/runtime semantics.
- Arbitrary qualified type diagnostics outside class heritage.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused namespace/name-resolution tests or fixtures

Do not touch:

- backend or runtime lowering unless a focused resolver test proves the diagnostic can only be produced later

## Acceptance criteria

- [ ] `classExtendingQualifiedName.ts` no longer silently build-passes when TypeScript reports TS2339 for `M.C`.
- [ ] `classExtendingQualifiedName2.ts` still build-passes with exported `M.C`.
- [ ] A focused regression covers private vs exported namespace class heritage lookup.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(class) or test(name)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingQualifiedName.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendingQualifiedName2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendingQualifiedName --detail --no-dashboard-data
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

Split from stale generated bucket `issues/open/1195-implement-classExtendingQualifiedName.md`.

Related but not duplicates:

- `issues/open/5225-w0-typed-wat-writer.md` handles a
  current unsupported qualified heritage implementation blocker. This issue is
  the semantic diagnostic for namespace-private qualified heritage after the
  representative `classExtendingQualifiedName.ts` path now build-passes.
- `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`
  handles value access such as `m1.fooExport()`, not class heritage diagnostics.

## Completion Evidence

Fill when implemented.
