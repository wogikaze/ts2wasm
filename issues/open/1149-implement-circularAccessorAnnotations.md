---
id: 1149
title: "Implement Circularaccessorannotations"
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
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1149.

## Summary

Triage circularAccessorAnnotations across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory `circularAccessorAnnotations` with diagnostics: parser-syntax. Fresh coverage and triage now show `ts2wasm build succeeded`; the original parser-syntax blocker is stale.

Problem: `circularAccessorAnnotations` no longer has a current compiler build blocker in this runner view. The TypeScript oracle still reports TS2502 circular accessor annotation diagnostics, but semantic parity is not enabled for this coverage window and is not the generated parser-syntax blocker.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularAccessorAnnotations.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularAccessorAnnotations.ts --detail
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularAccessorAnnotations.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularAccessorAnnotations.ts
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

- [x] none; current compiler build has no parser-syntax blocker on this path

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularAccessorAnnotations.ts`

## Duplicate detection

No exact open implementation issue was found for the original parser-syntax
blocker because that blocker no longer reproduces. Related accessor and
parser-syntax buckets are not matches for the current state:

- `issues/open/2003-implement-divergentAccessorsTypes-parser-syntax.md`
  is a different reference path and still needs its own triage.
- `issues/open/2608-implement-getterSetterSubtypeAssignment.md` is a
  different reference path and still needs its own triage.

Resolution:

```text
The original parser-syntax blocker is stale. The reference window now reports build_pass with semantic checking disabled, so no implementation-ready blocker is split from this generated bucket.
```

## Smart triage

### Smart triage: Build pass: circularAccessorAnnotations

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/circularAccessorAnnotations.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularAccessorAnnotations.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularAccessorAnnotations.ts --detail --no-dashboard-data
```

Source context:

```ts
declare const c1: {
    get foo(): typeof c1.foo;
}

declare const c2: {
    set foo(value: typeof c2.foo);
}

type T1 = {
    get foo(): T1["foo"];
}
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
tokens: ok; includes declare const, getter/setter signatures, typeof property references, and type aliases
ast: ok; ambient value declarations for c1, c2, and c3 are erased
resolved: ok; AmbientValue("c1"), AmbientValue("c2"), AmbientValue("c3")
```

TypeScript oracle evidence:

```text
TS2502: 'foo' is referenced directly or indirectly in its own type annotation.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularAccessorAnnotations.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, build_pass=1, unsupported=0
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularAccessorAnnotations.ts
result: pass; build succeeded and original parser-syntax blocker is stale
date: 2026-05-06
```

Remaining risks:

- TypeScript still reports TS2502 circular accessor annotation diagnostics; this is a future semantic-diagnostic parity gap, not the generated parser-syntax blocker closed here.
