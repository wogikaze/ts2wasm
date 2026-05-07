---
id: 1234
title: "Implement Classpropertyinferencefrombroadertypeconst"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Triage classPropertyInferenceFromBroaderTypeConst across 1 failing reference
test case and close it as superseded by the existing ambient value declaration
name-resolution issue.

## Problem

Reference test results previously showed 1 case failing in directory
`classPropertyInferenceFromBroaderTypeConst` with diagnostics: type-system.
Fresh triage shows the current first blocker is unresolved ambient
`declare const c: C`, not class property type inference.

Problem: `expectAB(c.D)` fails with `UnresolvedName` for `c` because
declaration-only ambient values are not yet visible to name resolution.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classPropertyInferenceFromBroaderTypeConst.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classPropertyInferenceFromBroaderTypeConst.ts --detail --no-dashboard-data
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classPropertyInferenceFromBroaderTypeConst.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classPropertyInferenceFromBroaderTypeConst.ts
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

- `reference/typescript/tests/cases/compiler/classPropertyInferenceFromBroaderTypeConst.ts`

## Duplicate detection

- `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md` - exact owner for `declare const` values being visible to expression name resolution
- broad name-resolution buckets are not exact owners for this current first blocker

## Smart triage

Fresh triage shows this generated type-system bucket is currently blocked by
ambient value declaration name resolution.

### Smart triage: classPropertyInferenceFromBroaderTypeConst

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Current compiler message: `unresolved name: c at 449..450`
- Path: `reference/typescript/tests/cases/compiler/classPropertyInferenceFromBroaderTypeConst.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classPropertyInferenceFromBroaderTypeConst.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classPropertyInferenceFromBroaderTypeConst.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Source context:

```ts
class C {
    D = DEFAULT;
}

declare const c: C;
declare function expectAB(x: AB): void;
expectAB(c.D);
c.D = 'B';
```

Compiler evidence:

```text
tokens: ok
ast: ok; Function expectAB and expression expectAB(c.D)
resolved: fails resolving ambient declared value `c`
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
binding c type: C
```

Superseding owner:

- `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`

## Completion evidence

Commits:

- Superseded by `issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classPropertyInferenceFromBroaderTypeConst.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; ambient `declare const c` unresolved, superseded by issue 5161
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classPropertyInferenceFromBroaderTypeConst.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; UnresolvedName/name-resolution
date: 2026-05-07
```

Remaining risks:

- After issue 5161 lands, this reference may expose the intended class property
  inference parity around wider declared const types.
