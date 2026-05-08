---
id: 1262
title: "Implement Coandcontravariantinferences Name Resolution"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1262.

## Summary

Triage coAndContraVariantInferences-name-resolution across 1 failing
reference test case and close it as superseded by the existing ambient value
declaration name-resolution issue.

## Problem

Reference test results showed 1 case failing in directory
`coAndContraVariantInferences-name-resolution` with diagnostics:
name-resolution. Fresh triage shows the current first blocker is unresolved
ambient `declare const fn`, not a co/contravariant inference behavior.

Problem: `fn.call(null, x)` fails with `UnresolvedName` for `fn` because
declaration-only ambient values are not yet visible to name resolution.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences8.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences8.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5161 covers the current first blocker
- [x] Supersede this generated bucket without creating a duplicate child
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
- [x] Superseding issue 5161 contains the implementation-ready ambient value declaration scope
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences8.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences8.ts
```

Not run:

- `cargo fmt --all --check`; issue close only, no Rust code changed
- `cargo nextest run`; issue close only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/coAndContraVariantInferences8.ts`

## Duplicate detection

- `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md` - exact owner for `declare const` values being visible to expression name resolution
- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, title overlap)
- `issues/done/437-implement-name-resolution.md` - Implement name resolution (same feature label, title overlap)
- `issues/done/648-implement-argumentsAsPropertyName-name-resolution.md` - Implement Argumentsaspropertyname Name Resolution (same feature label, title overlap)
- `issues/done/654-implement-argumentsReferenceInConstructor-name-resolution.md` - Implement Argumentsreferenceinconstructor Name Resolution (same feature label, title overlap)
- `issues/done/657-implement-argumentsReferenceInMethod-name-resolution.md` - Implement Argumentsreferenceinmethod Name Resolution (same feature label, title overlap)
- `issues/done/693-implement-arrayToLocaleStringES-name-resolution.md` - Implement Arraytolocalestringes Name Resolution (same feature label, title overlap)
- `issues/done/733-implement-assignmentCompatability-name-resolution.md` - Implement Assignmentcompatability Name Resolution (same feature label, title overlap)

## Smart triage

Fresh triage shows this generated co/contravariant inference bucket is
currently blocked by ambient value declaration name resolution.

### Smart triage: coAndContraVariantInferences8

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Current compiler message: `unresolved name: fn at 216..218`
- Path: `reference/typescript/tests/cases/compiler/coAndContraVariantInferences8.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences8.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences8.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Source context:

```ts
declare const fn: (() => void) | ((a: number) => void);

declare const x: number;
declare const y: any;

fn.call(null, x);
fn.call(null, y);
```

Compiler evidence:

```text
tokens: ok
ast: ok; two call expressions with member callee `fn.call`
resolved: fails resolving ambient declared value `fn`
visible symbols: fn, x, y
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
binding fn type: (() => void) | ((a: number) => void)
```

Superseding owner:

- `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`

## Completion evidence

Commits:

- Superseded by `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences8.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; ambient `declare const fn` unresolved, superseded by issue 5161
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences8.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; UnresolvedName/name-resolution
date: 2026-05-07
```

Remaining risks:

- After issue 5161 lands, this reference may expose the intended call
  compatibility or co/contravariant inference parity.
