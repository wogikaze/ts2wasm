---
id: 5306
title: "Report export assignment with other exports"
type: bug
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Add the narrow front-end diagnostic for `export = expr;` when the same source
file also contains another exported declaration. This is a diagnostic boundary,
not CommonJS module support.

## Problem

`reference/typescript/tests/cases/compiler/ExportAssignment8.ts` is a minimal
case where TypeScript reports TS2309, but ts2wasm stops earlier with the generic
issue-055 static export boundary.

Problem: `ExportAssignment8.ts` stops at generic issue-055 instead of reporting the specific `export =` plus other exports rule.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ExportAssignment8.ts
```

Observed ts2wasm failure:

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 42..48
```

Source context:

```text
1 | // @target: es2015
2 | // @module: commonjs
3 | export = B;
4 |
5 | export class C {
6 | }
```

TypeScript oracle reports:

```text
TS2309: An export assignment cannot be used in a module with other exported elements.
topLevel: ExportAssignment "export = B;"
topLevel: ClassDeclaration "export class C { }"
```

## Desired final state

The front-end recognizes this `export =` shape and emits a specific diagnostic
for mixing export assignments with other exports before module graph support is
needed.

## Scope

In scope:

- [ ] Recognize top-level `export = expr;` as a diagnostic target.
- [ ] Detect one additional exported declaration in the same source file.
- [ ] Add a focused fixture or parser/front-end test for this exact pattern.

Out of scope:

- CommonJS module loading.
- Runtime lowering for module exports.
- Follow-up diagnostics revealed in larger lodash namespace tests.

## Affected paths

Expected:

- `crates/frontend/src/`
- `fixtures/`

Do not touch:

- unrelated backend/runtime code

## Acceptance criteria

- [ ] `ExportAssignment8.ts` no longer reports generic `issue-055: unsupported static export` as its first blocker.
- [ ] The replacement diagnostic names the invalid `export =` plus other exported declaration rule and preserves the `export = B;` span.
- [ ] Focused regression coverage covers `export = B;` followed by `export class C {}`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/ExportAssignment8.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated buckets:

- `issues/open/548-implement-ExportAssignment.md`
- `issues/open/1435-implement-conflictingDeclarationsImportFromNamespace.md`

After this focused diagnostic lands, re-triage the larger namespace bucket to
decide the next blocker.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
