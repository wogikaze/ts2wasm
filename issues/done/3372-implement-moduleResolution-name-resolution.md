---
id: 3372
title: "Implement Moduleresolution Name Resolution (audit reopened #3372)"
type: maintenance
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5005, 5161]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as superseded by
`issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`.
Fresh triage for `moduleResolution_automaticTypeDirectiveNames.ts` reaches the
existing declaration-only ambient `declare const` value visibility boundary.

## Problem

Reference test results show 1 case failing in directory
`moduleResolution-name-resolution` with diagnostics: name-resolution. Fresh
triage shows the current blocker is:

```text
UnresolvedName: unresolved name: `a` at 233..234
```

Problem: the parser erases `declare const a` declarations from the executable
AST, so name resolution cannot see the ambient value for the later `a;`
expression. This duplicates issue 5161.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolution_automaticTypeDirectiveNames.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_automaticTypeDirectiveNames.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5161
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closure and owner note

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
- [x] Existing issue 5161 owns resolver-visible metadata for `declare const` values
- [x] This closure includes failing path, diagnostic code, source context, visible symbols, parser/TypeScript AST evidence, and exact diagnostic/stdout change
- [x] No child issue is needed because the current blocker is already implementation-ready in issue 5161

## Validation

Required commands for this closure:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_automaticTypeDirectiveNames.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_automaticTypeDirectiveNames.ts
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Not run:

- Cargo gates; no Rust source changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by issue 5161

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolution_automaticTypeDirectiveNames.ts`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (triaged - superseded by test262 metadata issues) (same feature label, title overlap)
- `issues/open/437-implement-name-resolution.md` - Implement name resolution (same feature label, title overlap)
- `issues/open/648-implement-argumentsAsPropertyName-name-resolution.md` - Implement Argumentsaspropertyname Name Resolution (same feature label, title overlap)
- `issues/open/654-implement-argumentsReferenceInConstructor-name-resolution.md` - Implement Argumentsreferenceinconstructor Name Resolution (same feature label, title overlap)
- `issues/open/657-implement-argumentsReferenceInMethod-name-resolution.md` - Implement Argumentsreferenceinmethod Name Resolution (same feature label, title overlap)
- `issues/open/693-implement-arrayToLocaleStringES-name-resolution.md` - Implement Arraytolocalestringes Name Resolution (same feature label, title overlap)
- `issues/open/733-implement-assignmentCompatability-name-resolution.md` - Implement Assignmentcompatability Name Resolution (same feature label, title overlap)

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_automaticTypeDirectiveNames.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_automaticTypeDirectiveNames.ts
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
reference/typescript/tests/cases/compiler/moduleResolution_automaticTypeDirectiveNames.ts: UnresolvedName/name-resolution
```

Current evidence:

```text
tokens: ok; two `declare const a` declarations and later `a;` are tokenized
ast: ok; declaration-only ambient consts are erased, executable AST contains only Expr Ident a
resolved: UnresolvedName for `a` at 233..234
TypeScript oracle: TS2451 duplicate block-scoped variable `a` for the two ambient declarations
```

Superseding issue:

- `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- local closure commit; see git log for this issue file

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_automaticTypeDirectiveNames.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnresolvedName/name-resolution
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_automaticTypeDirectiveNames.ts
result: pass; current blocker is ambient `declare const a` name resolution, superseded by issue 5161
date: 2026-05-08
```

Remaining risks:

- After issue 5161 lands, this reference may expose duplicate ambient
  declaration diagnostics, automatic type directive semantics, or deeper
  module-resolution behavior.
## Close note

Superseded by issue 5161, which owns declaration-only ambient values being
visible to name resolution.

superseded-by: 5161

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/3372-implement-moduleResolution-name-resolution.md` before this move
- `issues/done/3372-implement-moduleResolution-name-resolution.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
