---
id: 3348
title: "Implement Modulememberwithouttypeannotation"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as a stale generated bucket. Fresh coverage for both
`moduleMemberWithoutTypeAnnotation` reference files reports `build_pass`, so no
current compiler build blocker remains to split.

## Problem

Reference test results previously showed 2 cases failing in directory
`moduleMemberWithoutTypeAnnotation` with diagnostics: import-export. Fresh
coverage no longer reproduces either build blocker:

```text
reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation2.ts: build_pass
reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation1.ts: build_pass
```

Problem: the generated bucket is stale and should not stay in the blocked queue.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation2.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

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
- [x] Fresh coverage proves both generated failures are no longer current build blockers
- [x] This closed bucket preserves exact reference paths, build-pass triage output, source context, visible symbols, parser token evidence, and TypeScript oracle diagnostics
- [x] No child issue was needed because there is no current build blocker

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation2.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation2.ts`
- `reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh coverage on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation --detail --no-dashboard-data
```

Result:

```text
executed=2
build_pass=2
unsupported=0
reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation2.ts: build_pass
reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation1.ts: build_pass
```

Fresh triage for `moduleMemberWithoutTypeAnnotation1.ts`:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation1.ts
```

Result:

```text
BuildPass: ts2wasm build succeeded
visible symbols include SyntaxCursor, PositionedElement, PositionedToken, SyntaxNode, positionedToken, childIndex, and VariableWidthTokenWithTrailingTrivia
```

TypeScript oracle still reports semantic diagnostics:

```text
TS2322: Type 'null' is not assignable to type 'SyntaxNode'.
TS2322: Type 'null' is not assignable to type 'PositionedToken'.
```

Fresh triage for `moduleMemberWithoutTypeAnnotation2.ts`:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation2.ts
```

Result:

```text
BuildPass: ts2wasm build succeeded
visible symbols include diagnosticWriter and Alert
```

TypeScript oracle still reports semantic diagnostics:

```text
TS2339: Property 'Alert' does not exist on type 'never'.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation --detail --no-dashboard-data
result: pass; executed=2, build_pass=2
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation1.ts
result: pass; BuildPass
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleMemberWithoutTypeAnnotation2.ts
result: pass; BuildPass
date: 2026-05-08
```

Remaining risks:

- Semantic parity is still incomplete for TypeScript diagnostics TS2322 and
  TS2339, but those are not current compiler build blockers in this generated
  bucket.
