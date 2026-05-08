---
id: 1167
title: "Implement Circulartypeofwithfunctionmodule"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5244]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1167.

## Summary

Triage circularTypeofWithFunctionModule across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `circularTypeofWithFunctionModule` with diagnostics: import-export. Fresh triage shows parsing succeeds through `return maker.Bar`; the current first blocker is lowering treating `maker.Bar` as unsupported function metadata instead of a namespace-merged static property.

Problem: `circularTypeofWithFunctionModule.ts` is not a standalone generated import/export bucket in the current runner view. The actionable first blocker is namespace/function merge static property access, split to issue 5244.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts --detail
```

## Desired final state

This generated bucket is closed after splitting `issues/done/5244-date-timezone-formatting-policy.md`. Do not implement directly from this bucket.

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

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5244-date-timezone-formatting-policy.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts`

## Duplicate detection

- `issues/done/062f-function-object-metadata.md` owns the supported `name` and `length` metadata subset and explicit unsupported metadata diagnostics; it does not implement namespace-merged function static properties.
- `issues/done/1212-implement-classFunctionMerging-import-export.md` is a related generated bucket, but fresh triage shows it is now a build pass, not a focused implementation-ready owner.
- Broad import/export buckets are not exact owners for the current lowering diagnostic.

## Smart triage

Fresh triage shows this generated import/export bucket is currently blocked by
function/namespace merge static property lowering.

### Smart triage: circularTypeofWithFunctionModule

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `issue-062f: function maker metadata property Bar is not supported`
- Path: `reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Source context:

```ts
class Foo {}

function maker(value: string): typeof maker.Bar {
    return maker.Bar;
}

namespace maker {
    export class Bar extends Foo {}
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; ClassDecl Foo, Function maker returning Member(Ident maker, Bar)
resolved/lowered: issue-062f function `maker` metadata property `Bar` is not supported
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
function maker type: typeof Bar
```

Split result:

- `issues/done/5244-date-timezone-formatting-policy.md`

## Completion evidence

Fill only when moving to `done/`.

The `circularTypeofWithFunctionModule` bucket is complete. The current failure is split to issue 5244.

Commits:

- split to `issues/done/5244-date-timezone-formatting-policy.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax/import-export
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularTypeofWithFunctionModule.ts
result: pass; lower_program reports unsupported function metadata property `Bar`, split to issue 5244
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5244 may expose later namespace/type-space parity around `typeof maker.Bar`.
