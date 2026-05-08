---
id: 5459
title: "Check nested object literal excess properties through contextual types"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report TypeScript-compatible excess-property diagnostics for nested fresh object
literals when an outer contextual type determines the nested target property
type.

Split from generated bucket
`issues/done/3478-implement-nestedFreshLiteral.md`.

## Problem

Problem: `nestedFreshLiteral.ts` now builds successfully in ts2wasm, so the old
parser-syntax bucket is stale. TypeScript still reports an excess-property
diagnostic for the innermost fresh object literal:

```text
TS2561: Object literal may only specify known properties, but 'colour' does not
exist in type 'CSSProps'. Did you mean to write 'color'?
```

The representative assigns a nested object literal to `NestedCSSProps`; the
`nested.prop` contextual type is `CSSProps`, whose only optional property is
`color`. The checker should preserve freshness through the nested object
literal path and reject `colour`.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedFreshLiteral.ts
```

Representative source:

```ts
interface CSSProps {
  color?: string
}
interface NestedCSSProps {
  nested?: NestedSelector
}
interface NestedSelector {
  prop: CSSProps;
}

let stylen: NestedCSSProps = {
  nested: { prop: { colour: 'red' } }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; Let stylen = Object { nested: Object { prop: Object { colour: String } } }
resolved: ok; the nested object literal is preserved
ts2wasm diagnostic: BuildPass / pass
TypeScript oracle: TS2561 at line 14, character 21, property 'colour'
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedFreshLiteral.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=1
semantic_pass=0
unsupported=0
semantic_enabled=0
unsupported_diagcodes=
unsupported_features=
```

## Desired final state

The semantic checker propagates contextual object types through nested fresh
object literals and reports excess properties at the nested literal that first
violates the expected property set. `nestedFreshLiteral.ts` should no longer be
classified as a plain build pass when semantic parity is enabled for this
reference case.

## Scope

In scope:

- [ ] Preserve object-literal freshness while checking nested object literal
  properties under a contextual type.
- [ ] Resolve the contextual type of `nested.prop` from `NestedCSSProps` through
  `NestedSelector` to `CSSProps`.
- [ ] Report an excess-property diagnostic for `colour` when checking against
  `CSSProps`.
- [ ] Add focused checker coverage for nested object literal contextual typing.
- [ ] Re-run `nestedFreshLiteral.ts` triage and record any next blocker.

Out of scope:

- Object-literal freshness with spread, tracked by
  `issues/open/3645-implement-objectLiteralFreshnessWithSpread.md`.
- Array literal contextual inference, tracked separately by generated array
  literal inference buckets such as
  `issues/open/687-implement-arrayLiteralTypeInference.md`.
- Suggestion ranking or spelling-correction quality for `color` vs `colour`
  beyond preserving a clear excess-property diagnostic.
- Full semantic parity enablement for all TypeScript reference tests.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- focused semantic/checker tests

Do not touch:

- backend/runtime code unless this representative advances to a runtime-owned
  blocker after semantic checking
- unrelated freshness generated buckets except for explicit lifecycle updates

## Acceptance criteria

- [ ] `let x: { prop: { color?: string } } = { prop: { colour: "red" } }`
  reports an excess-property diagnostic for `colour`.
- [ ] `nestedFreshLiteral.ts` no longer appears as a plain `BuildPass` when the
  relevant semantic parity path is enabled.
- [ ] A valid nested object literal with `color` instead of `colour` still
  builds.
- [ ] A focused test proves the diagnostic is attached to the innermost object
  literal property, not to the outer `nested` or `prop` properties.
- [ ] If the representative advances to a new blocker, this issue records that
  blocker before closure.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedFreshLiteral.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedFreshLiteral.ts --detail --no-dashboard-data
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

This issue is intentionally narrower than broad literal freshness propagation
work. The observable behavior is the nested object literal contextual typing
path demonstrated by `nestedFreshLiteral.ts`.

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
