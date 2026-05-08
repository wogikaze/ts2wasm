---
id: 1134
title: "Implement Checkinheritedproperty"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1134.

## Summary

Triage checkInheritedProperty across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory `checkInheritedProperty` with diagnostics: parser-syntax. Fresh coverage and triage now show `ts2wasm build succeeded`; the original parser-syntax blocker is stale.

Problem: `checkInheritedProperty` no longer has a current compiler blocker in this runner view. The TypeScript oracle still reports TS2729 property-initialization diagnostics, but semantic parity is not enabled for this coverage window and is not the generated parser-syntax blocker.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkInheritedProperty.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkInheritedProperty.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkInheritedProperty.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkInheritedProperty.ts
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

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkInheritedProperty.ts`

## Duplicate detection

- `issues/open/200-implement-parser-syntax.md` is related only as historical parser-syntax cleanup; the current TSC path now builds.

## Smart triage

### Smart triage: Build pass: checkInheritedProperty

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/checkInheritedProperty.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkInheritedProperty.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkInheritedProperty.ts --detail --no-dashboard-data
```

Source context:

```text
class Base {
}

declare const BaseFactory: new() => Base & { c: string }

class Derived extends BaseFactory {
    a = this.b
    b = "abc"
}
```

Current compiler result:

```text
ts2wasm build succeeded
```

Compiler evidence:

- Tokens succeed for `declare const BaseFactory: new() => Base & { c: string }`, `class Derived extends BaseFactory`, and the class fields.
- AST succeeds with `ClassDecl Base`, `AmbientValueDecl BaseFactory`, and `ClassDecl Derived extends Ident BaseFactory`.
- Resolved dump succeeds with `AmbientValue("BaseFactory")` and `ClassDecl Derived extends Some("BaseFactory")`.

TypeScript oracle evidence:

```text
TS2729: Property 'b' is used before its initialization.
```

Resolution:

```text
The original parser-syntax blocker is stale. The reference window now reports build_pass with semantic checking disabled, so no implementation-ready blocker is split from this generated bucket.
```

## Completion evidence

Fill only when moving to `done/`.

checkInheritedProperty triage is complete. The current compiler result is
build_pass, so this generated parser-syntax bucket is stale.

Commits:

- stale build-pass cleanup

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkInheritedProperty.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, build_pass=1, unsupported=0
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkInheritedProperty.ts
result: pass; build succeeded and original parser-syntax blocker is stale
date: 2026-05-06
```

Remaining risks:

- none
