---
id: 1160
title: "Implement Circularobjectliteralaccessors"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Triage circularObjectLiteralAccessors across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory `circularObjectLiteralAccessors` with diagnostics: class-accessor. Fresh coverage and triage now show `ts2wasm build succeeded`; the original class-accessor blocker is stale.

Problem: `circularObjectLiteralAccessors.ts` no longer has a current compiler build blocker in this runner view. TypeScript also reports no diagnostics.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularObjectLiteralAccessors.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularObjectLiteralAccessors.ts --detail
```

## Desired final state

This generated bucket is closed as stale because the representative path now reports `build_pass`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as stale build-pass instead of creating a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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

- [x] Duplicate candidates below are confirmed as no-match
- [x] This closed issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] This closed issue includes path, build-pass status, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularObjectLiteralAccessors.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularObjectLiteralAccessors.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current compiler build has no class-accessor blocker on this path

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularObjectLiteralAccessors.ts`

## Duplicate detection

No exact implementation child is created because the original class-accessor
blocker no longer reproduces. Fresh triage only found this generated bucket as
a same-path candidate.

## Smart triage

### Smart triage: Build pass

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/circularObjectLiteralAccessors.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularObjectLiteralAccessors.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularObjectLiteralAccessors.ts --detail --no-dashboard-data
```

Source context:

```ts
const a = {
    b: {
        get foo(): string {
            return a.foo;
        },
        set foo(value: string) {
            a.foo = value;
        }
    },
    foo: ''
};
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
unsupported_features=
```

Compiler evidence:

```text
tokens: ok; includes object literal get/set accessors
ast: ok; nested object has FunctionExpr names "get foo" and "set foo"
resolved: ok; getter returns a.foo and setter assigns a.foo = value
visible symbols before failure: binding a
```

TypeScript oracle evidence:

```text
TypeScript reports ok with no diagnostics.
Oracle type for a is { b: { foo: string; }; foo: string; }.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularObjectLiteralAccessors.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, build_pass=1, unsupported=0
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularObjectLiteralAccessors.ts
result: pass; build succeeded and original class-accessor blocker is stale
date: 2026-05-06
```

Remaining risks:

- none
