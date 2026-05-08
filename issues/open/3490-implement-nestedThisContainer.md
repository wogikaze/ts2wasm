---
id: 3490
title: "Implement Nestedthiscontainer"
type: spike
area: reference/triage
class: done
priority: P2
depends_on: []
blocks: [5218]
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nestedThisContainer across 1 failing reference test cases and split this bucket into implementation-ready child issues.

Closed as superseded by
`issues/open/5218a-support-nested-function-closures-capturing-this.md`. Fresh
triage shows this generated runtime-subset bucket reaches the same issue-062e
nested-function `this` closure boundary for a property-assigned function.

## Problem

Reference test results show 1 cases fail in directory `nestedThisContainer` with diagnostics: runtime-subset. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: nestedThisContainer has 1 current reference failure, but the blocker
is already represented by issue 5218 rather than needing a new generated-bucket
implementation issue.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedThisContainer.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedThisContainer.ts --detail
```

## Desired final state

This generated bucket is closed. Implementation should proceed through
`issues/open/5218a-support-nested-function-closures-capturing-this.md`.

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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Superseding issue contains an exact reference-triage command for the
  issue-062e nested-function `this` closure family
- [x] This issue includes failing path, diagnostic code, source context,
  visible symbols, parser evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nestedThisContainer.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nestedThisContainer.ts
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

- [x] superseded by: `issues/open/5218a-support-nested-function-closures-capturing-this.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nestedThisContainer.ts`

## Duplicate detection

- `issues/open/5218a-support-nested-function-closures-capturing-this.md` owns the
  current issue-062e nested-function `this` capture runtime boundary. Its
  existing representatives cover object-literal property functions and
  contextual/object-literal receivers; this bucket adds the property-assignment
  function variant.
- `issues/open/5179-report-implicit-this-before-closure-runtime-guard.md` is
  related but not exact. That issue handles TypeScript implicit-`this`
  diagnostics before the runtime guard; `nestedThisContainer.ts` has no
  TypeScript oracle diagnostics.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage runtime subset: nestedThisContainer

- Issue class: triage-needed
- Feature label: runtime-subset
- Diagnostic: UnsupportedRuntimeSubset / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/nestedThisContainer.ts
```

Current compiler diagnostic:

```text
UnsupportedRuntimeSubset: issue-062e: nested function `` closures with `this` or `arguments` are not supported in this slice
```

Focused coverage:

```text
suite=tsc
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0

reference/typescript/tests/cases/compiler/nestedThisContainer.ts: UnsupportedSyntax: unknown-unsupported
```

Source context:

```ts
type Foo = any;

const foo: Foo = {};

foo.bar = function () {
    const self: Foo = this;
};

foo.zab = (function () {
    const self: Foo = this;
});
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "foo",
    "line": 6,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "self",
    "line": 9,
    "column": 5
  },
  {
    "kind": "binding",
    "name": "self",
    "line": 13,
    "column": 5
  }
]
```

Compiler evidence:

```text
tokens: ok; includes type alias erasure, `const foo: Foo = {}`,
property assignments `foo.bar` and `foo.zab`, anonymous function expressions,
and `This` tokens
ast: ok; both assignments are PropertyAssign values containing FunctionExpr
bodies with `const self = this`
resolved/lowered: UnsupportedRuntimeSubset issue-062e nested function closure
with `this` or `arguments`
TypeScript oracle: ok, diagnostics=[]
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nestedThisContainer.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nestedThisContainer.ts
result: pass; issue-062e nested function closure with this/arguments; superseded by issue 5218
date: 2026-05-08
```

Remaining risks:

- Issue 5218 may expose a narrower property-assignment-specific runtime or
  diagnostic blocker after the object-literal representative advances.
