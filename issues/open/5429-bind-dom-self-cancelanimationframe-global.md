---
id: 5429
title: "Bind DOM self.cancelAnimationFrame global"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Bind the DOM `self` global far enough for a qualified
`self.cancelAnimationFrame(0)` call to avoid generic `UnresolvedName`, or
report a source-spanned unsupported DOM animation-frame diagnostic.

## Problem

`multiExtendsSplitInterfaces1.ts` is currently a two-line DOM global reference,
not an interface inheritance case. Name resolution stops on `self`.

Problem: DOM `self.cancelAnimationFrame(0)` currently fails with generic
`UnresolvedName` for `self`.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiExtendsSplitInterfaces1.ts
```

Observed result:

```text
UnresolvedName: unresolved name: `self` at 19..23
```

Source:

```ts
// @target: es2015
self.cancelAnimationFrame(0);
```

Compiler evidence:

```text
tokens: ok through self.cancelAnimationFrame(0)
ast: Call(Member(Ident self, cancelAnimationFrame), Number 0)
resolved: UnresolvedName for self during resolve_names
TypeScript oracle: accepts the file with diagnostics=[]
```

## Desired final state

The compiler no longer reports generic `UnresolvedName` for the DOM `self`
global in this reference. It either binds `self.cancelAnimationFrame` as a known
DOM global/member boundary or reports a precise unsupported DOM animation-frame
diagnostic at `self` or `cancelAnimationFrame`.

## Scope

In scope:

- [ ] Bind `self` as a known DOM/global receiver for `cancelAnimationFrame`.
- [ ] Preserve ordinary unresolved-name diagnostics for unrelated unknown
      globals.
- [ ] Add focused coverage for `self.cancelAnimationFrame(0)`.
- [ ] Re-run the representative reference triage and record the next blocker.

Out of scope:

- Full browser animation-frame runtime scheduling.
- Broad DOM lib declaration modeling.
- The DOM `setTimeout` global, tracked separately by issue 5386.
- Interface inheritance diagnostics; the representative file no longer
  contains interface declarations.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/src/`
- focused resolver/builtin/global tests or fixtures

Do not touch:

- backend timer scheduling or event-loop implementation unless resolver support
  already advances to a reviewed runtime boundary.
- unrelated DOM APIs beyond `self.cancelAnimationFrame`.

## Acceptance criteria

- [ ] `multiExtendsSplitInterfaces1.ts` no longer reports
      `UnresolvedName: unresolved name: self`.
- [ ] A focused regression covers `self.cancelAnimationFrame(0)`.
- [ ] An unrelated unknown global still reports `UnresolvedName`.
- [ ] If animation frames remain unsupported, the diagnostic names the DOM
      animation-frame boundary instead of generic name resolution.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(global) or test(builtin) or test(name)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiExtendsSplitInterfaces1.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiExtendsSplitInterfaces1.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from `issues/done/3402-implement-multiExtendsSplitInterfaces.md`.

Related but distinct:

- `issues/open/5386-bind-dom-settimeout-global.md` covers DOM `setTimeout` and
  explicitly leaves unrelated DOM APIs out of scope.

## Completion evidence

Fill when implemented.
