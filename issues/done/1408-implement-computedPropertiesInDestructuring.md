---
id: 1408
title: "Implement Computedpropertiesindestructuring"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5001]
blocks: [5297]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Triage computedPropertiesInDestructuring across 4 failing reference test cases
and split the current first blocker into implementation-ready child issue 5297.

## Problem

Reference test results originally showed 4 cases failing in directory
`computedPropertiesInDestructuring` with diagnostics: destructuring. Fresh
triage shows the representative file now parses computed object binding aliases
but stops at the runtime subset boundary inherited from issue 251.

Problem: `computedPropertiesInDestructuring1.ts` reports
`UnsupportedRuntimeSubset` for `let {[foo]: bar} = {bar: "bar"};`; issue 5297
now owns the actionable fix.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed because the actionable work has been split into
issue 5297. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative evidence in the child issue

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
- [x] At least one child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and compiler/TypeScript evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only
- `cargo nextest run`; issue metadata only

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5297-lower-computed-object-binding-aliases.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts`
- `reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1_ES6.ts`
- `reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring2_ES6.ts`
- `reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring2.ts`

## Duplicate detection

- `issues/done/5180-parse-computed-property-object-binding-patterns.md` is
  related but not a duplicate: it covers parser acceptance for computed binding
  keys. The current representative already has `tokens: ok` and `ast: ok`.
- `issues/done/251-implement-destructuring-binding-runtime-semantics.md`
  completed the supported destructuring subset and intentionally leaves
  unsupported forms behind source-spanned issue-251 diagnostics.
- Broad generated destructuring buckets such as issue 425 are not
  implementation-ready for this exact runtime subset boundary.

Resolution:

```text
Split to issue 5297: lower computed object binding aliases.
```

## Smart triage

### Smart triage: Triage runtime subset: computedPropertiesInDestructuring1

- Issue class: `triage-needed`
- Feature label: `runtime-subset`
- Diagnostic: `UnsupportedRuntimeSubset` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts
```

Failure location:

```text
message: issue-251: object binding aliases must use identifier keys in this runtime slice at 81..113
line: 4, column: 4
```

Source context:

```ts
let foo = "bar";
let {[foo]: bar} = {bar: "bar"};

let {["bar"]: bar2} = {bar: "bar"};
```

Compiler evidence:

```text
tokens: ok; includes LeftBrace LeftBracket Ident("foo") RightBracket Colon Ident("bar")
ast: ok; computed binding key and target are represented in the binding name
resolved: UnsupportedRuntimeSubset issue-251 object binding aliases must use identifier keys
```

TypeScript oracle evidence:

```text
TS2537 diagnostics for invalid index signatures are reported after TypeScript
accepts the computed object binding syntax.
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=destructuring:1
semantic_enabled=0
```

## Completion evidence

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts
result: pass; reproduces issue-251 computed object binding alias runtime subset boundary
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesInDestructuring1.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, blocked=0
date: 2026-05-07
```

Remaining risks:

- After issue 5297 advances the first declaration shape, the same reference
  family is expected to expose string-literal computed keys, call-expression
  computed keys, nested patterns, parameters, and destructuring assignment
  variants as separate blockers.
