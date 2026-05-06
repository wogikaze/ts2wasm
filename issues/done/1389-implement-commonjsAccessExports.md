---
id: 1389
title: "Implement Commonjsaccessexports"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed as superseded by
`issues/done/056-implement-name-resolution.md`.

Fresh focused triage shows `commonjsAccessExports.ts` currently stops at a
genuinely unresolved top-level `exports` identifier. TypeScript also reports
TS2304 `Cannot find name 'exports'` for the same invalid source, so this is not
a standalone CommonJS implementation order.

## Problem

Reference test results originally showed 1 case failing in directory
`commonjsAccessExports` with diagnostics: import-export. Fresh focused triage
on 2026-05-07 reports `UnresolvedName` / `name-resolution` instead.

Problem: `commonjsAccessExports.ts` uses `exports.x` and `exports.Cls` in a
virtual JavaScript section without Node/CommonJS ambient declarations. The
compiler reports `UnresolvedName` for `exports`, matching TypeScript's own
TS2304 diagnostics for the fixture.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonjsAccessExports.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonjsAccessExports.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
commonjsAccessExports.ts: UnresolvedName for `exports`
coverage: executed=1, build_pass=0, unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

TypeScript oracle evidence:

```text
TS2304: Cannot find name 'exports'.
Reported for each `exports` use at lines 8, 9, 14, and 19.
```

## Desired final state

This generated bucket is closed. The current diagnostic is covered by issue 056:
unresolved identifiers should produce `UnresolvedName` only when genuinely
unresolved. If a future coverage policy wants oracle-matching TypeScript
diagnostics to count as semantic pass for invalid source, that should be split
as coverage/oracle accounting work, not CommonJS runtime support.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the completed name-resolution diagnostic issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- CommonJS `exports` ambient/runtime support
- Coverage scoring changes for oracle-matching invalid TypeScript fixtures

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
- [x] Existing issue 056 covers genuinely unresolved identifier diagnostics
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonjsAccessExports.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonjsAccessExports.ts
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

- [x] superseded by: `issues/done/056-implement-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commonjsAccessExports.ts`

## Duplicate detection

- `issues/done/056-implement-name-resolution.md` owns the expected
  `UnresolvedName` diagnostic behavior for genuinely unresolved identifiers.
- `issues/open/064-implement-name-resolution.md` and
  `issues/open/437-implement-name-resolution.md` are broad blocked/generated
  buckets and should not receive this standalone invalid-source case.
- No open CommonJS/module issue matched this current first diagnostic; the
  fixture does not reach CommonJS lowering.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: commonjsAccessExports

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/commonjsAccessExports.ts
```

Source context:

```text
// @target: es2015
// @module: commonjs
// @allowJs: true
// @checkJs: true
// @noEmit: true

// @Filename: /a.js
exports.x = 0;
exports.x;
```

AST evidence:

```text
Expr(PropertyAssign(Ident("exports").x = Number(0)))
Expr(Member(Ident("exports").x))
Expr(PropertyAssign(Ident("exports").Cls = FunctionExpr(...)))
Let(instance = New(Member(Ident("exports").Cls), []))
```

Resolver evidence:

```text
[pipeline] resolve_names
error: [UnresolvedName] unresolved name: `exports`
```

TypeScript oracle evidence:

```text
typescriptVersion: 6.0.3
diagnostics:
- TS2304 Cannot find name 'exports' at line 8, character 1
- TS2304 Cannot find name 'exports' at line 9, character 1
- TS2304 Cannot find name 'exports' at line 14, character 5
- TS2304 Cannot find name 'exports' at line 19, character 22
```

## Completion evidence

Commits:

- superseded by `issues/done/056-implement-name-resolution.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonjsAccessExports.ts
result: pass; reproduced oracle-matching `UnresolvedName` diagnostic for `exports`
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonjsAccessExports.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-07
```

Remaining risks:

- Future coverage accounting may need to classify oracle-matching TypeScript
  diagnostics separately from unsupported feature failures.
