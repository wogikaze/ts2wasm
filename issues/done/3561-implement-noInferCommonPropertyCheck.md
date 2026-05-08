---
id: 3561
title: "Implement Noinfercommonpropertycheck"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5161]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage noInferCommonPropertyCheck across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows this bucket is not blocked by `NoInfer` inference yet. The
current compiler stops earlier because declaration-only ambient const bindings
are erased and later references such as `partialObj1` are unresolved. That exact
ambient value name-resolution boundary is tracked by issue 5161.

Problem: `noInferCommonPropertyCheck1.ts` is superseded by issue 5161 until
ambient `declare const` values are resolver-visible.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noInferCommonPropertyCheck1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noInferCommonPropertyCheck1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5161
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
- [x] Issue 5161 contains the exact reference path and current diagnostic
- [x] This issue records failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the diagnostic/stdout change required after issue 5161

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noInferCommonPropertyCheck1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noInferCommonPropertyCheck1.ts
```

Not run:

- broad Rust gates; no source implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noInferCommonPropertyCheck1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-08:

- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Message: `unresolved name: \`partialObj1\` at 304..315`
- Source context: `test1(partialObj1, someObj1);`
- Visible symbols before failure include ambient bindings `partialObj1`,
  `partialObj2`, and `someObj1` from preceding `declare const` declarations.
- Tokens and AST succeed. The executable AST contains the three `test*` calls
  and erased ambient declarations; `resolve_names` stops at `partialObj1`.
- TypeScript oracle reports later TS2559 common-property diagnostics for
  `someObj1`, so actual `NoInfer`/common-property checking remains unproven
  until issue 5161 advances past ambient const value resolution.

## Completion evidence

Superseded by issue 5161.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noInferCommonPropertyCheck1.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noInferCommonPropertyCheck1.ts
result: pass; current blocker is ambient `declare const partialObj1` name resolution, superseded by issue 5161
date: 2026-05-08
```

Remaining risks:

- After issue 5161 resolves ambient const values, this path should be retriaged
  for the later TS2559 `NoInfer` common-property diagnostics.
