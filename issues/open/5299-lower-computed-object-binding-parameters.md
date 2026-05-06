---
id: 5299
title: "Lower computed object binding parameters"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Support computed object binding aliases in function parameters for the first
static-key arrow-parameter shape:

```ts
const b = ({ [`key`]: renamed }) => renamed;
```

## Problem

Problem: computed object binding aliases in parameters parse, but name
resolution rejects them with the issue-251 runtime-subset boundary.

Current failure: `computerPropertiesInES5ShouldBeTransformed.ts` reports
`UnsupportedRuntimeSubset: object binding aliases must use identifier keys in
this runtime slice at 54..87`.

## Desired final state

The representative arrow parameter binds `renamed` from the `key` property, or
advances to the next narrower unsupported diagnostic in the same reference file.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/computerPropertiesInES5ShouldBeTransformed.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computerPropertiesInES5ShouldBeTransformed.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: executed=1, build_pass=0, unsupported=1, blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
line 3, column 13
failure: issue-251 object binding aliases must use identifier keys at 54..87
```

Compiler evidence:

```text
tokens: ok; LeftParen LeftBrace LeftBracket TemplateLiteral("key") RightBracket Colon Ident("renamed")
ast: ok; ArrowFn param "{[String { value: \"key\" }]: renamed}"
resolved: UnsupportedRuntimeSubset issue-251 object binding aliases must use identifier keys
TypeScript oracle: ok, diagnostics=[]
```

## Scope

In scope:

- [ ] Lower the static template-key parameter binding and add a focused regression.

Out of scope:

- Declaration destructuring, owned by issue 5297.
- Non-static computed keys in parameters.
- Destructuring assignment expressions.
- Broad ES5 transform/declaration emit parity.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/` or focused fixtures

Do not touch:

- unrelated module, enum, or class lowering code

## Acceptance criteria

- [ ] The representative triage no longer reports `object binding aliases must use identifier keys` at `54..87`.
- [ ] A focused regression proves ``const b = ({ [`key`]: renamed }) => renamed;`` returns the bound property.
- [ ] Any next blocker in `computerPropertiesInES5ShouldBeTransformed.ts` is recorded here or split if outside this parameter shape.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(destructuring) or test(node_diff)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computerPropertiesInES5ShouldBeTransformed.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computerPropertiesInES5ShouldBeTransformed.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket `issues/done/1415-implement-computerPropertiesInES.md`.

Related but not duplicates:

- Issue 5297 handles declaration binding aliases and explicitly excludes
  parameter binding patterns.
- Issue 251 completed the supported destructuring runtime subset and now emits
  this source-spanned boundary.

## Completion Evidence

Fill only when moving to `done/`.
