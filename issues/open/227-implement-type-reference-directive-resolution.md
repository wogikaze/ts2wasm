---
id: 227
title: "Implement type reference directive resolution"
type: feature
area: frontend/semantics
class: design-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Implement or explicitly diagnose triple-slash `/// <reference types="..."/>` directive resolution for TypeScript inputs.

## Problem

The issue 060 tsgo limit-120 classification window found unsupported `processingDiagnostic*` cases that exercise type-reference directive processing in multi-file TypeScript tests. These cases are now classified as `type-directive-resolution` instead of `unknown-unsupported`.

## Desired final state

Type-reference directives are resolved consistently with the selected TypeScript compatibility subset, or unsupported forms produce precise diagnostics tied to this issue.

## Scope

In scope:

- [ ] Recognize triple-slash `reference types` directives in TypeScript sources.
- [ ] Decide whether the current compiler should resolve referenced type packages or emit a precise unsupported diagnostic.
- [ ] Preserve `skipLibCheck` and `@ts-ignore` handling expectations where they affect directive diagnostics.
- [ ] Add regression fixtures for resolved, missing, ignored, and skipped type-reference directive cases.

Out of scope:

- [ ] Full package manager integration.
- [ ] Declaration emit beyond behavior needed for directive diagnostics.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/compiler/src/`
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] The classified tsgo processing-diagnostic cases no longer report `type-directive-resolution`.
- [ ] Unsupported type-reference directive forms, if any remain, have precise issue-linked diagnostics.
- [ ] Regression fixtures cover missing type directives, `skipLibCheck`, and `@ts-ignore` interactions.
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --limit 120
```

Not run:

- none

## Notes

Created from issue 060 classification evidence on 2026-04-28.

Reference-backed affected files in the limit-120 window:

- `reference/typescript-go/testdata/tests/cases/compiler/processingDiagnostic.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/processingDiagnosticSkipLibCheck.ts`
- `reference/typescript-go/testdata/tests/cases/compiler/processingDiagnosticTsIgnore.ts`

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
