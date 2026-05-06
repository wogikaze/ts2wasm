---
id: 1375
title: "Implement Commentsonobjectliteral Object Literal"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: [5218]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed after fresh triage split the bucket outcomes:

- `commentsOnObjectLiteral4.ts` is stale and now builds successfully.
- `commentsOnObjectLiteral3.ts` is superseded by
  `issues/open/5218-support-nested-function-closures-capturing-this.md`, the
  existing implementation-ready issue for object-literal nested functions that
  hit the issue-062e `this` capture runtime guard.

## Problem

Reference test results originally showed 2 cases failing in directory
`commentsOnObjectLiteral-object-literal` with diagnostics: object-literal.
Fresh focused triage on 2026-05-07 shows object literal syntax parses for both
representatives. One path builds; the other reaches lowering and stops on the
existing nested-function `this` closure boundary.

Problem: the only remaining current blocker in this generated bucket is
`commentsOnObjectLiteral3.ts` hitting `issue-062e` for an object-literal getter
that reads `this.prop`.

## Current failure

Representative reproductions:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral3.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral4.ts
```

Coverage windows:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral3.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral4.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
commentsOnObjectLiteral3.ts: UnsupportedRuntimeSubset; issue-062e nested function `get a` closures with `this` or `arguments`
commentsOnObjectLiteral4.ts: BuildPass; ts2wasm build succeeded
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5218-support-nested-function-closures-capturing-this.md`; the
`commentsOnObjectLiteral4.ts` representative needs no child issue.

## Scope

In scope:

- [x] Inspect the smart triage reports below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede the remaining issue-062e object-literal `this` capture blocker
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
- [x] Superseding issue contains an exact `reference-triage` command for the issue-062e family
- [x] This issue includes failing paths, diagnostic code, source context, visible symbols, parser/TypeScript AST evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference paths and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral3.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral4.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral3.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral4.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/open/5218-support-nested-function-closures-capturing-this.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsOnObjectLiteral3.ts`
- `reference/typescript/tests/cases/compiler/commentsOnObjectLiteral4.ts`

## Duplicate detection

- `issues/open/5218-support-nested-function-closures-capturing-this.md` owns the
  current issue-062e object-literal nested-function `this` capture boundary.
  Its representative is a normal object-literal property function; this bucket
  adds the getter/setter accessor variant as a related representative.
- `issues/open/5179-report-implicit-this-before-closure-runtime-guard.md` is
  related but not exact. That issue handles a TypeScript implicit-`this`
  diagnostic before the runtime guard; `commentsOnObjectLiteral3.ts` has no
  TypeScript oracle diagnostics.
- `commentsOnObjectLiteral4.ts` does not need a child issue because it now
  builds successfully.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage runtime subset: commentsOnObjectLiteral3

- Issue class: triage-needed
- Feature label: runtime-subset
- Diagnostic: UnsupportedRuntimeSubset / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/commentsOnObjectLiteral3.ts
```

Source context:

```ts
var v = {
 prop: 1,
 func: function () {
 },
 func1() {
 },
 get a() { return this.prop; },
 set a(value) { this.prop = value; },
}
```

Compiler evidence:

```text
tokens: ok through object properties, function property, method shorthand, getter, and setter
ast: ok; getter and setter are FunctionExpr values named "get a" and "set a"
resolved/lowered: issue-062e nested function `get a` closures with `this` or `arguments`
TypeScript oracle: ok, diagnostics=[]
```

```text
### Smart triage: Build pass: commentsOnObjectLiteral4

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/commentsOnObjectLiteral4.ts
```

Compiler evidence:

```text
tokens: ok through getter with return type annotation
ast: ok; object property "bar" is FunctionExpr name "get bar"
resolved: ok; Let v = Object([("bar", FunctionExpr ...)])
build: ts2wasm build succeeded
TypeScript oracle: ok, diagnostics=[]
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral3.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsOnObjectLiteral4.ts --detail --no-dashboard-data
result: build_pass=1, unsupported=0, blocked=0
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral3.ts
result: issue-062e nested function `get a` closure with this/arguments; superseded by issue 5218
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsOnObjectLiteral4.ts
result: BuildPass; ts2wasm build succeeded
date: 2026-05-07
```

Remaining risks:

- Issue 5218 may expose a narrower accessor-specific runtime or diagnostic
  blocker after the ordinary object-literal function case advances.
